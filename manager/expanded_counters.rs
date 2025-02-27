mod counters {
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
    pub fn query(loader: &Loader, report_path: &Path) {
        let selected_scopes;
        {
            #[allow(dead_code)]
            enum ProcMacroHack {
                Value = (
                    "load loader { relations(selected_mir_cfgs, subscopes), } output\nselected_scopes(build: Build, mir_body_def_path: DefPath, scope: Scope,\nparent: Scope, safety: ScopeSafety, explicit_unsafe_group: u32, check_mode:\nBlockCheckMode, span: Span,)\nselected_scopes(build, mir_body_def_path, scope, parent, safety,\nexplicit_unsafe_group, check_mode, span) :-\nselected_mir_cfgs(build, _, mir_body_def_path, parent),\nsubscopes(.parent=parent, .child=scope, .safety=safety,\n.explicit_unsafe_group=explicit_unsafe_group, .check_mode=check_mode,\n.span=span).selected_scopes(build, mir_body_def_path, scope, parent, safety,\nexplicit_unsafe_group, check_mode, span) :-\nselected_scopes(.build=build, .mir_body_def_path=mir_body_def_path,\n.scope=parent),\nsubscopes(.parent=parent, .child=scope, .safety=safety,\n.explicit_unsafe_group=explicit_unsafe_group, .check_mode=check_mode,\n.span=span).",
                    0,
                )
                    .1,
            }
            {
                use corpus_database::types::*;
                use corpus_database::RelationElement;
                use corpus_database::VecOfRelationElementAdapter;
                use corpus_database::VecIntoRelationElementAdapter;
                let selected_mir_cfgs = loader.load_selected_mir_cfgs().to_tuple_vec();
                let subscopes = loader.load_subscopes().to_tuple_vec();
                {
                    let mut iteration = datafrog::Iteration::new();
                    let var_selected_mir_cfgs = datafrog::Relation::<
                        (Build, Item, DefPath, Scope),
                    >::from_vec(selected_mir_cfgs);
                    let var_subscopes = datafrog::Relation::<
                        (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span),
                    >::from_vec(subscopes);
                    let var_selected_scopes = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                Scope,
                                Scope,
                                ScopeSafety,
                                u32,
                                BlockCheckMode,
                                Span,
                            ),
                        >("selected_scopes");
                    let var_selected_mir_cfgs_1 = iteration
                        .variable::<
                            (Build, Item, DefPath, Scope),
                        >("selected_mir_cfgs_1");
                    let var_subscopes_2 = iteration
                        .variable::<
                            (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span),
                        >("subscopes_2");
                    let var_selected_mir_cfgs_1_3 = iteration
                        .variable::<
                            ((Scope,), (Build, DefPath)),
                        >("selected_mir_cfgs_1_3");
                    let var_subscopes_2_4 = iteration
                        .variable::<
                            ((Scope,), (Scope, ScopeSafety, BlockCheckMode, u32, Span)),
                        >("subscopes_2_4");
                    let var_selected_scopes_5 = iteration
                        .variable::<
                            (
                                Scope,
                                Build,
                                DefPath,
                                Scope,
                                ScopeSafety,
                                BlockCheckMode,
                                u32,
                                Span,
                            ),
                        >("selected_scopes_5");
                    let var_selected_scopes_6 = iteration
                        .variable::<((Scope,), (Build, DefPath))>("selected_scopes_6");
                    let var_subscopes_2_7 = iteration
                        .variable::<
                            ((Scope,), (Scope, ScopeSafety, BlockCheckMode, u32, Span)),
                        >("subscopes_2_7");
                    let var_selected_scopes_8 = iteration
                        .variable::<
                            (
                                Scope,
                                Build,
                                DefPath,
                                Scope,
                                ScopeSafety,
                                BlockCheckMode,
                                u32,
                                Span,
                            ),
                        >("selected_scopes_8");
                    var_selected_mir_cfgs_1.insert(var_selected_mir_cfgs);
                    var_subscopes_2.insert(var_subscopes);
                    while iteration.changed() {
                        var_selected_mir_cfgs_1_3
                            .from_map(
                                &var_selected_mir_cfgs_1,
                                |&(build, _, mir_body_def_path, parent)| (
                                    (parent,),
                                    (build, mir_body_def_path),
                                ),
                            );
                        var_subscopes_2_4
                            .from_map(
                                &var_subscopes_2,
                                |
                                    &(
                                        parent,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    )|
                                (
                                    (parent,),
                                    (scope, safety, check_mode, explicit_unsafe_group, span),
                                ),
                            );
                        var_selected_scopes_5
                            .from_join(
                                &var_selected_mir_cfgs_1_3,
                                &var_subscopes_2_4,
                                |
                                    &(parent,),
                                    &(build, mir_body_def_path),
                                    &(scope, safety, check_mode, explicit_unsafe_group, span)|
                                (
                                    parent,
                                    build,
                                    mir_body_def_path,
                                    scope,
                                    safety,
                                    check_mode,
                                    explicit_unsafe_group,
                                    span,
                                ),
                            );
                        var_selected_scopes
                            .from_map(
                                &var_selected_scopes_5,
                                |
                                    &(
                                        parent,
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    )|
                                (
                                    build,
                                    mir_body_def_path,
                                    scope,
                                    parent,
                                    safety,
                                    explicit_unsafe_group,
                                    check_mode,
                                    span,
                                ),
                            );
                        var_selected_scopes_6
                            .from_map(
                                &var_selected_scopes,
                                |&(build, mir_body_def_path, parent, _, _, _, _, _)| (
                                    (parent,),
                                    (build, mir_body_def_path),
                                ),
                            );
                        var_subscopes_2_7
                            .from_map(
                                &var_subscopes_2,
                                |
                                    &(
                                        parent,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    )|
                                (
                                    (parent,),
                                    (scope, safety, check_mode, explicit_unsafe_group, span),
                                ),
                            );
                        var_selected_scopes_8
                            .from_join(
                                &var_selected_scopes_6,
                                &var_subscopes_2_7,
                                |
                                    &(parent,),
                                    &(build, mir_body_def_path),
                                    &(scope, safety, check_mode, explicit_unsafe_group, span)|
                                (
                                    parent,
                                    build,
                                    mir_body_def_path,
                                    scope,
                                    safety,
                                    check_mode,
                                    explicit_unsafe_group,
                                    span,
                                ),
                            );
                        var_selected_scopes
                            .from_map(
                                &var_selected_scopes_8,
                                |
                                    &(
                                        parent,
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    )|
                                (
                                    build,
                                    mir_body_def_path,
                                    scope,
                                    parent,
                                    safety,
                                    explicit_unsafe_group,
                                    check_mode,
                                    span,
                                ),
                            );
                    }
                    selected_scopes = var_selected_scopes.complete();
                }
            }
        }
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("selected_scopes.len = {0}", selected_scopes.len()),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        loader.store_selected_scopes(selected_scopes.elements);
        let selected_scopes = loader.load_selected_scopes();
        let strings = loader.load_strings();
        let def_path_resolver = DefPathResolver::new(loader);
        let span_resolver = SpanResolver::new(loader);
        let selected_builds: HashSet<_> = loader
            .load_iter_selected_builds()
            .map(|(build, _package, _version, _krate, _crate_hash, _edition)| build)
            .collect();
        let mut unsafe_blocks_relation = Vec::new();
        let mut unsafe_blocks = Vec::new();
        let mut unsafe_root_scopes = HashMap::new();
        let iter = selected_scopes
            .tuple_iter()
            .filter(|
                (
                    _build,
                    _mir_body_def_path,
                    _scope,
                    _parent,
                    safety,
                    _explicit_unsafe_group,
                    _check_mode,
                    _span,
                )|
            { *safety == types::ScopeSafety::ExplicitUnsafe })
            .safe_group_by(|
                &(
                    _build,
                    def_path,
                    _scope,
                    _parent,
                    _safety,
                    explicit_unsafe_group,
                    _check_mode,
                    _span,
                )|
            { (def_path, explicit_unsafe_group) });
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
            ) in &group {
                children.insert(scope);
            }
            let mut found = false;
            for (
                build,
                mir_body_def_path,
                scope,
                parent,
                _safety,
                _explicit_unsafe_group,
                check_mode,
                span,
            ) in group {
                if !children.contains(&parent) {
                    if !!found {
                        ::core::panicking::panic("assertion failed: !found")
                    }
                    found = true;
                    unsafe_blocks_relation
                        .push((
                            build,
                            mir_body_def_path,
                            scope,
                            span_resolver.get_expansion_kind(span),
                            check_mode,
                            span,
                        ));
                    if !selected_builds.contains(&build) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Unsafe block from non-selected build: {0:?}",
                                    def_path_resolver.resolve(mir_body_def_path),
                                ),
                            );
                        }
                    }
                    unsafe_blocks
                        .push((
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
            if !found {
                ::core::panicking::panic("assertion failed: found")
            }
        }
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("counter: {0}", counter),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Computed unsafe blocks: {0}", unsafe_blocks.len()),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        loader.store_unsafe_blocks(unsafe_blocks_relation);
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe blocks."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "unsafe_blocks"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in unsafe_blocks {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe block report."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let statements = loader.load_statements();
        let unsafe_statements: Vec<_> = statements
            .tuple_iter()
            .flat_map(|(stmt, block, index, kind, scope)| {
                unsafe_root_scopes
                    .get(&scope)
                    .map(|&(unsafe_scope, build, check_mode)| {
                        (build, stmt, block, index, kind, unsafe_scope, check_mode)
                    })
            })
            .collect();
        loader.store_unsafe_statements(unsafe_statements);
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe statements."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let terminators = loader.load_terminators();
        let unsafe_terminators: Vec<_> = terminators
            .tuple_iter()
            .flat_map(|(block, kind, scope)| {
                unsafe_root_scopes
                    .get(&scope)
                    .map(|&(unsafe_scope, build, check_mode)| {
                        (build, block, kind, unsafe_scope, check_mode)
                    })
            })
            .collect();
        loader.store_unsafe_terminators(unsafe_terminators);
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe terminators."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let functions_unsafe_blocks;
        {
            #[allow(dead_code)]
            enum ProcMacroHack {
                Value = (
                    "load loader { relations(selected_mir_cfgs, unsafe_blocks), } output\nfunctions_unsafe_blocks(build: Build, function: Item, scope: Scope,\nexpansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)\nfunctions_unsafe_blocks(build, function, scope, expansion_kind, check_mode) :-\nunsafe_blocks(build, mir_body_def_path, scope, expansion_kind, check_mode, _),\nselected_mir_cfgs(build, function, mir_body_def_path, _).",
                    0,
                )
                    .1,
            }
            {
                use corpus_database::types::*;
                use corpus_database::RelationElement;
                use corpus_database::VecOfRelationElementAdapter;
                use corpus_database::VecIntoRelationElementAdapter;
                let selected_mir_cfgs = loader.load_selected_mir_cfgs().to_tuple_vec();
                let unsafe_blocks = loader.load_unsafe_blocks().to_tuple_vec();
                {
                    let mut iteration = datafrog::Iteration::new();
                    let var_selected_mir_cfgs = datafrog::Relation::<
                        (Build, Item, DefPath, Scope),
                    >::from_vec(selected_mir_cfgs);
                    let var_unsafe_blocks = datafrog::Relation::<
                        (Build, DefPath, Scope, SpanExpansionKind, BlockCheckMode, Span),
                    >::from_vec(unsafe_blocks);
                    let var_functions_unsafe_blocks = iteration
                        .variable::<
                            (Build, Item, Scope, SpanExpansionKind, BlockCheckMode),
                        >("functions_unsafe_blocks");
                    let var_unsafe_blocks_1 = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                Scope,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Span,
                            ),
                        >("unsafe_blocks_1");
                    let var_selected_mir_cfgs_2 = iteration
                        .variable::<
                            (Build, Item, DefPath, Scope),
                        >("selected_mir_cfgs_2");
                    let var_unsafe_blocks_1_3 = iteration
                        .variable::<
                            (
                                (Build, DefPath),
                                (Scope, SpanExpansionKind, BlockCheckMode),
                            ),
                        >("unsafe_blocks_1_3");
                    let var_selected_mir_cfgs_2_4 = iteration
                        .variable::<
                            ((Build, DefPath), (Item,)),
                        >("selected_mir_cfgs_2_4");
                    let var_functions_unsafe_blocks_5 = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                Scope,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Item,
                            ),
                        >("functions_unsafe_blocks_5");
                    var_unsafe_blocks_1.insert(var_unsafe_blocks);
                    var_selected_mir_cfgs_2.insert(var_selected_mir_cfgs);
                    while iteration.changed() {
                        var_unsafe_blocks_1_3
                            .from_map(
                                &var_unsafe_blocks_1,
                                |
                                    &(
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        expansion_kind,
                                        check_mode,
                                        _,
                                    )|
                                (
                                    (build, mir_body_def_path),
                                    (scope, expansion_kind, check_mode),
                                ),
                            );
                        var_selected_mir_cfgs_2_4
                            .from_map(
                                &var_selected_mir_cfgs_2,
                                |&(build, function, mir_body_def_path, _)| (
                                    (build, mir_body_def_path),
                                    (function,),
                                ),
                            );
                        var_functions_unsafe_blocks_5
                            .from_join(
                                &var_unsafe_blocks_1_3,
                                &var_selected_mir_cfgs_2_4,
                                |
                                    &(build, mir_body_def_path),
                                    &(scope, expansion_kind, check_mode),
                                    &(function,)|
                                (
                                    build,
                                    mir_body_def_path,
                                    scope,
                                    expansion_kind,
                                    check_mode,
                                    function,
                                ),
                            );
                        var_functions_unsafe_blocks
                            .from_map(
                                &var_functions_unsafe_blocks_5,
                                |
                                    &(
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        expansion_kind,
                                        check_mode,
                                        function,
                                    )|
                                (build, function, scope, expansion_kind, check_mode),
                            );
                    }
                    functions_unsafe_blocks = var_functions_unsafe_blocks.complete();
                }
            }
        }
        let functions_unsafe_blocks = functions_unsafe_blocks.elements;
        let function_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
            .iter()
            .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| {
                *function
            })
            .into_iter()
            .map(|(function, group)| (function, group.count()))
            .collect();
        let function_user_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
            .iter()
            .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
                *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
            })
            .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| {
                *function
            })
            .into_iter()
            .map(|(function, group)| (function, group.count()))
            .collect();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!(
                        "functions_unsafe_blocks.len = {0}",
                        functions_unsafe_blocks.len(),
                    ),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "&functions_unsafe_blocks"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in &functions_unsafe_blocks {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        loader.store_functions_unsafe_blocks(functions_unsafe_blocks);
        let abis = loader.load_abis();
        let trait_items = loader.load_trait_items();
        let trait_items: HashSet<_> = trait_items
            .tuple_iter()
            .map(|(_trait_id, def_path, _defaultness)| def_path)
            .collect();
        let selected_function_definitions = loader.load_selected_function_definitions();
        let selected_function_definitions = selected_function_definitions
            .tuple_iter()
            .map(|
                (
                    build,
                    item,
                    def_path,
                    module,
                    visibility,
                    unsafety,
                    abi,
                    _return_ty,
                    uses_unsafe,
                )|
            {
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
                    function_unsafe_block_counts.get(&item).cloned().unwrap_or(0),
                    function_user_unsafe_block_counts.get(&item).cloned().unwrap_or(0),
                    trait_items.contains(&def_path),
                )
            });
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "selected_function_definitions"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in selected_function_definitions {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
    }
    pub fn new_query(loader: &Loader, report_path: &Path) {
        let selected_thir_blocks;
        {
            #[allow(dead_code)]
            enum ProcMacroHack {
                Value = (
                    "load loader { relations(selected_thir_bodies, thir_blocks), } output\nselected_thir_blocks(build: Build, thir_body_def_path: DefPath, parent:\nThirBlock, block: ThirBlock, safety: ScopeSafety, check_mode: BlockCheckMode,\nspan: Span,)\nselected_thir_blocks(build, thir_body_def_path, parent, block, safety,\ncheck_mode, span) :- thir_blocks(parent, block, safety, check_mode, span),\nselected_thir_bodies(build, _, thir_body_def_path,\nparent).selected_thir_blocks(build, thir_body_def_path, parent, block, safety,\ncheck_mode, span) :-\nselected_thir_blocks(.build=build, .thir_body_def_path=thir_body_def_path,\n.block=parent), thir_blocks(parent, block, safety, check_mode, span).",
                    0,
                )
                    .1,
            }
            {
                use corpus_database::types::*;
                use corpus_database::RelationElement;
                use corpus_database::VecOfRelationElementAdapter;
                use corpus_database::VecIntoRelationElementAdapter;
                let selected_thir_bodies = loader
                    .load_selected_thir_bodies()
                    .to_tuple_vec();
                let thir_blocks = loader.load_thir_blocks().to_tuple_vec();
                {
                    let mut iteration = datafrog::Iteration::new();
                    let var_selected_thir_bodies = datafrog::Relation::<
                        (Build, Item, DefPath, ThirBlock),
                    >::from_vec(selected_thir_bodies);
                    let var_thir_blocks = datafrog::Relation::<
                        (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span),
                    >::from_vec(thir_blocks);
                    let var_selected_thir_blocks = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                ThirBlock,
                                ThirBlock,
                                ScopeSafety,
                                BlockCheckMode,
                                Span,
                            ),
                        >("selected_thir_blocks");
                    let var_thir_blocks_1 = iteration
                        .variable::<
                            (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span),
                        >("thir_blocks_1");
                    let var_selected_thir_bodies_2 = iteration
                        .variable::<
                            (Build, Item, DefPath, ThirBlock),
                        >("selected_thir_bodies_2");
                    let var_thir_blocks_1_3 = iteration
                        .variable::<
                            (
                                (ThirBlock,),
                                (ThirBlock, ScopeSafety, BlockCheckMode, Span),
                            ),
                        >("thir_blocks_1_3");
                    let var_selected_thir_bodies_2_4 = iteration
                        .variable::<
                            ((ThirBlock,), (Build, DefPath)),
                        >("selected_thir_bodies_2_4");
                    let var_selected_thir_blocks_5 = iteration
                        .variable::<
                            (
                                ThirBlock,
                                ThirBlock,
                                ScopeSafety,
                                BlockCheckMode,
                                Span,
                                Build,
                                DefPath,
                            ),
                        >("selected_thir_blocks_5");
                    let var_selected_thir_blocks_6 = iteration
                        .variable::<
                            ((ThirBlock,), (Build, DefPath)),
                        >("selected_thir_blocks_6");
                    let var_thir_blocks_1_7 = iteration
                        .variable::<
                            (
                                (ThirBlock,),
                                (ThirBlock, ScopeSafety, BlockCheckMode, Span),
                            ),
                        >("thir_blocks_1_7");
                    let var_selected_thir_blocks_8 = iteration
                        .variable::<
                            (
                                ThirBlock,
                                Build,
                                DefPath,
                                ThirBlock,
                                ScopeSafety,
                                BlockCheckMode,
                                Span,
                            ),
                        >("selected_thir_blocks_8");
                    var_thir_blocks_1.insert(var_thir_blocks);
                    var_selected_thir_bodies_2.insert(var_selected_thir_bodies);
                    while iteration.changed() {
                        var_thir_blocks_1_3
                            .from_map(
                                &var_thir_blocks_1,
                                |&(parent, block, safety, check_mode, span)| (
                                    (parent,),
                                    (block, safety, check_mode, span),
                                ),
                            );
                        var_selected_thir_bodies_2_4
                            .from_map(
                                &var_selected_thir_bodies_2,
                                |&(build, _, thir_body_def_path, parent)| (
                                    (parent,),
                                    (build, thir_body_def_path),
                                ),
                            );
                        var_selected_thir_blocks_5
                            .from_join(
                                &var_thir_blocks_1_3,
                                &var_selected_thir_bodies_2_4,
                                |
                                    &(parent,),
                                    &(block, safety, check_mode, span),
                                    &(build, thir_body_def_path)|
                                (
                                    parent,
                                    block,
                                    safety,
                                    check_mode,
                                    span,
                                    build,
                                    thir_body_def_path,
                                ),
                            );
                        var_selected_thir_blocks
                            .from_map(
                                &var_selected_thir_blocks_5,
                                |
                                    &(
                                        parent,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                        build,
                                        thir_body_def_path,
                                    )|
                                (
                                    build,
                                    thir_body_def_path,
                                    parent,
                                    block,
                                    safety,
                                    check_mode,
                                    span,
                                ),
                            );
                        var_selected_thir_blocks_6
                            .from_map(
                                &var_selected_thir_blocks,
                                |&(build, thir_body_def_path, _, parent, _, _, _)| (
                                    (parent,),
                                    (build, thir_body_def_path),
                                ),
                            );
                        var_thir_blocks_1_7
                            .from_map(
                                &var_thir_blocks_1,
                                |&(parent, block, safety, check_mode, span)| (
                                    (parent,),
                                    (block, safety, check_mode, span),
                                ),
                            );
                        var_selected_thir_blocks_8
                            .from_join(
                                &var_selected_thir_blocks_6,
                                &var_thir_blocks_1_7,
                                |
                                    &(parent,),
                                    &(build, thir_body_def_path),
                                    &(block, safety, check_mode, span)|
                                (
                                    parent,
                                    build,
                                    thir_body_def_path,
                                    block,
                                    safety,
                                    check_mode,
                                    span,
                                ),
                            );
                        var_selected_thir_blocks
                            .from_map(
                                &var_selected_thir_blocks_8,
                                |
                                    &(
                                        parent,
                                        build,
                                        thir_body_def_path,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                    )|
                                (
                                    build,
                                    thir_body_def_path,
                                    parent,
                                    block,
                                    safety,
                                    check_mode,
                                    span,
                                ),
                            );
                    }
                    selected_thir_blocks = var_selected_thir_blocks.complete();
                }
            }
        };
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!(
                        "selected_thir_blocks.len = {0}",
                        selected_thir_blocks.len(),
                    ),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        loader.store_selected_thir_blocks(selected_thir_blocks.elements);
        let selected_thir_blocks = loader.load_selected_thir_blocks();
        let def_path_resolver = DefPathResolver::new(loader);
        let span_resolver = SpanResolver::new(loader);
        let strings = loader.load_strings();
        let mut unsafe_thir_blocks_relation = Vec::new();
        let mut unsafe_thir_blocks = Vec::new();
        let mut unsafe_thir_block_to_build_and_checkmode = HashMap::new();
        for (build, thir_body_def_path, _parent, block, _safety, check_mode, span) in selected_thir_blocks
            .tuple_iter()
            .filter(|
                (
                    _build,
                    _thir_body_def_path,
                    _parent,
                    _block,
                    safety,
                    _check_mode,
                    _span,
                )|
            { *safety == types::ScopeSafety::ExplicitUnsafe })
        {
            unsafe_thir_blocks_relation
                .push((
                    build,
                    thir_body_def_path,
                    block,
                    span_resolver.get_expansion_kind(span),
                    check_mode,
                    span,
                ));
            unsafe_thir_blocks
                .push((
                    build,
                    def_path_resolver.resolve(thir_body_def_path),
                    block,
                    check_mode.to_string(),
                    span_resolver.resolve(span),
                ));
            unsafe_thir_block_to_build_and_checkmode.insert(block, (build, check_mode));
        }
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!(
                        "Computed unsafe thir blocks: {0}",
                        unsafe_thir_blocks.len(),
                    ),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        loader.store_unsafe_thir_blocks(unsafe_thir_blocks_relation);
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe thir blocks."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "unsafe_thir_blocks"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in unsafe_thir_blocks {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe thir block report."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let unsafe_thir_blocks_relation = loader.load_unsafe_thir_blocks();
        let unsafe_thir_statements = || {
            let thir_statements = loader.load_iter_thir_stmts();
            thir_statements
                .filter_map(|(stmt, _block, closest_unsafe_block, index)| {
                    let &(build, check_mode) = unsafe_thir_block_to_build_and_checkmode
                        .get(&closest_unsafe_block)?;
                    Some((build, stmt, closest_unsafe_block, index, check_mode))
                })
        };
        loader.store_iter_unsafe_thir_stmts(unsafe_thir_statements());
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe thir statements."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let unsafe_thir_statements = unsafe_thir_statements();
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "unsafe_thir_statements"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in unsafe_thir_statements {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Saved unsafe thir statement report."),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let functions_unsafe_thir_blocks;
        {
            #[allow(dead_code)]
            enum ProcMacroHack {
                Value = (
                    "load loader { relations(selected_thir_bodies, unsafe_thir_blocks), } output\nfunctions_unsafe_thir_blocks(build: Build, function: Item, block: ThirBlock,\nexpansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)\nfunctions_unsafe_thir_blocks(build, function, block, expansion_kind,\ncheck_mode) :-\nunsafe_thir_blocks(build, thir_body_def_path, block, expansion_kind,\ncheck_mode, _), selected_thir_bodies(build, function, thir_body_def_path, _).",
                    0,
                )
                    .1,
            }
            {
                use corpus_database::types::*;
                use corpus_database::RelationElement;
                use corpus_database::VecOfRelationElementAdapter;
                use corpus_database::VecIntoRelationElementAdapter;
                let selected_thir_bodies = loader
                    .load_selected_thir_bodies()
                    .to_tuple_vec();
                let unsafe_thir_blocks = loader.load_unsafe_thir_blocks().to_tuple_vec();
                {
                    let mut iteration = datafrog::Iteration::new();
                    let var_selected_thir_bodies = datafrog::Relation::<
                        (Build, Item, DefPath, ThirBlock),
                    >::from_vec(selected_thir_bodies);
                    let var_unsafe_thir_blocks = datafrog::Relation::<
                        (
                            Build,
                            DefPath,
                            ThirBlock,
                            SpanExpansionKind,
                            BlockCheckMode,
                            Span,
                        ),
                    >::from_vec(unsafe_thir_blocks);
                    let var_functions_unsafe_thir_blocks = iteration
                        .variable::<
                            (Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode),
                        >("functions_unsafe_thir_blocks");
                    let var_unsafe_thir_blocks_1 = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                ThirBlock,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Span,
                            ),
                        >("unsafe_thir_blocks_1");
                    let var_selected_thir_bodies_2 = iteration
                        .variable::<
                            (Build, Item, DefPath, ThirBlock),
                        >("selected_thir_bodies_2");
                    let var_unsafe_thir_blocks_1_3 = iteration
                        .variable::<
                            (
                                (Build, DefPath),
                                (ThirBlock, SpanExpansionKind, BlockCheckMode),
                            ),
                        >("unsafe_thir_blocks_1_3");
                    let var_selected_thir_bodies_2_4 = iteration
                        .variable::<
                            ((Build, DefPath), (Item,)),
                        >("selected_thir_bodies_2_4");
                    let var_functions_unsafe_thir_blocks_5 = iteration
                        .variable::<
                            (
                                Build,
                                DefPath,
                                ThirBlock,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Item,
                            ),
                        >("functions_unsafe_thir_blocks_5");
                    var_unsafe_thir_blocks_1.insert(var_unsafe_thir_blocks);
                    var_selected_thir_bodies_2.insert(var_selected_thir_bodies);
                    while iteration.changed() {
                        var_unsafe_thir_blocks_1_3
                            .from_map(
                                &var_unsafe_thir_blocks_1,
                                |
                                    &(
                                        build,
                                        thir_body_def_path,
                                        block,
                                        expansion_kind,
                                        check_mode,
                                        _,
                                    )|
                                (
                                    (build, thir_body_def_path),
                                    (block, expansion_kind, check_mode),
                                ),
                            );
                        var_selected_thir_bodies_2_4
                            .from_map(
                                &var_selected_thir_bodies_2,
                                |&(build, function, thir_body_def_path, _)| (
                                    (build, thir_body_def_path),
                                    (function,),
                                ),
                            );
                        var_functions_unsafe_thir_blocks_5
                            .from_join(
                                &var_unsafe_thir_blocks_1_3,
                                &var_selected_thir_bodies_2_4,
                                |
                                    &(build, thir_body_def_path),
                                    &(block, expansion_kind, check_mode),
                                    &(function,)|
                                (
                                    build,
                                    thir_body_def_path,
                                    block,
                                    expansion_kind,
                                    check_mode,
                                    function,
                                ),
                            );
                        var_functions_unsafe_thir_blocks
                            .from_map(
                                &var_functions_unsafe_thir_blocks_5,
                                |
                                    &(
                                        build,
                                        thir_body_def_path,
                                        block,
                                        expansion_kind,
                                        check_mode,
                                        function,
                                    )|
                                (build, function, block, expansion_kind, check_mode),
                            );
                    }
                    functions_unsafe_thir_blocks = var_functions_unsafe_thir_blocks
                        .complete();
                }
            }
        }
        let functions_unsafe_thir_blocks = functions_unsafe_thir_blocks.elements;
        let function_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
            .iter()
            .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| {
                *function
            })
            .into_iter()
            .map(|(function, group)| (function, group.count()))
            .collect();
        let function_user_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
            .iter()
            .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
                *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
            })
            .safe_group_by(|(_build, function, _scope, _expansion_kind, _check_mode)| {
                *function
            })
            .into_iter()
            .map(|(function, group)| (function, group.count()))
            .collect();
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!(
                        "functions_unsafe_thir_blocks.len = {0}",
                        functions_unsafe_thir_blocks.len(),
                    ),
                    lvl,
                    &(
                        "corpus_manager::queries::counters",
                        "corpus_manager::queries::counters",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}.csv", "&functions_unsafe_thir_blocks"),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in &functions_unsafe_thir_blocks {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
        loader.store_functions_unsafe_thir_blocks(functions_unsafe_thir_blocks);
        let abis = loader.load_abis();
        let trait_items = loader.load_trait_items();
        let trait_items: HashSet<_> = trait_items
            .tuple_iter()
            .map(|(_trait_id, def_path, _defaultness)| def_path)
            .collect();
        let selected_function_definitions = loader.load_selected_function_definitions();
        let selected_function_definitions_thir_counts = selected_function_definitions
            .tuple_iter()
            .map(|
                (
                    build,
                    item,
                    def_path,
                    module,
                    visibility,
                    unsafety,
                    abi,
                    _return_ty,
                    uses_unsafe,
                )|
            {
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
                    function_unsafe_thir_block_counts.get(&item).cloned().unwrap_or(0),
                    function_user_unsafe_thir_block_counts
                        .get(&item)
                        .cloned()
                        .unwrap_or(0),
                    trait_items.contains(&def_path),
                )
            });
        if !report_path.exists() {
            std::fs::create_dir(report_path).unwrap();
        }
        let file_path = report_path
            .join(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "{0}.csv",
                            "selected_function_definitions_thir_counts",
                        ),
                    );
                    res
                }),
            );
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in selected_function_definitions_thir_counts {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
    }
}
