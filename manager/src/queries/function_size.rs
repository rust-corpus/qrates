//! Report function sizes in MIR statements.

use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::types::ThirExpr;
use corpus_database::{tables::Loader, types};
use corpus_queries_derive::datapond_query;
use std::collections::HashMap;
use std::path::Path;

// Compute the derived relations `selected_function_thir_sizes` and
// `selected_build_thir_sizes`.
fn collect_function_sizes(loader: &Loader) {
    // block to associated data mapping
    let function_thir_blocks: HashMap<_, _> = {
        let selected_blocks = loader.load_iter_selected_thir_blocks();
        selected_blocks
            .map(
                |(build, thir_body_def_path, _parent, block, safety, check_mode, _span)| {
                    (block, (build, thir_body_def_path, safety, check_mode))
                },
            )
            .collect()
    };

    let function_definitions: HashMap<_, _> = {
        let selected_function_definitions = loader.load_iter_selected_function_definitions();
        let selected_function_definitions: HashMap<_, _> = selected_function_definitions
            .map(
                |(
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
        let selected_thir_bodies = loader.load_iter_selected_thir_bodies();
        selected_thir_bodies
            .flat_map(|(_build, item, body_def_path, _root_block)| {
                selected_function_definitions
                    .get(&item)
                    .map(|&def| (body_def_path, def))
            })
            .collect()
    };

    let mut selected_function_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
    let mut selected_build_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();

    for (_stmt, block, closest_unsafe_block, _index) in loader.load_iter_thir_stmts() {
        if let Some(&(build, thir_body_def_path, safety, check_mode)) = function_thir_blocks
            .get(&closest_unsafe_block)
            .or(function_thir_blocks.get(&block))
        // For sizes of safe blocks
        {
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_build_thir_sizes_map.entry(build).or_default();
                *build_stmt += 1;
                if safety != types::BlockSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if check_mode == types::BlockCheckMode::UnsafeBlockUserProvided {
                    *build_user_unsafe_stmt += 1;
                }
            }
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_function_thir_sizes_map
                        .entry(thir_body_def_path)
                        .or_default();
                *build_stmt += 1;
                if safety != types::BlockSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if check_mode == types::BlockCheckMode::UnsafeBlockUserProvided {
                    *build_user_unsafe_stmt += 1;
                }
            }
        }
    }

    // count a block's trailing expression as statement as well
    let no_thir_expr: ThirExpr = 0u64.into();
    // let thir_block_expr_and_closest_unsafe;
    // datapond_query! {
    //     load loader {
    //         relations(thir_block_expr, thir_exprs),
    //     }
    //     output thir_block_expr_and_closest_unsafe(
    //         block: ThirBlock,
    //         expr: ThirExpr,
    //         closest_unsafe_block: ThirBlock,
    //     )

    //     thir_block_expr_and_closest_unsafe(block, expr, closest_unsafe_block) :-
    //         thir_block_expr(block, expr),
    //         thir_exprs(.expr=expr, .closest_unsafe_block=closest_unsafe_block).
    // }

    let thir_trailing_expr_to_block: HashMap<_, _> = loader
        .load_iter_thir_block_expr()
        .flat_map(|(block, expr)| {
            if expr == no_thir_expr {
                None
            } else {
                Some((expr, block))
            }
        })
        .collect();

    for (expr, _, closest_unsafe_block, _, _) in loader.load_iter_thir_exprs() {
        let Some(&block) = thir_trailing_expr_to_block.get(&expr) else {
            continue;
        };

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
                if safety != types::BlockSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if check_mode == types::BlockCheckMode::UnsafeBlockUserProvided {
                    *build_user_unsafe_stmt += 1;
                }
            }
            {
                let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) =
                    selected_function_thir_sizes_map
                        .entry(thir_body_def_path)
                        .or_default();
                *build_stmt += 1;
                if safety != types::BlockSafety::Safe {
                    *build_unsafe_stmt += 1;
                }
                if check_mode == types::BlockCheckMode::UnsafeBlockUserProvided {
                    *build_user_unsafe_stmt += 1;
                }
            }
        }
    }

    let selected_build_thir_sizes = selected_build_thir_sizes_map.into_iter().map(
        |(build, (stmt, unsafe_stmt, user_unsafe_stmt))| {
            (build, stmt, unsafe_stmt, user_unsafe_stmt)
        },
    );
    loader.store_iter_selected_build_thir_sizes(selected_build_thir_sizes);

    let selected_function_thir_sizes = selected_function_thir_sizes_map.into_iter().flat_map(
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
    );

    loader.store_iter_selected_function_thir_sizes(selected_function_thir_sizes);
}

fn report_function_sizes(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let def_path_resolver = DefPathResolver::new(loader);
    let abis = loader.load_abis();
    let strings = loader.load_strings();

    let selected_build_thir_sizes = loader.load_selected_build_thir_sizes();

    let selected_build_thir_sizes =
        selected_build_thir_sizes
            .iter()
            .map(|(build, stmt, unsafe_stmt, user_unsafe_stmt)| {
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
        |(
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
                strings.get_unwrap(abis.get_unwrap(abi)),
                uses_unsafe,
                stmt,
                unsafe_stmt,
                user_unsafe_stmt,
            )
        },
    );
    write_csv!(report_path, selected_function_thir_sizes);
}

pub fn query(loader: &Loader, report_path: &Path) {
    collect_function_sizes(loader);
    report_function_sizes(loader, report_path);
}
