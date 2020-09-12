//! Report information about traits and their implementations.

use super::utils::DefPathResolver;
use super::utils::GroupByIterator;
use crate::write_csv;
use corpus_database::tables::Loader;
use log::info;
use std::collections::HashMap;
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let selected_builds = loader.load_selected_builds();
    let def_paths = loader.load_def_paths();
    let all_traits_relation = loader.load_traits();
    let def_path_resolver = DefPathResolver::new(loader);
    info!("Loaded relations.");

    let all_traits = all_traits_relation.iter().map(
        |&(item, def_path, _name, visibility, is_auto, is_marker, unsafety)| {
            (
                def_path_resolver.resolve(def_path),
                item,
                visibility.to_string(),
                unsafety.to_string(),
                is_auto,
                is_marker,
            )
        },
    );
    info!("Writing CSV of all_traits.len={}", all_traits.len());
    write_csv!(report_path, all_traits);

    let selected_traits_relation = super::utils::filter_selected(
        all_traits_relation.iter(),
        &selected_builds,
        &def_paths,
        |&(_item, def_path, _name, _visibility, _is_auto, _is_marker, _unsafety)| def_path,
        |build, &(item, def_path, name, visibility, is_auto, is_marker, unsafety)| {
            (
                build, item, def_path, name, visibility, is_auto, is_marker, unsafety,
            )
        },
    );

    info!("selected_traits.len = {}", selected_traits_relation.len());

    let trait_impls = loader.load_trait_impls();
    let trait_impl_counts: HashMap<_, _> = trait_impls
        .iter()
        .safe_group_by(|(_item, _typ, trait_def_path)| trait_def_path)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .collect();

    let selected_traits = selected_traits_relation.iter().map(
        |&(build, item, def_path, _name, visibility, is_auto, is_marker, unsafety)| {
            (
                build,
                def_path_resolver.resolve(def_path),
                item,
                visibility.to_string(),
                unsafety.to_string(),
                is_auto,
                is_marker,
                trait_impl_counts.get(&def_path).cloned().unwrap_or(0),
            )
        },
    );

    info!(
        "Writing CSV of selected_traits.len={}",
        selected_traits.len()
    );
    write_csv!(report_path, selected_traits);

    let selected_impl_definitions_relation = super::utils::filter_selected(
        loader.load_impl_definitions().iter(),
        &selected_builds,
        &def_paths,
        |&(
            def_path,
            _item,
            _module,
            _name,
            _visibility,
            _unsafety,
            _polarity,
            _defaultness,
            _constness,
            _typ,
        )| def_path,
        |build,
         &(
            def_path,
            item,
            module,
            name,
            visibility,
            unsafety,
            polarity,
            defaultness,
            constness,
            typ,
        )| {
            (
                build,
                def_path,
                item,
                module,
                name,
                visibility,
                unsafety,
                polarity,
                defaultness,
                constness,
                typ,
            )
        },
    );
    info!(
        "selected_impl_definitions.len = {}",
        selected_impl_definitions_relation.len()
    );

    let impl_traits: HashMap<_, _> = trait_impls
        .iter()
        .map(|(item, _typ, trait_def_path)| (item, trait_def_path))
        .collect();
    let selected_impl_definitions = selected_impl_definitions_relation.into_iter().flat_map(
        |(
            build,
            def_path,
            item,
            _module,
            _name,
            visibility,
            unsafety,
            polarity,
            defaultness,
            constness,
            _typ,
        )| {
            impl_traits.get(&item).map(|&trait_def_path| {
                (
                    build,
                    def_path_resolver.resolve(def_path),
                    item,
                    visibility.to_string(),
                    unsafety.to_string(),
                    polarity.to_string(),
                    defaultness.to_string(),
                    constness.to_string(),
                    def_path_resolver.resolve(*trait_def_path),
                )
            })
        },
    );

    info!("Writing CSV of selected impl definitions of traits");
    write_csv!(report_path, selected_impl_definitions);
}
