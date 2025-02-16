//! Report function sizes in MIR statements.

use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::types::ThirExpr;
use corpus_database::{tables::Loader, types};
use corpus_queries_derive::datapond_query;
use std::collections::HashMap;
use std::path::Path;

// Compute the derived relations `selected_function_sizes` and
// `selected_build_sizes`.
fn collect_function_sizes(loader: &Loader) {
    let function_scopes: HashMap<_, _> = {
        let selected_scopes = loader.load_selected_scopes();
        selected_scopes
            .iter()
            .map(
                |&(
                    build,
                    mir_body_def_path,
                    scope,
                    _parent,
                    safety,
                    _explicit_unsafe_group,
                    check_mode,
                    _span,
                )| { (scope, (build, mir_body_def_path, safety, check_mode)) },
            )
            .collect()
    };

    let function_definitions: HashMap<_, _> = {
        let selected_function_definitions = loader.load_selected_function_definitions();
        let selected_function_definitions: HashMap<_, _> = selected_function_definitions
            .iter()
            .map(
                |&(
                    build,
                    item,
                    def_path,
                    module,
                    visibility,
                    unsafety,
                    abi,
                    return_ty,
                    uses_unsafe,
                )| {
                    (
                        item,
                        (
                            build,
                            item,
                            def_path,
                            module,
                            visibility,
                            unsafety,
                            abi,
                            return_ty,
                            uses_unsafe,
                        ),
                    )
                },
            )
            .collect();
        let selected_mir_cfgs = loader.load_selected_mir_cfgs();
        selected_mir_cfgs
            .iter()
            .flat_map(|&(_build, item, body_def_path, _root_scope)| {
                selected_function_definitions
                    .get(&item)
                    .map(|&def| (body_def_path, def))
            })
            .collect()
    };

    let mut selected_function_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
    let mut selected_build_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();

    for (_stmt, _block, _index, _kind, scope) in loader.load_statements().iter() {
        if let Some(&(build, mir_body_def_path, safety, check_mode)) = function_scopes.get(scope) {
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_build_sizes_map.entry(build).or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_function_sizes_map
                        .entry(mir_body_def_path)
                        .or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
        }
    }

    let selected_build_sizes = selected_build_sizes_map
        .into_iter()
        .map(|(build, (stmt, unsafe_stmt, user_unsafe_stmt))| {
            (build, stmt, unsafe_stmt, user_unsafe_stmt)
        })
        .collect();

    loader.store_selected_build_sizes(selected_build_sizes);

    let selected_function_sizes = selected_function_sizes_map
        .into_iter()
        .flat_map(
            |(mir_body_def_path, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                function_definitions.get(&mir_body_def_path).map(
                    |&(
                        build,
                        item,
                        def_path,
                        _module,
                        visibility,
                        unsafety,
                        abi,
                        _return_ty,
                        uses_unsafe,
                    )| {
                        (
                            build,
                            item,
                            def_path,
                            visibility,
                            unsafety,
                            abi,
                            uses_unsafe,
                            stmt,
                            unsafe_stmt,
                            user_unsafe_stmt,
                        )
                    },
                )
            },
        )
        .collect();

    loader.store_selected_function_sizes(selected_function_sizes);
}

fn report_function_sizes(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let def_path_resolver = DefPathResolver::new(loader);
    let abis = loader.load_abis();
    let strings = loader.load_strings();

    let selected_build_sizes = loader.load_selected_build_sizes();

    let selected_build_sizes =
        selected_build_sizes
            .iter()
            .map(|&(build, stmt, unsafe_stmt, user_unsafe_stmt)| {
                (
                    build_resolver.resolve(build),
                    stmt,
                    unsafe_stmt,
                    user_unsafe_stmt,
                )
            });
    write_csv!(report_path, selected_build_sizes);

    let selected_function_sizes = loader.load_selected_function_sizes();
    let selected_function_sizes = selected_function_sizes.iter().map(
        |&(
            build,
            item,
            def_path,
            visibility,
            unsafety,
            abi,
            uses_unsafe,
            stmt,
            unsafe_stmt,
            user_unsafe_stmt,
        )| {
            (
                build_resolver.resolve(build),
                item,
                def_path_resolver.resolve(def_path),
                visibility.to_string(),
                unsafety.to_string(),
                &strings[abis[abi]],
                uses_unsafe,
                stmt,
                unsafe_stmt,
                user_unsafe_stmt,
            )
        },
    );
    write_csv!(report_path, selected_function_sizes);
}

pub fn query(loader: &Loader, report_path: &Path) {
    collect_function_sizes(loader);
    report_function_sizes(loader, report_path);
}

// Compute the derived relations `selected_function_thir_sizes` and
// `selected_build_thir_sizes`.
fn new_collect_function_sizes(loader: &Loader) {
    // block to associated data mapping
    let function_thir_blocks: HashMap<_, _> = {
        let selected_blocks = loader.load_selected_thir_blocks();
        selected_blocks
            .iter()
            .map(
                |&(build, thir_body_def_path, _parent, block, safety, check_mode, _span)| {
                    (block, (build, thir_body_def_path, safety, check_mode))
                },
            )
            .collect()
    };

    let function_definitions: HashMap<_, _> = {
        let selected_function_definitions = loader.load_selected_function_definitions();
        let selected_function_definitions: HashMap<_, _> = selected_function_definitions
            .iter()
            .map(
                |&(
                    build,
                    item,
                    def_path,
                    module,
                    visibility,
                    unsafety,
                    abi,
                    return_ty,
                    uses_unsafe,
                )| {
                    (
                        item,
                        (
                            build,
                            item,
                            def_path,
                            module,
                            visibility,
                            unsafety,
                            abi,
                            return_ty,
                            uses_unsafe,
                        ),
                    )
                },
            )
            .collect();
        let selected_thir_bodies = loader.load_selected_thir_bodies();
        selected_thir_bodies
            .iter()
            .flat_map(|&(_build, item, body_def_path, _root_block)| {
                selected_function_definitions
                    .get(&item)
                    .map(|&def| (body_def_path, def))
            })
            .collect()
    };

    let mut selected_function_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
    let mut selected_build_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();

    for (_stmt, block, closest_unsafe_block, _index) in loader.load_thir_stmts().iter() {
        if let Some(&(build, thir_body_def_path, safety, check_mode)) = function_thir_blocks
            .get(closest_unsafe_block)
            .or(function_thir_blocks.get(block))
        // For sizes of safe blocks
        {
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_build_thir_sizes_map.entry(build).or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_function_thir_sizes_map
                        .entry(thir_body_def_path)
                        .or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
        }
    }

    // count a block's trailing expression as statement as well
    let no_thir_expr: ThirExpr = 0u64.into();
    let thir_block_expr_and_closest_unsafe;
    datapond_query! {
        load loader {
            relations(thir_block_expr, thir_exprs),
        }
        output thir_block_expr_and_closest_unsafe(
            block: ThirBlock,
            expr: ThirExpr,
            closest_unsafe_block: ThirBlock,
        )

        thir_block_expr_and_closest_unsafe(block, expr, closest_unsafe_block) :-
            thir_block_expr(block, expr),
            thir_exprs(.expr=expr, .closest_unsafe_block=closest_unsafe_block).
    }
    for &(block, expr, closest_unsafe_block) in thir_block_expr_and_closest_unsafe.elements.iter() {
        if expr == no_thir_expr {
            continue;
        }

        if let Some(&(build, thir_body_def_path, safety, check_mode)) = function_thir_blocks
            .get(&closest_unsafe_block)
            .or(function_thir_blocks.get(&block))
        {
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_build_thir_sizes_map.entry(build).or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_function_thir_sizes_map
                        .entry(thir_body_def_path)
                        .or_default();
                *build_stmt += 1;
                if safety != types::ScopeSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if safety == types::ScopeSafety::FnUnsafe
                    || check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                {
                    *build_user_unsafe_stmt += 1;
                }
            }
        }
    }

    let selected_build_thir_sizes = selected_build_thir_sizes_map
        .into_iter()
        .map(|(build, (stmt, unsafe_stmt, user_unsafe_stmt))| {
            (build, stmt, unsafe_stmt, user_unsafe_stmt)
        })
        .collect();

    loader.store_selected_build_thir_sizes(selected_build_thir_sizes);

    let selected_function_thir_sizes = selected_function_thir_sizes_map
        .into_iter()
        .flat_map(
            |(thir_body_def_path, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                function_definitions.get(&thir_body_def_path).map(
                    |&(
                        build,
                        item,
                        def_path,
                        _module,
                        visibility,
                        unsafety,
                        abi,
                        _return_ty,
                        uses_unsafe,
                    )| {
                        (
                            build,
                            item,
                            def_path,
                            visibility,
                            unsafety,
                            abi,
                            uses_unsafe,
                            stmt,
                            unsafe_stmt,
                            user_unsafe_stmt,
                        )
                    },
                )
            },
        )
        .collect();

    loader.store_selected_function_thir_sizes(selected_function_thir_sizes);
}

fn new_report_function_sizes(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let def_path_resolver = DefPathResolver::new(loader);
    let abis = loader.load_abis();
    let strings = loader.load_strings();

    let selected_build_thir_sizes = loader.load_selected_build_thir_sizes();

    let selected_build_thir_sizes =
        selected_build_thir_sizes
            .iter()
            .map(|&(build, stmt, unsafe_stmt, user_unsafe_stmt)| {
                (
                    build_resolver.resolve(build),
                    stmt,
                    unsafe_stmt,
                    user_unsafe_stmt,
                )
            });
    write_csv!(report_path, selected_build_thir_sizes);

    let selected_function_thir_sizes = loader.load_selected_function_thir_sizes();
    let selected_function_thir_sizes = selected_function_thir_sizes.iter().map(
        |&(
            build,
            item,
            def_path,
            visibility,
            unsafety,
            abi,
            uses_unsafe,
            stmt,
            unsafe_stmt,
            user_unsafe_stmt,
        )| {
            (
                build_resolver.resolve(build),
                item,
                def_path_resolver.resolve(def_path),
                visibility.to_string(),
                unsafety.to_string(),
                &strings[abis[abi]],
                uses_unsafe,
                stmt,
                unsafe_stmt,
                user_unsafe_stmt,
            )
        },
    );
    write_csv!(report_path, selected_function_thir_sizes);
}

pub fn new_query(loader: &Loader, report_path: &Path) {
    new_collect_function_sizes(loader);
    new_report_function_sizes(loader, report_path);
}
