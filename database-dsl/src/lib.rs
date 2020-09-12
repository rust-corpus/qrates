use proc_macro2::TokenStream;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

mod ast;
mod generator;
mod parser;
mod queries;

pub fn parse_schema(core_schema_path: &Path, derived_relations_path: &Path) -> ast::DatabaseSchema {
    let mut core_schema_file = File::open(core_schema_path).unwrap();
    let mut core_schema_content = String::new();
    core_schema_file
        .read_to_string(&mut core_schema_content)
        .unwrap();
    let mut schema: ast::DatabaseSchema = match syn::parse_str(&core_schema_content) {
        Ok(config) => config,
        Err(err) => panic!("Error: {:?} (at {:?})", err, err.span().start()),
    };
    let mut derived_relations_file = File::open(derived_relations_path).unwrap();
    let mut derived_relations_content = String::new();
    derived_relations_file
        .read_to_string(&mut derived_relations_content)
        .unwrap();
    let derived_relations: ast::Relations = match syn::parse_str(&derived_relations_content) {
        Ok(relations) => relations,
        Err(err) => panic!("Error: {:?} (at {:?}", err, err.span().start()),
    };
    schema.derived_relations = derived_relations.into();
    schema
}

pub fn generate_definition(dest_path: &Path, schema: ast::DatabaseSchema) {
    let tokens = generator::generate_tokens(schema);
    let mut file = File::create(dest_path).unwrap();
    file.write(tokens.to_string().as_bytes()).unwrap();
}

pub fn generate_query(schema: ast::DatabaseSchema, input: TokenStream) -> TokenStream {
    queries::generate(schema, input)
}
