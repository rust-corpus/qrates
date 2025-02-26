// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! The implementation of interning tables and relations.

use log::info;
use redb::{ReadOnlyTable, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde_derive::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, fmt::Debug, ops::{Deref, DerefMut}, path::PathBuf};

use crate::{get_new_disk_map_temp_dir, storage::Hack};

mod relation_element;
pub use relation_element::*;

/// A table that expresses a relation between elements.
pub struct Relation<T: DiskMapValue> {
    pub(crate) facts: DiskVec<T>,
}

impl<T: DiskMapValue> Default for Relation<T> {
    fn default() -> Self {
        Self { facts: DiskVec::create_override(get_new_disk_map_temp_dir()) }
    }
}

impl<T: DiskMapValue> Relation<T> {
    pub fn insert(&mut self, fact: T) {
        self.facts.push(fact);
    }
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.facts.iter()
    }
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = T>) {
        self.facts.insert_iter(iter);
    }

    pub(crate) fn from_disk_vec(facts: DiskVec<T>) -> Self {
        Self { facts }
    }

    pub fn create_override_in(path: impl AsRef<std::path::Path>) -> Self {
        let facts = DiskVec::create_override(path);
        Self { facts }
    }

    pub fn from_iter_override(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = T>) -> Self {
        let facts = DiskVec::from_iter_override(path, iter);
        Self { facts }
    }
}

impl<T> Relation<RelationElement<T>>
    where RelationElement<T>: DiskMapValue
{
    pub fn into_tuple_vec(self) -> Vec<T> {
        self.facts.iter().map(|re| re.into_inner()).collect()
    }
    pub fn to_tuple_vec(&self) -> Vec<T> {
        self.tuple_iter().collect()
    }
    pub fn tuple_iter(&self) -> impl Iterator<Item = T> {
        self.facts.iter().map(|re| re.into_inner())
    }

    pub fn from_tuple_iter_override(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = T>) -> Self {
        let facts = DiskVec::from_iter_override(path, iter.into_iter().map(RelationElement));
        Self { facts }
    }
}

impl<T: DiskMapValue> Into<Vec<T>> for Relation<T> {
    fn into(self) -> Vec<T> {
        self.iter().collect()
    }
}

impl<T: DiskMapValue> Into<Vec<T>> for Relation<RelationElement<T>>
where RelationElement<T>: DiskMapValue
{
    fn into(self) -> Vec<T> {
        self.into_tuple_vec()
    }
}


impl<T: DiskMapValue> From<Vec<T>> for Relation<T> {
    fn from(facts: Vec<T>) -> Self {
        Self { facts: DiskVec::from_iter_override(get_new_disk_map_temp_dir(), facts) }
    }
}

impl<T> From<Vec<T>> for Relation<RelationElement<T>>
where
    RelationElement<T>: DiskMapValue,
{
    fn from(facts: Vec<T>) -> Self {
        Self { facts: DiskVec::from_iter_override(get_new_disk_map_temp_dir(), facts.into_iter().map(RelationElement)) }
    }
}

pub struct RelationMap<K, V>
where K: DiskMapKey,
        V: DiskMapValue,
{
    pub(crate) map: DiskMap<K, V>,
}

impl<K, V> Default for RelationMap<K, V>
where K: DiskMapKey,
        V: DiskMapValue,
{
    fn default() -> Self {
        Self {
            map: DiskMap::create_override(get_new_disk_map_temp_dir()),
        }
    }
}

impl<K, V> RelationMap<K, V>
where K: DiskMapKey,
        V: DiskMapValue,
{
    pub fn from_iter_override(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = (K, V)>) -> Self {
        let map = DiskMap::from_iter_override(path, iter);
        Self { map }
    }

    #[track_caller]
    pub fn get_redb(&self, key: K) -> Option<V> {
        self.map.get(key)
    }

    #[track_caller]
    pub fn r(&self, key: K) -> V {
        self.get_redb(key).unwrap()
    }
}

pub trait InterningTableKey: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> + redb::Key {}
impl<T> InterningTableKey for T where T: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> + redb::Key {}
pub trait InterningTableValue: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key +  'static {}
impl<T> InterningTableValue for T where T: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key + 'static {}

#[derive(Deserialize, Serialize)]
#[serde(from = "Vec<V>")]
/// A table that holds the interned values.
pub struct InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    pub(crate) contents: Vec<V>,
    #[serde(skip_serializing)]
    inv_contents: HashMap<V, K>,
    #[serde(skip_serializing)]
    pub(crate) db: Option<redb::Database>,
    #[serde(skip_serializing)]
    pub(crate) read_only_table: Option<redb::ReadOnlyTable<u64, Hack<V>>>,
    #[serde(skip_serializing)]
    pub(crate) read_only_inv_table: Option<redb::ReadOnlyTable<V, u64>>,
}

impl<K, V> Default for InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    fn default() -> Self {
        Self {
            contents: Vec::new(),
            inv_contents: HashMap::new(),
            db: None,
            read_only_table: None,
            read_only_inv_table: None,
        }
    }
}

impl<K, V> From<Vec<V>> for InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    fn from(contents: Vec<V>) -> Self {
        // info!("From<Vec<V>>");
        let inv_contents = contents
            .iter()
            .enumerate()
            .map(|(k, v)| (v.clone(), k.into()))
            .collect();
        Self {
            contents,
            inv_contents,
            db: None,
            read_only_table: None,
            read_only_inv_table: None,
        }
    }
}

impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: for<'a> InterningTableValue<SelfType<'a> = V>,
{
    pub(crate) fn intern(&mut self, value: V) -> K {
        if self.inv_contents.contains_key(&value) {
            self.inv_contents[&value]
        } else {
            let new_key = self.contents.len().into();
            self.inv_contents.insert(value.clone(), new_key);
            self.contents.push(value);
            new_key
        }
    }
    pub fn iter_values(&self) -> impl Iterator<Item = &V> {
        self.contents.iter()
    }
    pub fn into_iter(self) -> impl Iterator<Item = (K, V)> {
        self.contents
            .into_iter()
            .enumerate()
            .map(|(k, v)| (k.into(), v))
    }
    pub fn len(&self) -> usize {
        self.contents.len()
    }
}

impl<K> InterningTable<K, String>
where
    K: InterningTableKey,
{
    // pub fn lookup_str(&self, value: &str) -> Option<K> {
    //     self.inv_contents.get(value).cloned()
    // }

    pub fn lookup_str(&self, value: &str) -> Option<K> {
        if let Some(table) = &self.read_only_inv_table {
            // TODO: to_string() is unfortunate.
            return Some((table.get(&value.to_string()).ok()??.value() as usize).into());
        }
        None
    }
}

impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    // pub fn lookup(&self, value: &V) -> Option<K> {
    //     self.inv_contents.get(value).cloned()
    // }

    pub fn lookup(&self, value: &V) -> Option<K> {
        if let Some(table) = &self.read_only_inv_table {
            return Some((table.get(value).ok()??.value() as usize).into());
        }
        None
    }

    pub fn get_redb(&self, key: K) -> Option<V> {
        let index: usize = key.into();
        if let Some(table) = &self.read_only_table {
            return Some(table.get(index as u64).ok()??.value());
        }
        None
    }

    pub fn r(&self, key: K) -> V {
        self.get_redb(key).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {
        let rot = self.read_only_table.as_ref().unwrap();

        rot.iter().unwrap().map(|res| {
            let (k, v) = res.unwrap();
            ((k.value() as usize).into(), v.value())
        })    
    }
}

// impl<K, V> std::ops::Index<K> for InterningTable<K, V>
// where
//     K: InterningTableKey,
//     V: InterningTableValue,
// {
//     type Output = V;
//     fn index(&self, key: K) -> &Self::Output {
//         let index: usize = key.into();

//         // if let Some(table) = &self.read_only_table {
//         //     return &table.get(index as u64).unwrap().unwrap().value();
//         // }

//         &self.contents[index]
//     }
// }

impl<K, V> Into<Vec<(K, V)>> for &InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    fn into(self) -> Vec<(K, V)> {
        self.iter().collect()
    }
}


pub trait DiskMapKey: Eq + std::hash::Hash + redb::Key + 'static + for<'a> Borrow<Self::SelfType<'a>> + for<'a> redb::Value<SelfType<'a> = Self> {}
impl<T> DiskMapKey for T where T: Eq + std::hash::Hash + redb::Key + 'static + for<'a> Borrow<Self::SelfType<'a>> + for<'a> redb::Value<SelfType<'a> = Self> {}
pub trait DiskMapValue: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key + 'static {}
impl<T> DiskMapValue for T where T: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key + 'static {}

/// DiskMap<K, V> is essentially a HashMap<K, V> that is backed by a disk file.
/// Currently it uses a redb::Database backend, and as such it needs a file path to live.
/// 
/// The functions panic whenever an unexpected database-related error occurs.
pub struct DiskMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
{
    pub(crate) db: redb::Database,
    write_cache: Vec<(K, V)>,
    path: PathBuf,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: DiskMapKey, V: DiskMapValue> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        eprintln!("Dropping DiskMap at {:?}, thus flushing", self.path);
        self.flush();
    }
}

pub(crate) const DISK_MAP_WRITE_CACHE_SIZE: usize = 100_000;
pub(crate) const DISK_MAP_REDB_CACHE_SIZE: usize = 50_000_000;

impl<K: DiskMapKey, V: DiskMapValue> DiskMap<K, V> {
    pub fn create_override(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref();
        // delete file at path if it exists
        if std::fs::metadata(path).is_ok() {
            std::fs::remove_file(path).unwrap();
        }

        Self::create_or_open(path)
    }

    fn create_self_table(&mut self) {
        let write_txn = self.db.begin_write().unwrap();
        {
            let table_def: TableDefinition<K, V> = TableDefinition::new("table");
            write_txn.open_table(table_def).unwrap();
        }
        write_txn.commit().unwrap();
    }

    pub fn create_or_open(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref();
        eprintln!("Opening DiskMap at {:?}", path);
        let mut builder = redb::Database::builder();
        builder.set_cache_size(DISK_MAP_REDB_CACHE_SIZE);

        let db = builder.create(path).unwrap();

        let mut diskmap = Self {
            db,
            path: path.to_path_buf(),
            write_cache: Vec::with_capacity(DISK_MAP_WRITE_CACHE_SIZE),
            _phantom: std::marker::PhantomData,
        };
        diskmap.create_self_table();
        diskmap
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Create a new DiskMap from an iterator of key-value pairs. Destroys the file at path if it exists.
    pub fn from_iter_override(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = (K, V)>) -> Self {
        let mut map = Self::create_override(path);
        map.insert_iter(iter);
        map
    }

    /// Create a new DiskMap from an iterator of key-value pairs. If the file at path exists, it will be opened.
    pub fn from_iter_append(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = (K, V)>) -> Self {
        let mut map = Self::create_or_open(path);
        map.insert_iter(iter);
        map
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = (K, V)>) {
        // to preserve insertion order. Likely not important for our usecase.
        self.flush();

        let write_txn = self.db.begin_write().unwrap();
        
        {
            let table_def: TableDefinition<K, V> = TableDefinition::new("table");
            let mut table = write_txn.open_table(table_def).unwrap();

            for (k, v) in iter {
                table.insert(k, v).unwrap();
            }
        }

        write_txn.commit().unwrap();
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.write_cache.push((key, value));
        if self.write_cache.len() > DISK_MAP_WRITE_CACHE_SIZE {
            self.flush();
        }
    }

    pub(crate) fn flush(&mut self) {
        let write_txn = self.db.begin_write().unwrap();
        
        {
            let table_def: TableDefinition<K, V> = TableDefinition::new("table");
            let mut table = write_txn.open_table(table_def).unwrap();

            for (k, v) in self.write_cache.drain(..) {
                table.insert(k, v).unwrap();
            }
        }

        write_txn.commit().unwrap();
    }

    pub fn get(&self, key: K) -> Option<V> {
        let read_txn = self.db.begin_read().unwrap();

        let table_def: TableDefinition<K, V> = TableDefinition::new("table");
        let table = read_txn.open_table(table_def).unwrap();

        let result = table.get(key).ok()?;

        result.map(|v| v.value())
    }

    pub fn r(&self, key: K) -> V {
        self.get(key).unwrap()
    }

    pub fn len(&self) -> u64 {
        let read_txn = self.db.begin_read().unwrap();

        let table_def: TableDefinition<K, V> = TableDefinition::new("table");
        let table = read_txn.open_table(table_def).unwrap();

        table.len().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        let read_txn = self.db.begin_read().unwrap();

        let table_def: TableDefinition<K, V> = TableDefinition::new("table");
        read_txn.open_table(table_def).unwrap().range::<K>(..).unwrap().map(|res| {
            let (k, v) = res.unwrap();
            (k.value(), v.value())
        })
    }
}

impl<K: DiskMapKey, V: DiskMapValue> FromIterator<(K, V)> for DiskMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = Self::create_override(get_new_disk_map_temp_dir());
        map.insert_iter(iter);
        map
    }
}


/// DiskVec<V> is essentially a Vec<V> that is backed by a disk file.
/// It is currently backed by DiskMap<u64, V>, where the key is the index of the value in the Vec.
/// 
/// An important invariant is that the indices are compact, i.e. there are no "holes" in the Vec.
/// Otherwise pushes will overwrite existing values, because the new index is computed from the 'length'.
pub struct DiskVec<V: DiskMapValue> {
    pub(crate) map: DiskMap<u64, V>,
    length: u64,
}

impl<V: DiskMapValue> DiskVec<V> {
    pub(crate) fn from_map(map: DiskMap<u64, V>) -> Self {
        let length = map.len();
        Self {
            map,
            length,
        }
    }

    pub fn create_override(path: impl AsRef<std::path::Path>) -> Self {
        let map = DiskMap::create_override(path);
        Self {
            map,
            length: 0,
        }
    }

    pub fn create_or_open(path: impl AsRef<std::path::Path>) -> Self {
        let map = DiskMap::create_or_open(path);
        let length = map.len();
        Self {
            map,
            length,
        }
    }

    pub fn len(&self) -> usize {
        assert_eq!(self.map.len(), self.length, "DiskVec invariant violated: map.len() != length");
        self.length as usize
    }

    pub fn from_iter_override(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = V>) -> Self {
        let map = DiskMap::from_iter_override(path, iter.into_iter().enumerate().map(|(k, v)| (k as u64, v)));
        let length = map.len();
        Self {
            map,
            length,
        }
    }

    pub fn from_iter_append(path: impl AsRef<std::path::Path>, iter: impl IntoIterator<Item = V>) -> Self {
        let mut map: DiskMap<u64, V> = DiskMap::create_or_open(path);
        // need to shift the enumerate indices by the old length 
        let old_length = map.len();

        map.insert_iter(iter.into_iter().enumerate().map(|(k, v)| (k as u64 + old_length as u64, v)));

        let length = map.len();
        Self {
            map,
            length,
        }
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = V>) {
        let old_length = self.length;
        self.map.insert_iter(iter.into_iter().enumerate().map(|(k, v)| (k as u64 + old_length, v)));
        self.length = self.map.len();
    }

    pub fn push(&mut self, value: V) {
        self.map.insert(self.length, value);
        self.length += 1;
    }

    pub fn get(&self, index: u64) -> Option<V> {
        self.map.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = V> {
        self.map.iter().map(|(_, v)| v)
    }
}