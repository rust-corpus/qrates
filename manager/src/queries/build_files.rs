//! Report information about custom build files (`build.rs`).

use crate::write_csv;
use cargo::core::Package;
use corpus_database::{
    tables::Loader,
    types::{Build, InternedString, PackageVersion},
};
use corpus_queries_derive::datapond_query;
use std::path::Path;

pub fn query(loader: &Loader, report_path: &Path) {
    let strings = loader.load_strings();
    let crate_names = loader.load_crate_names();
    let builds = loader.load_builds_as_vec();
    let package_names = loader.load_package_names();
    let package_versions = loader.load_package_versions();

    let Some(crate_name) = strings.lookup_str("build_script_build") else {
        // Provide empty csv
        let build_script_crates: Vec<(
            Build,
            InternedString,
            InternedString,
            InternedString,
            String,
        )> = Vec::new();
        write_csv!(report_path, build_script_crates);
        return;
    };
    let krate = crate_names.lookup(&crate_name).unwrap();

    let build_script_builds: Vec<_> = builds
        .iter()
        .filter(
            |(_build, _package, _version, build_crate, _crate_hash, _edition)| {
                krate == *build_crate
            },
        )
        .map(|&(build, package, version, krate, crate_hash, _edition)| {
            (build, package, version, krate, crate_hash)
        })
        .collect();
    let build_script_crates: Vec<_> = build_script_builds
        .iter()
        .map(|&(build, package, version, krate, crate_hash)| {
            (
                build,
                package_names[package],
                package_versions[version],
                crate_names[krate],
                format!("{:x}", crate_hash),
            )
        })
        .collect();
    write_csv!(report_path, build_script_crates);
    datapond_query! {
        load loader {
            interning_tables_as_relations(def_paths),
            relations(mir_cfgs, subscopes),
        }
        input build_script_builds(
            build: Build,
            package: Package,
            version: PackageVersion,
            krate: Krate,
            crate_hash: CrateHash,
        )

        internal build_scopes(build: Build, scope: Scope)
        build_scopes(build, scope) :-
            build_script_builds(.build=build, .krate=krate, .crate_hash=crate_hash),
            def_paths(.def_path=def_path, .krate=krate, .crate_hash=crate_hash),
            mir_cfgs(_, def_path, scope).
        build_scopes(build, scope) :-
            build_scopes(build, parent),
            subscopes(parent, scope, _, _, _, _).
    };
}
