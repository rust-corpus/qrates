//! Report unsafe block sizes by MIR statements.

use super::utils::BuildResolver;
use super::utils::GroupByIterator;
use crate::write_csv;
use corpus_database::tables::Loader;
use corpus_database::types::ThirBlock;
use corpus_database::types::ThirExpr;
use corpus_queries_derive::datapond_query;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let mut unsafe_thir_blocks_sizes_by_stmts_map: HashMap<ThirBlock, usize> = HashMap::new();
    for (_build, _stmt, block, _index, _check_mode) in loader.load_iter_unsafe_thir_stmts() {
        let count = unsafe_thir_blocks_sizes_by_stmts_map
            .entry(block)
            .or_insert(0);
        *count += 1;
    }

    let no_thir_expr: ThirExpr = 0u64.into();

    let mut trailing_exprs: HashSet<ThirExpr> = HashSet::new();
    for (_block, expr) in loader.load_iter_thir_block_expr() {
        if expr != no_thir_expr {
            trailing_exprs.insert(expr);
        }
    }
    // let mut thir_exprs_call: HashSet<ThirExpr> = HashSet::new();
    // for (expr, _ty, _fun, _safety, _abi, _retty) in loader.load_iter_thir_exprs_call() {
    //     thir_exprs_call.insert(expr);
    // }
    let thir_exprs_call_map = loader.load_thir_exprs_call_relation_map();

    let mut unsafe_block_to_count_trailing_expr: HashMap<ThirBlock, usize> = HashMap::new();
    let mut unsafe_thir_blocks_to_call_expr_count: HashMap<ThirBlock, usize> = HashMap::new();
    for (expr, _block, closest_unsafe_block, _, _) in loader.load_iter_thir_exprs() {
        if trailing_exprs.contains(&expr) {
            let count = unsafe_block_to_count_trailing_expr
                .entry(closest_unsafe_block)
                .or_insert(0);
            *count += 1;
        }
        // order matters here, we want to only access redb if we already know the expr is a trailing expr
        if thir_exprs_call_map.get(expr).is_some() {
            let count = unsafe_thir_blocks_to_call_expr_count
                .entry(closest_unsafe_block)
                .or_insert(0);
            *count += 1;
        }
    }

    let unsafe_thir_block_sizes = loader.load_iter_unsafe_thir_blocks().map(
        |(build, _def_path, block, _span_expansion, check_mode, _span)| {
            (
                build,
                build_resolver.resolve(build),
                block,
                check_mode.to_string(),
                unsafe_thir_blocks_sizes_by_stmts_map
                    .get(&block)
                    .copied()
                    .unwrap_or(0),
                unsafe_thir_blocks_to_call_expr_count
                    .get(&block)
                    .copied()
                    .unwrap_or(0),
                unsafe_block_to_count_trailing_expr
                    .get(&block)
                    .copied()
                    .unwrap_or(0),
            )
        },
    );

    write_csv!(report_path, unsafe_thir_block_sizes);
}
