use proc_macro2::TokenStream;
use quote::quote;

use crate::ast;

// Turns extraction-time data structures (backed by Vec/HashMap) into query-time structures (backed by DiskMap).
// Only used for testing.
pub fn generate_mem_to_disk_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let relations_to_disk = relations_to_disk_function(schema);
    let interning_tables_to_disk = interning_tables_to_disk_function(schema);
    quote! {
        #relations_to_disk
        #interning_tables_to_disk
    }
}

fn relations_to_disk_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let store_fields = schema.relations.iter().map(|relation| {
        let name = &relation.name;
        quote! {
            #name: relations.#name.into(),
        }
    });

    quote! {
        fn relations_to_disk(
            relations: Relations,
        ) -> DiskRelations {
            DiskRelations {
                #(#store_fields)*
            }
        }
    }
}

fn interning_tables_to_disk_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let store_fields = schema.interning_tables.iter().map(|interning_table| {
        let name = &interning_table.name;
        quote! {
            #name: interning_tables.#name.into(),
        }
    });

    quote! {
        fn interning_tables_to_disk(
            interning_tables: InterningTables,
        ) -> DiskInterningTables {
            DiskInterningTables {
                #(#store_fields)*
            }
        }
    }
}
