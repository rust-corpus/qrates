use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

/// Generate a function that prints statistics about the data stored in the database.
pub(super) fn generate_status_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut print_counters = TokenStream::new();
    for id in &schema.incremental_ids {
        let field_name = id.get_field_name();
        let name_str = format!("{}", field_name);
        print_counters.extend(quote! {
            println!("counter {} value: {}", #name_str, self.counters.#field_name);
        });
    }
    let mut print_interning_tables = TokenStream::new();
    for table in &schema.interning_tables {
        let name = &table.name;
        let name_str = format!("{}", name);
        print_interning_tables.extend(quote! {
            println!("interning table {} count: {}", #name_str, self.interning_tables.#name.len());
        });
    }
    let mut print_relations = TokenStream::new();
    for relation in &schema.relations {
        let name = &relation.name;
        let name_str = format!("{}", name);
        print_relations.extend(quote! {
            println!("relation {} count: {}", #name_str, self.relations.#name.len());
        });
    }
    quote! {
        pub fn print_statistics(&self) {
            #print_counters
            #print_interning_tables
            #print_relations
        }
    }
}
