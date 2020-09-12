use crate::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(super) fn generate_interning_tables(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut fields = TokenStream::new();
    let mut conversions = TokenStream::new();
    let mut longest = 2;
    for ast::InterningTable {
        ref name,
        ref key,
        ref value,
    } in &schema.interning_tables
    {
        let key_type = &key.name;
        let field = quote! {
            pub #name: InterningTable<#key_type, #value>,
        };
        fields.extend(field);
        if let syn::Type::Tuple(syn::TypeTuple { elems, .. }) = value {
            if longest < elems.len() {
                longest = elems.len();
            }
        }
    }
    let mut args = TokenStream::new();
    let mut type_args = TokenStream::new();
    let mut type_constraints = TokenStream::new();
    for i in 0..longest {
        let arg = syn::Ident::new(&format!("v{}", i), Span::call_site());
        let type_arg = syn::Ident::new(&format!("V{}", i), Span::call_site());
        args.extend(quote! {#arg,});
        type_args.extend(quote! {#type_arg,});
        type_constraints.extend(quote! {
            #type_arg: crate::data_structures::InterningTableValue,
        });
        conversions.extend(quote! {
            impl<K, #type_args> Into<Vec<(K, #type_args)>> for InterningTable<K, (#type_args)>
                where
                    K: crate::data_structures::InterningTableKey,
                    #type_constraints
            {
                fn into(self) -> Vec<(K, #type_args)> {
                    self.contents.into_iter().enumerate().map(|(i, (#args))| {
                        (i.into(), #args)
                    }).collect()
                }
            }

        });
    }
    quote! {
        use crate::data_structures::InterningTable;
        #[derive(Default, Deserialize, Serialize)]
        /// Interning tables.
        pub struct InterningTables {
            #fields
        }
        #conversions
    }
}
