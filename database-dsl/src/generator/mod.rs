use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

mod counters;
mod debug;
mod interning_tables;
mod loader;
mod merge;
mod registration;
mod relations;
mod storage;
mod types;
mod utils;

pub(crate) fn generate_tokens(schema: ast::DatabaseSchema) -> TokenStream {
    let types = types::generate_types(&schema);
    let tables = interning_tables::generate_interning_tables(&schema);
    let relations = relations::generate_relations(&schema);
    let (counters, counter_functions) = counters::generate_counters(&schema);
    let registration_functions = registration::generate_registration_functions(&schema);
    let load_save_functions = storage::generate_load_save_functions(&schema);
    let (loader_functions, loader_cache_fields) = loader::generate_loader_functions(&schema);
    let merge_functions = merge::generate_merge_functions(&schema);
    let debug_functions = debug::generate_status_functions(&schema);
    quote! {
        pub mod types {
            use serde_derive::{Deserialize, Serialize};
            #types
        }
        pub mod tables {
            use std::path::{Path, PathBuf};
            use std::collections::HashMap;
            use anyhow::Result;
            use serde_derive::{Deserialize, Serialize};
            use super::types::*;
            #tables
            #relations
            #counters

            #[derive(Default, Deserialize, Serialize)]
            pub struct Tables {
                /// Relations between Rust program elements.
                pub(crate) relations: Relations,
                /// Counters used for generating ids.
                pub(crate) counters: Counters,
                /// Interning tables that link typed ids to untyped interning ids.
                pub(crate) interning_tables: InterningTables,
            }

            impl Tables {
                #registration_functions
            }

            impl Tables {
                #counter_functions
            }

            impl Tables {
                #debug_functions
            }

            #merge_functions

            #load_save_functions

            #[derive(Default)]
            pub struct Loader {
                pub(crate) database_root: PathBuf,
                #loader_cache_fields
            }

            impl Loader {
                pub fn new(database_root: PathBuf) -> Self {
                    Self { database_root, ..Loader::default() }
                }
                #loader_functions
            }
        }
    }
}
