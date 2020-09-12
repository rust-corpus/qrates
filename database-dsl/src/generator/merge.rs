use super::utils::NameGenerator;
use crate::ast;
use log::debug;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;

pub(super) fn generate_merge_functions(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut name_generator = NameGenerator::new(String::from("tmp"));
    let mut tokens = TokenStream::new();
    let mut field_tokens = TokenStream::new();
    let mut field_init_tokens = TokenStream::new();
    for relation in &schema.relations {
        if let Some(ast::RelationKey { source, target }) = &relation.key {
            let mut key_types = TokenStream::new();
            for ident in source {
                let typ = &relation
                    .parameters
                    .iter()
                    .find(|parameter| &parameter.name == ident)
                    .unwrap()
                    .typ;
                key_types.extend(quote! {#typ,});
            }
            let target_type = relation
                .get_relation_key_target_type()
                .map(|typ| quote! {#typ})
                .unwrap_or_else(|| quote! {()});
            let target_name = target
                .as_ref()
                .map(|name| {
                    quote! {#name}
                })
                .unwrap_or_else(|| quote! {()});
            let map_name = relation.get_merge_map_name();
            let name = &relation.name;
            field_tokens.extend(quote! {
                #map_name: std::collections::HashMap<(#key_types), #target_type>,
            });

            let mut parameter_tokens = TokenStream::new();
            for parameter in &relation.parameters {
                let parameter_name = &parameter.name;
                if source.contains(parameter_name)
                    || target
                        .as_ref()
                        .map(|name| name == parameter_name)
                        .unwrap_or(false)
                {
                    parameter_tokens.extend(quote! {#parameter_name,});
                } else {
                    parameter_tokens.extend(quote! {_,});
                }
            }
            let mut ident_tokens = TokenStream::new();
            for ident in source {
                ident_tokens.extend(quote! {#ident,});
            }
            field_init_tokens.extend(quote! {
                #map_name: tables.relations.#name.iter().map(
                    |&(#parameter_tokens)| ((#ident_tokens), #target_name)
                ).collect(),
            });
        }
    }
    tokens.extend(merge_interning_tables(schema, &mut name_generator));
    tokens.extend(merge_relations(schema, &mut name_generator));
    tokens.extend(merge_counters(schema));
    quote! {

        pub struct TableMerger {
            pub(crate) tables: Tables,
            #field_tokens
        }

        impl TableMerger {
            pub fn new(tables: super::tables::Tables) -> Self {
                Self {
                    #field_init_tokens
                    tables,
                }
            }
            pub fn merge(&mut self, other: super::tables::Tables) {
                #tokens
            }
            pub fn tables(&mut self) -> &mut Tables {
                &mut self.tables
            }
        }
    }
}

fn merge_interning_tables(
    schema: &ast::DatabaseSchema,
    name_generator: &mut NameGenerator,
) -> TokenStream {
    let mut interning_remap = HashMap::new();
    let mut tokens = TokenStream::new();
    let mut tuple_iterning_tables = Vec::new();
    for table in &schema.interning_tables {
        let name = &table.name;
        if let syn::Type::Tuple(ref values) = table.value {
            tuple_iterning_tables.push((table, values));
        } else {
            let arg_remap = if let Some(map) = interning_remap.get(&table.value) {
                quote! {
                    let new_value = #map[&value];
                }
            } else {
                quote! {
                    let new_value = value;
                }
            };
            tokens.extend(quote! {
                let #name: HashMap<_, _> = other
                   .interning_tables
                   .#name
                   .into_iter()
                   .map(|(key, value)| {
                       #arg_remap
                       let new_key = self.tables.interning_tables.#name.intern(new_value);
                       (key, new_key)
                   })
                   .collect();
            });
            interning_remap.insert(table.get_key_type(), name);
        }
    }
    for (table, values) in tuple_iterning_tables {
        let name = &table.name;
        let mut args = TokenStream::new();
        let mut params = TokenStream::new();
        let mut arg_remap = TokenStream::new();
        for value_type in values.elems.pairs().map(|pair| pair.into_value()) {
            let param = name_generator.get_fresh_ident();
            let arg = name_generator.get_fresh_ident();
            if let Some(map) = interning_remap.get(value_type) {
                arg_remap.extend(quote! {
                    let #arg = #map[&#param];
                });
            } else {
                debug!("Not an interned type: {:?}", value_type);
                assert!(schema.get_type_kind(value_type).is_custom_id());
                arg_remap.extend(quote! {
                    let #arg = #param;
                });
            }
            args.extend(quote! {#arg,});
            params.extend(quote! {#param,})
        }
        tokens.extend(quote! {
            let #name: HashMap<_, _> = other
                .interning_tables
                .#name
                .into_iter()
                .map(|(key, (#params))| {
                    #arg_remap
                    let new_key = self.tables.interning_tables.#name.intern((#args));
                    (key, new_key)
                })
                .collect();
        });
    }
    tokens
}

fn merge_relations(
    schema: &ast::DatabaseSchema,
    name_generator: &mut NameGenerator,
) -> TokenStream {
    let mut tokens = TokenStream::new();
    let mut relation_with_target_remap_tokens = TokenStream::new();
    let mut relation_without_target_remap_tokens = TokenStream::new();

    let mut updatable_fields = HashMap::new();
    for relation in &schema.relations {
        if let Some(ast::RelationKey {
            target: Some(target),
            ..
        }) = &relation.key
        {
            let typ = &relation
                .parameters
                .iter()
                .find(|p| &p.name == target)
                .unwrap()
                .typ;
            if !updatable_fields.contains_key(typ) {
                let map_name = name_generator.get_fresh_ident();
                tokens.extend(quote! {
                    let mut #map_name: HashMap<#typ, #typ> = HashMap::new();
                });
                updatable_fields.insert(typ, map_name);
            }
        }
    }

    // TODO: The current algorithm relies on the order in which keyed
    // relations are merged (all dependencies must be merged before merging
    // dependents). This should be fixed.
    for relation in &schema.relations {
        let name = &relation.name;
        let mut params = TokenStream::new();
        let mut params_remap = TokenStream::new();
        let mut new_params = TokenStream::new();
        let mut new_target = None;
        let mut new_source = TokenStream::new();
        for param in &relation.parameters {
            let param_name = &param.name;
            params.extend(quote! { #param_name, });
            let new_name = name_generator.get_fresh_ident();
            new_params.extend(quote! { #new_name, });
            if relation.is_relation_key_target(param_name) {
                new_target = Some(new_name.clone());
            }
            if relation.is_in_relation_key_source(param_name) {
                new_source.extend(quote! {#new_name,});
            }
            match schema.get_type_kind(&param.typ) {
                ast::TypeKind::CustomId | ast::TypeKind::Enum | ast::TypeKind::RustType => {
                    params_remap.extend(quote! {
                        let #new_name = *#param_name;
                    });
                }
                ast::TypeKind::IncrementalId(id) => {
                    let counter_name = id.get_field_name();
                    let constant_count = id.constants.len();
                    let typ = &id.typ;
                    let shift = quote! {
                        #param_name.shift(
                            self.tables.counters.#counter_name-(#constant_count as #typ)
                        )
                    };
                    if constant_count > 0 {
                        params_remap.extend(quote! {
                            let #new_name = if #param_name.index() >= #constant_count {
                                #shift
                            } else {
                                *#param_name
                            };
                        });
                    } else {
                        params_remap.extend(quote! {
                            let #new_name = #shift;
                        });
                    }
                }
                ast::TypeKind::InternedId(table) => {
                    let map = &table.name;
                    params_remap.extend(quote! {
                        let #new_name = #map[#param_name];
                    });
                }
            }
            if let Some(map_name) = updatable_fields.get(&param.typ) {
                let new_tmp_name = name_generator.get_fresh_ident();
                params_remap.extend(quote! {
                    let #new_name = if let Some(#new_tmp_name) = #map_name.get(&#new_name) {
                        *#new_tmp_name
                    } else {
                        #new_name
                    };
                });
            }
        }
        let filter_tokens = if relation.key.is_some() {
            let map_name = relation.get_merge_map_name();
            let target = relation.get_relation_key_target();
            let new_id = if target.is_some() {
                name_generator.get_fresh_ident().to_token_stream()
            } else {
                quote! {_}
            };
            let update_tokens = relation
                .get_relation_key_target_type()
                .map(|typ| {
                    let re_map_name = &updatable_fields[typ];
                    let parameter_name = new_target.as_ref().unwrap();
                    quote! {
                        #re_map_name.insert(#parameter_name, *#new_id);
                    }
                })
                .unwrap_or_default();
            let new_target = new_target
                .map(|v| v.to_token_stream())
                .unwrap_or_else(|| quote! {()});
            quote! {
                if let Some(#new_id) = self.#map_name.get(&(#new_source)) {
                    #update_tokens
                    continue
                } else {
                    self.#map_name.insert((#new_source), #new_target);
                }
            }
        } else {
            TokenStream::new()
        };
        let target_tokens = if relation.get_relation_key_target().is_some() {
            &mut relation_with_target_remap_tokens
        } else {
            &mut relation_without_target_remap_tokens
        };
        target_tokens.extend(quote! {
            for (#params) in other.relations.#name.iter() {
                #params_remap
                #filter_tokens
                self.tables
                    .relations
                    .#name
                    .insert((#new_params));
            }
        });
    }
    tokens.extend(relation_with_target_remap_tokens);
    tokens.extend(relation_without_target_remap_tokens);
    tokens
}

fn merge_counters(schema: &ast::DatabaseSchema) -> TokenStream {
    let mut tokens = TokenStream::new();
    for id in &schema.incremental_ids {
        let field_name = id.get_field_name();
        let constant_count = id.constants.len();
        let typ = &id.typ;
        tokens.extend(quote! {
            self.tables.counters.#field_name +=
                other.counters.#field_name - (#constant_count as #typ);
        });
    }
    tokens
}
