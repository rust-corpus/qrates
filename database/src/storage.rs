// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Helper functions for serializing and deserializing.

use crate::data_structures::{InterningTable, InterningTableKey, InterningTableValue, Relation};
use crate::tables::Tables;
use failure::{Error, Fail};
use log::trace;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum LoadError {
    OpenFileError { path: PathBuf, error: String },
    InvalidBincode { path: PathBuf, error: String },
    InvalidJson { path: PathBuf, error: String },
}

impl Fail for LoadError {}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LoadError::OpenFileError { path, error } => {
                write!(f, "Failed to open file {:?}: {}", path, error)
            }
            LoadError::InvalidBincode { path, error } => {
                write!(f, "Invalid bincode {:?}: {}", path, error)
            }
            LoadError::InvalidJson { path, error } => {
                write!(f, "Invalid JSON {:?}: {}", path, error)
            }
        }
    }
}

type LoadResult<T> = Result<T, Error>;

pub fn load<T>(path: &Path) -> LoadResult<T>
where
    for<'de> T: Deserialize<'de>,
{
    trace!("[enter] load({:?})", path);
    let extension = path.extension().unwrap();
    let file = std::fs::File::open(path).map_err(|e| LoadError::OpenFileError {
        path: path.to_path_buf(),
        error: e.to_string(),
    })?;
    let result = if extension == "bincode" {
        bincode::deserialize_from(file).map_err(|e| {
            LoadError::InvalidBincode {
                path: path.to_path_buf(),
                error: e.to_string(),
            }
            .into()
        })
    } else if extension == "json" {
        serde_json::from_reader(file).map_err(|e| {
            LoadError::InvalidJson {
                path: path.to_path_buf(),
                error: e.to_string(),
            }
            .into()
        })
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

/// **Note:** this function is not marked unsafe just so that we could use unsafe blocks
/// to mark precisely where we are performing unsafe operations.
fn unsafe_load_vec<T: Copy>(
    expected_relation_hash: u64,
    mut path: std::path::PathBuf,
) -> Result<Vec<T>, Error> {
    path.set_extension("rc");
    trace!("[enter] load({:?})", path);
    let file = std::fs::File::open(&path).map_err(|e| LoadError::OpenFileError {
        path: path.to_path_buf(),
        error: e.to_string(),
    })?;
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

impl<T: Copy> Relation<T> {
    /// This function is safe only when T does not contain references or pointers.
    /// ``relation_hash`` – the hash of the relation schema. It is used to prevent
    /// loading relations that were saved with a different schema.
    /// ``path`` – the path **without** the extension.
    pub unsafe fn save(&self, relation_hash: u64, path: std::path::PathBuf) {
        unsafe_save_vec(&self.facts, relation_hash, path);
    }
    /// This function is safe only when T does not contain references or pointers.
    /// Also, ``relation_hash`` must be correctly initialized.
    pub unsafe fn load(
        expected_relation_hash: u64,
        path: std::path::PathBuf,
    ) -> Result<Self, Error> {
        unsafe_load_vec(expected_relation_hash, path).map(|vec| vec.into())
    }
}

impl<K, V> InterningTable<K, V>
where
    K: InterningTableKey,
    V: InterningTableValue + Copy,
{
    /// This function is safe only when K and V do not contain references or pointers.
    /// ``table_hash`` – the hash of the interning table schema. It is used to prevent
    /// loading an interning table that was saved with a different schema.
    /// ``path`` – the path **without** the extension.
    pub unsafe fn save(&self, table_hash: u64, path: std::path::PathBuf) {
        unsafe_save_vec(&self.contents, table_hash, path);
    }
    /// This function is safe only when T does not contain references or pointers.
    /// Also, ``relation_hash`` must be correctly initialized.
    pub unsafe fn load(
        expected_relation_hash: u64,
        path: std::path::PathBuf,
    ) -> Result<Self, Error> {
        unsafe_load_vec(expected_relation_hash, path).map(|vec| vec.into())
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
    pub fn load(path: &std::path::Path) -> Result<Self, Error> {
        load(path)
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
