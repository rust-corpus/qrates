// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


///
/// represents a "decl f(Mod, Mod);"
///
#[derive(Debug, Clone)]
pub struct Decl {
    pub name: String,
    pub arg_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub args: Vec<String>,
    /// vector with facts, each with bool indicating if it is negated
    pub facts: Vec<(Fact, bool)>,
}

#[derive(Debug, Clone)]
pub struct Fact { 
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(i32),
    Ident(String),
    Str(String),
    MethodCall{ target: Box<Expr>, name: String, args: Vec<Box<Expr>> },
    Op(Box<Expr>, Op, Box<Expr>)
}

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone)]
pub struct Action {
    pub name: String,
    pub target: String,
    pub rust_code: String
}


impl Rule {
    pub fn is_recursive(&self) -> bool {
        self.facts.iter().filter(|(f, _negated)| f.name == self.name).next().is_some()
    }
}

impl Fact {
    pub fn get_overlapping(&self, other: &Fact) -> Vec<String> {
        self.args.iter().filter(|s| other.args.contains(s)).map(|s| { s.clone() }).collect()
    }
}



