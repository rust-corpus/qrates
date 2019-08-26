// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

// This file contains several data structures to represent data extracted from
// `rustc::hir`. They can be serialized into several crate packages and then be
// 'linked' together.
//

use serde_derive::{Serialize, Deserialize};

/// Structure that identifies a crate uniquely.
/// Two crates with the same CrateIdentifier are guaranteed to have the same ast.
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct CrateIdentifier {
    pub name: String,
    //    pub version: (u64, u64, u64),
    pub config_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub metadata: CrateIdentifier,
    pub mods: Vec<Mod>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
    //pub types: Vec<Type>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Mod {
    pub name: String,

    /// if this is none, then the module is the root module of a crate
    pub parent_mod: Option<usize>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Function {
    pub name: String,
    pub is_unsafe: bool,
    pub is_const: bool,
    pub is_async: bool,
    pub abi: String,
    pub is_closure: bool,
    pub calls: Vec<GlobalDefPath>,
    pub containing_mod: usize,
    pub def_path: String,
    pub argument_types: Vec<Type>,
    pub return_type: Type,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub def_path: GlobalDefPath,
    pub fields: Vec<(String, Type)>,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub struct GlobalDefPath {
    pub crate_ident: CrateIdentifier,
    pub def_path: String,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub enum Type {
    Native(String),
    Path(String),
    Struct(GlobalDefPath),
    Tuple(Vec<Type>),
    Slice(Box<Type>),
    Reference { to: Box<Type>, is_mutable: bool },
    Other,
}

impl CrateIdentifier {
    ///
    /// @return a unique identifier containing the name and version of this crate
    /// that can be used as a filename for storing crate bundles.
    ///
    pub fn get_filename(&self) -> String {
        self.name.clone() + "_"
            //+ &self.version.0.to_string() + "_" 
            //+ &self.version.1.to_string() + "_" 
            //+ &self.version.2.to_string() + "-"
            + &self.config_hash
    }
}

impl Crate {
    pub fn new(name: &str, _version: (u64, u64, u64), config_hash: &str) -> Crate {
        Crate {
            metadata: CrateIdentifier {
                name: String::from(name),
                //version: version,
                config_hash: config_hash.to_owned(),
            },

            mods: vec![],
            structs: vec![],
            functions: vec![],
            //types: vec![],
        }
    }

    /*pub fn insert_type(&mut self, t: Type) -> usize {
        if let Some((existing, _)) = self.types.iter().enumerate().find(|(_, ty)| **ty == t) {
            existing
        }
        else {
            self.types.push(t); self.types.len() - 1
        }
    }*/
}

/*impl Default for Function {
    fn default() -> Self {
        Function {
            name: "".to_owned(),
            is_unsafe: false,
            calls: vec![],
            containing_mod: 0,
            def_path: "".to_owned()
            //def_id: DefIdWrapper::default(),
        }
    }
}
*/

impl GlobalDefPath {
    pub fn new(def_path_str: String, c: CrateIdentifier) -> Self {
        GlobalDefPath {
            //path: def.data.iter().map(GlobalDisambiguatedDefPathData::from).collect::<Vec<GlobalDisambiguatedDefPathData>>(),
            crate_ident: c,
            def_path: def_path_str,
        }
    }
}
