//! Report information about function calls.

use super::utils::GroupByIterator;
use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::path::Path;

/// Count how many functions are called from each unsafe block.
fn count_called_functions(loader: &Loader) {
    let unsafe_block_calls;
    datapond_query! {
        load loader {
            relations(unsafe_terminators, terminators_call),
        }
        output unsafe_block_calls(
            build: Build, block: BasicBlock, unsafe_scope: Scope,
            check_mode: BlockCheckMode, call: FunctionCall,
            unsafety: Unsafety, abi: Abi, return_ty: Type)
        unsafe_block_calls(
            build, block, unsafe_scope, check_mode, call, unsafety, abi, return_ty
        ) :-
            unsafe_terminators(
                .build=build, .block=block, .unsafe_scope=unsafe_scope,
                .check_mode=check_mode),
            terminators_call(
                .block=block, .call=call, .unsafety=unsafety,
                .abi=abi, .return_ty=return_ty).
    }
    let unsafe_block_calls_relation = unsafe_block_calls.elements;
    info!(
        "Number of calls in unsafe blocks: {}",
        unsafe_block_calls_relation.len()
    );

    let unsafe_block_call_counts_relation: Vec<_> = unsafe_block_calls_relation
        .iter()
        .safe_group_by(
            |&&(build, _block, unsafe_scope, check_mode, _call, _unsafety, _abi, _return_ty)| {
                (build, unsafe_scope, check_mode)
            },
        )
        .into_iter()
        .map(|((build, unsafe_scope, check_mode), group)| {
            (
                build,
                unsafe_scope,
                check_mode,
                group.count().try_into().unwrap(),
            )
        })
        .collect();
    info!(
        "Number of unsafe blocks with calls: {}",
        unsafe_block_call_counts_relation.len()
    );

    let blocks_with_calls: HashSet<_> = unsafe_block_call_counts_relation
        .iter()
        .map(|&(_build, unsafe_scope, _check_mode, _call_count)| unsafe_scope)
        .collect();

    let unsafe_block_no_calls_relation: Vec<_> = loader
        .load_unsafe_blocks()
        .iter()
        .filter(
            |(_build, _mir_body_def_path, scope, _expansion_kind, _check_mode, _span)| {
                !blocks_with_calls.contains(scope)
            },
        )
        .cloned()
        .collect();
    info!(
        "Number of unsafe blocks with non-const calls: {}",
        unsafe_block_no_calls_relation.len()
    );

    loader.store_unsafe_block_calls(unsafe_block_calls_relation);
    loader.store_unsafe_block_call_counts(unsafe_block_call_counts_relation);
    loader.store_unsafe_block_no_calls(unsafe_block_no_calls_relation);
}

/// Report how many function calls each unsafe block contains.
fn report_called_functions(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();

    let unsafe_block_calls = loader.load_unsafe_block_calls();
    let unsafe_block_calls = unsafe_block_calls.iter().map(
        |&(build, block, unsafe_scope, check_mode, call, unsafety, abi, _return_ty)| {
            (
                build,
                build_resolver.resolve(build),
                block,
                unsafe_scope,
                check_mode.to_string(),
                call,
                unsafety.to_string(),
                strings[abis[abi]].to_string(),
            )
        },
    );
    write_csv!(report_path, unsafe_block_calls);
    info!("reported unsafe_block_calls");

    let unsafe_block_call_counts = loader.load_unsafe_block_call_counts();
    let unsafe_block_call_counts =
        unsafe_block_call_counts
            .iter()
            .map(|&(build, unsafe_scope, check_mode, call_count)| {
                (
                    build,
                    build_resolver.resolve(build),
                    unsafe_scope,
                    check_mode.to_string(),
                    call_count,
                )
            });
    write_csv!(report_path, unsafe_block_call_counts);
    info!("reported unsafe_block_call_counts");

    let unsafe_block_no_calls = loader.load_unsafe_block_no_calls();
    let unsafe_block_no_calls = unsafe_block_no_calls.iter().map(
        |&(build, mir_body_def_path, scope, expansion_kind, check_mode, _span)| {
            (
                build,
                build_resolver.resolve(build),
                def_path_resolver.resolve(mir_body_def_path),
                scope,
                expansion_kind.to_string(),
                check_mode.to_string(),
            )
        },
    );
    write_csv!(report_path, unsafe_block_no_calls);
    info!("reported unsafe_block_no_calls");
}

/// Find all calls in unsafe functions that call non-constant targets. In other
/// words, find all calls that call function pointers.
fn report_non_const_call_targets(loader: &Loader, report_path: &Path) {
    let const_calls: HashSet<_> = loader
        .load_terminators_call_const_target()
        .iter()
        .map(|(call, _def_path)| *call)
        .collect();
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let unsafe_block_calls = loader.load_unsafe_block_calls();
    let non_const_calls = unsafe_block_calls.iter().flat_map(
        |(build, block, unsafe_scope, check_mode, call, unsafety, abi, _return_ty)| {
            if const_calls.contains(call) {
                None
            } else {
                Some((
                    build,
                    build_resolver.resolve(*build),
                    block,
                    unsafe_scope,
                    check_mode.to_string(),
                    call,
                    unsafety.to_string(),
                    strings[abis[*abi]].to_string(),
                ))
            }
        },
    );
    write_csv!(report_path, non_const_calls);
    info!("reported non_const_call_targets");
}

/// Find all calls in unsafe functions that call constant targets. The call
/// targets that appear as constants:
///
/// 1. Static function calls.
/// 2. Static method calls.
/// 3. Dynamic calls on trait objects.
/// 4. Calls of closures.
fn report_const_call_targets(loader: &Loader, report_path: &Path) {
    let const_calls_map: HashMap<_, _> = loader
        .load_terminators_call_const_target()
        .iter()
        .cloned()
        .collect();
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let unsafe_block_calls = loader.load_unsafe_block_calls();
    let const_calls = unsafe_block_calls.iter().flat_map(
        |(build, block, unsafe_scope, check_mode, call, unsafety, abi, _return_ty)| {
            const_calls_map.get(call).map(|def_path| {
                Some((
                    build,
                    build_resolver.resolve(*build),
                    def_path_resolver.resolve(*def_path),
                    block,
                    unsafe_scope,
                    check_mode.to_string(),
                    call,
                    unsafety.to_string(),
                    strings[abis[*abi]].to_string(),
                ))
            })
        },
    );
    write_csv!(report_path, const_calls);
    info!("reported const_call_targets");
}

pub fn query(loader: &Loader, report_path: &Path) {
    count_called_functions(loader);
    report_called_functions(loader, report_path);
    report_non_const_call_targets(loader, report_path);
    report_const_call_targets(loader, report_path);
}
