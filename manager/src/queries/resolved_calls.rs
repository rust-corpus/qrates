//! Report information about calls in our codebase.
//! For trait methods whose receiver is statically known, report this resolved type rather than the trait.

use crate::write_csv;
use corpus_database::tables::Loader;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

/// Gathers data on all calls made.
/// This query reports, for each call, descriptions of:
/// - the receiver of a trait method (if applicable) & its generics
/// - the call's target method & the generics of its type & function
/// - the crate of the call site and of the called function
/// - if applicable, the first macro involved in the call stack that is not from the calling crate
///
/// The produced table is deduplicated, with each row instead saying how many times it occurs.
///
/// As an example, for a call in crate `foo` to `Option::map` mapping an optional string slice (`&str`) to an owned `String`, these would be:
/// - empty strings for the receiver and its generics, since this is not part of a trait.
/// - `core::option::Option<T>::map`, `&str`, and `String, $fn`
/// - `foo` and `core` (the crates involved)
///
/// Note that function references & closures are always reported as `$fn`.
/// Further, and perhaps unexpectedly, the path to the target includes generic parameters, but they are simply what the corresponding `impl` block calls them, not the actual types used---these are found in the type generics (here, `&str` is the value of `T`).
pub fn query(loader: &Loader, report_path: &Path) {
    let call_target = loader.load_thir_exprs_call_const_target_as_map();
    let call_target_self = loader.load_thir_exprs_call_const_target_self_as_map();
    let call_target_desc: HashMap<_, _> = loader
        .load_thir_exprs_call_const_target_desc()
        .iter()
        .map(|(call, desc, function_generics, type_generics)| {
            (call, (desc, function_generics, type_generics))
        })
        .collect();
    let call_target_macro = loader.load_thir_exprs_call_macro_backtrace_as_map();

    let strings = loader.load_strings();
    let def_paths = loader.load_def_paths();
    let crate_names = loader.load_crate_names();

    let type_descriptions: HashMap<_, _> = loader
        .load_type_description()
        .iter()
        .map(|(ty, desc, generics)| (ty, (desc, generics)))
        .collect();

    let exprs_to_body_map = loader.load_thir_exprs_to_thir_body_relation_map();
    let thir_body_to_bodies = loader.load_thir_bodies_relation_map();
    let expr_to_def_path = |expr| {
        let body = exprs_to_body_map.get_unwrap(expr);
        let (_, def_path) = thir_body_to_bodies.get_unwrap(body);
        def_path
    };


    let all_calls = loader.load_thir_exprs_call();
    let all_calls = all_calls.iter().filter_map(
        |(expr, _ty, fun, _unsafety, _abi, _return_ty)| {
            let target = call_target.get(&fun)?; // none for function pointers
            let (target_desc, function_generics, type_generics) = call_target_desc[&fun];

            let def_path = expr_to_def_path(expr);

            let (caller_crate, _, _, _, _) = def_paths.get_unwrap(def_path);
            let caller_crate_name = strings.get_unwrap(crate_names.get_unwrap(caller_crate));
            let (target_crate, _, _, _, _) = def_paths.get_unwrap(*target);
            let target_crate_name = strings.get_unwrap(crate_names.get_unwrap(target_crate));

            let (receiver_name, receiver_generics) = call_target_self.get(&fun).map_or_else(
                || ("".to_string(), "".to_string()),
                |typ| {
                    let (desc, generics) = type_descriptions[typ];
                    (strings.get_unwrap(desc), strings.get_unwrap(generics))
                },
            );

            let macro_path = call_target_macro
                .get(&fun)
                .map_or("".to_string(), |path| strings.get_unwrap(*path));

            Some((
                receiver_name,
                receiver_generics,
                strings.get_unwrap(target_desc),
                strings.get_unwrap(type_generics),
                strings.get_unwrap(function_generics),
                caller_crate_name,
                target_crate_name,
                macro_path,
            ))
        },
    );

    let counts: HashMap<_, i32> = all_calls.fold(HashMap::new(), |mut counts, row| {
        *counts.entry(row).or_insert(0) += 1;
        counts
    });
    let all_calls = counts
        .iter()
        .map(|((a, b, c, d, e, f, g, h), count)| (a, b, c, d, e, f, g, h, count));

    // sort for much better gzip compression
    let all_calls: Vec<_> = all_calls
        .sorted_by_key(|(_, _, target, ..)| target.clone())
        .collect();

    let cross_crate_calls = all_calls
        .iter()
        .filter(|&(_, _, _, _, _, caller_crate, target_crate, _, _)| caller_crate != target_crate);
    write_csv!(report_path, cross_crate_calls);

    write_csv!(report_path, all_calls);
}
