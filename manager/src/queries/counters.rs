//! Compute unsafe blocks and unsafe statements.

use super::utils::{DefPathResolver, GroupByIterator, SpanResolver};
use crate::queries::utils::BuildResolver;
use crate::write_csv;
use corpus_database::{tables::Loader, types};
use corpus_database::DiskMap;
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn new_query(loader: &Loader, report_path: &Path) {
    let mut thir_block_parent_to_children: DiskMap<_, Vec<_>> = DiskMap::create_temp();
    for (parent, child, _safety, _check_mode, _span) in loader.load_iter_thir_blocks() {
        let mut children = thir_block_parent_to_children.get(parent).unwrap_or_default();
        children.push(child);
        thir_block_parent_to_children.insert(parent, children);
        // thir_block_parent_to_children.entry(parent).or_insert_with(Vec::new).push(child);
    }

    // map a root block to build and thir_body_def_path
    let mut map_selected_thir_bodies_to_data: HashMap<_, _> = loader.load_iter_selected_thir_bodies().map(
        |(build, item, thir_body_def_path, body)| {
            (body, (build, thir_body_def_path))
        },
    ).collect();

    // root_block = key into map_selected_thir_bodies_to_data
    // (root_block, block)
    let mut selected_thir_blocks: HashSet<_> = loader.load_iter_selected_thir_bodies().map(
        |(build, item, thir_body_def_path, body)| {
            (body, body)
        },
    ).collect();

    let mut stack: Vec<_> = selected_thir_blocks.iter().cloned().collect();
    while let Some((key, block)) = stack.pop() {
        let Some(children) = thir_block_parent_to_children.get(block) else {
            continue;
        };
        for child in children {
            if selected_thir_blocks.insert((key, child)) {
                stack.push((key, child));
            } else {
                panic!("Child has multiple parent THIR blocks: {:?}", child);
            }
        }
    }

    let thir_block_data_map = loader.load_thir_blocks_redb_map();
    let full_selected_thir_blocks = selected_thir_blocks.iter().filter_map(
        |&(root_block, block)| {
            if root_block == block {
                // we're looking at a block from selected_thir_bodies. this block has no parent or other data, and we also skipped it in the datapond query.
                return None;
            }
            let (build, thir_body_def_path) = *map_selected_thir_bodies_to_data.get(&root_block).unwrap();
            let (parent, safety, check_mode, span) = thir_block_data_map.r(block);
            Some((build, thir_body_def_path, parent, block, safety, check_mode, span))
        },
    );
    loader.store_iter_selected_thir_blocks(full_selected_thir_blocks);


    // let selected_thir_blocks;

    // // TODO: optimize away transitive closure somehow?
    // datapond_query!(
    //     load loader {
    //         relations(selected_thir_bodies, thir_blocks),
    //     }
    //     output selected_thir_blocks(
    //         build: Build,
    //         thir_body_def_path: DefPath,
    //         parent: ThirBlock,
    //         block: ThirBlock,
    //         safety: ScopeSafety,
    //         check_mode: BlockCheckMode,
    //         span: Span,
    //     )
    //     selected_thir_blocks(
    //         build, thir_body_def_path, parent, block, safety, check_mode, span
    //     ) :-
    //         thir_blocks(parent, block, safety, check_mode, span),
    //         selected_thir_bodies(build, _, thir_body_def_path, parent).

    //     selected_thir_blocks(
    //         build, thir_body_def_path, parent, block, safety, check_mode, span
    //     ) :-
    //         selected_thir_blocks(.build=build, .thir_body_def_path=thir_body_def_path, .block=parent),
    //         thir_blocks(parent, block, safety, check_mode, span).
    // );
    // info!("selected_thir_blocks.len = {}", selected_thir_blocks.len());
    // loader.store_selected_thir_blocks(selected_thir_blocks.elements);
    let selected_thir_blocks = loader.load_selected_thir_blocks();
    // {
    //     let selected_thir_blocks = selected_thir_blocks.tuple_iter();
    //     write_csv!(report_path, selected_thir_blocks);
    // }

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

    // let unsafe_thir_blocks_relation = loader.load_unsafe_thir_blocks();

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

    // TODO: delete me, or move me somewhere else
    // count the kinds of closures that exist
    let closure_kinds = loader.load_thir_exprs_closure_kind();
    let mut fn_count = 0;
    let mut fnmut_count = 0;
    let mut fnonce_count = 0;
    for (_, kind) in closure_kinds.tuple_iter() {
        match kind {
            types::ClosureKind::Fn => fn_count += 1,
            types::ClosureKind::FnMut => fnmut_count += 1,
            types::ClosureKind::FnOnce => fnonce_count += 1,
            types::ClosureKind::Unknown => {},
        }
    }
    info!("[all builds] fn_count = {}, fnmut_count = {}, fnonce_count = {}", fn_count, fnmut_count, fnonce_count);
}
