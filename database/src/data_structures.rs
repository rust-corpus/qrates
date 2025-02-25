// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! The implementation of interning tables and relations.

use log::{info, warn};
use redb::{ReadableTable, TableDefinition};
use serde_derive::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::HashMap};

use crate::storage::Hack;

#[derive(Deserialize, Serialize)]
/// A table that expresses a relation between elements.
pub struct Relation<T> {
    pub(crate) facts: Vec<T>,
}

impl<T> Default for Relation<T> {
    fn default() -> Self {
        Self { facts: Vec::new() }
    }
}

impl<T> Relation<T> {
    pub fn insert(&mut self, fact: T) {
        self.facts.push(fact);
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.facts.iter()
    }
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.facts.into_iter()
    }
    pub fn len(&self) -> usize {
        self.facts.len()
    }
}

impl<T> Into<Vec<T>> for Relation<T> {
    fn into(self) -> Vec<T> {
        self.facts
    }
}

impl<T> From<Vec<T>> for Relation<T> {
    fn from(facts: Vec<T>) -> Self {
        Self { facts }
    }
}

pub trait RelationMapKey: Eq + std::hash::Hash + redb::Key + 'static {}
impl<T> RelationMapKey for T where T: Eq + std::hash::Hash + redb::Key + 'static {}
pub trait RelationMapValue: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key + 'static {}
impl<T> RelationMapValue for T where T: Eq + std::hash::Hash + Clone + for<'a> redb::Value<SelfType<'a> = Self> + redb::Key + 'static {}

pub struct RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
{
    pub(crate) contents: HashMap<K, V>,
    pub(crate) db: Option<redb::Database>,
    // pub(crate) read_only_table: Option<redb::ReadOnlyTable<K, V>>,
    pub(crate) rot_name: Option<String>,
}

impl<K, V> Default for RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
{
    fn default() -> Self {
        Self {
            contents: HashMap::new(),
            db: None,
            // read_only_table: None,
            rot_name: None,
        }
    }
}

impl<K, V> From<HashMap<K, V>> for RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
{
    fn from(contents: HashMap<K, V>) -> Self {
        Self {
            contents,
            db: None,
            // read_only_table: None,
            rot_name: None,
        }
    }
}

impl<K, V> RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
        for<'a> &'a K: Borrow<<K as redb::Value>::SelfType<'a>>
{
    fn get_rot(&self) -> Option<redb::ReadOnlyTable<K, V>> {
        let table_def = TableDefinition::<K, V>::new(self.rot_name.as_ref()?);
        let read_txn = self.db.as_ref()?.begin_read().ok()?;
        let table = read_txn.open_table(table_def).ok()?;
        Some(table)
    }

    #[track_caller]
    pub fn get_redb(&self, key: K) -> Option<V> {
        if let Some(table) = self.get_rot() {
            return Some(table.get(&key).ok()??.value());
        }
        None
    }

    #[track_caller]
    pub fn r(&self, key: K) -> V {
        self.get_redb(key).unwrap()
    }
}

pub trait InterningTableKey: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> + redb::Key +'static {}
impl<T> InterningTableKey for T where T: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> + redb::Key + 'static {}
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
    pub(crate) rot_table_name: Option<String>,
    // note: benched with a cached ReadOnlyTable, and read speeds are within a few percent, but transactional memory usage explodes.
    // pub(crate) read_only_table: Option<redb::ReadOnlyTable<u64, Hack<V>>>,
    #[serde(skip_serializing)]
    pub(crate) rot_inv_table_name: Option<String>,
    // pub(crate) read_only_inv_table: Option<redb::ReadOnlyTable<V, u64>>,
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
            rot_table_name: None,
            rot_inv_table_name: None,
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
            rot_table_name: None,
            rot_inv_table_name: None,
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
        if let Some(table) = self.get_rot_inv() {
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

    #[inline]
    // #[track_caller]
    fn get_rot(&self) -> Option<redb::ReadOnlyTable<u64, Hack<V>>> {
        // eprintln!("self.rot_table_name.as_ref() = {:?}", self.rot_table_name.as_ref());
        // eprintln!("self.db.is_some() = {:?}", self.db.is_some());
        // let table_def = TableDefinition::<u64, Hack<V>>::new(self.rot_inv_table_name.as_ref()?);
        // let read_txn = self.db.as_ref()?.begin_read().ok()?;
        // let table = read_txn.open_table(table_def).ok()?;
        // Some(table)

        let table_def = TableDefinition::<u64, Hack<V>>::new(self.rot_table_name.as_ref()?);
        let read_txn = self.db.as_ref().unwrap().begin_read().unwrap();
        let table = read_txn.open_table(table_def).unwrap();
        Some(table)
    }

    #[inline]
    #[track_caller]
    fn get_rot_inv(&self) -> Option<redb::ReadOnlyTable<V, u64>> {
        let table_def = TableDefinition::<V, u64>::new(self.rot_inv_table_name.as_ref()?);
        let read_txn = self.db.as_ref()?.begin_read().ok()?;
        let table = read_txn.open_table(table_def).ok()?;
        Some(table)
    }

    pub fn lookup(&self, value: &V) -> Option<K> {
        if let Some(table) = self.get_rot_inv() {
            return Some((table.get(value).ok()??.value() as usize).into());
        }
        None
    }

    pub fn get_redb(&self, key: K) -> Option<V> {
        let index: usize = key.into();
        if let Some(table) = self.get_rot() {
            return Some(table.get(index as u64).ok()??.value());
        }
        None
    }

    pub fn r(&self, key: K) -> V {
        self.get_redb(key).unwrap()
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, V)> + '_ {

        let rot = self.get_rot().unwrap();

        let iter = rot.iter().unwrap().map(move |res| {
            let (k, v) = res.unwrap();
            ((k.value() as usize).into(), v.value())
        });

        // TODO: fixme
        warn!("expensive InterningTable::iter called");

        iter.collect::<Vec<_>>().into_iter()

    }
}

// struct RotOwnedIter<K, V>
// where 
// K: InterningTableKey,
// V: InterningTableValue,
// {
//     rot: redb::ReadOnlyTable<K, V>,
// }

// impl<K, V> Iterator for RotOwnedIter<K, V>
// where
// K: InterningTableKey,
// V: InterningTableValue,
// {
//     type Item = (K, V);
//     fn next(&mut self) -> Option<Self::Item> {
//         self.rot.iter().unwrap().next().map(|res| {
//             let (k, v) = res.unwrap();
//             (k, v)
//         })
//     }
// }

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
