//! Collect information about unsafe types.

use super::utils::GroupByIterator;
use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_database::types;
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::HashSet;
use std::path::Path;

fn report_types_foreign(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let types_foreign = loader.load_types_foreign();
    let types_foreign = types_foreign
        .iter()
        .map(|&(typ, def_path)| (typ, def_path_resolver.resolve(def_path)));
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
    let unsafe_cell_types_relation: Vec<_> = loader
        .load_types_adt_def()
        .iter()
        .flat_map(|&(typ, def_path, _, _, _)| {
            let (_, _, _, _, def_path_summary) = def_paths[def_path];
            if def_path_summary == unsafe_cell_summary_id {
                Some((typ, def_path))
            } else {
                None
            }
        })
        .collect();
    info!(
        "Number of UnsafeCell types: {}",
        unsafe_cell_types_relation.len()
    );
    let def_path_resolver = DefPathResolver::new(loader);
    let unsafe_cell_types = unsafe_cell_types_relation
        .iter()
        .map(|&(typ, def_path)| (typ, def_path_resolver.resolve(def_path)));
    write_csv!(report_path, unsafe_cell_types);
    loader.store_types_unsafe_cell(unsafe_cell_types_relation);
}

fn collect_union_types(loader: &Loader) {
    let union_types: Vec<_> = loader
        .load_types_adt_def()
        .iter()
        .flat_map(|&(typ, def_path, kind, _, _)| {
            if kind == types::AdtKind::Union {
                Some((typ, def_path))
            } else {
                None
            }
        })
        .collect();
    info!("Number of union types: {}", union_types.len());
    loader.store_types_union(union_types);
}

fn collect_unsafe_types(loader: &Loader) {
    let public_visibility = vec![(types::TyVisibility::Public,)];

    let unsafe_types;
    datapond_query! {
        load loader {
            relations(
                types_unsafe_cell, types_union, types_raw_ptr, types_foreign,
                types_adt_field, types_array, types_slice, types_ref, types_tuple_element),
        }
        input public_visibility(visibility: TyVisibility)
        output unsafe_types(typ: Type)
        unsafe_types(typ) :- types_unsafe_cell(.typ=typ).
        unsafe_types(typ) :- types_union(.typ=typ).
        unsafe_types(typ) :- types_raw_ptr(.typ=typ).
        unsafe_types(typ) :- types_foreign(.typ=typ).

        unsafe_types(typ) :-
            public_visibility(visibility),
            unsafe_types(field_type),
            types_adt_field(.adt=typ, .visibility=visibility, .typ=field_type).

        unsafe_types(typ) :-
            unsafe_types(element_type),
            types_array(typ, element_type).
        unsafe_types(typ) :-
            unsafe_types(element_type),
            types_slice(typ, element_type).
        unsafe_types(typ) :-
            unsafe_types(target_type),
            types_ref(typ, target_type, _).

        unsafe_types(typ) :-
            unsafe_types(element_type),
            types_tuple_element(typ, _, element_type).
    }

    info!("Number of unsafe types: {}", unsafe_types.elements.len());
    loader.store_unsafe_types(unsafe_types.elements);
}

fn report_unsafe_type_defs(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let type_kinds = loader.load_type_kinds();
    let unsafe_types: HashSet<_> = loader
        .load_unsafe_types()
        .iter()
        .map(|&(typ,)| typ)
        .collect();
    assert_eq!(unsafe_types.len(), loader.load_unsafe_types().len());
    let type_defs = loader.load_selected_type_defs();
    let unsafe_type_defs = type_defs.iter().flat_map(
        |&(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
            if unsafe_types.contains(&typ) {
                Some((
                    build,
                    build_resolver.resolve(build),
                    item,
                    typ,
                    def_path_resolver.resolve(def_path),
                    &strings[name],
                    visibility.to_string(),
                    &strings[type_kinds[type_kind]],
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
    let unsafe_types: HashSet<_> = loader
        .load_unsafe_types()
        .iter()
        .map(|&(typ,)| typ)
        .collect();
    let safe_wrapper_types: Vec<_> = loader
        .load_types_adt_field()
        .iter()
        .safe_group_by(|&(_field, adt, _index, _def_path, _ident, _visibility, _typ)| adt)
        .into_iter()
        .flat_map(|(key, group)| {
            let mut contains_unsafe_field = false;
            for &(_field, _adt, _index, _def_path, _ident, visibility, typ) in group {
                if unsafe_types.contains(&typ) {
                    contains_unsafe_field = true;
                    if visibility == types::TyVisibility::Public {
                        // Unsafe field is public, the type is not a safe wrapper.
                        return None;
                    }
                }
            }
            if contains_unsafe_field {
                Some((*key,))
            } else {
                None
            }
        })
        .collect();

    info!("Number of safe wrapper types: {}", safe_wrapper_types.len());
    loader.store_safe_wrapper_types(safe_wrapper_types);
}

fn report_safe_wrapper_type_defs(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let type_kinds = loader.load_type_kinds();
    let safe_wrapper_types: HashSet<_> = loader
        .load_safe_wrapper_types()
        .iter()
        .map(|&(typ,)| typ)
        .collect();
    assert_eq!(
        safe_wrapper_types.len(),
        loader.load_safe_wrapper_types().len()
    );
    let type_defs = loader.load_selected_type_defs();
    let safe_wrapper_type_defs = type_defs.iter().flat_map(
        |&(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
            if safe_wrapper_types.contains(&typ) {
                Some((
                    build,
                    build_resolver.resolve(build),
                    item,
                    typ,
                    def_path_resolver.resolve(def_path),
                    &strings[name],
                    visibility.to_string(),
                    &strings[type_kinds[type_kind]],
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
