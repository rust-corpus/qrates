use std::str::FromStr;

use super::utils::is_copy_type;
use crate::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(super) fn generate_load_save_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let load_multifile_relations = load_multifile_relations_function(schema);
    let load_counters = load_counters_function();
    let load_interning_tables = load_multifle_interning_function(schema);
    let store_multifile_relations = store_multifile_relations_function(schema);
    let store_counters = store_counters_function();
    let store_interning_tables = store_multifle_interning_function(schema);
    quote! {
        impl DiskTables {
            pub fn load_multifile(
                database_root: &Path
            ) -> Result<DiskTables> {
                let relations = load_multifile_relations(&database_root.join("relations"))?;
                let counters = load_counters(&database_root.join("counters.bincode"))?;
                let interning_tables = load_interning_tables(&database_root.join("interning"))?;
                Ok(DiskTables {
                    relations,
                    counters,
                    interning_tables,
                })
            }
            pub fn store_multifile(&mut self, database_root: &Path) -> Result<()> {
                let relations_path = database_root.join("relations");
                std::fs::create_dir_all(&relations_path)?;
                store_multifile_relations(&mut self.relations, &relations_path);
                let counters_path = database_root.join("counters.bincode");
                store_counters(&self.counters, &counters_path);
                let interning_tables_path = &database_root.join("interning");
                std::fs::create_dir_all(&interning_tables_path)?;
                store_multifile_interning_tables(
                    &mut self.interning_tables,
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
            #name: Relation::load(#relation_hash, path.join(#file_name))?,
        });
    }
    quote! {
        pub fn load_multifile_relations(path: &Path) -> Result<DiskRelations> {
            Ok(DiskRelations {
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
            { relations.#name.save(#relation_hash, path.join(#file_name)) }
        });
        if let Some(intern_key @ ast::RelationMapKey { source, source_idx }) =
            &relation.relation_map_key
        {
            // save by into_iter the relations vec

            let key = &relation.parameters[*source_idx].typ;
            let value = intern_key.get_value_type(&relation.parameters);

            let intern_table_hash = relation_hash;
            let intern_table_file_name = format!("{}_relation_map", name);

            let source_idx_str = TokenStream::from_str(&format!("{}", source_idx)).unwrap();

            let non_source_idxs: Vec<TokenStream> = (0..relation.parameters.len())
                .filter(|idx| *idx != *source_idx)
                .map(|idx| TokenStream::from_str(&format!("{idx}")).unwrap())
                .collect();

            store_fields.extend(quote! {
                {
                    let path = path.join(#intern_table_file_name);
                    let iter = relations.#name.iter().map(|fact| {
                        (fact.#source_idx_str, (#(fact.#non_source_idxs),*))
                    });
                    let mut relation_map: RelationMap<#key, #value> = RelationMap::from_iter_override(&path, iter);
                    relation_map.save(#intern_table_hash, path);
                }
            });
        }
    }
    quote! {
        pub fn store_multifile_relations(
            relations: &mut DiskRelations,
            path: &Path
        ) {
            #store_fields
        }
    }
}

fn load_counters_function() -> TokenStream {
    quote! {
        fn load_counters(path: &Path) -> Result<Counters> {
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
        let table_hash = table.get_hash();
        let file_name = name.to_string();
        load_fields.extend(quote! {
            #name: { DiskInterningTable::load(#table_hash, path.join(#file_name))? },
        });
    }
    quote! {
        fn load_interning_tables(path: &Path) -> Result<DiskInterningTables> {
            Ok(DiskInterningTables {
                #load_fields
            })
        }
    }
}

fn store_multifle_interning_function(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut store_fields = TokenStream::new();
    for table in &schema.interning_tables {
        let ast::InterningTable { name, value, .. } = table;
        let table_hash = table.get_hash();
        let file_name = name.to_string();
        store_fields.extend(quote! {
            { interning_tables.#name.save(#table_hash, path.join(#file_name)); }
        });
    }
    quote! {
        fn store_multifile_interning_tables(
            interning_tables: &mut DiskInterningTables,
            path: &Path
        ) {
            #store_fields
        }
    }
}
