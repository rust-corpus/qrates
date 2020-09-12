use crate::ast;
use proc_macro2::TokenStream;
use quote::quote;

pub(super) fn generate_counters(schema: &ast::DatabaseSchema) -> (TokenStream, TokenStream) {
    let mut fields = TokenStream::new();
    let mut getter_functions = TokenStream::new();
    let mut counter_functions = TokenStream::new();
    let mut default_impls = TokenStream::new();
    for id in &schema.incremental_ids {
        let ast::IncrementalId {
            ref name,
            ref typ,
            ref constants,
        } = id;
        let field_name = id.get_field_name();
        fields.extend(quote! {
            pub(crate) #field_name: #typ,
        });
        let get_fresh_name = id.get_generator_fn_name();
        getter_functions.extend(quote! {
            fn #get_fresh_name(&mut self) -> #name {
                let value = self.#field_name.into();
                self.#field_name += 1;
                value
            }
        });
        counter_functions.extend(quote! {
            pub fn #get_fresh_name(&mut self) -> #name {
                self.counters.#get_fresh_name()
            }
        });
        for constant in constants {
            let get_constant_name = constant.get_getter_name();
            let value = &constant.value;
            getter_functions.extend(quote! {
                fn #get_constant_name(&mut self) -> #name {
                    #value.into()
                }
            });
            counter_functions.extend(quote! {
                pub fn #get_constant_name(&mut self) -> #name {
                    self.counters.#get_constant_name()
                }
            });
        }
        let default_value = id.get_default_value();
        default_impls.extend(quote! {
           #field_name: #default_value,
        });
    }
    let counters = quote! {
        #[derive(Deserialize, Serialize)]
        /// Counters for generating unique identifiers.
        pub struct Counters {
            #fields
        }
        impl Counters {
            #getter_functions
        }
        impl Default for Counters {
            fn default() -> Self {
                Self {
                    #default_impls
                }
            }
        }
    };
    (counters, counter_functions)
}
