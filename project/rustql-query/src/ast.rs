
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Context {
    pub queries: BTreeMap<String, Box<Query>>,
    pub main_query: Box<Query>
}

#[derive(Debug)]
pub enum Query {
    Simple {
        var_decls: Vec<VarDecl>,
        conditions: Option<Box<Expr>>,
        selections: Vec<Box<Expr>>,
    },
    Union(Box<Query>, Box<Query>)
}

#[derive(Debug)]
pub struct VarDecl {
    pub name: String,
    pub type_name: String
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


