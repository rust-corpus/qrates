use super::utils::NameGenerator;
use crate::ast;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

pub(super) fn generate_registration_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut functions = TokenStream::new();
    for table in &schema.interning_tables {
        if let syn::Type::Tuple(ref tuple) = table.value {
            let function = generate_intern_tuple_registration(tuple, table, schema);
            functions.extend(function);
        } else {
            let function = generate_intern_value_registration(table, schema);
            functions.extend(function);
        };
    }
    for relation in &schema.relations {
        let function = generate_relation_registration(relation, schema);
        functions.extend(function);
    }
    functions
}

fn generate_intern_tuple_registration(
    values: &syn::TypeTuple,
    table: &ast::InterningTable,
    schema: &ast::DatabaseSchema,
) -> TokenStream {
    let registration_function_name = table.get_registration_function_name();
    let key_type = table.get_key_type();
    let mut interning_tokens = TokenStream::new();
    let mut param_tokens = TokenStream::new();
    let mut arg_tokens = TokenStream::new();

    for value_type in values.elems.pairs().map(|pair| pair.into_value()) {
        let template = format!("value_{}", value_type.to_token_stream()).to_lowercase();
        let mut name_generator = NameGenerator::new(template);
        let (final_name, param_type, tokens) =
            generate_interning_type(value_type, schema, &mut name_generator);
        let param_name = name_generator.get_ident();
        param_tokens.extend(quote! {
            #param_name: #param_type,
        });
        interning_tokens.extend(tokens);
        arg_tokens.extend(quote! {
            #final_name,
        });
    }
    let table_name = &table.name;
    quote! {
        pub fn #registration_function_name(&mut self, #param_tokens) -> #key_type {
            #interning_tokens
            self.interning_tables.#table_name.intern((#arg_tokens))
        }
    }
}

fn generate_intern_value_registration(
    table: &ast::InterningTable,
    schema: &ast::DatabaseSchema,
) -> TokenStream {
    let registration_function_name = table.get_registration_function_name();
    let key_type = table.get_key_type();

    let mut name_generator = NameGenerator::new(String::from("value"));
    let (final_name, param_type, tokens) =
        generate_interning_type(&key_type, schema, &mut name_generator);
    let param_name = name_generator.get_ident();
    quote! {
        pub fn #registration_function_name(&mut self, #param_name: #param_type) -> #key_type {
            #tokens
            #final_name
        }
    }
}

fn generate_relation_registration(
    relation: &ast::Relation,
    schema: &ast::DatabaseSchema,
) -> TokenStream {
    let registration_function_name = relation.get_registration_function_name();
    let mut param_tokens = TokenStream::new();
    let mut return_tokens = TokenStream::new();
    let mut return_type_tokens = TokenStream::new();
    let mut interning_tokens = TokenStream::new();
    let mut arg_tokens = TokenStream::new();
    for ast::RelationParameter {
        name,
        typ,
        is_autogenerated,
    } in &relation.parameters
    {
        if *is_autogenerated {
            let id = schema
                .find_incremental_id(typ)
                .expect("Only incremental IDs can be marked with `auto`.");
            let generator_fn_name = id.get_generator_fn_name();
            interning_tokens.extend(quote! {
                let #name = self.counters.#generator_fn_name();
            });
            return_type_tokens.extend(quote! {#typ,});
            return_tokens.extend(quote! {#name,});
            arg_tokens.extend(quote! {#name,});
        } else {
            let mut name_generator = NameGenerator::new(name.to_string());
            let (final_name, param_type, tokens) =
                generate_interning_type(typ, schema, &mut name_generator);
            let param_name = name_generator.get_ident();
            param_tokens.extend(quote! {
                #param_name: #param_type,
            });
            interning_tokens.extend(tokens);
            arg_tokens.extend(quote! {#final_name,});
        }
    }
    let table_name = &relation.name;
    quote! {
        pub fn #registration_function_name(&mut self, #param_tokens) -> (#return_type_tokens) {
            #interning_tokens
            self.relations.#table_name.insert((#arg_tokens));
            (#return_tokens)
        }
    }
}

/// Find a type that is not an interning key and an interning path
/// from it to the target type.
fn generate_interning_type(
    target_type: &syn::Type,
    schema: &ast::DatabaseSchema,
    name_generator: &mut NameGenerator,
) -> (syn::Ident, syn::Type, TokenStream) {
    let var_name = name_generator.get_ident();
    let mut found_type = None;
    let mut tokens = TokenStream::new();
    for table in &schema.interning_tables {
        if &table.get_key_type() == target_type {
            assert!(found_type.is_none(), "Ambigous interning tables");
            if let syn::Type::Tuple(_) = table.value {
                continue;
            }
            name_generator.inc();
            // let new_target_type = syn::Type::Path(syn::TypePath { qself: None, path: table.key.name.clone().into() });
            let (new_name, new_found_type, prefix) =
                generate_interning_type(&table.value, schema, name_generator);
            found_type = Some(new_found_type);
            tokens.extend(prefix);
            let table_name = &table.name;
            tokens.extend(quote! {
                let #var_name = self.interning_tables.#table_name.intern(#new_name);
            });
        }
    }
    if let Some(final_type) = found_type {
        (var_name, final_type, tokens)
    } else {
        (var_name, target_type.clone(), tokens)
    }
}
