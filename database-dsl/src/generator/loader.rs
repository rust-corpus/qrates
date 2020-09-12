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
            ..
        } = relation;
        let relation_hash = relation.get_hash();
        let file_name = format!("relations/{}", name);
        let load_fn_name = syn::Ident::new(&format!("load_{}", name), Span::call_site());
        let store_fn_name = syn::Ident::new(&format!("store_{}", name), Span::call_site());
        let mut types = TokenStream::new();
        for ast::RelationParameter { typ, .. } in parameters {
            types.extend(quote! {#typ,});
        }
        cache_field_tokens.extend(quote! {
            #name: std::cell::RefCell<Option<Vec<(#types)>>>,
        });
        function_tokens.extend(quote! {
            pub fn #load_fn_name(&self) -> std::cell::Ref<Vec<(#types)>> {
                if self.#name.borrow().is_none() {
                    let relation: Relation<(#types)> = unsafe { Relation::load(
                        #relation_hash,
                        self.database_root.join(#file_name)
                    ) }.unwrap();
                    *self.#name.borrow_mut() = Some(relation.into());
                }
                std::cell::Ref::map(self.#name.borrow(), |option| option.as_ref().unwrap())
            }
            pub fn #store_fn_name(&self, facts: Vec<(#types)>) {
                assert!(self.#name.borrow().is_none());
                let relation: Relation<(#types)> = facts.into();
                unsafe { relation.save(#relation_hash, self.database_root.join(#file_name)); }
                *self.#name.borrow_mut() = Some(relation.into());
            }
        });
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
        let load = if is_copy_type(value, schema) {
            let table_hash = table.get_hash();
            let file_name = format!("interning/{}", name);
            quote! {
                unsafe {
                    InterningTable::load(
                        #table_hash,
                        self.database_root.join(#file_name)
                    ).unwrap()
                }
            }
        } else {
            let file_name = format!("interning/{}.bincode", name);
            quote! {
                crate::storage::load(
                    &self.database_root.join(#file_name)
                ).unwrap()
            }
        };
        cache_field_tokens.extend(quote! {
            #name: std::cell::RefCell<Option<InterningTable<#key_type, #value>>>,
        });
        function_tokens.extend(quote! {
            pub fn #fn_name(&self) -> std::cell::Ref<InterningTable<#key_type, #value>> {
                if self.#name.borrow().is_none() {
                    *self.#name.borrow_mut() = Some(#load);
                }
                std::cell::Ref::map(self.#name.borrow(), |option| option.as_ref().unwrap())
            }
            pub fn #fn_name_as_vec(&self) -> Vec<(#types)> {
                let table: InterningTable<#key_type, #value> = #load;
                table.into()
            }
        });
    }
    (function_tokens, cache_field_tokens)
}
