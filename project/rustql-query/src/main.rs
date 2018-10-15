#![feature(box_patterns)]
extern crate lalrpop_util;
extern crate rustql_common;

pub mod querylang;
pub mod ast;
pub mod sem;
pub mod engine;

use std::io::{self, Read};
use engine::execute_query;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    match querylang::QueryContextParser::new().parse(&buffer) {
        Ok(ast) => execute_query(ast),
        Err(e) => {
            use lalrpop_util::ParseError::*;
            match e {
                UnrecognizedToken{ token: Some((_, tok, _), ), expected: exp } => {
                    println!("error parsing input: found token \"{}\" expected one of the following: {}", tok.1,
                             exp.iter().fold(String::new(), |acc, new| acc + &new + ", "));
                },
                _ => {
                    println!("error parsing input");
                }
            }
        }
    }
    Ok(())
}




