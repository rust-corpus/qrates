
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Context {
    pub queries: BTreeMap<String, Box<Query>>,
    pub main_query: Box<Query>
}

#[derive(Debug)]
pub struct Query {
    pub var_decls: Vec<VarDecl>,
    pub conditions: Option<Box<Expr>>,
    pub selections: Vec<Box<Expr>>,
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


