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

/// Count how many functions are called from each unsafe thir block.
fn count_called_functions(loader: &Loader) {
    // We join on `closest_unsafe_block`, because we don't want `unsafe { { foo(); } }` to be counted as a safe call.

    // too inefficient.
    /*
    let unsafe_thir_block_calls;
    datapond_query! {
        load loader {
            relations(unsafe_thir_blocks, thir_exprs, thir_exprs_call),
        }
        output unsafe_thir_block_calls(
            build: Build, block: ThirBlock,
            check_mode: BlockCheckMode, call: ThirExpr, fun: ThirExpr,
            unsafety: Safety, abi: Abi, return_ty: Type)
        unsafe_thir_block_calls(
            build, block, check_mode, call, fun, unsafety, abi, return_ty
        ) :-
            unsafe_thir_blocks(
                .build=build, .block=block, .check_mode=check_mode),
            thir_exprs(.closest_unsafe_block=block, .expr=call),
            thir_exprs_call(
                .expr=call, .fun=fun, .unsafety=unsafety,
                .abi=abi, .return_ty=return_ty).
    }
    let unsafe_thir_block_calls_relation = unsafe_thir_block_calls.elements;
    info!(
        "Number of calls in unsafe thir blocks: {}",
        unsafe_thir_block_calls_relation.len()
    );

    let unsafe_thir_block_call_counts_relation: Vec<_> = unsafe_thir_block_calls_relation
        .iter()
        .safe_group_by(
            |&&(build, block, check_mode, _call, _fun, _unsafety, _abi, _return_ty)| {
                (build, block, check_mode)
            },
        )
        .into_iter()
        .map(|((build, block, check_mode), group)| {
            (build, block, check_mode, group.count().try_into().unwrap())
        })
        .collect();

    info!(
        "Number of unsafe thir blocks with calls: {}",
        unsafe_thir_block_call_counts_relation.len()
    );
    */

    let mut unsafe_blocks_to_data = HashMap::new();
    for (build, _, block, _, check_mode, _) in loader.load_iter_unsafe_thir_blocks() {
        unsafe_blocks_to_data.insert(block, (build, check_mode));
    }
    // Uncomment below to be faster but use more memory.
    // let mut expr_to_call_data = HashMap::new();
    // for (call, ty, fun, unsafety, abi, return_ty) in loader.load_iter_thir_exprs_call() {
    //     expr_to_call_data.insert(call, (fun, unsafety, abi, return_ty));
    // }
    let expr_to_call_data = loader.load_thir_exprs_call_relation_map();

    let mut unsafe_thir_block_call_counts_map = HashMap::new();
    for (expr, _, closest_unsafe_block, _, _) in loader.load_iter_thir_exprs() {
        let Some((ty, fun, unsafety, abi, return_ty)) = expr_to_call_data.get(expr) else {
            continue;
        };
        let Some((build, check_mode)) = unsafe_blocks_to_data.get(&closest_unsafe_block) else {
            continue;
        };

        let count = unsafe_thir_block_call_counts_map
            .entry((*build, closest_unsafe_block, *check_mode))
            .or_insert(0);
        *count += 1;
    }

    // same as above, but for storing
    let iter_version =
        loader
            .load_iter_thir_exprs()
            .flat_map(|(expr, _, closest_unsafe_block, _, _)| {
                let (_ty, fun, unsafety, abi, return_ty) = expr_to_call_data.get(expr)?;
                let (build, check_mode) = unsafe_blocks_to_data.get(&closest_unsafe_block)?;
                Some((
                    *build,
                    closest_unsafe_block,
                    *check_mode,
                    expr,
                    fun,
                    unsafety,
                    abi,
                    return_ty,
                ))
            });

    info!(
        "Number of unsafe thir blocks with calls: {}",
        unsafe_thir_block_call_counts_map.len()
    );

    let unsafe_blocks_with_calls: HashSet<_> = unsafe_thir_block_call_counts_map
        .iter()
        .map(|(&(_build, block, _check_mode), _call_count)| block)
        .collect();

    let unsafe_thir_block_no_calls_relation: Vec<_> = loader
        .load_unsafe_thir_blocks()
        .iter()
        .filter(
            |(_build, _thir_body_def_path, block, _expansion_kind, _check_mode, _span)| {
                !unsafe_blocks_with_calls.contains(block)
            },
        )
        .collect();
    info!(
        "Number of unsafe thir blocks with non-const calls: {}",
        unsafe_thir_block_no_calls_relation.len()
    );

    loader.store_iter_unsafe_thir_block_calls(iter_version);
    loader.store_iter_unsafe_thir_block_call_counts(
        unsafe_thir_block_call_counts_map
            .into_iter()
            .map(|((build, block, check_mode), count)| (build, block, check_mode, count)),
    );
    loader.store_unsafe_thir_block_no_calls(unsafe_thir_block_no_calls_relation);
}

/// Report how many function calls each unsafe thir block contains.
fn report_called_functions(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();

    let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
    let unsafe_thir_block_calls = unsafe_thir_block_calls.iter().map(
        |(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
            (
                build,
                build_resolver.resolve(build),
                block,
                check_mode.to_string(),
                call,
                fun,
                unsafety.to_string(),
                strings.get_unwrap(abis.get_unwrap(abi)),
            )
        },
    );
    write_csv!(report_path, unsafe_thir_block_calls);
    info!("reported unsafe_thir_block_calls");

    let unsafe_thir_block_call_counts = loader.load_unsafe_thir_block_call_counts();
    let unsafe_thir_block_call_counts =
        unsafe_thir_block_call_counts
            .iter()
            .map(|(build, block, check_mode, call_count)| {
                (
                    build,
                    build_resolver.resolve(build),
                    block,
                    check_mode.to_string(),
                    call_count,
                )
            });
    write_csv!(report_path, unsafe_thir_block_call_counts);
    info!("reported unsafe_thir_block_call_counts");

    let unsafe_thir_block_no_calls = loader.load_unsafe_thir_block_no_calls();
    let unsafe_thir_block_no_calls = unsafe_thir_block_no_calls.iter().map(
        |(build, thir_body_def_path, block, expansion_kind, check_mode, _span)| {
            (
                build,
                build_resolver.resolve(build),
                def_path_resolver.resolve(thir_body_def_path),
                block,
                expansion_kind.to_string(),
                check_mode.to_string(),
            )
        },
    );
    write_csv!(report_path, unsafe_thir_block_no_calls);
    info!("reported unsafe_thir_block_no_calls");
}

/// Find all thir calls in unsafe functions that call non-constant targets. In other
/// words, find all calls that call function pointers.
fn report_non_const_call_targets(loader: &Loader, report_path: &Path) {
    let const_calls = loader.load_thir_exprs_call_const_target_relation_map();

    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
    let non_const_thir_calls = unsafe_thir_block_calls.iter().flat_map(
        |(build, block, _check_mode, call, fun, unsafety, abi, _return_ty)| {
            if const_calls.get(fun).is_some() {
                None
            } else {
                Some((
                    build,
                    build_resolver.resolve(build),
                    block,
                    call,
                    fun,
                    unsafety.to_string(),
                    strings.get_unwrap(abis.get_unwrap(abi)),
                ))
            }
        },
    );
    write_csv!(report_path, non_const_thir_calls);
}

/// Find all thir calls in unsafe functions that call constant targets. The call
/// targets that appear as constants:
///
/// 1. Static function calls.
/// 2. Static method calls.
/// 3. Dynamic calls on trait objects.
/// 4. Calls of closures.
fn report_const_call_targets(loader: &Loader, report_path: &Path) {
    let const_calls_map = loader.load_thir_exprs_call_const_target_relation_map();
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();
    let abis = loader.load_abis();
    let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
    let const_thir_calls = unsafe_thir_block_calls.iter().flat_map(
        |(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
            const_calls_map.get(fun).map(|def_path| {
                Some((
                    build,
                    build_resolver.resolve(build),
                    def_path_resolver.resolve(def_path),
                    block,
                    check_mode,
                    call,
                    fun,
                    unsafety.to_string(),
                    strings.get_unwrap(abis.get_unwrap(abi)),
                ))
            })
        },
    );
    write_csv!(report_path, const_thir_calls);
}

pub fn query(loader: &Loader, report_path: &Path) {
    count_called_functions(loader);
    report_called_functions(loader, report_path);
    report_non_const_call_targets(loader, report_path);
    report_const_call_targets(loader, report_path);
}
