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
            ref relation_map_key,
            ..
        } = relation;

        // names to provide basic doc comments
        let names = parameters.iter().map(|param| &param.name);
        let names = names
            .map(|name| name.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        let iter_doc_comment = format!("Stream over the relation with idents `({})`.", names);
        let load_doc_comment = format!("Load the relation with idents `({})`.", names);
        let store_doc_comment = format!("Store the relation with idents `({})`. Note that any RelationMaps derived from this relation will not be updated during this run.", names);

        let relation_hash = relation.get_hash();
        let file_name = format!("relations/{}", name);
        let load_fn_name = syn::Ident::new(&format!("load_{}", name), Span::call_site());
        let load_iter_fn_name = syn::Ident::new(&format!("load_iter_{}", name), Span::call_site());
        let store_fn_name = syn::Ident::new(&format!("store_{}", name), Span::call_site());
        let store_iter_fn_name =
            syn::Ident::new(&format!("store_iter_{}", name), Span::call_site());
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
                    let relation = Relation::<#relation_element_type>::load(
                            #relation_hash,
                            self.database_root.join(#file_name)
                        ).unwrap();

                    *self.#name.borrow_mut() = Some(relation);
                }
                std::cell::Ref::map(self.#name.borrow(), |option| option.as_ref().unwrap())
            }
            #[doc = #store_doc_comment]
            pub fn #store_fn_name(&self, facts: Vec<#tuple_element_type>) {
                self.#store_iter_fn_name(facts.into_iter());
            }
            #[doc = #store_doc_comment]
            pub fn #store_iter_fn_name(&self, facts: impl IntoIterator<Item = #tuple_element_type>) {
                // Note: Below restriction could potentially be lifted with enough tests. But we have no use for it right now.
                assert!(self.#name.borrow().is_none(), "Cannot store a relation that has already been loaded.");
                let mut relation = Relation::from_tuple_iter_override(self.database_root.join(#file_name), facts);
                relation.save(#relation_hash, self.database_root.join(#file_name));
                *self.#name.borrow_mut() = Some(relation);
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

        // If we have an key specified, also generate an "RelationMap"
        if let Some(relation_map_key) = relation_map_key {
            let ast::RelationMapKey { source_idx, .. } = relation_map_key;
            let key = &parameters[*source_idx].typ;
            let value = relation_map_key.get_value_type(&parameters);

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
                key_ident_name, value_ident_names
            );

            let relation_map_name =
                syn::Ident::new(&format!("{}_relation_map", name), Span::call_site());
            let relation_map_hash = relation_hash;
            let relation_map_file_name = format!("relations/{}_relation_map", name);
            let load_relation_map_fn_name =
                syn::Ident::new(&format!("load_{}", relation_map_name), Span::call_site());
            cache_field_tokens.extend(quote! {
                #relation_map_name: std::cell::RefCell<Option<RelationMap<#key, #value>>>,
            });
            function_tokens.extend(quote! {
                #[doc = #load_relation_map_doc_comment]
                pub fn #load_relation_map_fn_name(&self) -> std::cell::Ref<RelationMap<#key, #value>> {
                    if self.#relation_map_name.borrow().is_none() {
                        *self.#relation_map_name.borrow_mut() = Some(
                            RelationMap::load(
                                #relation_map_hash,
                                self.database_root.join(#relation_map_file_name)
                            ).unwrap());
                    }
                    std::cell::Ref::map(self.#relation_map_name.borrow(), |option| option.as_ref().unwrap())
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
                        #table_hash,
                        self.database_root.join(#file_name)
                    ).unwrap()
                }
            }
        };
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
