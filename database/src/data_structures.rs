// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! The implementation of interning tables and relations.

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

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
        Self { facts: facts }
    }
}

pub trait InterningTableKey: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> {}
impl<T> InterningTableKey for T where T: Copy + Eq + std::hash::Hash + From<usize> + Into<usize> {}
pub trait InterningTableValue: Eq + std::hash::Hash + Clone {}
impl<T> InterningTableValue for T where T: Eq + std::hash::Hash + Clone {}

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
}

impl<K> InterningTable<K, String>
where
    K: InterningTableKey,
{
    pub fn lookup_str(&self, value: &str) -> Option<K> {
        self.inv_contents.get(value).cloned()
    }
}

impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    pub fn lookup(&self, value: &V) -> Option<K> {
        self.inv_contents.get(value).cloned()
    }
}

impl<K, V> std::ops::Index<K> for InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        let index: usize = key.into();
        &self.contents[index]
    }
}

impl<K, V> Into<Vec<(K, V)>> for InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue,
{
    fn into(self) -> Vec<(K, V)> {
        self.contents
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.into(), v))
            .collect()
    }
}
