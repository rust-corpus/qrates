#![deny(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/schema.rs"));

mod data_structures;
mod storage;

pub use self::data_structures::InterningTable;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_crate_names() {
        // Create and populate with crate names tables 1.
        let mut tables1 = tables::Tables::default();
        assert_eq!(tables1.interning_tables.strings.len(), 0);
        let crate_name_1_1 = tables1.register_crate_names(String::from("crate1"));
        assert_eq!(tables1.interning_tables.strings.len(), 1);
        assert_eq!(tables1.interning_tables.crate_names.len(), 1);
        let crate_name_1_2 = tables1.register_crate_names(String::from("crate2"));
        assert_ne!(crate_name_1_1, crate_name_1_2);
        assert_eq!(tables1.interning_tables.strings.len(), 2);
        assert_eq!(tables1.interning_tables.crate_names.len(), 2);
        let crate_name_1_3 = tables1.register_crate_names(String::from("crate1"));
        assert_eq!(crate_name_1_1, crate_name_1_3);
        assert_eq!(tables1.interning_tables.strings.len(), 2);
        assert_eq!(tables1.interning_tables.crate_names.len(), 2);

        // Create and populate with crate names tables 2.
        let mut tables2 = tables::Tables::default();
        assert_eq!(tables2.interning_tables.strings.len(), 0);
        let crate_name_2_1 = tables2.register_crate_names(String::from("crate2"));
        assert_eq!(tables2.interning_tables.strings.len(), 1);
        assert_eq!(tables2.interning_tables.crate_names.len(), 1);
        let crate_name_2_2 = tables2.register_crate_names(String::from("crate4"));
        assert_ne!(crate_name_2_1, crate_name_2_2);
        assert_eq!(tables2.interning_tables.strings.len(), 2);
        assert_eq!(tables2.interning_tables.crate_names.len(), 2);

        // Create and populate with crate names tables 3.
        let mut tables3 = tables::Tables::default();
        assert_eq!(tables3.interning_tables.strings.len(), 0);
        assert_eq!(tables3.interning_tables.strings.len(), 0);
        let crate_name_3_1 = tables3.register_crate_names(String::from("crate1"));
        assert_eq!(tables3.interning_tables.strings.len(), 1);
        assert_eq!(tables3.interning_tables.crate_names.len(), 1);
        let crate_name_3_2 = tables3.register_crate_names(String::from("crate4"));
        assert_ne!(crate_name_3_1, crate_name_3_2);
        assert_eq!(tables3.interning_tables.strings.len(), 2);
        assert_eq!(tables3.interning_tables.crate_names.len(), 2);

        // Merge tables 1 and 2.
        let mut merger1 = tables::TableMerger::new(tables1);
        merger1.merge(tables2);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 3);
        assert_eq!(merger1.tables.interning_tables.crate_names.len(), 3);
        let expected = ["crate1", "crate2", "crate4"];
        for (cn, expected_name) in merger1
            .tables
            .interning_tables
            .crate_names
            .iter_values()
            .zip(&expected)
        {
            assert_eq!(&merger1.tables.interning_tables.strings[*cn], expected_name);
        }

        // Merge tables 1 and 3.
        merger1.merge(tables3);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 3);
        assert_eq!(merger1.tables.interning_tables.crate_names.len(), 3);
        let expected = ["crate1", "crate2", "crate4"];
        for (cn, expected_name) in merger1
            .tables
            .interning_tables
            .crate_names
            .iter_values()
            .zip(&expected)
        {
            assert_eq!(&merger1.tables.interning_tables.strings[*cn], expected_name);
        }
    }
    #[test]
    fn test_builds_and_build_crate_types() {
        // Create and populate tables 1.
        let mut tables1 = tables::Tables::default();
        let build_1_1 = tables1.register_builds(
            String::from("package1"),
            String::from("version1"),
            String::from("crate1"),
            1u128.into(),
            String::from("edition1"),
        );
        assert_eq!(tables1.interning_tables.builds.len(), 1);
        assert_eq!(tables1.interning_tables.strings.len(), 4);
        assert_eq!(tables1.interning_tables.package_names.len(), 1);
        assert_eq!(tables1.interning_tables.package_versions.len(), 1);
        assert_eq!(tables1.interning_tables.editions.len(), 1);
        tables1.register_build_crate_types(build_1_1, String::from("crate_type1"));
        assert_eq!(tables1.interning_tables.strings.len(), 5);
        assert_eq!(tables1.relations.build_crate_types.len(), 1);
        let build_1_2 = tables1.register_builds(
            String::from("package2"),
            String::from("version2"),
            String::from("crate2"),
            2u128.into(),
            String::from("edition2"),
        );
        assert_ne!(build_1_1, build_1_2);
        assert_eq!(tables1.interning_tables.builds.len(), 2);
        assert_eq!(tables1.interning_tables.strings.len(), 9);
        assert_eq!(tables1.interning_tables.package_names.len(), 2);
        assert_eq!(tables1.interning_tables.package_versions.len(), 2);
        assert_eq!(tables1.interning_tables.editions.len(), 2);
        let build_1_3 = tables1.register_builds(
            String::from("package1"),
            String::from("version2"),
            String::from("crate1"),
            2u128.into(),
            String::from("edition1"),
        );
        assert_ne!(build_1_1, build_1_3);
        assert_ne!(build_1_2, build_1_3);
        assert_eq!(tables1.interning_tables.builds.len(), 3);
        assert_eq!(tables1.interning_tables.strings.len(), 9);
        assert_eq!(tables1.interning_tables.package_names.len(), 2);
        assert_eq!(tables1.interning_tables.package_versions.len(), 2);
        assert_eq!(tables1.interning_tables.editions.len(), 2);
        tables1.register_build_crate_types(build_1_3, String::from("crate_type1"));
        assert_eq!(tables1.interning_tables.strings.len(), 9);
        assert_eq!(tables1.relations.build_crate_types.len(), 2);

        // Create and populate tables 2.
        let mut tables2 = tables::Tables::default();
        let build_2_1 = tables2.register_builds(
            String::from("package1"),
            String::from("version1"),
            String::from("crate1"),
            1u128.into(),
            String::from("edition1"),
        );
        assert_eq!(tables2.interning_tables.builds.len(), 1);
        assert_eq!(tables2.interning_tables.strings.len(), 4);
        assert_eq!(tables2.interning_tables.package_names.len(), 1);
        assert_eq!(tables2.interning_tables.package_versions.len(), 1);
        assert_eq!(tables2.interning_tables.editions.len(), 1);
        tables2.register_build_crate_types(build_2_1, String::from("crate_type1"));
        assert_eq!(tables2.interning_tables.strings.len(), 5);
        assert_eq!(tables2.relations.build_crate_types.len(), 1);
        let build_2_2 = tables2.register_builds(
            String::from("package1"),
            String::from("version1"),
            String::from("crate1"),
            1u128.into(),
            String::from("edition1"),
        );
        assert_eq!(build_2_1, build_2_2);
        assert_eq!(tables2.interning_tables.builds.len(), 1);
        assert_eq!(tables2.interning_tables.strings.len(), 5);
        assert_eq!(tables2.interning_tables.package_names.len(), 1);
        assert_eq!(tables2.interning_tables.package_versions.len(), 1);
        assert_eq!(tables2.interning_tables.editions.len(), 1);
        tables2.register_build_crate_types(build_2_2, String::from("crate_type1"));
        assert_eq!(tables2.interning_tables.strings.len(), 5);
        assert_eq!(tables2.relations.build_crate_types.len(), 2);
        let build_2_3 = tables2.register_builds(
            String::from("package1"),
            String::from("version2"),
            String::from("crate1"),
            2u128.into(),
            String::from("edition1"),
        );
        assert_ne!(build_2_1, build_2_3);
        assert_ne!(build_2_2, build_2_3);
        assert_eq!(tables2.interning_tables.builds.len(), 2);
        assert_eq!(tables2.interning_tables.strings.len(), 6);
        assert_eq!(tables2.interning_tables.package_names.len(), 1);
        assert_eq!(tables2.interning_tables.package_versions.len(), 2);
        assert_eq!(tables2.interning_tables.editions.len(), 1);
        tables2.register_build_crate_types(build_2_3, String::from("crate_type1"));
        assert_eq!(tables2.interning_tables.strings.len(), 6);
        assert_eq!(tables2.relations.build_crate_types.len(), 3);

        // Create and populate tables 3.
        let mut tables3 = tables::Tables::default();
        let build_3_1 = tables3.register_builds(
            String::from("package4"),
            String::from("version4"),
            String::from("crate4"),
            4u128.into(),
            String::from("edition2"),
        );
        assert_eq!(tables3.interning_tables.builds.len(), 1);
        assert_eq!(tables3.interning_tables.strings.len(), 4);
        assert_eq!(tables3.interning_tables.package_names.len(), 1);
        assert_eq!(tables3.interning_tables.package_versions.len(), 1);
        assert_eq!(tables3.interning_tables.editions.len(), 1);
        tables3.register_build_crate_types(build_3_1, String::from("crate_type2"));
        assert_eq!(tables3.interning_tables.strings.len(), 5);
        assert_eq!(tables3.relations.build_crate_types.len(), 1);
        let build_3_2 = tables3.register_builds(
            String::from("package2"),
            String::from("version2"),
            String::from("crate2"),
            2u128.into(),
            String::from("edition2"),
        );
        assert_ne!(build_3_1, build_3_2);
        assert_eq!(tables3.interning_tables.builds.len(), 2);
        assert_eq!(tables3.interning_tables.strings.len(), 8);
        assert_eq!(tables3.interning_tables.package_names.len(), 2);
        assert_eq!(tables3.interning_tables.package_versions.len(), 2);
        assert_eq!(tables3.interning_tables.editions.len(), 1);
        tables3.register_build_crate_types(build_3_1, String::from("crate_type2"));
        assert_eq!(tables3.interning_tables.strings.len(), 8);
        assert_eq!(tables3.relations.build_crate_types.len(), 2);

        // Merge tables 1 and 2.
        let mut merger1 = tables::TableMerger::new(tables1);
        merger1.merge(tables2);
        assert_eq!(merger1.tables.interning_tables.builds.len(), 3);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 9);
        assert_eq!(merger1.tables.interning_tables.package_names.len(), 2);
        assert_eq!(merger1.tables.interning_tables.package_versions.len(), 2);
        assert_eq!(merger1.tables.interning_tables.editions.len(), 2);
        assert_eq!(merger1.tables.relations.build_crate_types.len(), 5);
        let expected = [
            ("package1", "version1", "crate1", 1u128.into(), "edition1"),
            ("package2", "version2", "crate2", 2u128.into(), "edition2"),
            ("package1", "version2", "crate1", 2u128.into(), "edition1"),
        ];
        for (build, expected_uild) in merger1
            .tables
            .interning_tables
            .builds
            .iter_values()
            .zip(&expected)
        {
            let &(package, version, krate, crate_hash, edition) = build;
            let (package_e, version_e, krate_e, crate_hash_e, edition_e) = expected_uild;
            let p = merger1.tables.interning_tables.package_names[package];
            assert_eq!(&merger1.tables.interning_tables.strings[p], package_e);
            let v = merger1.tables.interning_tables.package_versions[version];
            assert_eq!(&merger1.tables.interning_tables.strings[v], version_e);
            let k = merger1.tables.interning_tables.crate_names[krate];
            assert_eq!(&merger1.tables.interning_tables.strings[k], krate_e);
            assert_eq!(crate_hash, *crate_hash_e);
            let e = merger1.tables.interning_tables.editions[edition];
            assert_eq!(&merger1.tables.interning_tables.strings[e], edition_e);
        }

        // Merge tables 1 and 3.
        merger1.merge(tables3);
        assert_eq!(merger1.tables.interning_tables.builds.len(), 4);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 13);
        assert_eq!(merger1.tables.interning_tables.package_names.len(), 3);
        assert_eq!(merger1.tables.interning_tables.package_versions.len(), 3);
        assert_eq!(merger1.tables.interning_tables.crate_names.len(), 3);
        assert_eq!(merger1.tables.interning_tables.editions.len(), 2);
        assert_eq!(merger1.tables.relations.build_crate_types.len(), 7);
        let expected = [
            ("package1", "version1", "crate1", 1u128.into(), "edition1"),
            ("package2", "version2", "crate2", 2u128.into(), "edition2"),
            ("package1", "version2", "crate1", 2u128.into(), "edition1"),
            ("package4", "version4", "crate4", 4u128.into(), "edition2"),
        ];
        for (build, expected_uild) in merger1
            .tables
            .interning_tables
            .builds
            .iter_values()
            .zip(&expected)
        {
            let &(package, version, krate, crate_hash, edition) = build;
            let (package_e, version_e, krate_e, crate_hash_e, edition_e) = expected_uild;
            let p = merger1.tables.interning_tables.package_names[package];
            assert_eq!(&merger1.tables.interning_tables.strings[p], package_e);
            let v = merger1.tables.interning_tables.package_versions[version];
            assert_eq!(&merger1.tables.interning_tables.strings[v], version_e);
            let k = merger1.tables.interning_tables.crate_names[krate];
            assert_eq!(&merger1.tables.interning_tables.strings[k], krate_e);
            assert_eq!(crate_hash, *crate_hash_e);
            let e = merger1.tables.interning_tables.editions[edition];
            assert_eq!(&merger1.tables.interning_tables.strings[e], edition_e);
        }
    }
    #[test]
    fn test_submodules() {
        // Create and populate tables 1.
        fn create_table_1() -> tables::Tables {
            let mut tables1 = tables::Tables::default();
            let build_1_1 = tables1.register_builds(
                String::from("package1"),
                String::from("version1"),
                String::from("crate1"),
                1u128.into(),
                String::from("edition1"),
            );
            assert_eq!(tables1.interning_tables.builds.len(), 1);
            assert_eq!(tables1.interning_tables.strings.len(), 4);
            assert_eq!(tables1.interning_tables.package_names.len(), 1);
            assert_eq!(tables1.interning_tables.package_versions.len(), 1);
            assert_eq!(tables1.interning_tables.crate_names.len(), 1);
            assert_eq!(tables1.interning_tables.editions.len(), 1);
            let (root_1_1,) = tables1.register_root_modules(build_1_1);
            assert_eq!(tables1.relations.root_modules.len(), 1);
            let def_path_1_1 = tables1.register_def_paths(
                String::from("crate1"),
                1u128.into(),
                String::from("relative_def_id1"),
                (1u64, 2u64).into(),
                String::from("summary_1"),
            );
            assert_eq!(tables1.interning_tables.def_paths.len(), 1);
            assert_eq!(tables1.interning_tables.relative_def_paths.len(), 1);
            assert_eq!(tables1.interning_tables.summary_keys.len(), 1);
            assert_eq!(tables1.interning_tables.strings.len(), 6);
            let def_path_1_2 = tables1.register_def_paths(
                String::from("crate1"),
                1u128.into(),
                String::from("relative_def_id2"),
                (3u64, 4u64).into(),
                String::from("summary_2"),
            );
            assert_eq!(tables1.interning_tables.def_paths.len(), 2);
            assert_eq!(tables1.interning_tables.relative_def_paths.len(), 2);
            assert_eq!(tables1.interning_tables.summary_keys.len(), 2);
            assert_eq!(tables1.interning_tables.strings.len(), 8);
            let (module_1_1,) = tables1.register_submodules(
                def_path_1_1,
                root_1_1,
                String::from("mod1"),
                types::TyVisibility::Public,
                String::from("abi1"),
            );
            assert_eq!(tables1.relations.submodules.len(), 1);
            assert_eq!(tables1.interning_tables.names.len(), 1);
            assert_eq!(tables1.interning_tables.abis.len(), 1);
            assert_eq!(tables1.interning_tables.strings.len(), 10);
            tables1.register_submodules(
                def_path_1_2,
                module_1_1,
                String::from("mod2"),
                types::TyVisibility::Public,
                String::from("abi1"),
            );
            assert_eq!(tables1.relations.submodules.len(), 2);
            assert_eq!(tables1.interning_tables.names.len(), 2);
            assert_eq!(tables1.interning_tables.abis.len(), 1);
            assert_eq!(tables1.interning_tables.strings.len(), 11);
            tables1
        }
        let tables1 = create_table_1();
        let tables2 = create_table_1();

        // Create and populate tables 3.
        fn create_table_3() -> tables::Tables {
            let mut tables2 = tables::Tables::default();
            let build_2_1 = tables2.register_builds(
                String::from("package2"),
                String::from("version1"),
                String::from("crate2"),
                1u128.into(),
                String::from("edition1"),
            );
            assert_eq!(tables2.interning_tables.builds.len(), 1);
            assert_eq!(tables2.interning_tables.strings.len(), 4);
            assert_eq!(tables2.interning_tables.package_names.len(), 1);
            assert_eq!(tables2.interning_tables.package_versions.len(), 1);
            assert_eq!(tables2.interning_tables.crate_names.len(), 1);
            assert_eq!(tables2.interning_tables.editions.len(), 1);
            let (root_2_1,) = tables2.register_root_modules(build_2_1);
            assert_eq!(tables2.relations.root_modules.len(), 1);
            let def_path_2_1 = tables2.register_def_paths(
                String::from("crate2"),
                1u128.into(),
                String::from("relative_def_id1"),
                (1u64, 2u64).into(),
                String::from("summary_1"),
            );
            assert_eq!(tables2.interning_tables.def_paths.len(), 1);
            assert_eq!(tables2.interning_tables.relative_def_paths.len(), 1);
            assert_eq!(tables2.interning_tables.summary_keys.len(), 1);
            assert_eq!(tables2.interning_tables.strings.len(), 6);
            let def_path_2_2 = tables2.register_def_paths(
                String::from("crate2"),
                1u128.into(),
                String::from("relative_def_id2"),
                (3u64, 4u64).into(),
                String::from("summary_2"),
            );
            assert_eq!(tables2.interning_tables.def_paths.len(), 2);
            assert_eq!(tables2.interning_tables.relative_def_paths.len(), 2);
            assert_eq!(tables2.interning_tables.summary_keys.len(), 2);
            assert_eq!(tables2.interning_tables.strings.len(), 8);
            let (module_2_1,) = tables2.register_submodules(
                def_path_2_1,
                root_2_1,
                String::from("mod1"),
                types::TyVisibility::Public,
                String::from("abi1"),
            );
            assert_eq!(tables2.relations.submodules.len(), 1);
            assert_eq!(tables2.interning_tables.names.len(), 1);
            assert_eq!(tables2.interning_tables.abis.len(), 1);
            assert_eq!(tables2.interning_tables.strings.len(), 10);
            tables2.register_submodules(
                def_path_2_2,
                module_2_1,
                String::from("mod3"),
                types::TyVisibility::Public,
                String::from("abi2"),
            );
            assert_eq!(tables2.relations.submodules.len(), 2);
            assert_eq!(tables2.interning_tables.names.len(), 2);
            assert_eq!(tables2.interning_tables.abis.len(), 2);
            assert_eq!(tables2.interning_tables.strings.len(), 12);
            tables2
        }
        let tables3 = create_table_3();

        let mut merger1 = tables::TableMerger::new(tables1);
        merger1.merge(tables2);
        assert_eq!(merger1.tables.counters.modules, 6);
        assert_eq!(merger1.tables.interning_tables.builds.len(), 1);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 11);
        assert_eq!(merger1.tables.interning_tables.package_names.len(), 1);
        assert_eq!(merger1.tables.interning_tables.package_versions.len(), 1);
        assert_eq!(merger1.tables.interning_tables.crate_names.len(), 1);
        assert_eq!(merger1.tables.interning_tables.editions.len(), 1);
        assert_eq!(merger1.tables.relations.root_modules.len(), 2);
        assert_eq!(merger1.tables.interning_tables.def_paths.len(), 2);
        assert_eq!(merger1.tables.interning_tables.relative_def_paths.len(), 2);
        assert_eq!(merger1.tables.interning_tables.summary_keys.len(), 2);
        assert_eq!(merger1.tables.relations.submodules.len(), 4);
        assert_eq!(merger1.tables.interning_tables.names.len(), 2);
        assert_eq!(merger1.tables.interning_tables.abis.len(), 1);
        let expected = [
            (0usize.into(), 0usize.into()),
            (0usize.into(), 3usize.into()),
        ];
        for ((build, root_module), (build_e, root_module_e)) in
            merger1.tables.relations.root_modules.iter().zip(&expected)
        {
            assert_eq!(build, build_e);
            assert_eq!(root_module, root_module_e);
        }
        let expected = [
            (
                "crate1",
                1u128.into(),
                "relative_def_id1",
                (1u64, 2u64).into(),
                "summary_1",
                0usize.into(),
                1usize.into(),
                "mod1",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id2",
                (3u64, 4u64).into(),
                "summary_2",
                1usize.into(),
                2usize.into(),
                "mod2",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id1",
                (1u64, 2u64).into(),
                "summary_1",
                3usize.into(),
                4usize.into(),
                "mod1",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id2",
                (3u64, 4u64).into(),
                "summary_2",
                4usize.into(),
                5usize.into(),
                "mod2",
                types::TyVisibility::Public,
                "abi1",
            ),
        ];
        for (submodule, submodule_e) in merger1.tables.relations.submodules.iter().zip(&expected) {
            let &(def_path, parent, child, name, visibility, abi) = submodule;
            let (krate, crate_hash, relative_def_id, def_path_hash, summary_id) =
                merger1.tables.interning_tables.def_paths[def_path];
            let (
                krate_e,
                crate_hash_e,
                relative_def_id_e,
                def_path_hash_e,
                summary_id_e,
                parent_e,
                child_e,
                name_e,
                visibility_e,
                abi_e,
            ) = submodule_e;
            let k = merger1.tables.interning_tables.crate_names[krate];
            assert_eq!(&merger1.tables.interning_tables.strings[k], krate_e);
            assert_eq!(crate_hash, *crate_hash_e);
            let r = merger1.tables.interning_tables.relative_def_paths[relative_def_id];
            assert_eq!(
                &merger1.tables.interning_tables.strings[r],
                relative_def_id_e
            );
            assert_eq!(def_path_hash, *def_path_hash_e);
            let s = merger1.tables.interning_tables.summary_keys[summary_id];
            assert_eq!(&merger1.tables.interning_tables.strings[s], summary_id_e);
            assert_eq!(parent, *parent_e);
            assert_eq!(child, *child_e);
            let n = merger1.tables.interning_tables.names[name];
            assert_eq!(&merger1.tables.interning_tables.strings[n], name_e);
            assert_eq!(visibility, *visibility_e);
            let a = merger1.tables.interning_tables.abis[abi];
            assert_eq!(&merger1.tables.interning_tables.strings[a], abi_e);
        }

        merger1.merge(tables3);
        assert_eq!(merger1.tables.counters.modules, 9);
        assert_eq!(merger1.tables.interning_tables.builds.len(), 2);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 15);
        assert_eq!(merger1.tables.interning_tables.package_names.len(), 2);
        assert_eq!(merger1.tables.interning_tables.package_versions.len(), 1);
        assert_eq!(merger1.tables.interning_tables.crate_names.len(), 2);
        assert_eq!(merger1.tables.interning_tables.editions.len(), 1);
        assert_eq!(merger1.tables.relations.root_modules.len(), 3);
        assert_eq!(merger1.tables.interning_tables.def_paths.len(), 4);
        assert_eq!(merger1.tables.interning_tables.relative_def_paths.len(), 2);
        assert_eq!(merger1.tables.interning_tables.summary_keys.len(), 2);
        assert_eq!(merger1.tables.relations.submodules.len(), 6);
        assert_eq!(merger1.tables.interning_tables.names.len(), 3);
        assert_eq!(merger1.tables.interning_tables.abis.len(), 2);
        let expected = [
            (0usize.into(), 0usize.into()),
            (0usize.into(), 3usize.into()),
            (1usize.into(), 6usize.into()),
        ];
        for ((build, root_module), (build_e, root_module_e)) in
            merger1.tables.relations.root_modules.iter().zip(&expected)
        {
            assert_eq!(build, build_e);
            assert_eq!(root_module, root_module_e);
        }
        let expected = [
            (
                "crate1",
                1u128.into(),
                "relative_def_id1",
                (1u64, 2u64).into(),
                "summary_1",
                0usize.into(),
                1usize.into(),
                "mod1",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id2",
                (3u64, 4u64).into(),
                "summary_2",
                1usize.into(),
                2usize.into(),
                "mod2",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id1",
                (1u64, 2u64).into(),
                "summary_1",
                3usize.into(),
                4usize.into(),
                "mod1",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate1",
                1u128.into(),
                "relative_def_id2",
                (3u64, 4u64).into(),
                "summary_2",
                4usize.into(),
                5usize.into(),
                "mod2",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate2",
                1u128.into(),
                "relative_def_id1",
                (1u64, 2u64).into(),
                "summary_1",
                6usize.into(),
                7usize.into(),
                "mod1",
                types::TyVisibility::Public,
                "abi1",
            ),
            (
                "crate2",
                1u128.into(),
                "relative_def_id2",
                (3u64, 4u64).into(),
                "summary_2",
                7usize.into(),
                8usize.into(),
                "mod3",
                types::TyVisibility::Public,
                "abi2",
            ),
        ];
        for (submodule, submodule_e) in merger1.tables.relations.submodules.iter().zip(&expected) {
            let &(def_path, parent, child, name, visibility, abi) = submodule;
            let (krate, crate_hash, relative_def_id, def_path_hash, summary_id) =
                merger1.tables.interning_tables.def_paths[def_path];
            let (
                krate_e,
                crate_hash_e,
                relative_def_id_e,
                def_path_hash_e,
                summary_id_e,
                parent_e,
                child_e,
                name_e,
                visibility_e,
                abi_e,
            ) = submodule_e;
            let k = merger1.tables.interning_tables.crate_names[krate];
            assert_eq!(&merger1.tables.interning_tables.strings[k], krate_e);
            assert_eq!(crate_hash, *crate_hash_e);
            let r = merger1.tables.interning_tables.relative_def_paths[relative_def_id];
            assert_eq!(
                &merger1.tables.interning_tables.strings[r],
                relative_def_id_e
            );
            assert_eq!(def_path_hash, *def_path_hash_e);
            let s = merger1.tables.interning_tables.summary_keys[summary_id];
            assert_eq!(&merger1.tables.interning_tables.strings[s], summary_id_e);
            assert_eq!(parent, *parent_e);
            assert_eq!(child, *child_e);
            let n = merger1.tables.interning_tables.names[name];
            assert_eq!(&merger1.tables.interning_tables.strings[n], name_e);
            assert_eq!(visibility, *visibility_e);
            let a = merger1.tables.interning_tables.abis[abi];
            assert_eq!(&merger1.tables.interning_tables.strings[a], abi_e);
        }
    }
    #[test]
    fn test_spans() {
        // Create and populate tables 1.
        fn create_table_1() -> tables::Tables {
            let mut tables = tables::Tables::default();
            let root_span = tables.get_root_parent_span();
            let (span1,) = tables.register_spans(
                root_span,
                types::SpanExpansionKind::Root,
                String::from("expansion1"),
                String::from("location1"),
                1,
                1,
            );
            tables.register_spans(
                span1,
                types::SpanExpansionKind::Root,
                String::from("expansion2"),
                String::from("location2"),
                2,
                2,
            );
            tables.register_spans(
                span1,
                types::SpanExpansionKind::MacroDerive,
                String::from("expansion3"),
                String::from("location2"),
                2,
                2,
            );
            assert_eq!(tables.interning_tables.strings.len(), 5);
            assert_eq!(tables.interning_tables.span_file_names.len(), 2);
            assert_eq!(tables.relations.spans.len(), 3);
            assert_eq!(tables.counters.spans, 4);
            tables
        }
        let tables1 = create_table_1();
        let tables3 = create_table_1();

        // Create and populate tables 2.
        fn create_table_2() -> tables::Tables {
            let mut tables = tables::Tables::default();
            let root_span = tables.get_root_parent_span();
            let (span1,) = tables.register_spans(
                root_span,
                types::SpanExpansionKind::Root,
                String::from("expansion2"),
                String::from("location2"),
                2,
                2,
            );
            let (span2,) = tables.register_spans(
                span1,
                types::SpanExpansionKind::Root,
                String::from("expansion3"),
                String::from("location4"),
                4,
                4,
            );
            let (span3,) = tables.register_spans(
                span2,
                types::SpanExpansionKind::Root,
                String::from("expansion4"),
                String::from("location2"),
                2,
                2,
            );
            tables.register_spans(
                span3,
                types::SpanExpansionKind::Root,
                String::from("expansion5"),
                String::from("location7"),
                7,
                7,
            );
            assert_eq!(tables.interning_tables.strings.len(), 7);
            assert_eq!(tables.interning_tables.span_file_names.len(), 3);
            assert_eq!(tables.relations.spans.len(), 4);
            assert_eq!(tables.counters.spans, 5);
            tables
        }
        let tables2 = create_table_2();

        let mut merger1 = tables::TableMerger::new(tables1);
        merger1.merge(tables2);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 9);
        assert_eq!(merger1.tables.interning_tables.span_file_names.len(), 4);
        assert_eq!(merger1.tables.relations.spans.len(), 7);
        assert_eq!(merger1.tables.counters.spans, 8);
        let expected = [
            (
                1usize.into(),
                0usize.into(),
                types::SpanExpansionKind::Root,
                "expansion1",
                "location1",
                1,
                1,
            ),
            (
                2usize.into(),
                1usize.into(),
                types::SpanExpansionKind::Root,
                "expansion2",
                "location2",
                2,
                2,
            ),
            (
                3usize.into(),
                1usize.into(),
                types::SpanExpansionKind::MacroDerive,
                "expansion3",
                "location2",
                2,
                2,
            ),
            (
                4usize.into(),
                0usize.into(),
                types::SpanExpansionKind::Root,
                "expansion2",
                "location2",
                2,
                2,
            ),
            (
                5usize.into(),
                4usize.into(),
                types::SpanExpansionKind::Root,
                "expansion3",
                "location4",
                4,
                4,
            ),
            (
                6usize.into(),
                5usize.into(),
                types::SpanExpansionKind::Root,
                "expansion4",
                "location2",
                2,
                2,
            ),
            (
                7usize.into(),
                6usize.into(),
                types::SpanExpansionKind::Root,
                "expansion5",
                "location7",
                7,
                7,
            ),
        ];
        for (span, span_e) in merger1.tables.relations.spans.iter().zip(&expected) {
            let &(child, parent, kind, expansion_descr, file_name, line, col) = span;
            let (child_e, parent_e, kind_e, expansion_e, file_name_e, line_e, col_e) = span_e;
            assert_eq!(parent, *parent_e);
            assert_eq!(child, *child_e);
            assert_eq!(kind, *kind_e);
            assert_eq!(
                &merger1.tables.interning_tables.strings[expansion_descr],
                expansion_e
            );
            let l = merger1.tables.interning_tables.span_file_names[file_name];
            assert_eq!(&merger1.tables.interning_tables.strings[l], file_name_e);
            assert_eq!(line, *line_e);
            assert_eq!(col, *col_e);
        }

        merger1.merge(tables3);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 9);
        assert_eq!(merger1.tables.interning_tables.span_file_names.len(), 4);
        assert_eq!(merger1.tables.relations.spans.len(), 10);
        assert_eq!(merger1.tables.counters.spans, 11);
        let expected = [
            (
                1usize.into(),
                0usize.into(),
                types::SpanExpansionKind::Root,
                "expansion1",
                "location1",
                1,
                1,
            ),
            (
                2usize.into(),
                1usize.into(),
                types::SpanExpansionKind::Root,
                "expansion2",
                "location2",
                2,
                2,
            ),
            (
                3usize.into(),
                1usize.into(),
                types::SpanExpansionKind::MacroDerive,
                "expansion3",
                "location2",
                2,
                2,
            ),
            (
                4usize.into(),
                0usize.into(),
                types::SpanExpansionKind::Root,
                "expansion2",
                "location2",
                2,
                2,
            ),
            (
                5usize.into(),
                4usize.into(),
                types::SpanExpansionKind::Root,
                "expansion3",
                "location4",
                4,
                4,
            ),
            (
                6usize.into(),
                5usize.into(),
                types::SpanExpansionKind::Root,
                "expansion4",
                "location2",
                2,
                2,
            ),
            (
                7usize.into(),
                6usize.into(),
                types::SpanExpansionKind::Root,
                "expansion5",
                "location7",
                7,
                7,
            ),
            (
                8usize.into(),
                0usize.into(),
                types::SpanExpansionKind::Root,
                "expansion1",
                "location1",
                1,
                1,
            ),
            (
                9usize.into(),
                8usize.into(),
                types::SpanExpansionKind::Root,
                "expansion2",
                "location2",
                2,
                2,
            ),
            (
                10usize.into(),
                8usize.into(),
                types::SpanExpansionKind::MacroDerive,
                "expansion3",
                "location2",
                2,
                2,
            ),
        ];
        for (span, span_e) in merger1.tables.relations.spans.iter().zip(&expected) {
            let &(child, parent, kind, expansion_descr, file_name, line, col) = span;
            let (child_e, parent_e, kind_e, expansion_e, file_name_e, line_e, col_e) = span_e;
            assert_eq!(parent, *parent_e);
            assert_eq!(child, *child_e);
            assert_eq!(kind, *kind_e);
            assert_eq!(
                &merger1.tables.interning_tables.strings[expansion_descr],
                expansion_e
            );
            let l = merger1.tables.interning_tables.span_file_names[file_name];
            assert_eq!(&merger1.tables.interning_tables.strings[l], file_name_e);
            assert_eq!(line, *line_e);
            assert_eq!(col, *col_e);
        }
    }

    fn create_def_path(tables: &mut tables::Tables, counter: u32) -> types::DefPath {
        tables.register_def_paths(
            String::from("crate1"),
            (counter as u128).into(),
            format!("relative_def_id_{}", counter),
            (counter as u64, 0u64).into(),
            format!("summary_{}", counter),
        )
    }

    #[test]
    fn test_types() {
        fn create_table_1() -> tables::Tables {
            let mut tables = tables::Tables::default();
            let (typ_bool,) = tables.register_types(String::from("TyPrimitive/Bool"));
            tables.register_types_primitive(typ_bool, types::TyPrimitive::Bool);
            let (typ_u64,) = tables.register_types(String::from("TyPrimitive/U64"));
            tables.register_types_primitive(typ_u64, types::TyPrimitive::U64);
            assert_eq!(tables.interning_tables.strings.len(), 2);
            assert_eq!(tables.relations.types_primitive.len(), 2);
            assert_eq!(tables.relations.types.len(), 2);
            assert_eq!(tables.counters.types, 2);

            let def_path0 = create_def_path(&mut tables, 0);
            let (typ_struct,) = tables.register_types(String::from("ADT"));
            tables.register_types_adt_def(
                typ_struct,
                def_path0,
                types::AdtKind::Struct,
                false,
                false,
            );
            let def_path1 = create_def_path(&mut tables, 1);
            tables.register_types_adt_variant(
                typ_struct,
                0u16.into(),
                def_path1,
                String::from("n/a"),
            );
            let def_path2 = create_def_path(&mut tables, 2);
            tables.register_types_adt_field(
                typ_struct,
                0u16.into(),
                def_path2,
                String::from("f"),
                types::TyVisibility::Public,
                typ_bool,
            );
            let def_path3 = create_def_path(&mut tables, 3);
            tables.register_types_adt_field(
                typ_struct,
                0u16.into(),
                def_path3,
                String::from("g"),
                types::TyVisibility::Public,
                typ_u64,
            );
            assert_eq!(tables.interning_tables.strings.len(), 15);
            assert_eq!(tables.relations.types_adt_def.len(), 1);
            assert_eq!(tables.relations.types_adt_variant.len(), 1);
            assert_eq!(tables.relations.types_adt_field.len(), 2);
            assert_eq!(tables.relations.types.len(), 3);
            assert_eq!(tables.counters.types, 3);

            let def_path4 = create_def_path(&mut tables, 4);
            let (typ_enum,) = tables.register_types(String::from("ADT"));
            tables.register_types_adt_def(typ_enum, def_path4, types::AdtKind::Enum, false, false);
            let def_path5 = create_def_path(&mut tables, 5);
            tables.register_types_adt_variant(typ_enum, 0u16.into(), def_path5, String::from("V1"));
            let def_path6 = create_def_path(&mut tables, 6);
            tables.register_types_adt_field(
                typ_enum,
                0u16.into(),
                def_path6,
                String::from("f"),
                types::TyVisibility::Public,
                typ_bool,
            );
            let def_path7 = create_def_path(&mut tables, 7);
            tables.register_types_adt_variant(typ_enum, 1u16.into(), def_path7, String::from("V2"));
            let def_path8 = create_def_path(&mut tables, 8);
            tables.register_types_adt_field(
                typ_enum,
                1u16.into(),
                def_path8,
                String::from("g"),
                types::TyVisibility::Public,
                typ_u64,
            );
            assert_eq!(tables.interning_tables.strings.len(), 27);
            assert_eq!(tables.relations.types_adt_def.len(), 2);
            assert_eq!(tables.relations.types_adt_variant.len(), 3);
            assert_eq!(tables.relations.types_adt_field.len(), 4);
            assert_eq!(tables.relations.types.len(), 4);
            assert_eq!(tables.counters.types, 4);

            let (typ_fn_ptr,) = tables.register_types(String::from("FnPtr"));
            tables.register_types_fn_ptr(typ_fn_ptr);
            assert_eq!(tables.interning_tables.strings.len(), 28);
            assert_eq!(tables.relations.types_fn_ptr.len(), 1);
            assert_eq!(tables.relations.types.len(), 5);
            assert_eq!(tables.counters.types, 5);

            let (typ_generic_a,) = tables.register_types(String::from("Generic"));
            tables.register_types_param(typ_generic_a, 0u32, String::from("A"));
            let (typ_generic_b,) = tables.register_types(String::from("Generic"));
            tables.register_types_param(typ_generic_b, 1u32, String::from("B"));
            assert_eq!(tables.interning_tables.strings.len(), 31);
            assert_eq!(tables.relations.types_param.len(), 2);
            assert_eq!(tables.relations.types.len(), 7);
            assert_eq!(tables.counters.types, 7);

            let (typ_tuple1,) = tables.register_types(String::from("Tuple"));
            tables.register_types_tuple(typ_tuple1);
            tables.register_types_tuple_element(typ_tuple1, 0u16.into(), typ_bool);
            tables.register_types_tuple_element(typ_tuple1, 1u16.into(), typ_u64);
            tables.register_types_tuple_element(typ_tuple1, 2u16.into(), typ_struct);
            tables.register_types_tuple_element(typ_tuple1, 3u16.into(), typ_generic_a);
            let (typ_tuple2,) = tables.register_types(String::from("Tuple"));
            tables.register_types_tuple(typ_tuple2);
            tables.register_types_tuple_element(typ_tuple2, 0u16.into(), typ_bool);
            tables.register_types_tuple_element(typ_tuple2, 1u16.into(), typ_bool);
            tables.register_types_tuple_element(typ_tuple2, 2u16.into(), typ_generic_a);
            tables.register_types_tuple_element(typ_tuple2, 3u16.into(), typ_enum);
            tables.register_types_tuple_element(typ_tuple2, 4u16.into(), typ_fn_ptr);
            tables.register_types_tuple_element(typ_tuple2, 5u16.into(), typ_generic_b);
            assert_eq!(tables.interning_tables.strings.len(), 32);
            assert_eq!(tables.relations.types_tuple.len(), 2);
            assert_eq!(tables.relations.types_tuple_element.len(), 10);
            assert_eq!(tables.relations.types.len(), 9);
            assert_eq!(tables.counters.types, 9);

            tables
        }
        let tables1 = create_table_1();
        let tables2 = create_table_1();

        let mut merger1 = tables::TableMerger::new(tables1);
        merger1.merge(tables2);
        assert_eq!(merger1.tables.interning_tables.strings.len(), 32);
        assert_eq!(merger1.tables.counters.types, 18);
        assert_eq!(merger1.tables.relations.types.len(), 18);
        let expected = [
            (0usize.into(), 0usize.into()),
            (1usize.into(), 1usize.into()),
            (2usize.into(), 2usize.into()),
            (3usize.into(), 2usize.into()),
            (4usize.into(), 3usize.into()),
            (5usize.into(), 4usize.into()),
            (6usize.into(), 4usize.into()),
            (7usize.into(), 5usize.into()),
            (8usize.into(), 5usize.into()),
            (9usize.into(), 0usize.into()),
            (10usize.into(), 1usize.into()),
            (11usize.into(), 2usize.into()),
            (12usize.into(), 2usize.into()),
            (13usize.into(), 3usize.into()),
            (14usize.into(), 4usize.into()),
            (15usize.into(), 4usize.into()),
            (16usize.into(), 5usize.into()),
            (17usize.into(), 5usize.into()),
        ];
        assert_eq!(expected.len(), merger1.tables.relations.types.len());
        for (actual, expected) in merger1.tables.relations.types.iter().zip(&expected) {
            assert_eq!(actual, expected);
        }
        assert_eq!(merger1.tables.relations.types.len(), expected.len());

        assert_eq!(merger1.tables.relations.types_primitive.len(), 4);
        let expected = [
            (0usize.into(), types::TyPrimitive::Bool),
            (1usize.into(), types::TyPrimitive::U64),
            (9usize.into(), types::TyPrimitive::Bool),
            (10usize.into(), types::TyPrimitive::U64),
        ];
        assert_eq!(
            merger1.tables.relations.types_primitive.len(),
            expected.len()
        );
        for (actual, expected) in merger1
            .tables
            .relations
            .types_primitive
            .iter()
            .zip(&expected)
        {
            assert_eq!(actual, expected);
        }
        assert_eq!(merger1.tables.relations.types_adt_def.len(), 4);
        let expected = [
            (
                2usize.into(),
                0u64.into(),
                types::AdtKind::Struct,
                false,
                false,
            ),
            (
                3usize.into(),
                4u64.into(),
                types::AdtKind::Enum,
                false,
                false,
            ),
            (
                11usize.into(),
                0u64.into(),
                types::AdtKind::Struct,
                false,
                false,
            ),
            (
                12usize.into(),
                4u64.into(),
                types::AdtKind::Enum,
                false,
                false,
            ),
        ];
        assert_eq!(merger1.tables.relations.types_adt_def.len(), expected.len());
        for (actual, expected) in merger1.tables.relations.types_adt_def.iter().zip(&expected) {
            assert_eq!(actual, expected);
        }

        assert_eq!(merger1.tables.relations.types_adt_variant.len(), 6);
        let expected = [
            (2usize.into(), 0u16.into(), 1u64.into(), 8u64.into()),
            (3usize.into(), 0u16.into(), 5u64.into(), 19u64.into()),
            (3usize.into(), 1u16.into(), 7u64.into(), 24u64.into()),
            (11usize.into(), 0u16.into(), 1u64.into(), 8u64.into()),
            (12usize.into(), 0u16.into(), 5u64.into(), 19u64.into()),
            (12usize.into(), 1u16.into(), 7u64.into(), 24u64.into()),
        ];
        assert_eq!(
            merger1.tables.relations.types_adt_variant.len(),
            expected.len()
        );
        for (actual, expected) in merger1
            .tables
            .relations
            .types_adt_variant
            .iter()
            .zip(&expected)
        {
            assert_eq!(actual, expected);
        }

        assert_eq!(merger1.tables.relations.types_adt_field.len(), 8);
        let expected = [
            (
                0usize.into(),
                2usize.into(),
                0u16.into(),
                2u64.into(),
                11u64.into(),
                types::TyVisibility::Public,
                0usize.into(),
            ),
            (
                1usize.into(),
                2usize.into(),
                0u16.into(),
                3u64.into(),
                14u64.into(),
                types::TyVisibility::Public,
                1usize.into(),
            ),
            (
                2usize.into(),
                3usize.into(),
                0u16.into(),
                6u64.into(),
                11u64.into(),
                types::TyVisibility::Public,
                0usize.into(),
            ),
            (
                3usize.into(),
                3usize.into(),
                1u16.into(),
                8u64.into(),
                14u64.into(),
                types::TyVisibility::Public,
                1usize.into(),
            ),
            (
                4usize.into(),
                11usize.into(),
                0u16.into(),
                2u64.into(),
                11u64.into(),
                types::TyVisibility::Public,
                9usize.into(),
            ),
            (
                5usize.into(),
                11usize.into(),
                0u16.into(),
                3u64.into(),
                14u64.into(),
                types::TyVisibility::Public,
                10usize.into(),
            ),
            (
                6usize.into(),
                12usize.into(),
                0u16.into(),
                6u64.into(),
                11u64.into(),
                types::TyVisibility::Public,
                9usize.into(),
            ),
            (
                7usize.into(),
                12usize.into(),
                1u16.into(),
                8u64.into(),
                14u64.into(),
                types::TyVisibility::Public,
                10usize.into(),
            ),
        ];
        assert_eq!(
            merger1.tables.relations.types_adt_field.len(),
            expected.len()
        );
        for (actual, expected) in merger1
            .tables
            .relations
            .types_adt_field
            .iter()
            .zip(&expected)
        {
            assert_eq!(actual, expected);
        }

        assert_eq!(merger1.tables.relations.types_fn_ptr.len(), 2);
        assert_eq!(
            merger1.tables.relations.types_fn_ptr.facts[0],
            (4u64.into(),)
        );
        assert_eq!(
            merger1.tables.relations.types_fn_ptr.facts[1],
            (13u64.into(),)
        );

        assert_eq!(merger1.tables.relations.types_param.len(), 4);
        let expected = [
            (5usize.into(), 0u32, 29usize.into()),
            (6usize.into(), 1u32, 30usize.into()),
            (14usize.into(), 0u32, 29usize.into()),
            (15usize.into(), 1u32, 30usize.into()),
        ];
        for (actual, expected) in merger1.tables.relations.types_param.iter().zip(&expected) {
            assert_eq!(actual, expected);
        }
        assert_eq!(merger1.tables.relations.types_tuple.len(), 4);
        let expected = [
            (7usize.into(),),
            (8usize.into(),),
            (16usize.into(),),
            (17usize.into(),),
        ];
        for (actual, expected) in merger1.tables.relations.types_tuple.iter().zip(&expected) {
            assert_eq!(actual, expected);
        }
        assert_eq!(merger1.tables.relations.types_tuple_element.len(), 20);
        let expected = [
            (7usize.into(), 0u16.into(), 0usize.into()),
            (7usize.into(), 1u16.into(), 1usize.into()),
            (7usize.into(), 2u16.into(), 2usize.into()),
            (7usize.into(), 3u16.into(), 5usize.into()),
            (8usize.into(), 0u16.into(), 0usize.into()),
            (8usize.into(), 1u16.into(), 0usize.into()),
            (8usize.into(), 2u16.into(), 5usize.into()),
            (8usize.into(), 3u16.into(), 3usize.into()),
            (8usize.into(), 4u16.into(), 4usize.into()),
            (8usize.into(), 5u16.into(), 6usize.into()),
            (16usize.into(), 0u16.into(), 9usize.into()),
            (16usize.into(), 1u16.into(), 10usize.into()),
            (16usize.into(), 2u16.into(), 11usize.into()),
            (16usize.into(), 3u16.into(), 14usize.into()),
            (17usize.into(), 0u16.into(), 9usize.into()),
            (17usize.into(), 1u16.into(), 9usize.into()),
            (17usize.into(), 2u16.into(), 14usize.into()),
            (17usize.into(), 3u16.into(), 12usize.into()),
            (17usize.into(), 4u16.into(), 13usize.into()),
            (17usize.into(), 5u16.into(), 15usize.into()),
        ];
        assert_eq!(
            merger1.tables.relations.types_tuple_element.len(),
            expected.len()
        );
        for (actual, expected) in merger1
            .tables
            .relations
            .types_tuple_element
            .iter()
            .zip(&expected)
        {
            assert_eq!(actual, expected);
        }
    }
}
