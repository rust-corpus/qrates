// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Module responsible for managing the database.

use corpus_database::tables;
use failure::Error;
use log::{debug, error, info, trace};
use log_derive::logfn;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::{ffi, fs, io};

pub struct DatabaseManager {
    loaded_crates_path: PathBuf,
    loaded_crates: HashSet<String>,
    database_root: PathBuf,
    database: tables::TableMerger,
}

impl DatabaseManager {
    pub fn new(database_root: &Path) -> Self {
        let database_root = database_root.to_path_buf();
        let loaded_crates_path = database_root.join("loaded_crates.json");
        let (loaded_crates, database) = if database_root.exists() {
            // The database already contains some crates.
            let file = fs::File::open(&loaded_crates_path).unwrap_or_else(|e| {
                panic!(
                    "The database state is corrupted. \
                     Failed to read the list of loaded crates {:?}: {}",
                    loaded_crates_path, e
                )
            });
            let loaded_crates = serde_json::from_reader(file).unwrap_or_else(|e| {
                panic!(
                    "The database state is corrupted. The crates list is invalid JSON {:?}: {}",
                    loaded_crates_path, e
                )
            });
            (
                loaded_crates,
                tables::Tables::load_multifile(&database_root).unwrap(),
            )
        } else {
            fs::create_dir_all(&database_root)
                .expect("Failed to create the directory for the database");
            (HashSet::new(), tables::Tables::default())
        };
        Self {
            loaded_crates_path,
            loaded_crates,
            database_root,
            database: tables::TableMerger::new(database),
        }
    }
    #[logfn(Trace)]
    pub fn update_database(&mut self, workspace_root: &Path) {
        let crates = self.scan_crates(&workspace_root.join("rust-corpus"));
        let mut success_counter = 0;
        let mut fail_counter = 0;
        for path in crates {
            trace!("Checking crate: {:?}", path);
            let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
            if self.loaded_crates.contains(&file_name) {
                debug!("Crate already loaded: {:?} {}", path, file_name);
            } else {
                info!("Loading crate ({}): {:?}", success_counter, path);
                match self.load_crate(file_name, path) {
                    Ok(()) => success_counter += 1,
                    Err(e) => {
                        fail_counter += 1;
                        error!("  Error occurred: {}", e)
                    }
                };
            }
        }
        info!("Successfully loaded {} crates", success_counter);
        if fail_counter > 0 {
            error!("Failed to load {} crates", fail_counter);
        }
        // Delete the loaded crates file so that if we crash, we know that we are
        // in a corrupted state.
        match fs::remove_file(&self.loaded_crates_path) {
            Ok(_) => {}
            Err(error) => {
                if error.kind() != io::ErrorKind::NotFound {
                    panic!("Failed to remove the loaded crates file.")
                }
            }
        }
        self.database
            .tables()
            .store_multifile(&self.database_root)
            .unwrap();
        info!("Successfully updated the database");
        let mut file = fs::File::create(&self.loaded_crates_path)
            .unwrap_or_else(|e| panic!("Unable to create {:?}: {}", self.loaded_crates_path, e));
        serde_json::to_writer_pretty(&mut file, &self.loaded_crates)
            .unwrap_or_else(|e| panic!("Unable to write {:?}: {}", self.loaded_crates_path, e));
        info!("Successfully saved the loaded crates list");
    }
    fn scan_crates(&self, workspace_root: &Path) -> impl Iterator<Item = PathBuf> {
        walkdir::WalkDir::new(workspace_root.canonicalize().unwrap())
            .into_iter()
            .filter_entry(|entry| entry.file_name() != "source")
            .map(|entry| entry.unwrap().into_path())
            .filter(|path| path.extension() == Some(ffi::OsStr::new("bincode")))
    }
    #[logfn(Trace)]
    fn load_crate(&mut self, file_name: String, crate_path: PathBuf) -> Result<(), Error> {
        let crate_tables = tables::Tables::load(&crate_path)?;
        self.database.merge(crate_tables);
        self.loaded_crates.insert(file_name);
        Ok(())
    }
}
