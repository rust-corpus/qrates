//! Collect information about unsafe types.

use super::utils::GroupByIterator;
use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_database::{types, DiskMap};
use corpus_queries_derive::datapond_query;
use log::{info, warn};
use std::collections::{HashMap, HashSet};
use std::path::Path;

fn report_types_foreign(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let types_foreign = loader.load_types_foreign();
    let types_foreign = types_foreign
        .iter()
        .map(|(typ, def_path)| (typ, def_path_resolver.resolve(def_path)));
    write_csv!(report_path, types_foreign);
}

fn collect_unsafe_cell_types(loader: &Loader, report_path: &Path) {
    let def_paths = loader.load_def_paths();
    let strings = loader.load_strings();
    let Some(unsafe_cell_summary_key) = strings.lookup_str("core.cell.UnsafeCell") else {
        // Provide empty csv and relation
        let unsafe_cell_types: Vec<(types::Type, types::DefPath)> = Vec::new();
        let unsafe_cell_types_relation: Vec<(types::Type, types::DefPath)> = Vec::new();
        write_csv!(report_path, unsafe_cell_types);

        loader.store_types_unsafe_cell(unsafe_cell_types_relation);
        return;
    };
    let unsafe_cell_summary_id = loader
        .load_summary_keys()
        .lookup(&unsafe_cell_summary_key)
        .unwrap();
    let get_unsafe_cell_types_relation = || {
        loader
            .load_iter_types_adt_def()
            .filter_map(|(typ, def_path, _, _, _)| {
                let (_, _, _, _, def_path_summary) = def_paths.get_unwrap(def_path);
                if def_path_summary == unsafe_cell_summary_id {
                    Some((typ, def_path))
                } else {
                    None
                }
            })
    };

    let def_path_resolver = DefPathResolver::new(loader);
    let mut count = 0;
    let unsafe_cell_types = get_unsafe_cell_types_relation().map(|(typ, def_path)| {
        count += 1;
        (typ, def_path_resolver.resolve(def_path))
    });
    write_csv!(report_path, unsafe_cell_types);
    info!("Number of UnsafeCell types: {}", count);
    loader.store_iter_types_unsafe_cell(get_unsafe_cell_types_relation());
}

fn collect_union_types(loader: &Loader) {
    let mut count = 0;
    let union_types = loader
        .load_iter_types_adt_def()
        .filter_map(|(typ, def_path, kind, _, _)| {
            if kind == types::AdtKind::Union {
                count += 1;
                Some((typ, def_path))
            } else {
                None
            }
        });

    loader.store_iter_types_union(union_types);
    info!("Number of union types: {}", count);
}

fn collect_unsafe_types(loader: &Loader) {
    let mut unsafe_types = HashSet::new();
    // base cases
    unsafe_types.extend(loader.load_iter_types_unsafe_cell().map(|(typ, _)| typ));
    unsafe_types.extend(loader.load_iter_types_union().map(|(typ, _)| typ));
    unsafe_types.extend(loader.load_iter_types_raw_ptr().map(|(typ, _, _)| typ));
    unsafe_types.extend(loader.load_iter_types_foreign().map(|(typ, _)| typ));

    // recursive cases
    // need a loop for recursion..
    // TODO: if a list is in reverse parent-child order this causes quadratic explosion
    // ^fix: for each relation, load the full list into memory and remove elements after they're inserted
    // actually, would still be quadratic. need to do a topological sort over all relations simultaneously

    // A way to actually do this properly:
    // for every 'kind' of aggregate type, i.e., types_array, types_slice, types_adt, ..., make them a relation map.
    // then store a DiskMap<type, bool> is_unsafe_type, and recurse down on all types applying memoization with the is_unsafe_type diskmap.
    let mut modified = true;
    while modified {
        modified = false;
        for (field, adt, index, def_path, ident, visibility, typ) in
            loader.load_iter_types_adt_field()
        {
            if visibility == types::TyVisibility::Public && unsafe_types.contains(&typ) {
                modified |= unsafe_types.insert(adt);
            }
        }
        for (typ, element_type) in loader.load_iter_types_array() {
            if unsafe_types.contains(&element_type) {
                modified |= unsafe_types.insert(typ);
            }
        }
        for (typ, element_type) in loader.load_iter_types_slice() {
            if unsafe_types.contains(&element_type) {
                modified |= unsafe_types.insert(typ);
            }
        }
        for (typ, target_type, _) in loader.load_iter_types_ref() {
            if unsafe_types.contains(&target_type) {
                modified |= unsafe_types.insert(typ);
            }
        }
        for (typ, _, element_type) in loader.load_iter_types_tuple_element() {
            if unsafe_types.contains(&element_type) {
                modified |= unsafe_types.insert(typ);
            }
        }
    }

    info!("Number of unsafe types: {}", unsafe_types.len());
    loader.store_iter_unsafe_types(unsafe_types.into_iter().map(|typ| (typ,)));
}

fn report_unsafe_type_defs(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let type_kinds = loader.load_type_kinds();
    let unsafe_types: HashSet<_> = loader
        .load_unsafe_types()
        .iter()
        .map(|(typ,)| typ)
        .collect();
    assert_eq!(unsafe_types.len(), loader.load_unsafe_types().len());
    let type_defs = loader.load_selected_type_defs();
    let unsafe_type_defs = type_defs.iter().flat_map(
        |(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
            if unsafe_types.contains(&typ) {
                Some((
                    build,
                    build_resolver.resolve(build),
                    item,
                    typ,
                    def_path_resolver.resolve(def_path),
                    strings.get_unwrap(name),
                    visibility.to_string(),
                    strings.get_unwrap(type_kinds.get_unwrap(type_kind)),
                    def_kind.to_string(),
                ))
            } else {
                None
            }
        },
    );
    write_csv!(report_path, unsafe_type_defs);
}

fn collect_safe_wrapper_types(loader: &Loader) {
    let unsafe_types: HashSet<_> = loader.load_iter_unsafe_types().map(|(typ,)| typ).collect();

    // Datapond way: (1.1GB)

    // let safe_wrapper_types: Vec<_> = loader
    //     .load_iter_types_adt_field()
    //     .safe_group_by(|&(_field, adt, _index, _def_path, _ident, _visibility, _typ)| adt)
    //     .into_iter()
    //     .flat_map(|(key, group)| {
    //         let mut contains_unsafe_field = false;
    //         for (_field, _adt, _index, _def_path, _ident, visibility, typ) in group {
    //             if unsafe_types.contains(&typ) {
    //                 contains_unsafe_field = true;
    //                 if visibility == types::TyVisibility::Public {
    //                     // Unsafe field is public, the type is not a safe wrapper.
    //                     return None;
    //                 }
    //             }
    //         }
    //         if contains_unsafe_field {
    //             Some((key,))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // hashmap way: (700MB)
    // let mut adt_to_field_types: HashMap<_, Vec<_>> = HashMap::new();
    // for (field, adt, index, def_path, ident, visibility, typ) in loader.load_iter_types_adt_field() {
    //     adt_to_field_types
    //         .entry(adt)
    //         .or_default()
    //         .push((visibility, typ));
    // }

    // let get_safe_wrapper_types = || {
    //     adt_to_field_types
    //     .iter()
    //         .filter_map(|(adt, fields)| {
    //             let mut contains_unsafe_field = false;
    //             for (visibility, typ) in fields {
    //                 if unsafe_types.contains(&typ) {
    //                     contains_unsafe_field = true;
    //                     if *visibility == types::TyVisibility::Public {
    //                         // Unsafe field is public, the type is not a safe wrapper.
    //                         return None;
    //                     }
    //                 }
    //             }
    //             if contains_unsafe_field {
    //                 Some((*adt,))
    //             } else {
    //                 None
    //             }
    //         })
    // };

    // attempt without hashmap:
    // 1. collect all adt types that have at least one unsafe field
    // 2. in the same pass, collect those which contain a public unsafe field
    // 3. compute the difference from 1 and 2 to get the safe wrapper types
    // - works and validated. ~50MB
    let mut adt_with_unsafe_field = HashSet::new();
    let mut adt_with_public_unsafe_field = HashSet::new();

    for (field, adt, index, def_path, ident, visibility, typ) in loader.load_iter_types_adt_field()
    {
        if unsafe_types.contains(&typ) {
            adt_with_unsafe_field.insert(adt);
            if visibility == types::TyVisibility::Public {
                adt_with_public_unsafe_field.insert(adt);
            }
        }
    }

    let mut count = 0;
    let safe_wrapper_types = adt_with_unsafe_field
        .difference(&adt_with_public_unsafe_field)
        .map(|&typ| {
            count += 1;
            (typ,)
        });

    loader.store_iter_safe_wrapper_types(safe_wrapper_types);
    info!("Number of safe wrapper types: {}", count);
}

fn report_safe_wrapper_type_defs(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let type_kinds = loader.load_type_kinds();
    let safe_wrapper_types: HashSet<_> = loader
        .load_safe_wrapper_types()
        .iter()
        .map(|(typ,)| typ)
        .collect();
    assert_eq!(
        safe_wrapper_types.len(),
        loader.load_safe_wrapper_types().len()
    );
    let type_defs = loader.load_selected_type_defs();
    let safe_wrapper_type_defs = type_defs.iter().flat_map(
        |(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
            if safe_wrapper_types.contains(&typ) {
                Some((
                    build,
                    build_resolver.resolve(build),
                    item,
                    typ,
                    def_path_resolver.resolve(def_path),
                    strings.get_unwrap(name),
                    visibility.to_string(),
                    strings.get_unwrap(type_kinds.get_unwrap(type_kind)),
                    def_kind.to_string(),
                ))
            } else {
                None
            }
        },
    );
    write_csv!(report_path, safe_wrapper_type_defs);
}

/// Find all types that have fields whose types are “unsafe”: `UnsafeCell`, raw pointers, unions.
pub fn query(loader: &Loader, report_path: &Path) {
    report_types_foreign(loader, report_path);
    collect_unsafe_cell_types(loader, report_path);
    collect_union_types(loader);
    collect_unsafe_types(loader);
    report_unsafe_type_defs(loader, report_path);
    collect_safe_wrapper_types(loader);
    report_safe_wrapper_type_defs(loader, report_path);
}
