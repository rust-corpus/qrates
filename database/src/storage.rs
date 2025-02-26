// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Helper functions for serializing and deserializing.

use crate::data_structures::{DiskMapKey, DiskMapValue, InterningTable, InterningTableKey, InterningTableValue, Relation, RelationMap, RelationMapKey, RelationMapValue};
use crate::tables::Tables;
use crate::tables::{store_multifile_relations, load_multifile_relations};
use crate::tables::Relations;
use crate::{DiskMap, DiskVec};
use anyhow::{Context, Result};
use log::trace;
use redb::TableDefinition;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::collections::HashMap;
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

/// **Note:** this function is not marked unsafe just so that we could use unsafe blocks
/// to mark precisely where we are performing unsafe operations.
fn unsafe_save_vec<T: Copy>(vec: &Vec<T>, schema_hash: u64, mut path: std::path::PathBuf) {
    path.set_extension("rc");
    trace!("[enter] save({:?})", path);
    let mut file = std::fs::File::create(&path)
        .unwrap_or_else(|e| panic!("Unable to create {:?}: {}", path, e));
    if cfg!(target_endian = "big") {
        unreachable!("We assume little endian machines");
    }
    let element_size = std::mem::size_of::<T>();
    let len = vec.len();
    file.write_all(&(schema_hash as u64).to_le_bytes()).unwrap();
    file.write_all(&(element_size as u64).to_le_bytes())
        .unwrap();
    file.write_all(&(len as u64).to_le_bytes()).unwrap();
    let begin_ptr = vec.as_ptr() as *const u8;
    // TODO: Is there a safe and equally fast way of doing this?
    let data = unsafe { std::slice::from_raw_parts(begin_ptr, element_size * len) };
    file.write_all(data).unwrap();
    trace!("[exit] save({:?})", path);
}

unsafe fn unsafe_load_vec<T: Copy>(
    expected_relation_hash: u64,
    mut path: std::path::PathBuf,
) -> Result<Vec<T>> {
    path.set_extension("rc");
    trace!("[enter] load({:?})", path);
    let file =
        std::fs::File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let mut buf_reader = BufReader::new(file);
    if cfg!(target_endian = "big") {
        unreachable!("We assume little endian machines");
    }
    let mut buf: [u8; 8] = [0u8; 8];
    assert_eq!(buf_reader.read(&mut buf)?, 8);
    let actual_relation_hash = u64::from_le_bytes(buf);
    assert_eq!(actual_relation_hash, expected_relation_hash);
    let expected_fact_size = std::mem::size_of::<T>();
    assert_eq!(buf_reader.read(&mut buf)?, 8);
    let actual_fact_size = u64::from_le_bytes(buf);
    assert_eq!(expected_fact_size, (actual_fact_size as usize));
    assert_eq!(buf_reader.read(&mut buf)?, 8);
    let len = u64::from_le_bytes(buf) as usize;
    let mut vec: Vec<T> = Vec::with_capacity(len);
    assert_eq!(vec.capacity(), len);
    let begin_ptr = vec.as_ptr() as *mut u8;
    // TODO: Is there a safe and equally fast way of doing this?
    let data = unsafe { std::slice::from_raw_parts_mut(begin_ptr, expected_fact_size * len) };
    let mut total_bytes_read = 0;
    loop {
        let remainder = &mut data[total_bytes_read..];
        let bytes_read = buf_reader.read(remainder)?;
        if bytes_read == 0 {
            break;
        }
        total_bytes_read += bytes_read;
    }
    assert_eq!(total_bytes_read, expected_fact_size * len);
    unsafe {
        vec.set_len(len);
    }
    trace!("[exit] load({:?})", path);
    Ok(vec)
}

impl<T: Copy + DiskMapValue> Relation<T> {
    /// This function is safe only when T does not contain references or pointers.
    /// ``relation_hash`` – the hash of the relation schema. It is used to prevent
    /// loading relations that were saved with a different schema.
    /// ``path`` – the path **without** the extension.
    pub unsafe fn save(&self, relation_hash: u64, path: std::path::PathBuf) {
        self.facts.save(path);

        // unsafe { save_elts_relation(self.facts.iter().cloned(), relation_hash, path) };

        // unsafe_save_vec(&self.facts, relation_hash, path);
    }
    /// This function is safe only when T does not contain references or pointers.
    /// Also, ``relation_hash`` must be correctly initialized.
    pub unsafe fn load(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Self> {

        let vec = DiskVec::load(path)?;
        Ok(Self::from_disk_vec(vec))

        // let iter = unsafe { load_elts_relation(expected_relation_hash, path)? };
        // let facts = iter.collect::<Vec<T>>();
        // Ok(facts.into())


        // unsafe { unsafe_load_vec(expected_relation_hash, path).map(|vec| vec.into()) }
    }
}


// TODO: rename to _stream
pub unsafe fn save_elts_relation<T: Copy>(elts: impl IntoIterator<Item = T>, relation_hash: u64, mut path: std::path::PathBuf) {
    path.set_extension("stream.rc");
    trace!("[enter] save_elts_relation({:?})", path);
    let mut file = std::fs::File::create(&path)
        .unwrap_or_else(|e| panic!("Unable to create {:?}: {}", path, e));
    if cfg!(target_endian = "big") {
        unreachable!("We assume little endian machines");
    }
    // do a streaming write.
    let element_size = std::mem::size_of::<T>();
    // we don't know length, since we receive an iterator. will just have to read until EOF.
    // let mut len = 
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&(relation_hash as u64).to_le_bytes()).unwrap();
    writer.write_all(&(element_size as u64).to_le_bytes())
        .unwrap();
    for elt in elts {
        // save the raw data, so we don't need T: Serialize
        let buf = unsafe {
            std::slice::from_raw_parts(&elt as *const T as *const u8, element_size)
        };
        writer.write_all(buf).unwrap();

        // bincode::serialize_into(&mut file, &elt).unwrap();
    }
}

pub unsafe fn load_elts_relation<T: Copy>(expected_relation_hash: u64, mut path: std::path::PathBuf) -> Result<impl Iterator<Item = T>> {
    path.set_extension("stream.rc");
    trace!("[enter] load_elts_relation({:?})", path);

    let file =
        std::fs::File::open(&path).with_context(|| format!("Failed to open file: {:?}", path))?;
    let mut buf_reader = BufReader::new(file);
    if cfg!(target_endian = "big") {
        unreachable!("We assume little endian machines");
    }
    let mut buf: [u8; 8] = [0u8; 8];
    assert_eq!(buf_reader.read(&mut buf)?, 8);
    let actual_relation_hash = u64::from_le_bytes(buf);
    assert_eq!(actual_relation_hash, expected_relation_hash);
    let expected_fact_size = std::mem::size_of::<T>();
    assert_eq!(buf_reader.read(&mut buf)?, 8);
    let actual_fact_size = u64::from_le_bytes(buf);
    assert_eq!(expected_fact_size, (actual_fact_size as usize));
    
    let mut donor = Vec::<T>::with_capacity(1);

    Ok(std::iter::from_fn(move || {
        // read a T from the file
        // obtain buf from raw parts of a Vec<T> to guarantee alignments for T
        let ptr = donor.as_mut_ptr() as *mut u8;
        let mut buf = unsafe {
            std::slice::from_raw_parts_mut(ptr, expected_fact_size)
        };

        // let mut buf = vec![0u8; expected_fact_size];
        match buf_reader.read_exact(&mut buf) {
            Ok(()) => {
                let elt = unsafe {
                    std::ptr::read(buf.as_ptr() as *const T)
                };
                Some(elt)
            },
            Err(_) => None,
        }
    }))

}

pub unsafe fn load_elts_relation_into_relation<T: Copy + DiskMapValue>(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Relation<T>> {
    let iter = unsafe { load_elts_relation(expected_relation_hash, path)? };
    Ok(iter.collect::<Vec<T>>().into())
}

#[derive(Debug)]
pub struct Hack<V: InterningTableValue>(V);

impl<V: InterningTableValue> AsRef<V> for Hack<V> {
    fn as_ref(&self) -> &V {
        &self.0
    }
}

impl<V: InterningTableValue> redb::Value for Hack<V> {
    type SelfType<'a> = V;
    type AsBytes<'a> = V::AsBytes<'a>;

    fn fixed_width() -> Option<usize> {
        V::fixed_width()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
        V::as_bytes(&value)
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where Self: 'a
     {
        V::from_bytes(data)
    }

    fn type_name() -> redb::TypeName {
        V::type_name()
    }
}


impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{


    /// This function is safe only when K and V do not contain references or pointers.
    /// ``table_hash`` – the hash of the interning table schema. It is used to prevent
    /// loading an interning table that was saved with a different schema.
    /// ``path`` – the path **without** the extension.
    pub unsafe fn save(&self, table_hash: u64, path: std::path::PathBuf) {
        // unsafe { save_elts_relation(self.contents.iter().cloned(), table_hash, path.clone()) };


        // also save into a `redb` database
        self.save_to_redb(table_hash, path);

        // unsafe_save_vec(&self.contents, table_hash, path);
    }

    fn save_to_redb(&self, table_hash: u64, mut path: std::path::PathBuf)
    //where V: redb::Value + std::borrow::Borrow<<V as redb::Value>::SelfType<'static>> + 'static
    {


        // save to a `redb` database
        path.set_extension("redb");

        let table_name = table_hash.to_string();
        let inv_table_name = format!("inv_{}", table_name);

        let db = redb::Database::create(path).unwrap();
        let mut write_txn = db.begin_write().unwrap();
        let table_definition = TableDefinition::<u64, Hack<V>>::new(&table_name);

        {


            // impl<V: for<'a>InterningTableValue<SelfType<'a> = V> + Copy> std::borrow::Borrow<Hack<V>> for Hack<V> {

            // }

            let mut table = write_txn.open_table(table_definition).unwrap();
            for (i, v) in self.contents.iter().enumerate() {
                let v = v.clone();
                table.insert(i as u64, v).unwrap();
            }
        }
        write_txn.commit().unwrap();

        let mut write_txn_inv = db.begin_write().unwrap();
        let table_definition_inv = TableDefinition::<V, u64>::new(&inv_table_name);
        {
            let mut table_inv = write_txn_inv.open_table(table_definition_inv).unwrap();
            for (i, v) in self.contents.iter().enumerate() {
                let v = v.clone();
                table_inv.insert(v, i as u64).unwrap();
            }
        }
        write_txn_inv.commit().unwrap();
    }


    /// This function is safe only when T does not contain references or pointers.
    /// Also, ``relation_hash`` must be correctly initialized.
    pub unsafe fn load(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Self> {
        // let iter = unsafe { load_elts_relation(expected_relation_hash, path.clone())? };
        // let contents = iter.collect::<Vec<V>>();
        let mut table: InterningTable<K, V> = vec![].into();
        

        table.load_redb(expected_relation_hash, path);

        Ok(table)

        // unsafe { unsafe_load_vec(expected_relation_hash, path).map(|vec| vec.into()) }
    }

    fn load_redb(&mut self, table_hash: u64, mut path: std::path::PathBuf) {
        path.set_extension("redb");
        let db = redb::Database::open(path).unwrap();
        let read_txn = db.begin_read().unwrap();
        let table_name = table_hash.to_string();
        let inv_table_name = format!("inv_{}", table_name);
        let table_definition = TableDefinition::<u64, Hack<V>>::new(&table_name);
        let inv_table_definition = TableDefinition::<V, u64>::new(&inv_table_name);
        let table = read_txn.open_table(table_definition).unwrap();
        let inv_table = read_txn.open_table(inv_table_definition).unwrap();
        self.db = Some(db);
        self.read_only_table = Some(table);
        self.read_only_inv_table = Some(inv_table);
    }
}

impl<K, V> RelationMap<K, V>
where K: RelationMapKey,
V: RelationMapValue,
for<'a>&'a K: Borrow<<K as redb::Value>::SelfType<'a>>
{
    pub fn save(&self, relation_hash: u64, path: std::path::PathBuf) {
        self.save_to_redb(relation_hash, path);
    }

    fn save_to_redb(&self, relation_hash: u64, mut path: std::path::PathBuf) {
        path.set_extension("redb");
        let db = redb::Database::create(path).unwrap();
        let mut write_txn = db.begin_write().unwrap();
        let table_name = relation_hash.to_string();
        let table_definition = TableDefinition::<K, V>::new(&table_name);
        {
            let mut table = write_txn.open_table(table_definition).unwrap();
            for (k, v) in self.contents.iter() {
                table.insert(k, v).unwrap();
            }
        }
        write_txn.commit().unwrap();
    }

    pub fn load(expected_relation_hash: u64, path: std::path::PathBuf) -> Result<Self> {
        let mut table: RelationMap<K, V> = HashMap::new().into();
        table.load_redb(expected_relation_hash, path);
        Ok(table)
    }

    fn load_redb(&mut self, relation_hash: u64, mut path: std::path::PathBuf) {
        path.set_extension("redb");
        let db = redb::Database::open(path).unwrap();
        let read_txn = db.begin_read().unwrap();
        let table_name = relation_hash.to_string();
        let table_definition = TableDefinition::<K, V>::new(&table_name);
        let table = read_txn.open_table(table_definition).unwrap();
        self.db = Some(db);
        self.read_only_table = Some(table);
    }
}

impl Tables {
    /// ``path`` – the path **without** the extension.
    pub fn save_json(&self, mut path: std::path::PathBuf) {
        panic!("Not implemented");
    }
    /// ``path`` – the path the **without** extension.
    pub fn save_bincode(&self, mut path: std::path::PathBuf) {
        let filename = path.file_name().unwrap().to_str().unwrap();
        let path_counters = path.with_file_name(format!("{}.counters.bincode", filename));
        let path_interning = path.with_file_name(format!("{}.interning.bincode", filename));
        // directory
        let path_relations = path.with_file_name(format!("{}.relations", filename));

        std::fs::create_dir_all(&path_relations).unwrap();

        save(&self.counters, &path_counters);
        save(&self.interning_tables, &path_interning);
        store_multifile_relations(&self.relations, &path_relations);
    }
    /// ``path`` – the path **with** the extension.
    pub fn load(path: &std::path::Path) -> Result<Self> {
        let ext = path.extension().unwrap();
        let path_without_ext = path.with_extension("");
        let filename = path_without_ext.file_name().unwrap().to_str().unwrap();
        let counters;
        {   
            let counters_filename = format!("{}.counters.{}", filename, ext.to_str().unwrap());
            let counters_path = path.with_file_name(counters_filename);
            counters = load(&counters_path)?;
        }
        let interning_tables;
        {
            let interning_tables_filename = format!("{}.interning.{}", filename, ext.to_str().unwrap());
            let interning_tables_path = path.with_file_name(interning_tables_filename);
            interning_tables = load(&interning_tables_path)?;
        }

        let relations;
        {
            // note: ignores json extension
            let relations_path = format!("{}.relations", filename);
            let mut path_root = path.with_file_name(relations_path);
            relations = load_multifile_relations(&path_root)?;
        }


        Ok(Tables {
            counters,
            interning_tables,
            relations,
        })
    }
}

impl<K: DiskMapKey, V: DiskMapValue> DiskMap<K, V> {
    pub fn save(&self, path: std::path::PathBuf) {
        // TODO: instead of assertion, just pretend it's already saved?
        assert_ne!(path, self.path());

        // create a new database at path and store self into it.
        let saved_dm = DiskMap::from_iter_override(path, self.iter());
        assert_eq!(saved_dm.len(), self.len());
    }

    pub fn load(path: std::path::PathBuf) -> Result<Self> {
        // assert that path exists
        assert!(path.exists());
        let saved_dm = DiskMap::create_or_open(path);
        Ok(saved_dm)
    }
}

impl<V: DiskMapValue> DiskVec<V> {
    pub fn save(&self, path: std::path::PathBuf) {
        self.map.save(path);
    }

    pub fn load(path: std::path::PathBuf) -> Result<Self> {
        let map = DiskMap::load(path)?;
        Ok(DiskVec::from_map(map))
    }
}

#[cfg(test)]
mod tests {

    use crate::data_structures::Relation;

    fn checker<T>(test_count: u32, facts: &Vec<T>)
    where
        T: Copy + std::fmt::Debug + std::cmp::PartialEq,
    {
        let relation: Relation<T> = facts.clone().into();
        let mut test_file = std::env::temp_dir();
        test_file.push(format!("rust-corpus-relation-saving-test-{}", test_count));
        let hash = 5;
        unsafe {
            relation.save(hash, test_file.clone());
        }
        let loaded_relation: Relation<T> = unsafe { Relation::load(hash, test_file) }.unwrap();
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
        let relation: Relation<u32> = vec![1, 2, 3, 4, 5].into();
        let mut test_file = std::env::temp_dir();
        test_file.push("rust-corpus-relation-saving-test-6");
        unsafe {
            relation.save(5, test_file.clone());
        }
        let _: Result<Relation<u32>, _> = unsafe { Relation::load(6, test_file) };
    }
}