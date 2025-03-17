use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

mod conversions;
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
    let mem_to_disk_functions = conversions::generate_mem_to_disk_functions(&schema);
    quote! {
        pub mod int_bytes_adapter {
            /// Trait for converting between bytes and integers.
            /// Glue code for redb byte conversions.
            pub trait QratesBytesAdapter {
                fn qrates_from_bytes(bytes: &[u8]) -> Self;
                fn qrates_as_bytes(&self) -> Vec<u8>;
            }

            impl QratesBytesAdapter for u8 {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    bytes[0]
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    vec![*self]
                }
            }

            impl QratesBytesAdapter for u16 {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    u16::from_le_bytes([bytes[0], bytes[1]])
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().into()
                }
            }

            impl QratesBytesAdapter for u32 {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().into()
                }
            }

            impl QratesBytesAdapter for u64 {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    ])
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().into()
                }
            }

            impl QratesBytesAdapter for usize {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    u64::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                    ]) as usize
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    (*self as u64).to_le_bytes().into()
                }
            }

            impl QratesBytesAdapter for u128 {
                fn qrates_from_bytes(bytes: &[u8]) -> Self {
                    u128::from_le_bytes([
                        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                        bytes[8], bytes[9], bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
                    ])
                }

                fn qrates_as_bytes(&self) -> Vec<u8> {
                    self.to_le_bytes().into()
                }
            }


        }

        pub mod types {
            use serde_derive::{Deserialize, Serialize};
            use super::int_bytes_adapter::QratesBytesAdapter;
            #types
        }
        pub mod tables {
            use std::path::{Path, PathBuf};
            use std::collections::HashMap;
            use std::ops::Deref;
            use anyhow::Result;
            use serde_derive::{Deserialize, Serialize};
            use super::types::*;
            use crate::data_structures::RelationMap;
            use crate::data_structures::DiskMap;
            use crate::data_structures::RelationElement;
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

            pub struct DiskTables {
                /// Relations between Rust program elements.
                pub(crate) relations: DiskRelations,
                /// Counters used for generating ids.
                pub(crate) counters: Counters,
                /// Interning tables that link typed ids to untyped interning ids.
                pub(crate) interning_tables: DiskInterningTables,
            }

            impl DiskTables {
                pub fn create_in(root: &Path) -> Result<Self> {
                    Ok(Self {
                        relations: DiskRelations::create_in(&root.join("relations"))?,
                        counters: Counters::default(),
                        interning_tables: DiskInterningTables::create_in(&root.join("interning"))?,
                    })
                }

                pub fn from_tables(tables: Tables) -> Self {
                    Self {
                        relations: relations_to_disk(tables.relations),
                        counters: tables.counters,
                        interning_tables: interning_tables_to_disk(tables.interning_tables),
                    }
                }
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

            #mem_to_disk_functions

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
