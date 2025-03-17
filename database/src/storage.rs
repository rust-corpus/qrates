// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Helper functions for serializing and deserializing.

use crate::data_structures::{
    DiskInterningKey, DiskInterningTable, DiskInterningValue, DiskMapKey, DiskMapValue,
    InterningTableValue, Relation, RelationMap,
};
use crate::tables::{DiskTables, Tables};
use crate::{DiskMap, DiskVec};
use anyhow::{Context, Result};
use log::trace;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::io::{BufReader, Read, Write};
use std::path::Path;

pub fn load<T>(path: &Path) -> Result<T>
where
    for<'de> T: Deserialize<'de>,
{
    trace!("[enter] load({:?})", path);
    let extension = path.extension().unwrap();
    let file =
        std::fs::File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let result = if extension == "bincode" {
        bincode::deserialize_from(file)
            .with_context(|| format!("Invalid bincode in file: {:?}", path))
    } else if extension == "json" {
        serde_json::from_reader(file).with_context(|| format!("Invalid json in file: {:?}", path))
    } else {
        unreachable!("Unknown extension: {:?}", extension);
    };
    trace!("[exit] load({:?})", path);
    result
}

pub fn save<T>(object: &T, path: &Path)
where
    T: Serialize,
{
    trace!("[enter] save({:?})", path);
    let extension = path.extension().unwrap();
    let mut file = std::fs::File::create(&path)
        .unwrap_or_else(|e| panic!("Unable to create {:?}: {}", path, e));
    if extension == "bincode" {
        bincode::serialize_into(file, object)
            .unwrap_or_else(|e| panic!("Unable to write {:?}: {}", path, e));
    } else if extension == "json" {
        serde_json::to_writer_pretty(&mut file, object)
            .unwrap_or_else(|e| panic!("Unable to write {:?}: {}", path, e));
    } else {
        unreachable!("Unknown extension: {:?}", extension);
    }
    trace!("[exit] save({:?})", path);
}

impl<T: Copy + DiskMapValue> Relation<T> {
    /// ``relation_hash`` – the hash of the relation schema. It is used to prevent
    /// loading relations that were saved with a different schema.
    /// ``path`` – the path **without** the extension.
    pub fn save(&mut self, relation_hash: u64, path: std::path::PathBuf) {
        self.facts.map.set_expected_hash(relation_hash);
        self.facts.save(path);
    }

    /// Also, ``relation_hash`` must be correctly initialized.
    pub fn load(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Self> {
        let vec = DiskVec::load(path)?;
        let loaded_hash = vec.map.read_stored_hash();
        assert_eq!(loaded_hash, Some(expected_relation_hash), "DiskVec hash check failed. The database was likely generated with a different version of your schema.");
        Ok(Self::from_disk_vec(vec))
    }
}

impl<K: DiskInterningKey, V: DiskInterningValue> DiskInterningTable<K, V> {
    pub fn save(&mut self, relation_hash: u64, path: std::path::PathBuf) {
        let contents_path = Self::get_contents_path(path.clone());
        let inv_map_path = Self::get_inv_map_path(path.clone());
        self.contents.map.set_expected_hash(relation_hash);
        self.inv_map.set_expected_hash(relation_hash);
        self.contents.save(contents_path);
        self.inv_map.save(inv_map_path);
    }

    pub fn load(expected_hash: u64, path: std::path::PathBuf) -> Result<Self> {
        let contents_path = Self::get_contents_path(path.clone());
        let inv_map_path = Self::get_inv_map_path(path.clone());
        let contents = DiskVec::load(contents_path)?;
        let inv_map = DiskMap::load(inv_map_path)?;
        let contents_hash = contents.map.read_stored_hash();
        let inv_map_hash = inv_map.read_stored_hash();
        assert_eq!(contents_hash, Some(expected_hash), "DiskVec hash check failed. The database was likely generated with a different version of your schema.");
        assert_eq!(inv_map_hash, Some(expected_hash), "DiskMap hash check failed. The database was likely generated with a different version of your schema.");
        Ok(Self { contents, inv_map })
    }
}

impl<K, V> RelationMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
    for<'a> &'a K: Borrow<<K as redb::Value>::SelfType<'a>>,
{
    pub fn save(&mut self, relation_hash: u64, path: std::path::PathBuf) {
        self.map.set_expected_hash(relation_hash);
        self.map.save(path);
    }

    pub fn load(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Self> {
        let map = DiskMap::load(path)?;
        let relation_hash = map.read_stored_hash();
        assert_eq!(relation_hash, Some(expected_relation_hash), "DiskMap hash check failed. The database was likely generated with a different version of your schema.");
        Ok(Self { map })
    }
}

impl Tables {
    /// ``path`` – the path **without** the extension.
    pub fn save_json(&self, mut path: std::path::PathBuf) {
        path.set_extension("json");
        save(&self, &path);
    }
    /// ``path`` – the path the **without** extension.
    pub fn save_bincode(&self, mut path: std::path::PathBuf) {
        path.set_extension("bincode");
        save(&self, &path);
    }
    /// ``path`` – the path **with** the extension.
    pub fn load(path: &std::path::Path) -> Result<Self> {
        load(path)
    }
}

impl<K: DiskMapKey, V: DiskMapValue> DiskMap<K, V> {
    pub fn save(&mut self, path: std::path::PathBuf) {
        self.flush();
        if path == self.path() {
            // already saved
            // TODO: could remove this restriction.
            assert!(
                !self.is_temp_map,
                "Cannot save a temporary map to its temporary path"
            );
            return;
        }

        // create a new database at path and store self into it.
        let mut saved_dm = DiskMap::from_iter_override(path, self.iter());
        if let Some(expected_hash) = self.read_stored_hash() {
            // Need to copy the expected hash to the new database in case we're storing to a new location.
            saved_dm.set_expected_hash(expected_hash);
        }
        assert_eq!(saved_dm.len(), self.len());
    }

    pub fn load(path: std::path::PathBuf) -> Result<Self> {
        assert!(path.exists());
        let saved_dm = DiskMap::create_or_open(path);
        Ok(saved_dm)
    }
}

impl<V: DiskMapValue> DiskVec<V> {
    pub fn save(&mut self, path: std::path::PathBuf) {
        self.map.save(path);
    }

    pub fn load(path: std::path::PathBuf) -> Result<Self> {
        let map = DiskMap::load(path)?;
        Ok(DiskVec::from_map(map))
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        data_structures::{DiskMapValue, Relation},
        set_disk_map_temp_dir_root,
    };

    fn init() {
        let mut t = std::env::temp_dir();
        t.push("tmp_diskmap");
        // create
        std::fs::create_dir_all(&t).unwrap();
        set_disk_map_temp_dir_root(t);
    }

    fn checker<T>(test_count: u32, facts: &Vec<T>)
    where
        T: Copy + std::fmt::Debug + std::cmp::PartialEq + DiskMapValue,
    {
        init();

        let mut relation: Relation<T> = facts.clone().into();
        let mut test_file = std::env::temp_dir();
        test_file.push(format!("rust-corpus-relation-saving-test-{}", test_count));
        let hash = 5;
        relation.save(hash, test_file.clone());
        let loaded_relation: Relation<T> = Relation::load(hash, test_file).unwrap();
        let loaded_facts: Vec<T> = loaded_relation.into();
        assert_eq!(facts.len(), loaded_facts.len());
        for (x, y) in facts.iter().zip(&loaded_facts) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn test_saving_and_loading_relations1() {
        checker(1, &vec![1u8, 2u8, 3u8]);
    }
    #[test]
    fn test_saving_and_loading_relations2() {
        checker(2, &vec![(1u8, 'a'), (2u8, 'b'), (3u8, 'c')]);
    }
    #[test]
    fn test_saving_and_loading_relations3() {
        checker(3, &vec![(1u8, 4u64), (2u8, 5u64), (3u8, 6u64)]);
    }
    #[test]
    fn test_saving_and_loading_relations4() {
        checker(
            4,
            &vec![
                (1u8, 4u64, 5u8, 6u16),
                (2u8, 5u64, 6u8, 9u16),
                (3u8, 6u64, 7u8, 10u16),
            ],
        );
    }
    #[test]
    fn test_saving_and_loading_relations5() {
        use rand::rngs::StdRng;
        use rand::{Rng, SeedableRng};
        let mut rng = StdRng::from_entropy();
        let mut facts: Vec<(u8, u64, u8, u16)> = Vec::with_capacity(100000);
        for _ in 0..facts.capacity() {
            facts.push((rng.gen(), rng.gen(), rng.gen(), rng.gen()))
        }
        checker(5, &facts);
    }
    #[test]
    #[should_panic]
    fn test_saving_and_loading_relations6() {
        init();
        let mut relation: Relation<u32> = vec![1, 2, 3, 4, 5].into();
        let mut test_file = std::env::temp_dir();
        test_file.push("rust-corpus-relation-saving-test-6");
        relation.save(5, test_file.clone());
        let _: Result<Relation<u32>, _> = Relation::load(6, test_file);
    }
}
