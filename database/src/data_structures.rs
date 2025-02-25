// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! The implementation of interning tables and relations.

use log::info;
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
    pub(crate) read_only_table: Option<redb::ReadOnlyTable<K, V>>,
}

impl<K, V> Default for RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
{
    fn default() -> Self {
        Self {
            contents: HashMap::new(),
            db: None,
            read_only_table: None,
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
            read_only_table: None,
        }
    }
}

impl<K, V> RelationMap<K, V>
where K: RelationMapKey,
        V: RelationMapValue,
        for<'a> &'a K: Borrow<<K as redb::Value>::SelfType<'a>>
{
    #[track_caller]
    pub fn get_redb(&self, key: K) -> Option<V> {
        if let Some(table) = &self.read_only_table {
            return Some(table.get(&key).ok()??.value());
        }
        None
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