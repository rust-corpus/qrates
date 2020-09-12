//! A query intended to find definitions of potentially non-tree data structures
//! implemented by using unsafe code.

use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_queries_derive::datapond_query;
use std::collections::HashSet;
use std::path::Path;

/// Try to find definitions of potentially non-tree data structures:
/// 1. contain raw pointers as fields;
/// 2. have no attributes, such as `#[repr(C)]`, indicating that the struct is
///    used for FFI.
fn report_non_tree_types(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let selected_adts = loader.load_selected_adts();
    let strings = loader.load_strings();
    let type_kinds = loader.load_type_kinds();

    let non_tree_types;
    datapond_query! {
        load loader {
            relations(types_adt_field, types_raw_ptr),
        }
        output non_tree_types(typ: Type)
        non_tree_types(adt) :-
            types_adt_field(.adt=adt, .typ=typ),
            types_raw_ptr(.typ=typ).
    }

    let non_tree_types: HashSet<_> = non_tree_types.elements.iter().map(|&(typ,)| typ).collect();
    let non_tree_adts = selected_adts.iter().flat_map(
        |&(
            build,
            item,
            typ,
            def_path,
            resolved_def_path,
            name,
            visibility,
            type_kind,
            def_kind,
            kind,
            c_repr,
            is_phantom,
        )| {
            if non_tree_types.contains(&typ) {
                Some((
                    build,
                    build_resolver.resolve(build),
                    item,
                    typ,
                    def_path_resolver.resolve(def_path),
                    def_path_resolver.resolve(resolved_def_path),
                    &strings[name],
                    visibility.to_string(),
                    &strings[type_kinds[type_kind]],
                    def_kind.to_string(),
                    kind.to_string(),
                    c_repr,
                    is_phantom,
                ))
            } else {
                None
            }
        },
    );
    write_csv!(report_path, non_tree_adts);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_non_tree_types(loader, report_path);
}
