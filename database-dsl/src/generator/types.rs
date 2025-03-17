use super::utils::is_numeric_type;
use crate::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(super) fn generate_types(schema: &ast::DatabaseSchema) -> TokenStream {
    let id_types = generate_id_types(schema);
    let enum_types = generate_enum_types(schema);
    quote! {
        #id_types
        #enum_types
    }
}

fn generate_id_types(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut tokens = TokenStream::new();
    for ast::CustomId {
        ref name,
        ref typ,
        items,
    } in &schema.custom_ids
    {
        tokens.extend(generate_id_decl(name, typ));
        for item in items {
            tokens.extend(quote! { #item });
        }
    }
    for ast::IncrementalId {
        ref name, ref typ, ..
    } in &schema.incremental_ids
    {
        tokens.extend(generate_id_decl(name, typ));
    }
    for ast::InterningTable {
        key: ast::InternedId { ref name, ref typ },
        ..
    } in &schema.interning_tables
    {
        tokens.extend(generate_id_decl(name, typ));
    }
    tokens
}

fn generate_id_decl(name: &syn::Ident, typ: &syn::Type) -> TokenStream {
    let mut tokens = quote! {
        #[derive(
            Debug, Eq, PartialEq, Hash, Clone, Copy,
            Deserialize, Serialize, PartialOrd, Ord, Default
        )]
        pub struct #name(pub(super) #typ);
    };
    if is_numeric_type(typ) {
        tokens.extend(quote! {
            impl From<#typ> for #name {
                fn from(value: #typ) -> Self {
                    Self(value)
                }
            }

            impl From<usize> for #name {
                fn from(value: usize) -> Self {
                    Self(value as #typ)
                }
            }

            impl Into<usize> for #name {
                fn into(self) -> usize {
                    self.0 as usize
                }
            }

            impl redb::Value for #name {
                type SelfType<'a> = Self;
                type AsBytes<'a> = Vec<u8>;

                fn fixed_width() -> Option<usize> {
                    Some(std::mem::size_of::<#typ>())
                }

                fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
                where Self: 'a
                {
                    let value = <#typ>::qrates_from_bytes(data);
                    Self(value)
                }

                fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
                    value.0.qrates_as_bytes()
                }

                fn type_name() -> redb::TypeName {
                    redb::TypeName::new(stringify!(#name))
                }

            }

            impl redb::Key for #name {
                fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
                    let value1 = <#typ>::qrates_from_bytes(data1);
                    let value2 = <#typ>::qrates_from_bytes(data2);
                    value1.cmp(&value2)
                }
            }

            impl #name {
                /// Shift the id by given `offset`.
                pub fn shift(&self, offset: #typ) -> Self {
                    Self(self.0.checked_add(offset).expect("Overflow!"))
                }
                /// Get the underlying index.
                pub fn index(&self) -> usize {
                    self.0 as usize
                }
            }
        });
    }
    tokens
}

fn generate_enum_types(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut tokens = TokenStream::new();
    for ast::Enum {
        ref item,
        ref default,
    } in &schema.enums
    {
        let enum_name = item.ident.clone();
        let mut new_item = item.clone();
        for (variant_id, variant) in new_item.variants.iter_mut().enumerate() {
            assert!(variant.discriminant.is_none());
            variant.discriminant = Some((
                syn::Token![=](Span::call_site()),
                syn::Expr::Lit(syn::ExprLit {
                    attrs: Vec::new(),
                    lit: syn::Lit::Int(syn::LitInt::new(
                        &variant_id.to_string(),
                        Span::call_site(),
                    )),
                }),
            ));
        }
        let item_names = new_item.variants.iter().map(|variant| &variant.ident);
        let item_discriminants = new_item
            .variants
            .iter()
            .map(|variant| variant.discriminant.as_ref().unwrap().1.clone());
        let enum_tokens = quote! {

            #[repr(u8)]
            #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Deserialize, Serialize, PartialOrd, Ord)]
            pub #new_item

            impl redb::Value for #enum_name {
                type SelfType<'a> = Self;
                type AsBytes<'a> = Vec<u8>;

                fn fixed_width() -> Option<usize> {
                    Some(1)
                }

                fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
                where Self: 'a
                {
                    let value = data[0];
                    Self::from_u8(value).unwrap()
                }

                fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a> {
                    vec![*value as u8]
                }

                fn type_name() -> redb::TypeName {
                    redb::TypeName::new(stringify!(#enum_name))
                }
            }

            impl redb::Key for #enum_name {
                fn compare(data1: &[u8], data2: &[u8]) -> std::cmp::Ordering {
                    let value1 = data1[0];
                    let value2 = data2[0];
                    value1.cmp(&value2)
                }
            }

            impl #enum_name {
                pub fn from_u8(value: u8) -> Option<Self> {
                    match value {
                        #(
                            #item_discriminants => Some(#enum_name::#item_names),
                        )*
                        _ => None,
                    }
                }
            }

            impl Default for #enum_name {
                fn default() -> Self {
                    #enum_name::#default
                }
            }

            impl std::fmt::Display for #enum_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }
        };
        tokens.extend(enum_tokens);
    }
    tokens
}
