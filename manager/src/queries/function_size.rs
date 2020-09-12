//! Report function sizes in MIR statements.

use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::{tables::Loader, types};
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
