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
    let terminators_call_const_target = loader.load_terminators_call_const_target_as_map();
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
    let terminators_call_const_target = loader.load_terminators_call_const_target_as_map();
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
        |&(_block, call, func, unsafety, abi, _return_ty, _destination, _span)| {
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

/// Report information about calls from unsafe thir blocks.
fn new_report_unsafe_block_calls(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let span_resolver = SpanResolver::new(loader);

    let def_paths = loader.load_def_paths();
    let fun_to_const_target_map = loader.load_thir_exprs_call_const_target_as_map();
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
    let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();

    let thir_block_to_span: HashMap<_, _> = loader
        .load_thir_blocks()
        .iter()
        .map(|&(_parent, block, _safety, _check_mode, span)| (block, span))
        .collect();

    let unsafe_thir_block_calls = unsafe_thir_block_calls.iter().map(
        |&(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
            let (
                target_crate_name,
                target_crate_hash,
                call_target_def_path,
                call_target,
                is_trait_item,
            ) = if let Some(target) = fun_to_const_target_map.get(&fun) {
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
            (
                build,
                build_resolver.resolve(build),
                block,
                span_resolver.resolve(thir_block_to_span[&block]),
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
    write_csv!(report_path, unsafe_thir_block_calls);
}

/// Report information about all thir calls in our codebase.
fn new_report_all_calls(loader: &Loader, report_path: &Path) {
    let def_paths = loader.load_def_paths();
    let fun_to_const_target_map = loader.load_thir_exprs_call_const_target_as_map();
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let summary_keys = loader.load_summary_keys();

    let all_calls = loader.load_thir_exprs_call();
    let all_thir_calls = all_calls.iter().map(
        |&(call, _fun_type, fun, unsafety, abi, _return_ty)| {
            let (call_target, is_trait_item) = if let Some(target) =
                fun_to_const_target_map.get(&fun)
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
                fun,
                unsafety.to_string(),
                strings[abis[abi]].to_string(),
                call_target,
                is_trait_item,
            )
        },
    );
    write_csv!(report_path, all_thir_calls);
}

pub fn new_query(loader: &Loader, report_path: &Path) {
    new_report_unsafe_block_calls(loader, report_path);
    new_report_all_calls(loader, report_path);
}
