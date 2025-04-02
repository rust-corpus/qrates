//! Report information about calls in our codebase. For calls from unsafe blocks
//! report additional information.

use super::utils::{BuildResolver, SpanResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Report information about calls from unsafe thir blocks.
fn report_unsafe_block_calls(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let span_resolver = SpanResolver::new(loader);

    let def_paths = loader.load_def_paths();
    let fun_to_const_target_map = loader.load_thir_exprs_call_const_target_relation_map();
    let crate_names = loader.load_crate_names();
    let relative_def_paths = loader.load_relative_def_paths();
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let summary_keys = loader.load_summary_keys();
    let unsafe_thir_block_calls = loader.load_iter_unsafe_thir_block_calls();

    let thir_block_map = loader.load_thir_blocks_relation_map();
    let thir_block_to_span = |block| {
        let (_parent, _, _, span) = thir_block_map.get(block).unwrap();
        span
    };

    let unsafe_thir_block_calls = unsafe_thir_block_calls.map(
        |(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
            let (
                target_crate_name,
                target_crate_hash,
                call_target_def_path,
                call_target,
                is_trait_item,
            ) = if let Some(target) = fun_to_const_target_map.get(fun) {
                let (crate_name, crate_hash, relative_def_path, _def_path_hash, summary_key) =
                    def_paths.get_unwrap(target);
                (
                    strings.get_unwrap(crate_names.get_unwrap(crate_name)),
                    format!("{:x}", crate_hash),
                    strings.get_unwrap(relative_def_paths.get_unwrap(relative_def_path)),
                    strings.get_unwrap(summary_keys.get_unwrap(summary_key)),
                    trait_items.contains(&target),
                )
            } else {
                (
                    "non-const".into(),
                    "non-const".into(),
                    "non-const".into(),
                    "non-const".into(),
                    false,
                )
            };
            (
                build,
                build_resolver.resolve(build),
                block,
                span_resolver.resolve(thir_block_to_span(block)),
                check_mode.to_string(),
                call,
                unsafety.to_string(),
                strings.get_unwrap(abis.get_unwrap(abi)),
                target_crate_name,
                target_crate_hash,
                call_target_def_path,
                call_target,
                is_trait_item, // Is the call target a trait item?
            )
        },
    );
    write_csv!(report_path, unsafe_thir_block_calls);
}

/// Report information about all thir calls in our codebase.
fn report_all_calls(loader: &Loader, report_path: &Path) {
    let def_paths = loader.load_def_paths();
    let fun_to_const_target_map = loader.load_thir_exprs_call_const_target_relation_map();
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let summary_keys = loader.load_summary_keys();

    let all_calls = loader.load_iter_thir_exprs_call();
    let all_thir_calls = all_calls.map(|(call, _fun_type, fun, unsafety, abi, _return_ty)| {
        let (call_target, is_trait_item) = if let Some(target) = fun_to_const_target_map.get(fun) {
            let (_crate_name, _crate_hash, _relative_def_path, _def_path_hash, summary_key) =
                def_paths.get_unwrap(target);
            (
                strings.get_unwrap(summary_keys.get_unwrap(summary_key)),
                trait_items.contains(&target),
            )
        } else {
            ("non-const".into(), false)
        };
        (
            call,
            fun,
            unsafety.to_string(),
            strings.get_unwrap(abis.get_unwrap(abi)).to_string(),
            call_target,
            is_trait_item,
        )
    });
    write_csv!(report_path, all_thir_calls);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_unsafe_block_calls(loader, report_path);
    report_all_calls(loader, report_path);
}
