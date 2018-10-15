use super::ast;
use super::sem;
use rustql_common::tuples;



pub fn execute_query(context: Box<ast::Context>) {
    let database: tuples::Database = tuples::Database::new();
    let ctxt = sem::Context::new();

    let functions: Vec<u64> = Vec::new();

    let box ast::Query {var_decls: ref vd, conditions: ref cond, selections: ref sel} = context.main_query;

    let mut input_types: Vec<String> = vec![];
    for v in &vd {
        v.
    }


    let mut from_vars: Vec<Vec<u64>> = Vec::new();
}



