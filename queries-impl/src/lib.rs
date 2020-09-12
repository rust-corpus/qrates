use corpus_database_dsl::{generate_query, parse_schema};
use proc_macro::TokenStream;
use proc_macro_hack::proc_macro_hack;
use std::env;
use std::path::Path;

#[proc_macro_hack]
pub fn datapond_query(input: TokenStream) -> TokenStream {
    let current_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let core_schema_path = Path::new(&current_dir).join("../database/src/schema.dl");
    assert!(core_schema_path.exists());
    let derived_relations_path = Path::new(&current_dir).join("../database/src/derived.dl");
    assert!(derived_relations_path.exists());
    let definition = parse_schema(&core_schema_path, &derived_relations_path);
    generate_query(definition, input.into()).into()
}
