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
    let mut seen_scopes = HashSet::new();
    let build_resolver = BuildResolver::new(loader);
    let unsafe_statements = loader.load_unsafe_statements();
    let unsafe_blocks_by_stmts = unsafe_statements.iter().safe_group_by(
        |(build, _stmt, _block, _index, _kind, unsafe_scope, check_mode)| {
            (build, unsafe_scope, check_mode)
        },
    );
    let unsafe_blocks_sizes_by_stmts: Vec<_> = unsafe_blocks_by_stmts
        .into_iter()
        .map(|((build, unsafe_scope, check_mode), group)| {
            assert!(
                !seen_scopes.contains(unsafe_scope),
                "duplicate scope: {:?} {:?}",
                build,
                unsafe_scope
            );
            seen_scopes.insert(unsafe_scope);
            (*build, unsafe_scope, check_mode, group.count())
        })
        .collect();

    let unsafe_terminators = loader.load_unsafe_terminators();
    let unsafe_blocks_by_terminators = unsafe_terminators.iter().safe_group_by(
        |(build, _block, _kind, unsafe_scope, check_mode)| (build, unsafe_scope, check_mode),
    );
    let unsafe_blocks_sizes_by_terminators: HashMap<_, _> = unsafe_blocks_by_terminators
        .into_iter()
        .map(|((build, unsafe_scope, check_mode), group)| {
            ((*build, unsafe_scope, check_mode), group.count())
        })
        .collect();

    let mut unsafe_block_sizes: Vec<_> = unsafe_blocks_sizes_by_stmts
        .into_iter()
        .map(|(build, unsafe_scope, check_mode, statement_count)| {
            let terminator_count = unsafe_blocks_sizes_by_terminators
                .get(&(build, unsafe_scope, check_mode))
                .cloned()
                .unwrap_or(0);
            (
                build,
                unsafe_scope,
                check_mode,
                statement_count,
                terminator_count,
            )
        })
        .collect();
    for ((build, unsafe_scope, check_mode), terminator_count) in unsafe_blocks_sizes_by_terminators
    {
        if !seen_scopes.contains(unsafe_scope) {
            unsafe_block_sizes.push((build, unsafe_scope, check_mode, 0, terminator_count))
        }
    }

    let unsafe_block_sizes = unsafe_block_sizes.into_iter().map(
        |(build, unsafe_scope, check_mode, statement_count, terminator_count)| {
            (
                build,
                build_resolver.resolve(build),
                unsafe_scope,
                check_mode.to_string(),
                statement_count,
                terminator_count,
            )
        },
    );
    write_csv!(report_path, unsafe_block_sizes);
}

pub fn new_query(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let unsafe_thir_stmts = loader.load_unsafe_thir_stmts();
    let unsafe_thir_blocks_by_stmts = unsafe_thir_stmts
        .iter()
        .safe_group_by(|(build, _stmt, block, _index, check_mode)| (build, block, check_mode));
    let unsafe_thir_blocks_sizes_by_stmts: Vec<_> = unsafe_thir_blocks_by_stmts
        .into_iter()
        .map(|((build, block, check_mode), group)| (*build, block, check_mode, group.count()))
        .collect();

    let unsafe_thir_blocks_sizes_by_stmts_map: HashMap<ThirBlock, usize> =
        unsafe_thir_blocks_sizes_by_stmts
            .iter()
            .map(|(build, block, check_mode, statement_count)| (**block, *statement_count))
            .collect();

    let NO_THIR_EXPR = 0u64.into();

    let closest_unsafe_block_trailing_expr;
    datapond_query! {
        load loader {
            relations(thir_block_expr, thir_exprs),
        }
        output closest_unsafe_block_trailing_expr(
            closest_unsafe_block: ThirBlock,
            expr: ThirExpr,
        )

        closest_unsafe_block_trailing_expr(closest_unsafe_block, expr) :-
            thir_block_expr(.expr=expr),
            thir_exprs(.expr=expr, .closest_unsafe_block=closest_unsafe_block).
    }

    let unsafe_block_to_count_trailing_expr: HashMap<ThirBlock, usize> =
        closest_unsafe_block_trailing_expr
            .elements
            .iter()
            .filter(|(block, expr)| *expr != NO_THIR_EXPR)
            .safe_group_by(|(block, _expr)| *block)
            .into_iter()
            .map(|(block, group)| (block, group.count()))
            .collect();

    let blocks_and_call_exprs;
    datapond_query! {
        load loader {
            relations(thir_exprs, thir_exprs_call),
        }

        output blocks_and_call_exprs(
            block: ThirBlock,
            call_expr: ThirExpr,
        )

        blocks_and_call_exprs(block, expr) :- thir_exprs_call(.expr=expr),
            thir_exprs(.expr=expr, .closest_unsafe_block=block).

    };

    let unsafe_thir_blocks_to_call_expr_count: HashMap<ThirBlock, usize> = blocks_and_call_exprs
        .elements
        .iter()
        .safe_group_by(|(block, _expr)| *block)
        .into_iter()
        .map(|(block, group)| (block, group.count()))
        .collect();

    let unsafe_blocks = loader.load_unsafe_thir_blocks();

    let unsafe_thir_block_sizes = unsafe_blocks.iter().map(
        |(build, _def_path, block, _span_expansion, check_mode, _span)| {
            (
                build,
                build_resolver.resolve(*build),
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
