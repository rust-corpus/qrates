//! Compute unsafe blocks and unsafe statements.

use super::utils::{DefPathResolver, GroupByIterator, SpanResolver};
use crate::queries::utils::BuildResolver;
use crate::write_csv;
use corpus_database::{tables::Loader, types};
use corpus_database::RelationElement as RE;
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn new_query(loader: &Loader, report_path: &Path) {
    let selected_thir_blocks;

    // TODO: optimize away transitive closure somehow?
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

    let mut unsafe_thir_block_to_build_and_checkmode = HashMap::new();

    for (
        build,
        thir_body_def_path,
        _parent,
        block,
        _safety, // for unsafe_thir_blocks, safety will always be ExplicitUnsafe
        check_mode,
        span,
    ) in selected_thir_blocks.tuple_iter().filter(
        |(_build, _thir_body_def_path, _parent, _block, safety, _check_mode, _span)| {
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
        unsafe_thir_block_to_build_and_checkmode.insert(block, (build, check_mode));
    }

    info!("Computed unsafe thir blocks: {}", unsafe_thir_blocks.len());
    loader.store_unsafe_thir_blocks(unsafe_thir_blocks_relation);
    info!("Saved unsafe thir blocks.");
    write_csv!(report_path, unsafe_thir_blocks);
    info!("Saved unsafe thir block report.");

    let unsafe_thir_blocks_relation = loader.load_unsafe_thir_blocks();

    // unsafe thir statements

    let unsafe_thir_statements = || {
        let thir_statements = loader.load_iter_thir_stmts();
        thir_statements
            .filter_map(|(stmt, _block, closest_unsafe_block, index)| {
                let &(build, check_mode) = unsafe_thir_block_to_build_and_checkmode.get(&closest_unsafe_block)?;
                
                Some((build, stmt, closest_unsafe_block, index, check_mode))
            })
    };

    // let thir_statements = loader.load_iter_thir_stmts();
    // let unsafe_thir_statements: Vec<_> = thir_statements
    //     .flat_map(|(stmt, _block, closest_unsafe_block, index)| {
    //         unsafe_thir_blocks_relation
    //             .iter()
    //             .filter(
    //                 move |&(
    //                     build,
    //                     _thir_body_def_path,
    //                     unsafe_block,
    //                     _expansion_kind,
    //                     _check_mode,
    //                     _span,
    //                 )| { *unsafe_block == closest_unsafe_block },
    //             )
    //             .map(
    //                 move |&(
    //                     build,
    //                     _thir_body_def_path,
    //                     _unsafe_block,
    //                     _expansion_kind,
    //                     check_mode,
    //                     span,
    //                 )| {
    //                     (build, stmt, closest_unsafe_block, index, check_mode)
    //                 },
    //             )
    //     })
    //     .collect();

    loader.store_iter_unsafe_thir_stmts(unsafe_thir_statements());
    info!("Saved unsafe thir statements.");
    let unsafe_thir_statements = unsafe_thir_statements();
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
        .tuple_iter()
        .map(|(_trait_id, def_path, _defaultness)| def_path)
        .collect();
    let selected_function_definitions = loader.load_selected_function_definitions();
    let selected_function_definitions_thir_counts = selected_function_definitions.tuple_iter().map(
        |(build, item, def_path, module, visibility, unsafety, abi, _return_ty, uses_unsafe)| {
            (
                build,
                def_path_resolver.resolve(def_path),
                item,
                def_path,
                module,
                visibility.to_string(),
                unsafety.to_string(),
                strings.r(abis.r(abi)),
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
