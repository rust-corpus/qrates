use super::utils::{BuildResolver, DefPathResolver};
use crate::write_csv;
use corpus_database::tables::Loader;
use log::info;
use std::collections::HashMap;
use std::path::Path;

/// Collect general information about types.
pub fn query(loader: &Loader, report_path: &Path) {
    let def_path_resolver = DefPathResolver::new(loader);
    let build_resolver = BuildResolver::new(loader);

    let selected_builds = loader.load_selected_builds();
    let def_paths = loader.load_def_paths();
    let strings = loader.load_strings();
    let type_defs = loader.load_type_defs();
    let type_kinds = loader.load_type_kinds();
    let types = loader.load_types_redb_map();
    // let types: HashMap<_, _> = loader.load_iter_types().collect();

    info!(
        "Number of all type definitions (type_defs): {}",
        type_defs.len()
    );
    let selected_type_defs_relation = super::utils::filter_selected(
        type_defs.tuple_iter(),
        selected_builds.tuple_iter(),
        &def_paths,
        |(_item, _typ, def_path, _name, _visibility, _kind)| def_path,
        |build, (item, typ, def_path, name, visibility, kind)| {
            (
                build,
                item,
                typ,
                def_path,
                name,
                visibility,
                types.r(typ),
                kind,
            )
        },
    );
    info!(
        "Number of selected type definitions (type_defs): {}",
        selected_type_defs_relation.len()
    );

    let selected_type_defs = selected_type_defs_relation.iter().map(
        |&(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
            (
                build_resolver.resolve(build),
                item,
                typ,
                def_path_resolver.resolve(def_path),
                strings.r(name),
                visibility.to_string(),
                strings.r(type_kinds.r(type_kind)),
                def_kind.to_string(),
            )
        },
    );
    write_csv!(report_path, selected_type_defs);

    // let adts: HashMap<_, _> = loader
    //     .load_types_adt_def()
    //     .iter()
    //     .map(|&(typ, def_path, kind, c_repr, is_phantom)| {
    //         (typ, (def_path, kind, c_repr, is_phantom))
    //     })
    //     .collect();

    let adts = loader.load_types_adt_def_redb_map();

    let selected_adts_relation: Vec<_> = selected_type_defs_relation
        .iter()
        .flat_map(
            |&(build, item, typ, def_path, name, visibility, type_kind, def_kind)| {
                adts.get_redb(typ)
                    .map(|(resolved_def_path, kind, c_repr, is_phantom)| {
                        (
                            build,
                            item,
                            typ,
                            def_path,
                            resolved_def_path,
                            name,
                            visibility,
                            type_kind,
                            def_kind,
                            kind,
                            c_repr,
                            is_phantom,
                        )
                    })
            },
        )
        .collect();
    info!("Number of selected ADTs: {}", selected_adts_relation.len());
    let selected_adts = selected_adts_relation.iter().map(
        |&(
            build,
            item,
            typ,
            def_path,
            resolved_def_path,
            name,
            visibility,
            type_kind,
            def_kind,
            kind,
            c_repr,
            is_phantom,
        )| {
            (
                build_resolver.resolve(build),
                item,
                typ,
                def_path_resolver.resolve(def_path),
                def_path_resolver.resolve(resolved_def_path),
                strings.r(name),
                visibility.to_string(),
                strings.r(type_kinds.r(type_kind)),
                def_kind.to_string(),
                kind.to_string(),
                c_repr,
                is_phantom,
            )
        },
    );
    write_csv!(report_path, selected_adts);

    let selected_adts_map: HashMap<_, _> = selected_adts_relation
        .iter()
        .map(
            |&(
                build,
                item,
                typ,
                def_path,
                resolved_def_path,
                name,
                visibility,
                type_kind,
                def_kind,
                kind,
                c_repr,
                is_phantom,
            )| {
                (
                    typ,
                    (
                        build,
                        item,
                        def_path,
                        resolved_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                    ),
                )
            },
        )
        .collect();

    let types_adt_field = loader.load_types_adt_field();
    let selected_adt_field_types_relation: Vec<_> = types_adt_field
        .tuple_iter()
        .flat_map(
            |(
                _field,
                adt,
                adt_variant,
                field_def_path,
                field_name,
                field_visibility,
                field_type,
            )| {
                selected_adts_map.get(&adt).map(
                    |&(
                        build,
                        item,
                        adt_def_path,
                        resolved_adt_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                    )| {
                        (
                            build,
                            item,
                            adt,
                            adt_variant,
                            adt_def_path,
                            resolved_adt_def_path,
                            field_def_path,
                            name,
                            visibility,
                            type_kind,
                            def_kind,
                            kind,
                            c_repr,
                            is_phantom,
                            field_name,
                            field_visibility,
                            field_type,
                            types.r(field_type),
                        )
                    },
                )
            },
        )
        .collect();
    info!(
        "Number of selected ADT fields: {}",
        selected_adt_field_types_relation.len()
    );
    let selected_adt_field_types = selected_adt_field_types_relation.iter().map(
        |&(
            build,
            item,
            adt,
            adt_variant,
            adt_def_path,
            resolved_adt_def_path,
            field_def_path,
            name,
            visibility,
            type_kind,
            def_kind,
            kind,
            c_repr,
            is_phantom,
            field_name,
            field_visibility,
            field_type,
            field_type_kind,
        )| {
            (
                build_resolver.resolve(build),
                item,
                adt,
                adt_variant,
                def_path_resolver.resolve(adt_def_path),
                def_path_resolver.resolve(resolved_adt_def_path),
                def_path_resolver.resolve(field_def_path),
                (
                    strings.r(name),
                    visibility.to_string(),
                    strings.r(type_kinds.r(type_kind)),
                    def_kind.to_string(),
                    kind.to_string(),
                    c_repr,
                    is_phantom,
                ),
                strings.r(field_name),
                field_visibility.to_string(),
                field_type,
                strings.r(type_kinds.r(field_type_kind)),
            )
        },
    );

    write_csv!(report_path, selected_adt_field_types);

    loader.store_selected_adts(selected_adts_relation);
    loader.store_selected_type_defs(selected_type_defs_relation);
    loader.store_selected_adt_field_types(selected_adt_field_types_relation);
}
