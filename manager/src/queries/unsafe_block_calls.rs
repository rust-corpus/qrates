//! Report information about calls in our codebase. For calls from unsafe blocks
//! report additional information.

use super::utils::{BuildResolver, SpanResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Report information about calls from unsafe blocks.
fn report_unsafe_block_calls(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let span_resolver = SpanResolver::new(loader);

    let def_paths = loader.load_def_paths();
    let terminators_call_const_target = loader.load_terminators_call_const_target();
    let terminators_call_const_target: HashMap<_, _> =
        terminators_call_const_target.iter().copied().collect();
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
    let subscopes = loader.load_subscopes();
    let scope_spans: HashMap<_, _> = subscopes
        .iter()
        .map(|&(_parent, child, _safety, _check_mode, _explicit_unsafe_group, span)| (child, span))
        .collect();
    let unsafe_block_calls = loader.load_unsafe_block_calls();

    let unsafe_block_calls = unsafe_block_calls.iter().map(
        |&(build, block, unsafe_scope, check_mode, call, unsafety, abi, _return_ty)| {
            let (
                target_crate_name,
                target_crate_hash,
                call_target_def_path,
                call_target,
                is_trait_item,
            ) = if let Some(target) = terminators_call_const_target.get(&call) {
                let (crate_name, crate_hash, relative_def_path, _def_path_hash, summary_key) =
                    def_paths[*target];
                (
                    strings[crate_names[crate_name]].as_ref(),
                    format!("{:x}", crate_hash),
                    strings[relative_def_paths[relative_def_path]].as_ref(),
                    strings[summary_keys[summary_key]].as_ref(),
                    trait_items.contains(target),
                )
            } else {
                (
                    "non-const",
                    "non-const".into(),
                    "non-const",
                    "non-const",
                    false,
                )
            };
            let unsafe_scope_span = scope_spans[&unsafe_scope];
            (
                build,
                build_resolver.resolve(build),
                block,
                unsafe_scope,
                span_resolver.resolve(unsafe_scope_span),
                check_mode.to_string(),
                call,
                unsafety.to_string(),
                strings[abis[abi]].to_string(),
                target_crate_name,
                target_crate_hash,
                call_target_def_path,
                call_target,
                is_trait_item, // Is the call target a trait item?
            )
        },
    );
    write_csv!(report_path, unsafe_block_calls);
}

/// Report information about all calls in our codebase.
fn report_all_calls(loader: &Loader, report_path: &Path) {
    let def_paths = loader.load_def_paths();
    let terminators_call_const_target = loader.load_terminators_call_const_target();
    let terminators_call_const_target: HashMap<_, _> =
        terminators_call_const_target.iter().copied().collect();
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let summary_keys = loader.load_summary_keys();

    let all_calls = loader.load_terminators_call();
    let all_calls = all_calls.iter().map(
        |&(_block, call, func, unsafety, abi, _return_ty, _destination, _cleanup, _span)| {
            let (call_target, is_trait_item) = if let Some(target) =
                terminators_call_const_target.get(&call)
            {
                let (_crate_name, _crate_hash, _relative_def_path, _def_path_hash, summary_key) =
                    def_paths[*target];
                (
                    strings[summary_keys[summary_key]].as_ref(),
                    trait_items.contains(target),
                )
            } else {
                ("non-const", false)
            };
            (
                call,
                func,
                unsafety.to_string(),
                strings[abis[abi]].to_string(),
                call_target,
                is_trait_item,
            )
        },
    );
    write_csv!(report_path, all_calls);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_unsafe_block_calls(loader, report_path);
    report_all_calls(loader, report_path);
}
