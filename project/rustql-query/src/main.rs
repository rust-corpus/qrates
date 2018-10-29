#![feature(box_patterns)]
#![feature(box_syntax)]

extern crate lalrpop_util;
extern crate rustql_common;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod querylang;
pub mod ast;
pub mod engine;

use std::io::{self, Read, Write};
use std::fs::File;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    match querylang::RuleListParser::new().parse(&buffer) {
        Ok(ast) => compile(ast),
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


fn compile(ast: Vec<ast::Rule>) {
    let code = engine::compile_query(ast);

    let mut rust_file = File::create("temp_rust_file.rs").expect("couldn't create temp file");
    rust_file.write_all(code.as_bytes());
}




