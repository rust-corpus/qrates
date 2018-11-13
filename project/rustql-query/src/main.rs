#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate lalrpop_util;
extern crate rustql_common;
extern crate libloading;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate bincode;

pub mod querylang;
pub mod ast;
pub mod engine;

use std::io::{self, Read, Write};
use std::process::Command;
use std::process::ExitStatus;
use std::fs::File;
use libloading::{Library, Symbol};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    match querylang::RuleListParser::new().parse(&buffer) {
        Ok((rules, decls)) => compile(rules, decls),
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


fn compile(ast: Vec<ast::Rule>, decls: Vec<ast::Decl>) {


    let code = engine::compile_query(ast, decls);

    let mut rust_file = File::create("temp_rust_file.rs").expect("couldn't create temp file");
    rust_file.write_all(code.as_bytes());

    let lib_path = "/tmp/libtemp_rust_file.so";

    let output = Command::new("rustc")
            .arg("temp_rust_file.rs")
            .arg("-O")
            .arg("--crate-type=cdylib")
            .arg("-L")
            .arg("../rustql-common/target/debug/deps")
            .arg("--extern")
            .arg("datafrog=../rustql-common/target/debug/deps/libdatafrog-6b64ac73f4a87f58.rlib")
            .arg("-o")
            .arg(lib_path)
            .output()
            .expect("failed to execute rustc");

    if output.status.success() {
        println!("compilation worked!");

        let lib = Library::new(lib_path).unwrap();
        let func: Symbol<unsafe fn(&rustql_common::tuples::RawDatabase) -> ()> = unsafe { lib.get(b"print_a").unwrap() };


        let db = File::open("../rustql-linker/database.db").expect("unable to open database file");
        let database: rustql_common::tuples::Database = bincode::deserialize_from(db).expect("unable to parse database");
        let raw = database.get_raw_database();

        unsafe { func(&raw) };
    }
    else {
        println!("compilation error. compiler says the following:");
        std::io::stdout().write(&output.stderr);
    }

    //run: rustc temp_rust_file.rs --crate-type=lib -L ../rustql-common/target/debug/deps --extern "datafrog=../rustql-common/target/debug/deps/libdatafrog-6b64ac73f4a87f58.rlib"
}




