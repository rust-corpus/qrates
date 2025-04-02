// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! The implementation of interning tables and relations.

use redb::{ReadableTableMetadata, TableDefinition};
use serde_derive::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap, path::PathBuf};

use crate::get_new_disk_map_temp_dir;

mod relation_element;
pub use relation_element::*;

/// A table that expresses a relation between elements.
/// 
/// Supports streaming iteration over the elements and streaming inserts.
/// Typically this should be a `Relation<RelationElement<(Column1, Column2, ... ColumnN)>>`.
/// 
/// Disk-backed.
pub struct Relation<T: DiskMapValue> {
    pub(crate) facts: DiskVec<T>,
}

impl<T: DiskMapValue> Default for Relation<T> {
    fn default() -> Self {
        let vec = DiskVec::create_temp();
        Self { facts: vec }
    }
}

impl<T: DiskMapValue> Relation<T> {
    pub fn insert(&mut self, fact: T) {
        self.facts.push(fact);
    }
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = T>) {
        self.facts.insert_iter(iter);
    }

    /// Iterates over the raw values of the relation as opposed to unwrapping into tuples.
    /// Use Relation::iter for unwrapped tuples.
    pub fn iter_raw(&self) -> impl Iterator<Item = T> {
        self.facts.iter()
    }

    pub(crate) fn from_disk_vec(facts: DiskVec<T>) -> Self {
        Self { facts }
    }

    pub fn create_override_in(path: impl AsRef<std::path::Path>) -> Self {
        let facts = DiskVec::create_override(path);
        Self { facts }
    }

    pub fn from_iter_override(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = T>,
    ) -> Self {
        let facts = DiskVec::from_iter_override(path, iter);
        Self { facts }
    }
}

impl<T> Relation<RelationElement<T>>
where
    RelationElement<T>: DiskMapValue,
{
    pub fn into_tuple_vec(self) -> Vec<T> {
        self.iter().collect()
    }
    pub fn to_tuple_vec(&self) -> Vec<T> {
        self.iter().collect()
    }
    pub fn iter(&self) -> impl Iterator<Item = T> {
        self.facts.iter().map(|re| re.into_inner())
    }

    pub fn from_tuple_iter_override(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = T>,
    ) -> Self {
        let facts: DiskVec<RelationElement<T>> =
            DiskVec::from_iter_override(path, iter.into_iter().map(RelationElement));
        Self { facts }
    }
}

impl<T: DiskMapValue> Into<Vec<T>> for Relation<T> {
    fn into(self) -> Vec<T> {
        self.iter_raw().collect()
    }
}

impl<T: DiskMapValue> Into<Vec<T>> for Relation<RelationElement<T>>
where
    RelationElement<T>: DiskMapValue,
{
    fn into(self) -> Vec<T> {
        self.into_tuple_vec()
    }
}

impl<T: DiskMapValue> From<Vec<T>> for Relation<T> {
    fn from(facts: Vec<T>) -> Self {
        let mut vec = DiskVec::create_temp();
        vec.insert_iter(facts);
        Self { facts: vec }
    }
}

impl<T> From<Vec<T>> for Relation<RelationElement<T>>
where
    RelationElement<T>: DiskMapValue,
{
    fn from(facts: Vec<T>) -> Self {
        let mut vec = DiskVec::create_temp();
        vec.insert_iter(facts.into_iter().map(RelationElement));
        Self { facts: vec }
    }
}

/// A relation converted into a pre-computed key-value map by taking a specific column as a key.
/// 
/// Created in the schema with `keyed by <column name>` syntax.
/// 
/// Disk-backed.
pub struct RelationMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
{
    pub(crate) map: DiskMap<K, V>,
}

impl<K, V> Default for RelationMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
{
    fn default() -> Self {
        let map = DiskMap::create_temp();
        Self { map }
    }
}

impl<K, V> RelationMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
{
    pub fn from_iter_override(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let map = DiskMap::from_iter_override(path, iter);
        Self { map }
    }

    #[track_caller]
    pub fn get(&self, key: K) -> Option<V> {
        self.map.get(key)
    }

    #[track_caller]
    pub fn get_unwrap(&self, key: K) -> V {
        self.get(key).unwrap()
    }
}

/// The type of keys used in disk-backed interning tables.
/// 
/// Because we intern into a vec, the key type must be a usize.
pub trait DiskInterningKey: DiskMapKey + DiskMapValue + Into<usize> + From<usize> + Clone {}
impl<T> DiskInterningKey for T where T: DiskMapKey + DiskMapValue + Into<usize> + From<usize> {}

/// The type of values used in disk-backed interning tables.
/// 
/// Because values can be looked up to get the corresponding key, the value type must also support being a key.
pub trait DiskInterningValue: DiskMapValue + DiskMapKey {}
impl<T> DiskInterningValue for T where T: DiskMapValue + DiskMapKey {}

/// A table that holds interned values.
/// 
/// Disk-backed.
pub struct DiskInterningTable<K, V>
where
    K: DiskInterningKey,
    V: DiskInterningValue,
{
    // TODO: optimization: Instead of using two DiskMaps (DiskVec is backed by DiskMap), could we just add a second index to a single diskmap?
    // The only reason we need `contents` is for `get`ting the value from the key.
    pub(crate) contents: DiskVec<V>,
    pub(crate) inv_map: DiskMap<V, K>,
}

impl<K> DiskInterningTable<K, String>
where
    K: DiskInterningKey,
{
    pub fn lookup_str(&self, value: &str) -> Option<K> {
        self.inv_map.get(value.to_string())
    }
}

impl<K, V> DiskInterningTable<K, V>
where
    K: DiskInterningKey,
    V: DiskInterningValue,
{
    pub fn lookup(&self, value: &V) -> Option<K> {
        self.inv_map.get(value.clone())
    }

    pub fn intern(&mut self, value: V) -> K {
        if let Some(key) = self.lookup(&value) {
            key
        } else {
            // eprintln!("Interning {:?}...", value);
            let new_key: K = self.contents.len().into();
            // eprintln!("...as {:?}", new_key);
            self.inv_map.insert(value.clone(), new_key.clone());
            self.contents.push(value);
            new_key
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        self.contents.iter_enumerated().map(|(k, v)| (k.into(), v))
    }

    pub fn iter_values(&self) -> impl Iterator<Item = V> {
        self.contents.iter()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn get(&self, key: K) -> Option<V> {
        self.contents.get(key.into() as u64)
    }

    pub fn get_unwrap(&self, key: K) -> V {
        self.get(key).unwrap()
    }

    pub fn create_override_in(path: impl AsRef<std::path::Path>) -> Self {
        let contents_path = Self::get_contents_path(path.as_ref());
        let inv_map_path = Self::get_inv_map_path(path.as_ref());
        let contents = DiskVec::create_override(contents_path);
        let inv_map = DiskMap::create_override(inv_map_path);
        Self { contents, inv_map }
    }

    pub(crate) fn get_contents_path(path: impl AsRef<std::path::Path>) -> PathBuf {
        let path: &std::path::Path = path.as_ref();
        let filename = path.file_name().unwrap();
        let filename = filename.to_str().unwrap();
        let filename = format!("{}_contents", filename);
        let mut pathbuf = path.to_path_buf();
        pathbuf.set_file_name(&filename);
        pathbuf
    }

    pub(crate) fn get_inv_map_path(path: impl AsRef<std::path::Path>) -> PathBuf {
        let path: &std::path::Path = path.as_ref();
        let filename = path.file_name().unwrap();
        let filename = filename.to_str().unwrap();
        let filename = format!("{}_inv_map", filename);
        let mut pathbuf = path.to_path_buf();
        pathbuf.set_file_name(&filename);
        pathbuf
    }
}

impl<K, V> Into<Vec<(K, V)>> for &DiskInterningTable<K, V>
where
    K: DiskInterningKey,
    V: DiskInterningValue,
{
    fn into(self) -> Vec<(K, V)> {
        self.iter().collect()
    }
}

impl<K, V> From<InterningTable<K, V>> for DiskInterningTable<K, V>
where
    K: DiskInterningKey + Copy,
    V: DiskInterningValue,
{
    fn from(value: InterningTable<K, V>) -> Self {
        let mut new_temp_contents = DiskVec::create_temp();
        let mut new_temp_inv_map = DiskMap::create_temp();

        for (k, v) in value.into_iter() {
            new_temp_contents.push(v.clone());
            new_temp_inv_map.insert(v, k);
        }

        Self {
            contents: new_temp_contents,
            inv_map: new_temp_inv_map,
        }
    }
}

pub trait InterningTableKey: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> {}
impl<T> InterningTableKey for T where T: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> {}
pub trait InterningTableValue: Eq + std::hash::Hash + Clone {}
impl<T> InterningTableValue for T where T: Eq + std::hash::Hash + Clone {}

#[derive(Deserialize, Serialize)]
#[serde(from = "Vec<V>")]
/// A table that holds the interned values.
/// 
/// Memory-only. Used during extraction.
pub struct InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    pub(crate) contents: Vec<V>,
    #[serde(skip_serializing)]
    inv_contents: HashMap<V, K>,
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
        }
    }
}

impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
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
    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {
        self.contents
            .iter()
            .enumerate()
            .map(|(k, v)| (k.into(), v.clone()))
    }
}

impl<K, V> Into<Vec<(K, V)>> for &InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    fn into(self) -> Vec<(K, V)> {
        self.contents
            .iter()
            .enumerate()
            .map(|(k, v)| (k.into(), v.clone()))
            .collect()
    }
}

/// The type of keys used in `DiskMap`.
/// 
/// Because `DiskMap` is the backend for `Relation`, `RelationMap`, and `DiskInterningTable`, their key types must implement this trait.
/// 
/// Because `DiskInterningTable` is a two-way map behind the scenes, the value type must also implement this trait.
pub trait DiskMapKey:
    Clone
    + Eq
    + std::hash::Hash
    + redb::Key
    + 'static
    + for<'a> Borrow<Self::SelfType<'a>>
    + for<'a> redb::Value<SelfType<'a> = Self>
{
}
impl<T> DiskMapKey for T where
    T: Clone
        + Eq
        + std::hash::Hash
        + redb::Key
        + 'static
        + for<'a> Borrow<Self::SelfType<'a>>
        + for<'a> redb::Value<SelfType<'a> = Self>
{
}

/// The type of keys used in `DiskMap`.
/// 
/// Because `DiskMap` is the backend for `Relation`, `RelationMap`, and `DiskInterningTable`, their value types must implement this trait.
pub trait DiskMapValue:
    Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + 'static
{
}
impl<T> DiskMapValue for T where
    T: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + 'static
{
}

/// DiskMap<K, V> is essentially a HashMap<K, V> that is backed by a disk file.
///
/// Currently it uses a redb::Database backend, and as such it needs a file path to live.
/// If no path is provided (e.g., from `DiskMap::create_temp()` or various std traits like Default or FromIterator),
/// then the DiskMap will be created in a temporary directory that will be deleted when the DiskMap is dropped.
///
/// The functions panic whenever an unexpected database-related error occurs.
pub struct DiskMap<K, V>
where
    K: DiskMapKey,
    V: DiskMapValue,
{
    pub(crate) db: redb::Database,
    write_cache: HashMap<K, V>,
    // if 'true', the file at 'path' will be deleted when the DiskMap is dropped and no flushing will happen.
    pub(crate) is_temp_map: bool,
    path: PathBuf,
    write_cache_size: usize,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: DiskMapKey, V: DiskMapValue> Drop for DiskMap<K, V> {
    fn drop(&mut self) {
        if self.is_temp_map {
            // eprintln!("Dropping tmp DiskMap at {:?} and deleting path", self.path);
            // Unsure why, but redb seems to not care about the file being deleted despite not being dropped yet.
            std::fs::remove_file(&self.path).unwrap();
        } else {
            // eprintln!("Dropping DiskMap at {:?}, thus flushing", self.path);
            self.flush();
        }
    }
}

pub(crate) const DISK_MAP_WRITE_CACHE_SIZE: usize = 100_000;
pub(crate) const DISK_MAP_REDB_CACHE_SIZE: usize = 50_000_000;

impl<K: DiskMapKey, V: DiskMapValue> DiskMap<K, V> {
    const HASH_TABLE_NAME: &str = "hash";
    const EXPECTED_HASH_KEY: u64 = 0;
    const DATA_TABLE_NAME: &str = "table";

    pub fn create_temp() -> Self {
        let path = get_new_disk_map_temp_dir();
        let mut map = Self::create_override(path);
        map.is_temp_map = true;
        map
    }

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
            let table_def: TableDefinition<K, V> = TableDefinition::new(Self::DATA_TABLE_NAME);
            write_txn.open_table(table_def).unwrap();
        }
        write_txn.commit().unwrap();
    }

    pub fn create_or_open(path: impl AsRef<std::path::Path>) -> Self {
        let path = path.as_ref();
        // eprintln!("Opening DiskMap at {:?}", path);
        let mut builder = redb::Database::builder();
        builder.set_cache_size(DISK_MAP_REDB_CACHE_SIZE);

        let db = builder.create(path).unwrap();

        let mut diskmap = Self {
            db,
            path: path.to_path_buf(),
            is_temp_map: false,
            write_cache: HashMap::with_capacity(DISK_MAP_WRITE_CACHE_SIZE),
            write_cache_size: DISK_MAP_WRITE_CACHE_SIZE,
            _phantom: std::marker::PhantomData,
        };
        diskmap.create_self_table();
        diskmap
    }

    fn open_data_table_ro(&self) -> redb::ReadOnlyTable<K, V> {
        let read_txn = self.db.begin_read().unwrap();
        let table_def: TableDefinition<K, V> = TableDefinition::new(Self::DATA_TABLE_NAME);
        read_txn.open_table(table_def).unwrap()
    }

    fn open_hash_table_ro(&self) -> Option<redb::ReadOnlyTable<u64, u64>> {
        let read_txn = self.db.begin_read().unwrap();
        let table_def: TableDefinition<u64, u64> = TableDefinition::new(Self::HASH_TABLE_NAME);
        read_txn.open_table(table_def).ok()
    }

    pub fn set_expected_hash(&mut self, expected_hash: u64) {
        let write_txn = self.db.begin_write().unwrap();
        {
            let table_def: TableDefinition<u64, u64> = TableDefinition::new(Self::HASH_TABLE_NAME);
            let mut table = write_txn.open_table(table_def).unwrap();
            table
                .insert(Self::EXPECTED_HASH_KEY, expected_hash)
                .unwrap();
        }
        write_txn.commit().unwrap();
    }

    pub fn read_stored_hash(&self) -> Option<u64> {
        let hash_table = self.open_hash_table_ro()?;
        let hash = hash_table.get(Self::EXPECTED_HASH_KEY).ok()??;
        Some(hash.value())
    }

    pub fn set_write_cache_size(&mut self, size: usize) {
        self.flush();
        self.write_cache_size = size;
        // Free old cache
        self.write_cache = HashMap::with_capacity(size);
    }

    pub fn path(&self) -> &std::path::Path {
        &self.path
    }

    /// Create a new DiskMap from an iterator of key-value pairs. Destroys the file at path if it exists.
    pub fn from_iter_override(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let mut map = Self::create_override(path);
        map.insert_iter(iter);
        map
    }

    /// Create a new DiskMap from an iterator of key-value pairs. If the file at path exists, it will be opened.
    pub fn from_iter_append(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let mut map = Self::create_or_open(path);
        map.insert_iter(iter);
        map
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = (K, V)>) {
        // to preserve insertion order. Likely not important for our usecase.
        self.flush();

        let write_txn = self.db.begin_write().unwrap();

        {
            let table_def: TableDefinition<K, V> = TableDefinition::new(Self::DATA_TABLE_NAME);
            let mut table = write_txn.open_table(table_def).unwrap();

            for (k, v) in iter {
                table.insert(k, v).unwrap();
            }
        }

        write_txn.commit().unwrap();
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.write_cache.insert(key, value);
        if self.write_cache.len() > self.write_cache_size {
            self.flush();
        }
    }

    pub(crate) fn flush(&mut self) {
        let write_txn = self.db.begin_write().unwrap();

        {
            let table_def: TableDefinition<K, V> = TableDefinition::new(Self::DATA_TABLE_NAME);
            let mut table = write_txn.open_table(table_def).unwrap();

            for (k, v) in self.write_cache.drain() {
                table.insert(k, v).unwrap();
            }
        }

        write_txn.commit().unwrap();
    }

    pub fn get(&self, key: K) -> Option<V> {
        if let Some(value) = self.write_cache.get(&key) {
            return Some(value.clone());
        }

        let table = self.open_data_table_ro();

        let result = table.get(key).ok()?;

        result.map(|v| v.value())
    }

    pub fn get_unwrap(&self, key: K) -> V {
        self.get(key).unwrap()
    }

    pub fn len(&self) -> u64 {
        assert!(
            self.write_cache.is_empty(),
            "DiskMap write cache not empty during len() call"
        );

        let table = self.open_data_table_ro();

        table.len().unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> {
        #[cfg(not(test))]
        assert!(
            self.write_cache.is_empty(),
            "DiskMap write cache not empty during iter() call"
        );

        let table = self.open_data_table_ro();
        let mut persistent_iter = table.range::<K>(..).unwrap().map(|res| {
            let (k, v) = res.unwrap();
            (k.value(), v.value())
        });

        #[cfg(test)]
        {
            // For now just for tests. Does some extra work to make iter work with up-to-date values in presence of an unflushed write cache.
            let mut write_cache = self.write_cache.clone();
            let mut write_cache_iter = None;

            std::iter::from_fn(move || {
                // First try to consume a value from the persistent iter and get its updated value from the write cache.
                if let Some((k, v)) = persistent_iter.next() {
                    if let Some(v) = write_cache.remove(&k) {
                        return Some((k, v));
                    } else {
                        return Some((k, v));
                    }
                } else {
                    if write_cache_iter.is_none() {
                        // First time we're here, so we need to store the remaining write cache.
                        write_cache_iter = Some(write_cache.clone().into_iter());
                    }
                }
                // Then consume the remaining write cache.

                write_cache_iter
                    .as_mut()
                    .unwrap()
                    .next()
                    .map(|(k, v)| (k.clone(), v.clone()))
            })
        }

        #[cfg(not(test))]
        {
            persistent_iter
        }
    }
}

impl<K: DiskMapKey, V: DiskMapValue> FromIterator<(K, V)> for DiskMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = Self::create_temp();
        map.insert_iter(iter);
        map
    }
}

/// DiskVec<V> is essentially a Vec<V> that is backed by a disk file.
/// It is currently backed by DiskMap<u64, V>, where the key is the index of the value in the Vec.
///
/// An important invariant is that the indices are compact and go from 0..length, i.e. there are no "holes" in the Vec.
/// Otherwise pushes will overwrite existing values, because the new index is computed from the 'length', and will
/// overwrite the value at that index in the table.
pub struct DiskVec<V: DiskMapValue> {
    pub(crate) map: DiskMap<u64, V>,
    length: u64,
}

impl<V: DiskMapValue> DiskVec<V> {
    pub(crate) fn from_map(map: DiskMap<u64, V>) -> Self {
        let length = map.len();
        Self { map, length }
    }

    pub fn create_temp() -> Self {
        let map = DiskMap::create_temp();
        Self { map, length: 0 }
    }

    pub fn create_override(path: impl AsRef<std::path::Path>) -> Self {
        let map = DiskMap::create_override(path);
        Self { map, length: 0 }
    }

    pub fn create_or_open(path: impl AsRef<std::path::Path>) -> Self {
        let map = DiskMap::create_or_open(path);
        let length = map.len();
        Self { map, length }
    }

    #[track_caller]
    pub fn len(&self) -> usize {
        // assert_eq!(self.map.len(), self.length, "DiskVec invariant violated: map.len() != length");
        self.length as usize
    }

    pub fn from_iter_override(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = V>,
    ) -> Self {
        let map = DiskMap::from_iter_override(
            path,
            iter.into_iter().enumerate().map(|(k, v)| (k as u64, v)),
        );
        let length = map.len();
        Self { map, length }
    }

    pub fn from_iter_append(
        path: impl AsRef<std::path::Path>,
        iter: impl IntoIterator<Item = V>,
    ) -> Self {
        let mut map: DiskMap<u64, V> = DiskMap::create_or_open(path);
        // need to shift the enumerate indices by the old length
        let old_length = map.len();

        map.insert_iter(
            iter.into_iter()
                .enumerate()
                .map(|(k, v)| (k as u64 + old_length as u64, v)),
        );

        let length = map.len();
        Self { map, length }
    }

    pub fn insert_iter(&mut self, iter: impl IntoIterator<Item = V>) {
        let old_length = self.length;
        self.map.insert_iter(
            iter.into_iter()
                .enumerate()
                .map(|(k, v)| (k as u64 + old_length, v)),
        );
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

    pub fn iter_enumerated(&self) -> impl Iterator<Item = (usize, V)> {
        self.map.iter().map(|(k, v)| (k as usize, v))
    }
}
