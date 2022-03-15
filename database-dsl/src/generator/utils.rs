use crate::ast;

#[derive(Debug, Clone)]
pub struct NameGenerator {
    name: String,
    counter: u32,
}

impl NameGenerator {
    pub fn new(name: String) -> Self {
        Self { name, counter: 0 }
    }
    /// Generate a new ident.
    pub fn inc(&mut self) {
        self.counter += 1;
    }
    /// Get the last generated ident.
    pub fn get_ident(&self) -> syn::Ident {
        syn::parse_str(&format!("{}_{}", self.name, self.counter)).unwrap()
    }
    /// Generate a new ident and return it.
    pub fn get_fresh_ident(&mut self) -> syn::Ident {
        self.counter += 1;
        syn::parse_str(&format!("{}_{}", self.name, self.counter)).unwrap()
    }
}

pub fn is_numeric_type(typ: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath {
        qself: None,
        ref path,
    }) = typ
    {
        path.is_ident("u8")
            || path.is_ident("u16")
            || path.is_ident("u32")
            || path.is_ident("u64")
            || path.is_ident("usize")
    } else {
        false
    }
}

pub fn is_copy_type(typ: &syn::Type, schema: &ast::DatabaseSchema) -> bool {
    match typ {
        syn::Type::Path(syn::TypePath { qself: None, path }) => match path.get_ident() {
            Some(ident) => match ident.to_string().as_ref() {
                "bool" | "u32" | "u64" | "u128" => true,
                _ => match schema.get_type_kind(typ) {
                    ast::TypeKind::CustomId
                    | ast::TypeKind::IncrementalId(_)
                    | ast::TypeKind::InternedId(_)
                    | ast::TypeKind::Enum => true,
                    ast::TypeKind::RustType => false,
                },
            },
            None => false,
        },
        syn::Type::Tuple(syn::TypeTuple { elems, .. }) => elems
            .iter()
            .all(|element_type| is_copy_type(element_type, schema)),
        _ => false,
    }
}
