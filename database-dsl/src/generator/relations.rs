use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_relations(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut fields = TokenStream::new();
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
            pub #name: Relation<(#parameter_tokens)>,
        });
    }
    quote! {
        use crate::data_structures::Relation;
        #[derive(Default, Deserialize, Serialize)]
        /// Relations between various entities of the Rust program.
        pub struct Relations {
            #fields
        }
    }
}
