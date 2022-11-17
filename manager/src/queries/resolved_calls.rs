//! Report information about calls in our codebase.
//! For trait methods whose receiver is statically known, report this resolved type rather than the trait.

use crate::write_csv;
use corpus_database::tables::Loader;
use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let call_target = loader.load_terminators_call_const_target_as_map();
    let call_target_self = loader.load_terminators_call_const_target_self_as_map();
    let call_target_desc: HashMap<_, _> = loader
        .load_terminators_call_const_target_desc()
        .iter()
        .copied()
        .map(|(call, desc, function_generics, type_generics)| {
            (call, (desc, function_generics, type_generics))
        })
        .collect();

    let strings = loader.load_strings();
    let def_paths = loader.load_def_paths();
    let crate_names = loader.load_crate_names();

    let type_descriptions: HashMap<_, _> = loader
        .load_type_description()
        .iter()
        .copied()
        .map(|(ty, desc, generics)| (ty, (desc, generics)))
        .collect();

    let basic_block_def_paths: HashMap<_, _> = loader
        .load_basic_blocks()
        .iter()
        .map(|&(bb, def_path, _kind)| (bb, def_path))
        .collect();

    let all_calls = loader.load_terminators_call();
    let all_calls = all_calls.iter().filter_map(
        |&(block, call, _func, _unsafety, _abi, _return_ty, _destination, _cleanup, _span)| {
            let target = call_target.get(&call)?; // none for function pointers
            let (target_desc, function_generics, type_generics) = call_target_desc[&call];

            let (caller_crate, _, _, _, _) = def_paths[basic_block_def_paths[&block]];
            let caller_crate_name = &strings[crate_names[caller_crate]];
            let (target_crate, _, _, _, _) = def_paths[*target];
            let target_crate_name = &strings[crate_names[target_crate]];

            let (receiver_name, receiver_generics) = call_target_self.get(&call).map_or_else(
                || ("", ""),
                |typ| {
                    let (desc, generics) = type_descriptions[typ];
                    (&strings[desc], &strings[generics])
                },
            );

            Some((
                receiver_name,
                receiver_generics,
                &strings[target_desc],
                &strings[type_generics],
                &strings[function_generics],
                caller_crate_name,
                target_crate_name,
            ))
        },
    );

    let counts: HashMap<_, i32> = all_calls.fold(HashMap::new(), |mut counts, row| {
        *counts.entry(row).or_insert(0) += 1;
        counts
    });
    let all_calls = counts
        .iter()
        .map(|((a, b, c, d, e, f, g), count)| (a, b, c, d, e, f, g, count));

    // sort for much better gzip compression
    let all_calls: Vec<_> = all_calls
        .sorted_by_key(|(_, _, target, ..)| target.clone())
        .collect();

    let cross_crate_calls = all_calls
        .iter()
        .filter(|&(_, _, _, _, _, caller_crate, target_crate, _)| caller_crate != target_crate);
    write_csv!(report_path, cross_crate_calls);

    write_csv!(report_path, all_calls);
}
