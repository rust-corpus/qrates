
use std::collections::BTreeMap;


///
/// represents a "decl f(Mod, Mod);"
///
#[derive(Debug)]
pub struct Decl {
    pub name: String,
    pub arg_types: Vec<String>,
}

#[derive(Debug)]
pub struct Rule {
    pub name: String,
    pub args: Vec<String>,
    pub facts: Vec<Fact>
}


#[derive(Debug, Clone)]
pub struct Fact { 
    pub name: String,
    pub args: Vec<String> 
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Ident(String),
    Str(String),
    MethodCall{ target: Box<Expr>, name: String, args: Vec<Box<Expr>> },
    Op(Box<Expr>, Op, Box<Expr>)
}

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div
}


impl Rule {
    pub fn is_recursive(&self) -> bool {
        self.facts.iter().filter(|f| f.name == self.name).next().is_some()
    }
}

impl Fact {
    pub fn get_overlapping(&self, other: &Fact) -> Vec<String> {
        self.args.iter().filter(|s| other.args.contains(s)).map(|s| { s.clone() }).collect()
    }
}



