// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use super::data;
use datafrog::Relation;
use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub crates: Vec<(Crate, data::CrateIdentifier)>,
    pub modules: Vec<(Mod, data::Mod)>,
    pub functions: Vec<(Function, data::Function)>,
    pub structs: Vec<(Struct, data::Struct)>,
    pub types: Vec<(Type, data::Type)>,

    /// An entry here means that the module is contained in the crate
    pub modules_in_crates: Vec<(Mod, Crate)>,

    /// An entry here means that the second module is a direct parent module
    /// of the first one
    pub modules_in_modules: Vec<(Mod, Mod)>,
    pub functions_in_modules: Vec<(Function, Mod)>,
    pub function_calls: Vec<(Function, Function)>,

    pub is_reference_to: Vec<(Type, Type)>,

    pub tuple: Vec<(Type, Type)>,
    pub slice: Vec<(Type, Type)>,

    pub argument_types: Vec<(Function, Type)>,

    pub is_struct_type: Vec<(Type, Struct)>,
    pub field_types: Vec<(Struct, Type)>,
    pub return_type: Vec<(Function, Type)>,

    /// hashmap used for reverse lookups of functions
    /// TODO refactor and probably only use one storage for functions
    #[serde(skip_serializing, skip_deserializing)]
    pub function_finder: HashMap<(data::CrateIdentifier, String), Function>,
    pub type_finder: HashMap<data::Type, Type>,
}

pub struct RawDatabase {
    pub functions: Relation<(Function,)>,
    pub structs: Relation<(Struct,)>,
    pub function_calls: Relation<(Function, Function)>,
    pub functions_in_modules: Relation<(Function, Mod)>,
    pub modules_in_crates: Relation<(Mod, Crate)>,
    pub is_unsafe: Relation<(Function,)>,
    pub is_type: Relation<(Type,)>,
    pub is_native: Relation<(Type,)>,
    pub is_reference_to: Relation<(Type, Type)>,
    pub tuple: Relation<(Type, Type)>,
    pub slice: Relation<(Type, Type)>,
    pub is_shared_reference: Relation<(Type,)>,
    pub is_mutable_reference: Relation<(Type,)>,
    pub argument_types: Relation<(Function, Type)>,
    pub is_struct_type: Relation<(Type, Struct)>,
    pub field_types: Relation<(Struct, Type)>,
    pub return_type: Relation<(Function, Type)>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Crate(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mod(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Function(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Struct(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Type(pub u64);

impl Database {
    pub fn new() -> Self {
        Database {
            crates: vec![],
            modules: vec![],
            functions: vec![],
            structs: vec![],
            types: vec![],
            modules_in_crates: vec![], // iteration.variable::<(Mod, Crate)>("modules_in_crates"),
            modules_in_modules: vec![], // iteration.variable::<(Mod, Mod)>("modules_in_modules"),
            functions_in_modules: vec![], // iteration.variable::<(Function, Mod)>("functions_in_modules"),
            function_calls: vec![], // iteration.variable::<(Function, Function)>("function_calls"),
            is_reference_to: vec![],
            tuple: vec![],
            slice: vec![],
            argument_types: vec![],
            is_struct_type: vec![],
            field_types: vec![],
            return_type: vec![],

            function_finder: HashMap::new(),
            type_finder: HashMap::new(),
        }
    }

    pub fn get_crate(&self, ci: &data::CrateIdentifier) -> Option<Crate> {
        self.crates.iter().find(|(_, c)| c == ci).map(|x| x.0)
    }

    pub fn get_module_in_crate(&self, c_id: Crate, _mod_name: &str) -> Option<Mod> {
        self.modules_in_crates
            .iter()
            .find(|(_m, c)| *c == c_id)
            .map(|(m, _c)| *m)
    }

    pub fn get_crate_of_function(&self, f_id: Function) -> Option<Crate> {
        let parent = self
            .functions_in_modules
            .iter()
            .find(|(f, _m)| *f == f_id)
            .map(|(_f, m)| *m)
            .unwrap();
        self.modules_in_crates
            .iter()
            .find(|(m, _c)| *m == parent)
            .map(|(_m, c)| *c)
        /*while let Some(p) = parent {
            parent = self.modules_in_modules.iter().filter(|(m1, m2)| m1 == parent).map(|x| x.1).next();
        }*/
    }

    // maybe rewrite in SQL or Datafrog
    pub fn get_function_in_crate(&self, c_id: Crate, f_def: &str) -> Option<Function> {
        self.functions
            .iter()
            .find(|(f_id, f)| {
                self.get_crate_of_function(*f_id) == Some(c_id) && f.def_path == f_def
            })
            .map(|(f_id, _f)| *f_id)
    }

    // maybe rewrite in SQL or Datafrog
    pub fn get_module_in_module(&self, m_id: Mod, _mod_name: &str) -> Option<Mod> {
        self.modules_in_modules
            .iter()
            .find(|(_m1, m2)| *m2 == m_id)
            .map(|(m1, _m2)| *m1)
    }

    //  pub fn find_function(&self, path: &data::GlobalDefPath) -> Option<Function> {
    //      let krate = self.get_crate(&path.crate_ident);

    //      if let Some(krate) = krate {
    //          /*for data::GlobalDisambiguatedDefPathData {data: path, disambiguator: dis } in &path.path {
    //              //if let data::GlobalDefPathData::Module(name) = path.data {
    //              //    //if get
    //              //}
    //          }*/
    //      }
    //      None
    //  }

    pub fn get_type(&self, ty: &data::Type) -> Option<Type> {
        self.type_finder.get(ty).map(|t| *t)
    }

    pub fn get_type_from_list(types: &Vec<(Type, data::Type)>, ty: &data::Type) -> Option<Type> {
        types.iter().find(|(_id, t)| t == ty).map(|(id, _t)| (*id))
    }

    pub fn search_module(&self, name: &str) -> Option<Mod> {
        self.modules
            .iter()
            .find(|(_id, m)| m.name == name)
            .map(|(id, _m)| *id)
    }

    pub fn get_module(&self, m: Mod) -> &data::Mod {
        &self.modules[m.0 as usize].1
    }

    pub fn add_type_or_get(
        links: &mut HashMap<data::Type, Type>,
        types: &mut Vec<(Type, data::Type)>,
        ty: &data::Type,
    ) -> Type {
        let mut type_id = links.get(&ty).map(|x| *x);

        if let None = type_id {
            let len = self::Type(types.len() as u64);
            type_id = Some(len);
            types.push((len, ty.clone()));
            links.insert(ty.clone(), len);
            println!("Found unknown type {:?}", ty);
        }

        type_id.unwrap()
    }

    pub fn link_types(&mut self) {
        for (ty_id, ty) in self.types.clone() {
            match ty {
                data::Type::Reference { .. } => {
                    let added = Self::add_type_or_get(&mut self.type_finder, &mut self.types, &ty);
                    self.is_reference_to.push((ty_id, added));
                }
                data::Type::Tuple(types) => {
                    for t in types.clone() {
                        let added =
                            Self::add_type_or_get(&mut self.type_finder, &mut self.types, &t);
                        self.tuple.push((ty_id, added));
                    }
                }
                data::Type::Slice(slice_ty) => {
                    let t = slice_ty.clone();
                    let added = Self::add_type_or_get(&mut self.type_finder, &mut self.types, &t);
                    self.slice.push((ty_id, added));
                }
                data::Type::Struct(ref s) => {
                    let s = self
                        .structs
                        .iter()
                        .find(|(_, s_info)| s_info.def_path == *s);
                    if let Some((s_id, _)) = s {
                        self.is_struct_type.push((ty_id, *s_id));
                    } else {
                        println!("Struct not found: {:?}", s);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn get_raw_database(&self) -> RawDatabase {
        RawDatabase {
            functions: self.functions.iter().map(|(c, _cd)| (*c,))
                .collect::<Vec<_>>().into(),
            structs: self.structs.iter().map(|(c, _cd)| (*c,))
                .collect::<Vec<_>>().into(),
            is_type: self.types.iter().map(|(c, _cd)| (*c,))
                .collect::<Vec<_>>().into(),
            is_native: self
                .types
                .iter()
                .filter(|(_i, typ)| {
                    match typ {
                        data::Type::Native(_) => {
                            // FIXME: This is never triggered.
                            true
                        },
                        _ => false,
                    }
                })
                .map(|(i, _typ)| (*i,))
                .collect::<Vec<_>>()
                .into(),
            function_calls: self.function_calls.iter().cloned()
                .collect::<Vec<_>>().into(),
            functions_in_modules: self.functions_in_modules.iter().cloned()
                .collect::<Vec<_>>().into(),
            modules_in_crates: self.modules_in_crates.iter().cloned()
                .collect::<Vec<_>>().into(),
            is_unsafe: self
                .functions
                .iter()
                .filter(|(_f, info)| info.is_unsafe)
                .map(|(c, _cd)| (*c,))
                .collect::<Vec<_>>()
                .into(),
            is_reference_to: self.is_reference_to.iter().map(|x| *x)
                .collect::<Vec<_>>().into(),
            is_mutable_reference: {
                let mutable_filter = |(_i, t): &&(Type, data::Type)| -> bool {
                    if let data::Type::Reference {
                        to: _,
                        is_mutable: m,
                    } = t
                    {
                        *m
                    } else {
                        false
                    }
                };
                self.types
                    .iter()
                    .filter(mutable_filter)
                    .map(|(i, _t)| (*i,))
                    .collect::<Vec<_>>()
                    .into()
            },
            is_shared_reference: {
                let shared_filter = |(_i, t): &&(Type, data::Type)| -> bool {
                    if let data::Type::Reference {
                        to: _,
                        is_mutable: m,
                    } = t
                    {
                        !(*m)
                    } else {
                        false
                    }
                };
                self.types
                    .iter()
                    .filter(shared_filter)
                    .map(|(i, _t)| (*i,))
                    .collect::<Vec<_>>()
                    .into()
            },
            tuple: self.tuple.iter().cloned()
                .collect::<Vec<_>>().into(),
            slice: self.slice.iter().cloned()
                .collect::<Vec<_>>().into(),
            argument_types: self.argument_types.iter().map(|x| *x)
                .collect::<Vec<_>>().into(),
            is_struct_type: self.is_struct_type.iter().map(|x| *x)
                .collect::<Vec<_>>().into(),
            field_types: self.field_types.iter().map(|x| *x)
                .collect::<Vec<_>>().into(),
            return_type: self.return_type.iter().map(|x| *x)
                .collect::<Vec<_>>().into(),
        }
    }
}
