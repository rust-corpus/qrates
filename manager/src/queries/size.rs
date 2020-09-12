//! Report unsafe block sizes by MIR statements.

use super::utils::BuildResolver;
use super::utils::GroupByIterator;
use crate::write_csv;
use corpus_database::tables::Loader;
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
