use crate::ast;
use proc_macro2::{Span, TokenStream};
use quote::quote;

pub(super) fn generate_interning_tables(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut fields = TokenStream::new();
    let mut disk_fields = TokenStream::new();
    let mut disk_init_fields = TokenStream::new();
    let mut conversions = TokenStream::new();
    let mut disk_conversions = TokenStream::new();
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
        let disk_field = quote! {
            pub #name: DiskInterningTable<#key_type, #value>,
        };
        disk_fields.extend(disk_field);
        let disk_init_field = quote! {
            #name: DiskInterningTable::create_override_in(root.join(stringify!(#name))),
        };
        disk_init_fields.extend(disk_init_field);
        if let syn::Type::Tuple(syn::TypeTuple { elems, .. }) = value {
            if longest < elems.len() {
                longest = elems.len();
            }
        }
    }
    let mut args = TokenStream::new();
    let mut type_args = TokenStream::new();
    let mut type_constraints = TokenStream::new();
    let mut disk_type_constraints = TokenStream::new();
    for i in 0..longest {
        let arg = syn::Ident::new(&format!("v{}", i), Span::call_site());
        let type_arg = syn::Ident::new(&format!("V{}", i), Span::call_site());
        args.extend(quote! {#arg,});
        type_args.extend(quote! {#type_arg,});
        type_constraints.extend(quote! {
            #type_arg: crate::data_structures::InterningTableValue,
        });
        disk_type_constraints.extend(quote! {
            #type_arg: crate::data_structures::DiskInterningValue,
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
        if i == 0 {
            // don't generate for length 1 tuples, because `redb` does not impl redb::Key for (K,) in 2.4.0.
            // The next update should include this, but for now we don't need this.
            continue;
        }
        disk_conversions.extend(quote! {
            impl<K, #type_args> Into<Vec<(K, #type_args)>> for &DiskInterningTable<K, (#type_args)>
                where
                    K: crate::data_structures::DiskInterningKey,
                    #disk_type_constraints
            {
                fn into(self) -> Vec<(K, #type_args)> {
                    self
                        .iter()
                        .map(|(i, (#args))| {
                            (i, #args)
                        })
                        .collect()
                }
            }
        });
    }
    quote! {
        use crate::data_structures::InterningTable;
        use crate::data_structures::DiskInterningTable;
        #[derive(Default, Deserialize, Serialize)]
        /// Interning tables. Used during extraction and as merge source.
        pub struct InterningTables {
            #fields
        }
        #conversions
        /// Disk interning tables. Used as merge result and during queries.
        pub struct DiskInterningTables {
            #disk_fields
        }
        impl DiskInterningTables {
            pub fn create_in(root: &Path) -> Result<Self> {
                Ok(Self {
                    #disk_init_fields
                })
            }
        }
        #disk_conversions
    }
}
