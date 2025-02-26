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
    let call_target = loader.load_terminators_call_const_target_as_map();
    let call_target_self = loader.load_terminators_call_const_target_self_as_map();
    let call_target_desc: HashMap<_, _> = loader
        .load_terminators_call_const_target_desc()
        .tuple_iter()
        .map(|(call, desc, function_generics, type_generics)| {
            (call, (desc, function_generics, type_generics))
        })
        .collect();
    let call_target_macro = loader.load_terminators_call_macro_backtrace_as_map();

    let strings = loader.load_strings();
    let def_paths = loader.load_def_paths();
    let crate_names = loader.load_crate_names();

    let type_descriptions: HashMap<_, _> = loader
        .load_type_description()
        .tuple_iter()
        .map(|(ty, desc, generics)| (ty, (desc, generics)))
        .collect();

    let basic_block_def_paths: HashMap<_, _> = loader
        .load_basic_blocks()
        .tuple_iter()
        .map(|(bb, def_path, _kind)| (bb, def_path))
        .collect();

    let all_calls = loader.load_terminators_call();
    let all_calls = all_calls.tuple_iter().filter_map(
        |(block, call, _func, _unsafety, _abi, _return_ty, _destination, _span)| {
            let target = call_target.get(&call)?; // none for function pointers
            let (target_desc, function_generics, type_generics) = call_target_desc[&call];

            let (caller_crate, _, _, _, _) = def_paths.r(basic_block_def_paths[&block]);
            let caller_crate_name = strings.r(crate_names.r(caller_crate));
            let (target_crate, _, _, _, _) = def_paths.r(*target);
            let target_crate_name = strings.r(crate_names.r(target_crate));

            let (receiver_name, receiver_generics) = call_target_self.get(&call).map_or_else(
                || ("".to_string(), "".to_string()),
                |typ| {
                    let (desc, generics) = type_descriptions[typ];
                    (strings.r(desc), strings.r(generics))
                },
            );

            let macro_path = call_target_macro
                .get(&call)
                .map_or("".to_string(), |path| strings.r(*path));

            Some((
                receiver_name,
                receiver_generics,
                strings.r(target_desc),
                strings.r(type_generics),
                strings.r(function_generics),
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
