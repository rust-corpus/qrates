use super::utils::GroupByIterator;
use crate::sources_list::CratesList;
use crate::write_csv;
use corpus_database::tables::Loader;
use itertools::Itertools;
use log::{error, info};
use std::collections::{HashMap, HashSet};
use std::path::Path;

fn collect_files(workspace_path: &Path) -> HashSet<(String, String)> {
    let mut set = HashSet::new();
    for package_dir in std::fs::read_dir(workspace_path.join("rust-corpus")).unwrap() {
        let mut path = package_dir.unwrap().path();
        let package_name = path.file_name().unwrap().to_str().unwrap().to_owned();
        path.push("files.json");
        if path.exists() {
            let file = std::fs::File::open(&path)
                .unwrap_or_else(|err| panic!("Failed to open {:?}: {}", path, err));
            let package_files: Vec<std::ffi::OsString> = serde_json::from_reader(file)
                .unwrap_or_else(|err| panic!("Failed to read CSV {:?}: {}", path, err));
            for package_file in package_files {
                set.insert((
                    package_name.clone(),
                    package_file.to_str().unwrap().to_owned(),
                ));
            }
        }
    }
    set
}

/// Filter the `builds` relation to contain only the builds we are “interested”
/// in analysing.
///
/// The filter performs the following:
///
/// 1.  Removes build scripts.
/// 2.  Takes the builds that are builds of the package and appear in the same
///     package directory.
/// 3.  Picks builds for all other packages so that we have a build for each
///     package/crate.
pub fn query(loader: &Loader, report_path: &Path, workspace_path: &Path, sources_list_path: &Path) {
    let crates_list = CratesList::load(sources_list_path);
    let original_crates_set: HashSet<_> = crates_list
        .iter()
        .map(|krate| (krate.name().to_string(), krate.version().to_string()))
        .collect();
    let original_crates_list = original_crates_set.iter();
    write_csv!(report_path, original_crates_list);

    let files = collect_files(workspace_path);

    let mut chosen_packages: HashMap<_, _> = crates_list
        .iter()
        .map(|krate| (krate.name().to_string(), false))
        .collect();
    info!(
        "The number of packages in the initial list: {}.",
        chosen_packages.len()
    );

    let strings = loader.load_strings();
    let builds = loader.load_builds_as_vec();
    let package_names = loader.load_package_names();
    let package_versions = loader.load_package_versions();
    let crate_names = loader.load_crate_names();
    let editions = loader.load_editions();
    let crate_types = loader.load_build_crate_types();
    let crate_types: HashMap<_, Vec<_>> = crate_types
        .iter()
        .safe_group_by(|&(build, _)| build)
        .into_iter()
        .map(|(build, group)| {
            (
                build,
                group.map(|(_build, crate_type)| *crate_type).collect(),
            )
        })
        .collect();

    // Builds of `build.rs` files.
    let mut build_script_builds = Vec::new();
    let mut build_script_builds_relation = Vec::new();

    let mut selected_builds_relation = Vec::new();
    let mut selected_builds = Vec::new();
    let mut candidate_builds_relation = Vec::new();
    let mut candidate_builds = Vec::new();
    let mut included_packages = HashSet::new();
    let mut all_builds = Vec::new();

    let mut drop_count = 0;
    let mut ignored_builds_count = 0;

    info!("Number of builds in total: {}", builds.len());

    for &(build, package, version, krate, crate_hash, edition) in builds.iter() {
        let krate_str = strings[crate_names[krate]].to_string();
        let package_str = strings[package_names[package]].clone();
        let version_str = strings[package_versions[version]].clone();
        let edition_str = strings[editions[edition]].to_string();
        all_builds.push((
            build,
            package_str.clone(),
            version_str.clone(),
            krate_str.clone(),
            format!("{:x}", crate_hash),
            edition_str.clone(),
        ));
        if krate_str.starts_with("build_script_") {
            build_script_builds_relation
                .push((build, package, version, krate, crate_hash, edition));
            build_script_builds.push((
                build,
                package_str,
                version_str,
                krate_str,
                format!("{:x}", crate_hash),
                edition_str,
            ));
        } else {
            let crate_types = if let Some(types) = crate_types.get(&build) {
                types
                    .iter()
                    .map(|&crate_type| &strings[crate_type])
                    .sorted()
                    .join(", ")
            } else {
                String::from("")
            };

            let directory_name = format!("{}-{}", package_str, version_str);
            let file_name = format!("{}_{:x}.bincode", krate_str, crate_hash);

            if files.contains(&(directory_name, file_name)) {
                let is_package_chosen = chosen_packages.get_mut(&package_str).unwrap();
                let key = (package_str, version_str);
                assert!(
                    original_crates_set.contains(&key),
                    "Including unexpected package version: {:?}",
                    key
                );
                let (package_str, version_str) = key;
                let included_key = (package_str, krate_str, crate_types);
                if !included_packages.contains(&included_key) {
                    included_packages.insert(included_key.clone());
                } else {
                    let (package_str, krate_str, crate_types) = included_key;
                    if crate_types == "proc-macro" {
                        // It seems that cargo compiles procedural macros twice
                        // to produce different output files.
                    } else if package_str == "charmhelpers" && version_str == "0.1.3" {
                        // charmhelpers transitively depends on itself and, as a
                        // result, is compiled twice.
                    } else {
                        error!(
                            "Dropping build to avoid duplicate entries: {} {} / {} {:x} {}.",
                            package_str, version_str, krate_str, crate_hash, crate_types,
                        );
                        drop_count += 1;
                    }
                    continue;
                }
                let (package_str, krate_str, crate_types) = included_key;
                selected_builds_relation
                    .push((build, package, version, krate, crate_hash, edition));
                selected_builds.push((
                    build,
                    package_str.clone(),
                    version_str,
                    krate_str,
                    format!("{:x}", crate_hash),
                    edition_str,
                    crate_types,
                ));
                *is_package_chosen = true;
            } else {
                candidate_builds_relation
                    .push((build, package, version, krate, crate_hash, edition));
                candidate_builds.push((
                    build,
                    package_str.clone(),
                    version_str,
                    krate_str,
                    format!("{:x}", crate_hash),
                    edition_str,
                    crate_types,
                ));
                ignored_builds_count += 1;
            }
        }
    }
    info!(
        "Included builds with the default configuration: {}",
        included_packages.len()
    );
    assert_eq!(drop_count, 0, "There were duplicates!");
    info!(
        "Number of unmatched builds after choosing only the default ones: {}",
        ignored_builds_count
    );
    info!(
        "Number of selected builds after choosing only the default ones: {}",
        selected_builds_relation.len()
    );
    info!(
        "Number of build scripts: {}",
        build_script_builds_relation.len()
    );
    let mut taken_candidates = HashSet::new();
    // There are some packages that did not compile with the default options,
    // but compiled as dependencies. Include them into the selected builds.
    for (relation, row) in candidate_builds_relation
        .into_iter()
        .zip(candidate_builds.into_iter())
    {
        let (build, package_str, version_str, krate_str, crate_hash, edition_str, crate_types) =
            row;
        let key = (package_str, version_str);
        if !original_crates_set.contains(&key) {
            continue;
        }
        let (package_str, version_str) = key;
        let key = (package_str, krate_str, crate_types);
        if taken_candidates.contains(&key) {
            let (package_str, krate_str, crate_types) = key;
            info!(
                "Duplicate candidate: {} {} / {} {} {}.",
                package_str, version_str, krate_str, crate_hash, crate_types,
            );
        } else if !included_packages.contains(&key) {
            included_packages.insert(key.clone());
            taken_candidates.insert(key.clone());
            // Not included package. Include it.
            let (package_str, krate_str, crate_types) = key;
            selected_builds.push((
                build,
                package_str,
                version_str,
                krate_str,
                crate_hash,
                edition_str,
                crate_types,
            ));
            selected_builds_relation.push(relation);
        }
    }
    info!(
        "Included builds with additional ones from the dependencies: {}",
        included_packages.len()
    );
    info!(
        "Number of unmatched builds after choosing only the default ones: {}",
        ignored_builds_count
    );
    info!(
        "Number of selected builds after choosing only the default ones: {}",
        selected_builds_relation.len()
    );
    loader.store_selected_builds(selected_builds_relation);
    loader.store_build_script_builds(build_script_builds_relation);
    write_csv!(report_path, selected_builds);
    write_csv!(report_path, build_script_builds);
    write_csv!(report_path, all_builds);
    let mut chosen_packages_count = 0;
    let mut not_chosen_packages_count = 0;
    for val in chosen_packages.values() {
        if *val {
            chosen_packages_count += 1;
        } else {
            not_chosen_packages_count += 1;
        }
    }
    info!(
        "Number of packages from which at least one build was selected: {}",
        chosen_packages_count
    );
    info!(
        "Number of packages from which no build was selected: {}",
        not_chosen_packages_count
    );
}
