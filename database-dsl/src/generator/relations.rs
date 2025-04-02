use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_relations(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut fields = TokenStream::new();
    let mut disk_fields = TokenStream::new();
    let mut disk_init_fields = TokenStream::new();

    for ast::Relation {
        ref name,
        ref parameters,
        ..
    } in &schema.relations
    {
        let mut parameter_tokens = TokenStream::new();
        for ast::RelationParameter { typ, .. } in parameters {
            parameter_tokens.extend(quote! {#typ,});
        }
        fields.extend(quote! {
            pub #name: Vec<(#parameter_tokens)>,
        });
        disk_fields.extend(quote! {
            pub #name: Relation<RelationElement<(#parameter_tokens)>>,
        });
        disk_init_fields.extend(quote! {
            #name: Relation::create_override_in(root.join(stringify!(#name))),
        });
    }
    quote! {
        use crate::data_structures::Relation;
        #[derive(Default, Serialize, Deserialize)]
        /// Relations between various entities of the Rust program.
        pub struct Relations {
            #fields
        }

        pub struct DiskRelations {
            #disk_fields
        }

        impl DiskRelations {
            pub fn create_in(root: &Path) -> Result<Self> {
                Ok(Self {
                    #disk_init_fields
                })
            }
        }
    }
}
