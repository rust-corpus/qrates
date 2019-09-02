// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(duration_float)]

extern crate csv;
extern crate glob;
extern crate lalrpop_util;
extern crate libloading;
extern crate rustql_common;

extern crate bincode;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate text_io;

pub mod ast;
pub mod engine;
pub mod querylang;

use csv::{Writer, WriterBuilder};
use glob::glob;
use libloading::{Library, Symbol};
use rustql_common::tuples;
use std::fs::{self, File};
use std::io::{self, Write};
use std::process::Command;
use std::time::Instant;

fn main() -> ! {
    let db = File::open("../rustql-linker/database.db").expect("unable to open database file");
    let database: rustql_common::tuples::Database =
        bincode::deserialize_from(db).expect("unable to parse database");

    println!("deserialized the database");
    let raw = database.get_raw_database();
    println!("created the raw database");

    let mut counter = 0;

    loop {
        print!("File with a query: ");
        io::stdout().flush().unwrap();
        let file_name: String = read!("{}\n");

        counter += 1;

        if file_name.len() == 0 {
            continue;
        }

        if let Ok(buffer) = fs::read_to_string(&file_name) {
            match querylang::RuleListParser::new().parse(&buffer) {
                Ok((rules, decls, actions)) => {
                    compile(rules, decls, actions, &raw, &database, counter)
                }
                Err(e) => {
                    use lalrpop_util::ParseError::*;
                    match e {
                        UnrecognizedToken {
                            token: Some((_, tok, _)),
                            expected: exp,
                        } => {
                            println!("error parsing input: found token \"{}\" expected one of the following: {}", tok.1,
                                     exp.iter().fold(String::new(), |acc, new| acc + &new + ", "));
                        }
                        _ => {
                            println!("error parsing input {:?}", e);
                        }
                    }
                }
            }
        }
    }
}

fn find_library(library_name: &str) -> String {
    let mut pattern = String::from("./target/release/deps/lib");
    pattern.push_str(library_name);
    pattern.push('-');
    pattern.push_str("*.rlib");
    let paths: Vec<_> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
    assert!(paths.len() == 1, "{:?}", paths);
    let mut string = paths[0].display().to_string();
    string.insert(0, '=');
    string.insert_str(0, library_name);
    string
}

fn compile(
    ast: Vec<ast::Rule>,
    decls: Vec<ast::Decl>,
    actions: Vec<ast::Action>,
    raw: &tuples::RawDatabase,
    database: &tuples::Database,
    counter: usize,
) {
    for _i in 0..1 {
        let beginning = Instant::now();

        let temp_rust_file_path = "/tmp/temp_rust_file.rs";
        let code = engine::compile_query(ast.clone(), decls.clone(), &actions);

        let mut rust_file = File::create(temp_rust_file_path).expect("couldn't create temp file");
        rust_file
            .write_all(code.as_bytes())
            .expect("Failed to write code");

        let lib_path = format!("/tmp/libtemp_rust_file_{}.so", counter);

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
            .arg("--extern")
            .arg(find_library("csv"))
            .arg("-o")
            .arg(&lib_path)
            .output()
            .expect("failed to execute rustc");

        if output.status.success() {
            eprintln!("compilation successful!");
            let after_compilation = Instant::now();

            println!(
                "compiled in {}",
                after_compilation.duration_since(beginning).as_secs_f64()
            );

            let lib = Library::new(lib_path).unwrap();
            /*
            let func: Symbol<unsafe fn(&rustql_common::tuples::RawDatabase) -> ()> = unsafe { lib.get(b"print_cool").unwrap() };
            */

            // measure time
            let after_loading_database = Instant::now();

            //println!("running actions");
            for action in &actions {
                if action.name == "for_each" {
                    let res: io::Result<
                        Symbol<
                            unsafe fn(
                                &rustql_common::tuples::RawDatabase,
                                &rustql_common::tuples::Database,
                            ) -> (),
                        >,
                    > = unsafe { lib.get((action.name.clone() + "_" + &action.target).as_bytes()) };
                    match res {
                        Ok(func) => {
                            unsafe { func(raw, database) };
                        }
                        Err(error) => {
                            println!("Error: {:?}", error);
                        }
                    }
                } else if action.name == "csv" {
                    let res: io::Result<
                        Symbol<
                            unsafe fn(
                                &mut Writer<File>,
                                &rustql_common::tuples::RawDatabase,
                                &rustql_common::tuples::Database,
                            ) -> (),
                        >,
                    > = unsafe { lib.get((action.name.clone() + "_" + &action.target).as_bytes()) };
                    match res {
                        Ok(func) => {
                            let path = action.target.clone() + ".csv";
                            let mut wtr = WriterBuilder::new().from_path(&path).unwrap();
                            unsafe { func(&mut wtr, raw, database) };
                        }
                        Err(error) => {
                            println!("Error: {:?}", error);
                        }
                    }
                } else {
                    println!("unknown action: {}", action.name);
                }
            }
            let after_running = Instant::now();
            println!(
                "ran all actions in {}",
                after_running
                    .duration_since(after_loading_database)
                    .as_secs_f64()
            );
        //println!("database loading took {}", after_loading_database.duration_since(before_loading_database).as_float_secs());
        //println!("#crates {}, #functions {}", database.crates.len(), database.functions.len());
        //unsafe { func(&raw) };
        } else {
            println!("compilation error. compiler says the following:");
            std::io::stdout().write(&output.stderr).unwrap();
        }
    }

    //run: rustc temp_rust_file.rs --crate-type=lib -L ../rustql-common/target/debug/deps --extern "datafrog=../rustql-common/target/debug/deps/libdatafrog-6b64ac73f4a87f58.rlib"
}
