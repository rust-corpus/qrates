use super::utils::is_copy_type;
use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_load_save_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let load_multifile_relations = load_multifile_relations_function(schema);
    let load_counters = load_counters_function();
    let load_interning_tables = load_multifle_interning_function(schema);
    let store_multifile_relations = store_multifile_relations_function(schema);
    let store_counters = store_counters_function();
    let store_interning_tables = store_multifle_interning_function(schema);
    quote! {
        impl Tables {
            pub fn load_multifile(
                database_root: &Path
            ) -> Result<Tables, Error> {
                let relations = load_multifile_relations(&database_root.join("relations"))?;
                let counters = load_counters(&database_root.join("counters.bincode"))?;
                let interning_tables = load_interning_tables(&database_root.join("interning"))?;
                Ok(Tables {
                    relations,
                    counters,
                    interning_tables,
                })
            }
            pub fn load_single_file(
                tables_file: &Path
            ) -> Result<Tables, Error> {
                crate::storage::load(tables_file)
            }
            pub fn store_multifile(&self, database_root: &Path) -> Result<(), Error> {
                let relations_path = database_root.join("relations");
                std::fs::create_dir_all(&relations_path)?;
                store_multifile_relations(&self.relations, &relations_path);
                let counters_path = database_root.join("counters.bincode");
                store_counters(&self.counters, &counters_path);
                let interning_tables_path = &database_root.join("interning");
                std::fs::create_dir_all(&interning_tables_path)?;
                store_multifile_interning_tables(
                    &self.interning_tables,
                    &interning_tables_path
                );
                Ok(())
            }
        }
        #load_multifile_relations
        #load_counters
        #load_interning_tables
        #store_multifile_relations
        #store_counters
        #store_interning_tables
    }
}

fn load_multifile_relations_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut load_fields = TokenStream::new();
    for relation in &schema.relations {
        let relation_hash = relation.get_hash();
        let name = &relation.name;
        let file_name = format!("{}", name);
        load_fields.extend(quote! {
            #name: unsafe { Relation::load(#relation_hash, path.join(#file_name)) }?,
        });
    }
    quote! {
        fn load_multifile_relations(path: &Path) -> Result<Relations, Error> {
            Ok(Relations {
                #load_fields
            })
        }
    }
}

fn store_multifile_relations_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut store_fields = TokenStream::new();
    for relation in &schema.relations {
        let name = &relation.name;
        let relation_hash = relation.get_hash();
        let file_name = name.to_string();
        store_fields.extend(quote! {
            unsafe { relations.#name.save(#relation_hash, path.join(#file_name)) }
        });
    }
    quote! {
        fn store_multifile_relations(
            relations: &Relations,
            path: &Path
        ) {
            #store_fields
        }
    }
}

fn load_counters_function() -> TokenStream {
    quote! {
        fn load_counters(path: &Path) -> Result<Counters, Error> {
            crate::storage::load(&path)
        }
    }
}

fn store_counters_function() -> TokenStream {
    quote! {
        fn store_counters(counters: &Counters, path: &Path) {
            crate::storage::save(counters, &path);
        }
    }
}

fn load_multifle_interning_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut load_fields = TokenStream::new();
    for table in &schema.interning_tables {
        let ast::InterningTable { name, value, .. } = table;
        if is_copy_type(value, schema) {
            let table_hash = table.get_hash();
            let file_name = name.to_string();
            load_fields.extend(quote! {
                #name: unsafe { InterningTable::load(#table_hash, path.join(#file_name))? },
            });
        } else {
            let file_name = format!("{}.bincode", name);
            load_fields.extend(quote! {
                #name: crate::storage::load(&path.join(#file_name))?,
            });
        }
    }
    quote! {
        fn load_interning_tables(path: &Path) -> Result<InterningTables, Error> {
            Ok(InterningTables {
                #load_fields
            })
        }
    }
}

fn store_multifle_interning_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut store_fields = TokenStream::new();
    for table in &schema.interning_tables {
        let ast::InterningTable { name, value, .. } = table;
        if is_copy_type(value, schema) {
            let table_hash = table.get_hash();
            let file_name = name.to_string();
            store_fields.extend(quote! {
                unsafe { interning_tables.#name.save(#table_hash, path.join(#file_name)); }
            });
        } else {
            let file_name = format!("{}.bincode", name);
            store_fields.extend(quote! {
                crate::storage::save(&interning_tables.#name, &path.join(#file_name));
            });
        }
    }
    quote! {
        fn store_multifile_interning_tables(
            interning_tables: &InterningTables,
            path: &Path
        ) {
            #store_fields
        }
    }
}
