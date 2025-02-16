use corpus_database::tables::Loader;
use corpus_queries_derive::datapond_query;
use log::info;
use std::collections::HashMap;

fn compute_selected_modules(loader: &Loader) {
    let selected_modules;
    datapond_query! {
        load loader {
            relations(selected_builds, root_modules, submodules),
        }
        output selected_modules(build: Build, module: Module)
        selected_modules(build, module) :- selected_builds(.build=build), root_modules(build, module).
        selected_modules(build, module) :- selected_modules(build, parent), submodules(.parent=parent, .child=module).
    }
    info!("Computed modules.");
    loader.store_selected_modules(selected_modules.elements);
    info!("Stored modules.");
}

/// Check that `(krate, crate_hash)` is a key. In other words, check that
/// `(krate, crate_hash)` uniquely identifies `build`.
fn check_crate_with_hash_is_key(loader: &Loader) {
    let builds = loader.load_builds_as_vec();
    let mut keys = HashMap::new();
    for &(build, _package, _version, krate, crate_hash, _edition) in builds.iter() {
        let key = (krate, crate_hash);
        assert!(
            !keys.contains_key(&key),
            "Duplicate for: {:?} {:?}",
            krate,
            crate_hash
        );
        keys.insert(key, build);
    }
}

fn compute_selected_functions_and_mir_cfgs(loader: &Loader) {
    check_crate_with_hash_is_key(loader);
    let selected_builds = loader.load_selected_builds();
    let def_paths = loader.load_def_paths();

    let selected_mir_cfgs = super::utils::filter_selected(
        loader.load_mir_cfgs().iter(),
        &selected_builds,
        &def_paths,
        |&(_item, body_def_path, _root_scope)| body_def_path,
        |build, &(item, body_def_path, root_scope)| (build, item, body_def_path, root_scope),
    );
    info!("selected_mir_cfgs.len = {}", selected_mir_cfgs.len());
    loader.store_selected_mir_cfgs(selected_mir_cfgs);

    // additionally compute selected thir bodies
    let selected_thir_bodies: Vec<(
        corpus_database::types::Build,
        corpus_database::types::Item,
        corpus_database::types::DefPath,
        corpus_database::types::ThirBlock,
    )> = super::utils::filter_selected(
        loader.load_thir_bodies().iter(),
        &selected_builds,
        &def_paths,
        |&(_item, body_def_path, _root_block)| body_def_path,
        |build, &(item, body_def_path, root_block)| (build, item, body_def_path, root_block),
    );
    info!("selected_thir_bodies.len = {}", selected_thir_bodies.len());
    loader.store_selected_thir_bodies(selected_thir_bodies);

    let function_unsafe_use: HashMap<_, _> =
        loader.load_function_unsafe_use().iter().cloned().collect();

    let selected_functions = super::utils::filter_selected(
        loader.load_function_definitions().iter(),
        &selected_builds,
        &def_paths,
        |&(_item, def_path, _module, _visibility, _unsafety, _abi, _return_ty)| def_path,
        |build, &(item, def_path, module, visibility, unsafety, abi, return_ty)| {
            let uses_unsafe = function_unsafe_use.get(&def_path).cloned().unwrap_or(false);
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
            )
        },
    );
    loader.store_selected_function_definitions(selected_functions);
}

/// Prepare the list of functions, which belong to the builds we want
/// to analyse (`selected_builds`).
pub fn query(loader: &Loader) {
    compute_selected_modules(loader);
    compute_selected_functions_and_mir_cfgs(loader);
}
