// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(duration_float)]

extern crate lalrpop_util;
extern crate rustql_common;
extern crate libloading;
extern crate glob;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate bincode;

pub mod querylang;
pub mod ast;
pub mod engine;

use glob::glob;
use std::io::{self, Read, Write};
use std::process::Command;
use std::process::ExitStatus;
use std::fs::File;
use libloading::{Library, Symbol};
use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    match querylang::RuleListParser::new().parse(&buffer) {
        Ok((rules, decls, actions)) => compile(rules, decls, actions),
        Err(e) => {
            use lalrpop_util::ParseError::*;
            match e {
                UnrecognizedToken{ token: Some((_, tok, _), ), expected: exp } => {
                    println!("error parsing input: found token \"{}\" expected one of the following: {}", tok.1,
                             exp.iter().fold(String::new(), |acc, new| acc + &new + ", "));
                },
                _ => {
                    println!("error parsing input {:?}", e);
                }
            }
        }
    }
    Ok(())
}

fn find_library(library_name: &str) -> String {
    let mut pattern = String::from("./target/release/deps/lib");
    pattern.push_str(library_name);
    pattern.push_str("*.rlib");
    let paths: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    assert!(paths.len() == 1, "{:?}", paths);
    let mut string = paths[0].display().to_string();
    string.insert(0, '=');
    string.insert_str(0, library_name);
    string
}

fn compile(ast: Vec<ast::Rule>, decls: Vec<ast::Decl>, actions: Vec<ast::Action>) {

    let db = File::open("../rustql-linker/database.db").expect("unable to open database file");
    let database: rustql_common::tuples::Database = bincode::deserialize_from(db).expect("unable to parse database");

    println!("deserialized the database");
    let raw = database.get_raw_database();
    println!("created the raw database");


    for _i in 0..1 {

    let beginning = Instant::now();

    let temp_rust_file_path = "/tmp/temp_rust_file.rs";
    let code = engine::compile_query(ast.clone(), decls.clone(), &actions);

    let mut rust_file = File::create(temp_rust_file_path).expect("couldn't create temp file");
    rust_file.write_all(code.as_bytes());

    let lib_path = "/tmp/libtemp_rust_file.so";

    let output = Command::new("rustc")
            .arg(temp_rust_file_path)
            .arg("-O")
//            .arg("-C")
//            .arg("opt-level=3")
            .arg("--crate-type=cdylib")
            .arg("-L")
            .arg("./target/release/deps")
            .arg("--extern")
            .arg(find_library("datafrog"))
            .arg("-o")
            .arg(lib_path)
            .output()
            .expect("failed to execute rustc");

    if output.status.success() {
        eprintln!("compilation successful!");
        let after_compilation = Instant::now();

        println!("compiled in {}", after_compilation.duration_since(beginning).as_float_secs());

        let lib = Library::new(lib_path).unwrap();
        /*
        let func: Symbol<unsafe fn(&rustql_common::tuples::RawDatabase) -> ()> = unsafe { lib.get(b"print_cool").unwrap() };
        */

        let before_loading_database = Instant::now();

        // measure time
        let after_loading_database = Instant::now();

        //println!("running actions");
        for action in &actions {
            if action.name == "for_each" {
                let func: Symbol<unsafe fn(&rustql_common::tuples::RawDatabase, &rustql_common::tuples::Database) -> ()> =
                    unsafe { lib.get((action.name.clone() + "_" + &action.target).as_bytes()).unwrap() };
                unsafe { func(&raw, &database) };
            }
            else {
                println!("unknown action: {}", action.name);
            }
        }
        let after_running = Instant::now();
        println!("ran all actions in {}", after_running.duration_since(after_loading_database).as_float_secs());
        //println!("database loading took {}", after_loading_database.duration_since(before_loading_database).as_float_secs());
        //println!("#crates {}, #functions {}", database.crates.len(), database.functions.len());
        //unsafe { func(&raw) };
    }
    else {
        println!("compilation error. compiler says the following:");
        std::io::stdout().write(&output.stderr);
    }
    }

    //run: rustc temp_rust_file.rs --crate-type=lib -L ../rustql-common/target/debug/deps --extern "datafrog=../rustql-common/target/debug/deps/libdatafrog-6b64ac73f4a87f58.rlib"
}




