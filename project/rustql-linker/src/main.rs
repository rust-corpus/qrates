extern crate rustql_common;
extern crate datafrog;
extern crate serde_json;
extern crate bincode;


#[feature(nll)]

use rustql_common::tuples;
use rustql_common::data;
use std::collections::HashMap;
use datafrog::{Iteration, Relation, Variable};
use std::fs::File;


const USE_JSON: bool = false;

const TARGET_DIR_VARNAME: &str = "EXTRACTOR_TARGET_DIR";

fn main() {
    let database = create_database();
    // println!("{:?}", database.crates);
    // println!("{:?}", database.modules);
    // println!("{:?}", database.functions.iter().map(|(_, f)| &f.name).collect::<Vec<&String>>());

    save_database(&database, "database.db");

    //run_query(&database);

    /*for val in database.modules_in_crates {
        println!("{:?}", val);
    }
    for val in database.modules_in_modules {
        println!("{:?}", val);
    }
    for val in database.functions_in_modules {
        println!("{:?}", val);
    }
    for val in database.function_calls {
        println!("{:?}", val);
    }*/
}

fn save_database(database: &tuples::Database, name: &str) {
    let file = File::create(name).expect("could not create database file");
    //serde_json::to_writer_pretty(file, database).expect("could not serialize to json");
    bincode::serialize_into(file, database).expect("could not serialize to json");
}

/*fn run_query(database: &tuples::Database) {
    let mut iteration = Iteration::new();

    let calls = Relation::from(database.function_calls.clone().into_iter().map(|(a, b)| (b, a)));

    let other_relation: Relation<(tuples::Function, tuples::Function)> = calls.iter().filter(|_| true).map(|x| *x).into();

    let calls_var = iteration.variable::<(tuples::Function, tuples::Function)>("calls");
    calls_var.insert(calls.into());

    let unsafe_infected = iteration.variable::<(tuples::Function, ())>("infected");

    // add all unsafe function to infected-variable
    unsafe_infected.insert(database.functions.iter().filter(|(_id, f)| f.is_unsafe ).map(|x| (x.0, ())).into());

    while iteration.changed() {
        unsafe_infected.from_join(&unsafe_infected, &calls_var, |_k, _, &l| (l, ()));
    }
    let infected: Vec<tuples::Function> = unsafe_infected.complete().into_iter().map(|x| x.0).collect();

    println!("functions that call unsafe functions are:");
    for f in infected.iter().map(|tuples::Function(id)| &database.functions[*id as usize].1) {
        println!("{:?}", f);
    }
}*/

fn create_database() -> tuples::Database {
    let mut database = tuples::Database::new();
    let crates = read_crates();
    database.crates = crates.iter().map(|c| c.metadata.clone()).zip(0..).map(|(a, b)| (tuples::Crate(b), a)).collect();

    for (krate, krate_id) in crates.iter().zip(0..) {
        let mod_offset = database.modules.len();
        let fn_offset = database.functions.len();
        let ty_offset = database.types.len();

        let module_map_to_global = |index: usize| index + mod_offset;
        let fn_map_to_global = |index: usize| index + fn_offset;

        database.modules.extend(krate.mods.iter().zip(
                mod_offset..).map(|(a, b)| (tuples::Mod(b as u64), a.clone())));
        database.functions.extend(krate.functions.iter().zip(
                database.functions.len()..).map(|(a, b)| (tuples::Function(b as u64), a.clone())));
        database.function_finder.extend(krate.functions.iter().zip(
                database.function_finder.len()..).map(|(a, b)| ((krate.metadata.clone(), a.def_path.clone()), (tuples::Function(b as u64)))));

        database.structs.extend(krate.structs.iter().zip(
                database.structs.len()..).map(|(a, b)| (tuples::Struct(b as u64), a.clone())));
        /*database.types.extend(krate.types.iter().zip(
                ty_offset..).map(|(a, b)| (tuples::Type(b as u64), a.clone())));*/

        for (m, mod_id) in krate.mods.iter().zip((0..).map(module_map_to_global)) {
            let tuple = (tuples::Mod(mod_id as u64), tuples::Crate(krate_id));
            database.modules_in_crates.push(tuple);
            if let Some(parent_id) = m.parent_mod.map(module_map_to_global) {
                let tuple = (tuples::Mod(mod_id as u64), tuples::Mod(parent_id as u64));
                database.modules_in_modules.push(tuple);
            }
        }

        for (f, fn_id) in krate.functions.iter().zip((0..).map(fn_map_to_global)) {
            let parent_id = module_map_to_global(f.containing_mod);
            let tuple = (tuples::Function(fn_id as u64), tuples::Mod(parent_id as u64));
            database.functions_in_modules.push(tuple);
        }
    }

    // create function call links
    let mut fails: usize = 0;
    for (f_id, func) in &database.functions {
        for call in &func.calls {
            //let crate_id = database.get_crate(&call.crate_ident).unwrap();
            //let called_id = database.get_function_in_crate(crate_id, &call.def_path);
            let called_id = database.function_finder.get(&(call.crate_ident.clone(), call.def_path.clone()));
            if let Some(called_id) = called_id {
                database.function_calls.push((*f_id, *called_id));
            }
            else {
                // TODO find out why it didn't work
                println!("unresolved function call to {:?}", call.def_path);
                println!("fns: {:?}", database.functions);
                fails += 1;
            }
        }

        for arg in &func.argument_types {
            let mut type_id = database.get_type(arg);
            if let None = type_id {
                let len = tuples::Type(database.types.len() as u64);
                type_id = Some(len);
                database.types.push((len, arg.clone()));
            }
            database.argument_types.push((*f_id, type_id.unwrap()))
        }
    }
    println!("ratio fails / calls: {}", fails as f64 / (database.function_calls.len() + fails) as f64);

    
    database.link_types();
    println!("linked structs and types, created {} links.", database.is_struct_type.len());

    // adding field types
    for (s_id, st) in &database.structs {
        for (_name, typ) in &st.fields {
            let mut type_id = database.get_type(&typ);
            if let None = type_id {
                let len = tuples::Type(database.types.len() as u64);
                type_id = Some(len);
                database.types.push((len, typ.clone()));
            }
            database.field_types.push((*s_id, type_id.unwrap()))
        }
    }

    for (f_id, f) in &database.functions {
        let mut type_id = database.get_type(&f.return_type);
        if let None = type_id {
            let len = tuples::Type(database.types.len() as u64);
            type_id = Some(len);
            database.types.push((len, f.return_type.clone()));
        }
        database.return_type.push((*f_id, type_id.unwrap()))
    }

    database
}

///
/// reads all crates in the crate folder into a data structure.
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
            let c = if USE_JSON {
                serde_json::from_reader(f).map_err(|_| ())
            }
            else {
                bincode::deserialize_from(f).map_err(|_| ())
            };

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


