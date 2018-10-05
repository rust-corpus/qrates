
extern crate rustql_common;
extern crate datafrog;
extern crate serde_json;

mod tuples;

use rustql_common::data;

const TARGET_DIR_VARNAME: &str = "EXTRACTOR_TARGET_DIR";

fn main() {
    let database = create_database();
    println!("{:?}", database.crates);
    println!("{:?}", database.modules);
    println!("{:?}", database.functions.iter().map(|(_, f)| &f.name).collect::<Vec<&String>>());
}

fn create_database() -> tuples::Database {
    let mut database = tuples::Database::new();
    let crates = read_crates();
    database.crates = crates.iter().map(|c| c.metadata.clone()).zip(0..).map(|(a, b)| (tuples::Crate(b), a)).collect();
    let 
    for krate in crates {
        database.modules.extend(krate.mods.into_iter().zip(
                database.modules.len()..).map(|(a, b)| (tuples::Mod(b as u64), a)));
        database.functions.extend(krate.functions.into_iter().zip(
                database.functions.len()..).map(|(a, b)| (tuples::Function(b as u64), a)));
    }

    database
}

///
/// reads all crates in the crate folder into a datastructure.
///
/// @warning the more crates there are, the more RAM it needs (a lot)
///
fn read_crates() -> Vec<data::Crate> {
    use std::env;
    use std::fs;
    use std::fs::File;

    let dirname = env::var(TARGET_DIR_VARNAME).unwrap_or(env::var("HOME").unwrap_or("/".to_owned()) +
                                                         "/.rustql/crates");

    let files = fs::read_dir(dirname).unwrap();
    let mut crates: Vec<data::Crate> = vec![];

    for file in files {
        if let Ok(path) = file {
            let f = File::open(path.path()).unwrap();
            let c = serde_json::from_reader(f);
            if let Ok(cr) = c {
                crates.push(cr);
            }
            else {
                panic!("error deserializing crate {:?}: {:?}", path.path(), c);
            }
        }
    }
    crates
}


