use std::str::FromStr;

use super::utils::is_copy_type;
use crate::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(super) fn generate_loader_functions(
    schema: &ast::DatabaseSchema,
) -> (TokenStream, TokenStream) {
    let mut function_tokens = TokenStream::new();
    let mut cache_field_tokens = TokenStream::new();
    for relation in schema.relations.iter().chain(&schema.derived_relations) {
        let ast::Relation {
            ref name,
            ref parameters,
            ref intern_key,
            ..
        } = relation;

        // names to provide basic doc comments
        let names = parameters.iter().map(|param| &param.name);
        let names = names.map(|name| name.to_string()).collect::<Vec<_>>().join(", ");
        let iter_doc_comment = format!("Stream over the relation with idents `({})`.", names);
        let load_doc_comment = format!("Load the relation with idents `({})`.", names);
        let store_doc_comment = format!("Store the relation with idents `({})`.", names);

        let relation_hash = relation.get_hash();
        let file_name = format!("relations/{}", name);
        let load_fn_name = syn::Ident::new(&format!("load_{}", name), Span::call_site());
        let load_iter_fn_name = syn::Ident::new(&format!("load_iter_{}", name), Span::call_site());
        let store_fn_name = syn::Ident::new(&format!("store_{}", name), Span::call_site());
        let store_iter_fn_name = syn::Ident::new(&format!("store_iter_{}", name), Span::call_site());
        let mut types = TokenStream::new();
        for ast::RelationParameter { typ, .. } in parameters {
            types.extend(quote! {#typ,});
        }
        let relation_element_type = quote! {RelationElement<(#types)>};
        let tuple_element_type = quote! {(#types)};
        cache_field_tokens.extend(quote! {
            #name: std::cell::RefCell<Option<Relation<#relation_element_type>>>,
        });
        function_tokens.extend(quote! {
            #[doc = #iter_doc_comment]
            pub fn #load_iter_fn_name(&self) -> impl Iterator<Item = #tuple_element_type> {
                self.#load_fn_name().iter()
            }
            #[doc = #load_doc_comment]
            pub fn #load_fn_name(&self) -> std::cell::Ref<Relation<#relation_element_type>> {
                if self.#name.borrow().is_none() {
                    // let relation: Relation<(#types)> = unsafe { Relation::load(
                    //     #relation_hash,
                    //     self.database_root.join(#file_name)
                    // ) }.unwrap();

                    let relation = Relation::<#relation_element_type>::load(
                            #relation_hash,
                            self.database_root.join(#file_name)
                        ).unwrap();

                    *self.#name.borrow_mut() = Some(relation);
                }
                std::cell::Ref::map(self.#name.borrow(), |option| option.as_ref().unwrap())
            }
            // TODO: when we're storing a relation that has a RelationMap, we should also store the corresponding relation map.
            // ^^ well, if we can completely deprecate using _both_ relationmaps and relations for any given relation, then this is not an issue.
            // ^^  also, we panic if we ever try to load a relationmap that does not exist. so we would catch this.
            // pub fn #store_fn_name(&self, facts: impl IntoIterator<Item = (#types)>) {
            #[doc = #store_doc_comment]
            pub fn #store_fn_name(&self, facts: Vec<#tuple_element_type>) {
                //assert!(self.#name.borrow().is_none());
                //let relation: Relation<(#types)> = facts.into();
                //unsafe { relation.save(#relation_hash, self.database_root.join(#file_name)); }
                //*self.#name.borrow_mut() = Some(relation.into());

                // unsafe { 
                //     save_elts_relation::<(#types)>(
                //         facts,
                //         #relation_hash,
                //         self.database_root.join(#file_name)
                //     );
                // }
                self.#store_iter_fn_name(facts.into_iter());
            }
            #[doc = #store_doc_comment]
            pub fn #store_iter_fn_name(&self, facts: impl IntoIterator<Item = #tuple_element_type>) {
                // TODO: store a root path on self.
                // then from_iter a relation, and store it in the cached field.
                // the from_iter already does saving (since it's a db), so we don't need to do any explicit saving.

                let relation = Relation::from_tuple_iter_override(self.database_root.join(#file_name), facts);
                *self.#name.borrow_mut() = Some(relation);

                //assert!(self.#name.borrow().is_none());
                //let relation: Relation<(#types)> = facts.into();
                //unsafe { relation.save(#relation_hash, self.database_root.join(#file_name)); }
                //*self.#name.borrow_mut() = Some(relation.into());

                // unsafe { 
                //     save_elts_relation::<#tuple_element_type>(
                //         facts,
                //         #relation_hash,
                //         self.database_root.join(#file_name)
                //     );
                // }
            }
        });

        if let [key, value] = parameters.as_slice() {
            let load_fn_name_as_map =
                syn::Ident::new(&format!("load_{}_as_map", name), Span::call_site());
            let key = &key.typ;
            let value = &value.typ;
            function_tokens.extend(quote! {
                pub fn #load_fn_name_as_map(&self) -> std::collections::HashMap<#key, #value> {
                    self.#load_iter_fn_name().collect()
                }
            });
        }

        // If we have an intern key specified, also generate an "RelationMap"
        if let Some(intern_key) = intern_key {
            let ast::RelationInternKey { source, source_idx } = intern_key;
            let key = &parameters[*source_idx].typ;
            let value = intern_key.get_value_type(&parameters);

            let key_ident_name = parameters[*source_idx].name.to_string();
            let value_ident_names: Vec<String> = parameters
                .iter()
                .enumerate()
                .filter_map(|(idx, param)| {
                    if idx != *source_idx {
                        Some(param.name.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            let value_ident_names = value_ident_names.join(", ");
            let load_relation_map_doc_comment = format!(
                "Load the map `{} => ({})` as a map.",
                key_ident_name,
                value_ident_names
            );


            let intern_table_name = syn::Ident::new(&format!("{}_redb_map", name), Span::call_site());
            let intern_table_hash = relation_hash;
            let intern_table_file_name = format!("relations/{}_relation_map", name);
            let load_intern_table_fn_name =
                syn::Ident::new(&format!("load_{}", intern_table_name), Span::call_site());
            cache_field_tokens.extend(quote! {
                #intern_table_name: std::cell::RefCell<Option<RelationMap<#key, #value>>>,
            });
            function_tokens.extend(quote! {
                #[doc = #load_relation_map_doc_comment]
                pub fn #load_intern_table_fn_name(&self) -> std::cell::Ref<RelationMap<#key, #value>> {
                    if self.#intern_table_name.borrow().is_none() {
                        *self.#intern_table_name.borrow_mut() = Some(
                            RelationMap::load(
                                #intern_table_hash,
                                self.database_root.join(#intern_table_file_name)
                            ).unwrap());
                    }
                    std::cell::Ref::map(self.#intern_table_name.borrow(), |option| option.as_ref().unwrap())
                }
            });

            let source_idx_str = TokenStream::from_str(&format!("{}", source_idx)).unwrap();

            let non_source_idxs: Vec<TokenStream> = (0..parameters.len())
                .filter(|idx| *idx != *source_idx)
                .map(|idx| {
                    TokenStream::from_str(&format!("{idx}")).unwrap()
                })
                .collect();

            // also create a storer function for the intern table
            let store_intern_table_fn_name =
                syn::Ident::new(&format!("store_{}", intern_table_name), Span::call_site());
            function_tokens.extend(quote! {
                pub fn #store_intern_table_fn_name(&self, facts: impl IntoIterator<Item = (#types)>) {
                    // create a relation map
                    let path = self.database_root.join(#intern_table_file_name);
                    let iter = facts.into_iter().map(|fact| {
                        (fact.#source_idx_str, (#(fact.#non_source_idxs),*))
                    });
                    let mut map: RelationMap<#key, #value> = RelationMap::from_iter_override(&path, iter);
                    // let facts_mapped: HashMap<#key, #value> = facts.into_iter().map(|fact| {
                    //     (fact.#source_idx_str, (#(fact.#non_source_idxs),*))
                    // }).collect();
                    // let intern_table: RelationMap<#key, #value> = facts_mapped.into();
                    // intern_table.save(#intern_table_hash, self.database_root.join(#intern_table_file_name));
                    map.save(#intern_table_hash, path);

                }
            });
        }
    }
    for table in &schema.interning_tables {
        let ast::InterningTable { name, key, value } = table;
        let fn_name = syn::Ident::new(&format!("load_{}", name), Span::call_site());
        let fn_name_as_vec = syn::Ident::new(&format!("load_{}_as_vec", name), Span::call_site());
        let key_type = &key.name;
        let mut types = TokenStream::new();
        types.extend(quote! {#key_type,});
        match value {
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                for elem in elems {
                    types.extend(quote! {#elem,});
                }
            }
            _ => {
                types.extend(quote! {#value,});
            }
        }
        let load = {
            let table_hash = table.get_hash();
            let file_name = format!("interning/{}", name);
            quote! {
                {
                    DiskInterningTable::load(
                        self.database_root.join(#file_name)
                    ).unwrap()
                }
            }
        };
        // let load = if is_copy_type(value, schema) {
        //     let table_hash = table.get_hash();
        //     let file_name = format!("interning/{}", name);
        //     quote! {
        //         unsafe {
        //             InterningTable::load(
        //                 #table_hash,
        //                 self.database_root.join(#file_name)
        //             ).unwrap()
        //         }
        //     }
        // } else {
        //     let file_name = format!("interning/{}.bincode", name);
        //     quote! {
        //         crate::storage::load(
        //             &self.database_root.join(#file_name)
        //         ).unwrap()
        //     }
        // };
        cache_field_tokens.extend(quote! {
            #name: std::cell::RefCell<Option<DiskInterningTable<#key_type, #value>>>,
        });
        function_tokens.extend(quote! {
            pub fn #fn_name(&self) -> std::cell::Ref<DiskInterningTable<#key_type, #value>> {
                if self.#name.borrow().is_none() {
                    *self.#name.borrow_mut() = Some(#load);
                }
                std::cell::Ref::map(self.#name.borrow(), |option| option.as_ref().unwrap())
            }
            pub fn #fn_name_as_vec(&self) -> Vec<(#types)> {
                let table = self.#fn_name();
                table.deref().into()
            }
        });
    }
    (function_tokens, cache_field_tokens)
}
