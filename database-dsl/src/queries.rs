use crate::ast;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::Token;

pub(crate) fn generate(schema: ast::DatabaseSchema, input: TokenStream) -> TokenStream {
    let program: Program = match syn::parse2(input) {
        Ok(parsed_result) => parsed_result,
        Err(err) => return TokenStream::from(err.to_compile_error()),
    };
    let (mut tokens, datapond_input) = program.to_tokens(schema);
    let datafrog = datapond::generate_datafrog(datapond_input);
    tokens.extend(datafrog);
    let tokens = quote! {
        {
            use corpus_database::types::*;
            #tokens
        }
    };
    tokens
}

mod kw {
    syn::custom_keyword!(load);
}

struct LoadInstruction {
    operation: syn::Ident,
    args: Vec<syn::Ident>,
}

impl Parse for LoadInstruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let operation = input.parse()?;
        let content;
        syn::parenthesized!(content in input);
        let punctuated: syn::punctuated::Punctuated<_, Token![,]> =
            content.parse_terminated(syn::Ident::parse)?;
        let args = punctuated
            .into_pairs()
            .map(|pair| pair.into_value())
            .collect();
        Ok(LoadInstruction { operation, args })
    }
}

struct LoadInstructions {
    loader: Option<syn::Ident>,
    instructions: Vec<LoadInstruction>,
}

impl Parse for LoadInstructions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<kw::load>()?;
        let loader = input.parse()?;
        let content;
        syn::braced!(content in input);
        let punctuated: syn::punctuated::Punctuated<_, Token![,]> =
            content.parse_terminated(LoadInstruction::parse)?;
        let instructions = punctuated
            .into_pairs()
            .map(|pair| pair.into_value())
            .collect();
        Ok(LoadInstructions {
            loader: Some(loader),
            instructions: instructions,
        })
    }
}

struct Program {
    load_instructions: LoadInstructions,
    datapond_program: TokenStream,
}

impl Parse for Program {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let load_instructions = if lookahead.peek(kw::load) {
            input.parse()?
        } else {
            LoadInstructions {
                loader: None,
                instructions: Vec::new(),
            }
        };
        let datapond_program = input.cursor().token_stream();
        // Skip the remaining tokens.
        input.step(|cursor| {
            let mut rest = *cursor;
            while let Some((_, next)) = rest.token_tree() {
                rest = next;
            }
            Ok(((), rest))
        })?;
        Ok(Program {
            load_instructions,
            datapond_program,
        })
    }
}

impl Program {
    fn to_tokens(self, schema: ast::DatabaseSchema) -> (TokenStream, TokenStream) {
        let mut pre_tokens = TokenStream::new();
        let mut datapond_tokens = TokenStream::new();
        if let Some(loader) = &self.load_instructions.loader {
            for LoadInstruction { operation, args } in self.load_instructions.instructions {
                let (pre, datapond) = match operation.to_string().as_ref() {
                    "interning_tables_as_relations" => {
                        load_interning_tables_as_relations(&schema, loader, args)
                    }
                    "relations" => load_relations(&schema, loader, args),
                    _ => unreachable!("Unknown operation: {}", operation),
                };
                pre_tokens.extend(pre);
                datapond_tokens.extend(datapond);
            }
            datapond_tokens.extend(self.datapond_program);
            (pre_tokens, datapond_tokens)
        } else {
            (TokenStream::new(), self.datapond_program)
        }
    }
}

fn load_interning_tables_as_relations(
    schema: &ast::DatabaseSchema,
    loader: &syn::Ident,
    args: Vec<syn::Ident>,
) -> (TokenStream, TokenStream) {
    let mut pre_tokens = TokenStream::new();
    let mut datapond_tokens = TokenStream::new();
    for arg in args {
        let table = schema
            .find_interning_table(&arg)
            .expect("Not found interning table.");
        let name = &table.name;
        let load_fn_name = syn::Ident::new(&format!("load_{}_as_vec", name), Span::call_site());
        pre_tokens.extend(quote! {
            let #name = #loader.#load_fn_name();
        });
        let arg = convert_to_named_arg(&table.key.name);
        let mut args = quote! {#arg,};
        match &table.value {
            syn::Type::Tuple(syn::TypeTuple { elems, .. }) => {
                for elem in elems {
                    if let syn::Type::Path(syn::TypePath { qself: None, path }) = elem {
                        let arg = convert_to_named_arg(path.get_ident().unwrap());
                        args.extend(quote! {#arg,});
                    } else {
                        unreachable!();
                    }
                }
            }
            syn::Type::Path(syn::TypePath { qself: None, path }) => {
                let arg = convert_to_named_arg(path.get_ident().unwrap());
                args.extend(quote! {#arg,});
            }
            _ => unreachable!(),
        }
        datapond_tokens.extend(quote! {
            input #name(#args)
        });
    }
    (pre_tokens, datapond_tokens)
}

fn load_relations(
    schema: &ast::DatabaseSchema,
    loader: &syn::Ident,
    args: Vec<syn::Ident>,
) -> (TokenStream, TokenStream) {
    let mut pre_tokens = TokenStream::new();
    let mut datapond_tokens = TokenStream::new();
    for arg in args {
        let relation = schema.find_relation(&arg).unwrap_or_else(|| {
            unreachable!("Not found relation: {}", arg);
        });
        let name = &relation.name;
        let load_fn_name = syn::Ident::new(&format!("load_{}", name), Span::call_site());
        pre_tokens.extend(quote! {
            let #name = #loader.#load_fn_name().clone();
        });
        let mut args = TokenStream::new();
        for ast::RelationParameter { name, typ, .. } in &relation.parameters {
            args.extend(quote! {
                #name: #typ,
            });
        }
        datapond_tokens.extend(quote! {
            input #name(#args)
        });
    }
    (pre_tokens, datapond_tokens)
}

fn convert_to_named_arg(typ: &syn::Ident) -> TokenStream {
    let mut var_name = String::new();
    let mut first = true;
    for c in typ.to_string().chars() {
        if c.is_uppercase() {
            if !first {
                var_name.push('_');
            }
            var_name.extend(c.to_lowercase());
        } else {
            var_name.push(c);
        }
        first = false;
    }
    let var = syn::Ident::new(&var_name, Span::call_site());
    quote! {#var: #typ}
}
