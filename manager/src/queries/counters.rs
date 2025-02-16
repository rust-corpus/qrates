//! Compute unsafe blocks and unsafe statements.

use super::utils::{DefPathResolver, GroupByIterator, SpanResolver};
use crate::queries::utils::BuildResolver;
use crate::write_csv;
use corpus_database::{tables::Loader, types};
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let selected_scopes;
    datapond_query! {
        load loader {
            relations(selected_mir_cfgs, subscopes),
        }
        output selected_scopes(
            build: Build,
            mir_body_def_path: DefPath,
            scope: Scope,
            parent: Scope,
            safety: ScopeSafety,
            explicit_unsafe_group: u32,
            check_mode: BlockCheckMode,
            span: Span,
        )
        selected_scopes(
            build, mir_body_def_path, scope, parent, safety,
            explicit_unsafe_group, check_mode, span
        ) :-
            selected_mir_cfgs(build, _, mir_body_def_path, parent),
            subscopes(.parent=parent, .child=scope, .safety=safety,
                      .explicit_unsafe_group=explicit_unsafe_group,
                      .check_mode=check_mode, .span=span).
        selected_scopes(
            build, mir_body_def_path, scope, parent, safety,
            explicit_unsafe_group, check_mode, span
        ) :-
            selected_scopes(.build=build, .mir_body_def_path=mir_body_def_path, .scope=parent),
            subscopes(.parent=parent, .child=scope, .safety=safety,
                      .explicit_unsafe_group=explicit_unsafe_group,
                      .check_mode=check_mode, .span=span).
    }
    info!("selected_scopes.len = {}", selected_scopes.len());
    loader.store_selected_scopes(selected_scopes.elements);
    let selected_scopes = loader.load_selected_scopes();

    let strings = loader.load_strings();
    let def_path_resolver = DefPathResolver::new(loader);
    let span_resolver = SpanResolver::new(loader);
    let selected_builds: HashSet<_> = loader
        .load_selected_builds()
        .iter()
        .map(|(build, _package, _version, _krate, _crate_hash, _edition)| *build)
        .collect();

    // Group by (mir_body_def_path, explicit_unsafe_group) and drop all with scopes
    // with parents within a group. In this way, we should be left with the root scope
    // that introduced the unsafe block.
    let mut unsafe_blocks_relation = Vec::new();
    let mut unsafe_blocks = Vec::new();
    // Maps each scope inside an unsafe block to a root scope of that unsafe block.
    let mut unsafe_root_scopes = HashMap::new();
    let iter = selected_scopes
        .iter()
        .filter(
            |&(
                _build,
                _mir_body_def_path,
                _scope,
                _parent,
                safety,
                _explicit_unsafe_group,
                _check_mode,
                _span,
            )| { *safety == types::ScopeSafety::ExplicitUnsafe },
        )
        .safe_group_by(
            |(
                _build,
                def_path,
                _scope,
                _parent,
                _safety,
                explicit_unsafe_group,
                _check_mode,
                _span,
            )| { (def_path, explicit_unsafe_group) },
        );
    let mut counter = 0;
    for (_, group) in iter.into_iter() {
        counter += 1;

        let mut children = HashSet::new();
        let group: Vec<_> = group.collect();
        for &(
            _build,
            _mir_body_def_path,
            scope,
            _parent,
            _safety,
            _explicit_unsafe_group,
            _check_mode,
            _span,
        ) in &group
        {
            children.insert(scope);
        }
        let mut found = false;
        for &(
            build,
            mir_body_def_path,
            scope,
            parent,
            _safety,
            _explicit_unsafe_group,
            check_mode,
            span,
        ) in group
        {
            if !children.contains(&parent) {
                assert!(!found);
                found = true;
                unsafe_blocks_relation.push((
                    build,
                    mir_body_def_path,
                    scope,
                    span_resolver.get_expansion_kind(span),
                    check_mode,
                    span,
                ));
                assert!(
                    selected_builds.contains(&build),
                    "Unsafe block from non-selected build: {:?}",
                    def_path_resolver.resolve(mir_body_def_path)
                );
                unsafe_blocks.push((
                    build,
                    def_path_resolver.resolve(mir_body_def_path),
                    scope,
                    check_mode.to_string(),
                    span_resolver.resolve(span),
                ));
                for &subscope in &children {
                    unsafe_root_scopes.insert(subscope, (scope, build, check_mode));
                }
            }
        }
        assert!(found);
    }
    info!("counter: {}", counter);
    info!("Computed unsafe blocks: {}", unsafe_blocks.len());
    loader.store_unsafe_blocks(unsafe_blocks_relation);
    info!("Saved unsafe blocks.");
    write_csv!(report_path, unsafe_blocks);
    info!("Saved unsafe block report.");

    let statements = loader.load_statements();
    let unsafe_statements: Vec<_> = statements
        .iter()
        .flat_map(|&(stmt, block, index, kind, scope)| {
            unsafe_root_scopes
                .get(&scope)
                .map(|&(unsafe_scope, build, check_mode)| {
                    (build, stmt, block, index, kind, unsafe_scope, check_mode)
                })
        })
        .collect();
    // write_csv!(report_path, &unsafe_statements);
    loader.store_unsafe_statements(unsafe_statements);
    info!("Saved unsafe statements.");

    let terminators = loader.load_terminators();
    let unsafe_terminators: Vec<_> = terminators
        .iter()
        .flat_map(|&(block, kind, scope)| {
            unsafe_root_scopes
                .get(&scope)
                .map(|&(unsafe_scope, build, check_mode)| {
                    (build, block, kind, unsafe_scope, check_mode)
                })
        })
        .collect();
    loader.store_unsafe_terminators(unsafe_terminators);
    info!("Saved unsafe terminators.");

    let functions_unsafe_blocks;
    datapond_query! {
        load loader {
            relations(selected_mir_cfgs, unsafe_blocks),
        }
        output functions_unsafe_blocks(
            build: Build, function: Item, scope: Scope,
            expansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)
        functions_unsafe_blocks(build, function, scope, expansion_kind, check_mode) :-
            unsafe_blocks(build, mir_body_def_path, scope, expansion_kind, check_mode, _),
            selected_mir_cfgs(build, function, mir_body_def_path, _).
    }
    let functions_unsafe_blocks = functions_unsafe_blocks.elements;
    let function_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
        .iter()
        .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| *function)
        .into_iter()
        .map(|(function, group)| (function, group.count()))
        .collect();
    let function_user_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
        .iter()
        .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
            *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
        })
        .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| *function)
        .into_iter()
        .map(|(function, group)| (function, group.count()))
        .collect();
    info!(
        "functions_unsafe_blocks.len = {}",
        functions_unsafe_blocks.len()
    );
    write_csv!(report_path, &functions_unsafe_blocks);
    loader.store_functions_unsafe_blocks(functions_unsafe_blocks);

    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let selected_function_definitions = loader.load_selected_function_definitions();
    let selected_function_definitions = selected_function_definitions.iter().map(
        |&(build, item, def_path, module, visibility, unsafety, abi, _return_ty, uses_unsafe)| {
            (
                build,
                def_path_resolver.resolve(def_path),
                item,
                def_path,
                module,
                visibility.to_string(),
                unsafety.to_string(),
                &strings[abis[abi]],
                uses_unsafe,
                function_unsafe_block_counts
                    .get(&item)
                    .cloned()
                    .unwrap_or(0),
                function_user_unsafe_block_counts
                    .get(&item)
                    .cloned()
                    .unwrap_or(0),
                trait_items.contains(&def_path),
            )
        },
    );
    write_csv!(report_path, selected_function_definitions);
}

pub fn new_query(loader: &Loader, report_path: &Path) {
    let selected_thir_blocks;

    datapond_query!(
        load loader {
            relations(selected_thir_bodies, thir_blocks),
        }
        output selected_thir_blocks(
            build: Build,
            thir_body_def_path: DefPath,
            parent: ThirBlock,
            block: ThirBlock,
            safety: ScopeSafety,
            check_mode: BlockCheckMode,
            span: Span,
        )
        selected_thir_blocks(
            build, thir_body_def_path, parent, block, safety, check_mode, span
        ) :-
            thir_blocks(parent, block, safety, check_mode, span),
            selected_thir_bodies(build, _, thir_body_def_path, parent).

        selected_thir_blocks(
            build, thir_body_def_path, parent, block, safety, check_mode, span
        ) :-
            selected_thir_blocks(.build=build, .thir_body_def_path=thir_body_def_path, .block=parent),
            thir_blocks(parent, block, safety, check_mode, span).
    );
    info!("selected_thir_blocks.len = {}", selected_thir_blocks.len());
    loader.store_selected_thir_blocks(selected_thir_blocks.elements);
    let selected_thir_blocks = loader.load_selected_thir_blocks();

    let def_path_resolver = DefPathResolver::new(loader);
    let span_resolver = SpanResolver::new(loader);
    let strings = loader.load_strings();

    let mut unsafe_thir_blocks_relation = Vec::new();
    let mut unsafe_thir_blocks = Vec::new();

    for &(
        build,
        thir_body_def_path,
        _parent,
        block,
        _safety, // for unsafe_thir_blocks, safety will always be ExplicitUnsafe
        check_mode,
        span,
    ) in selected_thir_blocks.iter().filter(
        |&(_build, _thir_body_def_path, _parent, _block, safety, _check_mode, _span)| {
            *safety == types::ScopeSafety::ExplicitUnsafe
        },
    ) {
        unsafe_thir_blocks_relation.push((
            build,
            thir_body_def_path,
            block,
            span_resolver.get_expansion_kind(span),
            check_mode,
            span,
        ));
        unsafe_thir_blocks.push((
            build,
            def_path_resolver.resolve(thir_body_def_path),
            block,
            check_mode.to_string(),
            span_resolver.resolve(span),
        ));
    }

    info!("Computed unsafe thir blocks: {}", unsafe_thir_blocks.len());
    loader.store_unsafe_thir_blocks(unsafe_thir_blocks_relation);
    info!("Saved unsafe thir blocks.");
    write_csv!(report_path, unsafe_thir_blocks);
    info!("Saved unsafe thir block report.");

    let unsafe_thir_blocks_relation = loader.load_unsafe_thir_blocks();

    // unsafe thir statements
    let thir_statements = loader.load_thir_stmts();
    let unsafe_thir_statements: Vec<_> = thir_statements
        .iter()
        .flat_map(|&(stmt, _block, closest_unsafe_block, index)| {
            unsafe_thir_blocks_relation
                .iter()
                .filter(
                    move |&(
                        build,
                        _thir_body_def_path,
                        unsafe_block,
                        _expansion_kind,
                        _check_mode,
                        _span,
                    )| { *unsafe_block == closest_unsafe_block },
                )
                .map(
                    move |&(
                        build,
                        _thir_body_def_path,
                        _unsafe_block,
                        _expansion_kind,
                        check_mode,
                        span,
                    )| {
                        (build, stmt, closest_unsafe_block, index, check_mode)
                    },
                )
        })
        .collect();

    loader.store_unsafe_thir_stmts(unsafe_thir_statements.clone());
    info!("Saved unsafe thir statements.");
    write_csv!(report_path, unsafe_thir_statements);
    info!("Saved unsafe thir statement report.");

    let functions_unsafe_thir_blocks;
    datapond_query! {
        load loader {
            relations(selected_thir_bodies, unsafe_thir_blocks),
        }
        output functions_unsafe_thir_blocks(
            build: Build, function: Item, block: ThirBlock,
            expansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)
        functions_unsafe_thir_blocks(build, function, block, expansion_kind, check_mode) :-
            unsafe_thir_blocks(build, thir_body_def_path, block, expansion_kind, check_mode, _),
            selected_thir_bodies(build, function, thir_body_def_path, _).
    }
    let functions_unsafe_thir_blocks = functions_unsafe_thir_blocks.elements;
    let function_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
        .iter()
        .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| *function)
        .into_iter()
        .map(|(function, group)| (function, group.count()))
        .collect();
    let function_user_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
        .iter()
        .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
            *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
        })
        .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| *function)
        .into_iter()
        .map(|(function, group)| (function, group.count()))
        .collect();
    info!(
        "functions_unsafe_thir_blocks.len = {}",
        functions_unsafe_thir_blocks.len()
    );
    write_csv!(report_path, &functions_unsafe_thir_blocks);
    loader.store_functions_unsafe_thir_blocks(functions_unsafe_thir_blocks);

    let abis = loader.load_abis();
    let trait_items = loader.load_trait_items();
    let trait_items: HashSet<_> = trait_items
        .iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let selected_function_definitions = loader.load_selected_function_definitions();
    let selected_function_definitions_thir_counts = selected_function_definitions.iter().map(
        |&(build, item, def_path, module, visibility, unsafety, abi, _return_ty, uses_unsafe)| {
            (
                build,
                def_path_resolver.resolve(def_path),
                item,
                def_path,
                module,
                visibility.to_string(),
                unsafety.to_string(),
                &strings[abis[abi]],
                uses_unsafe,
                function_unsafe_thir_block_counts
                    .get(&item)
                    .cloned()
                    .unwrap_or(0),
                function_user_unsafe_thir_block_counts
                    .get(&item)
                    .cloned()
                    .unwrap_or(0),
                trait_items.contains(&def_path),
            )
        },
    );
    write_csv!(report_path, selected_function_definitions_thir_counts);
}
