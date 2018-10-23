
use std::collections::BTreeMap;
use std::fs::File;

use super::ast;
use super::sem;
use rustql_common::tuples;


pub fn create_sem_query(q: ast::Query, ctxt: &mut sem::Context) -> sem::Relation
{
    // maps variable names to their type
    let mut variables: BTreeMap<String, String> = BTreeMap::new();
    match q {
        ast::Query::Simple{ var_decls, conditions, selections } => {
            read_variables(&var_decls, &mut variables);
            let mut relations: Vec<sem::Relation> = vec![];
            let mut relation: sem::Relation = sem::Relation::Native("None".to_owned());

            let selection = selections.iter().next();
            if let Some(box ast::Expr::Ident(name)) = selection {
                let var_type = variables.get(name);
                if let Some(typ) = var_type {
                    if let Some(typ) = ctxt.get_type(typ) {
                        relation = sem::Relation::Native(typ.rust_name.clone());
                    }
                    else {
                        panic!("type not found!");
                    }
                }
                else {
                    panic!("not a valid type!");
                }
            }

            relation

            /*
            sem::Query {
                transfromations: selections.into_iter().map(|box s| -> Option<String> {
                        if let ast::Expr::Ident(ref id) = s {
                            variables.get(id).map(String::clone)
                        }
                        else {
                            None
                        }
                    })
                    .filter(|x| x.is_some())
                    .map(Option::unwrap)
                    .map(|s| sem::Transformation::Filter{
                        var: sem::Variable{ name: s }, filter: sem::FilterFunc{}})
                    .collect()
            }*/
        },
        ast::Query::Union(box query1, box query2) => {
            //sem::Query { transfromations: vec![] }
            sem::Relation::Native("NotSupported".to_owned())
        }
    }
}


pub fn read_variables(decls: &Vec<ast::VarDecl>, map: &mut BTreeMap<String, String>) {
    for var in decls {
        map.insert(var.name.clone(), var.type_name.clone());
    }
}


pub fn execute_query(context: Box<ast::Context>) {
    let database: tuples::Database = tuples::Database::new();
    let ctxt_file = File::open("context.json").expect("context file not found");
    let mut ctxt = serde_json::from_reader(ctxt_file).unwrap();// = sem::Context::new();

    let functions: Vec<u64> = Vec::new();

    let semq = create_sem_query(*context.main_query, &mut ctxt);

    if let sem::Relation::Transformation(box sem::Transformation::Filter{ ref var, ref filter }) = semq {
        let typ = ctxt.get_type(&var.name);
        println!("type {:?}", typ);
    }

    println!("{:?}", semq);
    //println!("{}", query_to_rust(semq, &ctxt));
    //println!("{}", serde_json::to_string_pretty(&ctxt).unwrap());

    //let trans1 = sem::Transformation::Filter { scan: sem::RelationId(0), filter: sem::FilterFunc{} };
    //let rs = generate_rust(&ctxt, &trans1);
    //println!("{}", rs);


    let mut from_vars: Vec<Vec<u64>> = Vec::new();
}


/*
pub fn query_to_rust(q: sem::Relation, ctxt: &sem::Context) -> String {
    let mut query = String::new();
    let mut index = 0;
    for trans in &q.transfromations {
        match trans {
            sem::Transformation::Filter{ var, filter } => {
                let var_type = &ctxt.get_type(&var.name).unwrap().rust_name;
                query += &("fn filter_".to_owned() + &index.to_string() + 
                           "(var: Relation<(" + var_type + ")>) -> Relation<(" + var_type + ")> {\n");
                query += "}";
            }
        }
        index += 1;
    }
    query
}
*/

/*
pub fn generate_rust(c: &sem::Context, trans: &sem::Transformation) -> String {
    use sem::Transformation::*;
    match trans {
        Filter { scan, filter } => {
            //let scan_rel = c.get_relation(*scan);
            /*
            let mut header = "fn filter".to_owned() + &scan.0.to_string() + "(";
            for (ref ty, id) in scan_rel.types.iter().zip(0..) {
                header += "arg";
                header += &id.to_string();
                header += ": ";
                header += &c.get_type(ty).unwrap().rust_name;
                header += ", ";
            }
            header += ")";

            let mut body = "";
            

            header*/
            "".to_owned()
        },
        _ => {"".to_owned()}
    }
}
*/


