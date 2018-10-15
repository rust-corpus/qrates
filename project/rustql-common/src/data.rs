//
// This file contains several data structures to represent data extracted from
// `rustc::hir`. They can be serialized into several crate packages and then be
// 'linked' together.
//

use crate::rustc::hir::map::definitions::{DefPath, DefPathData};
use crate::rustc::hir;
use crate::rustc::hir::def_id::DefIndex;
use serde::{Serializer, Deserializer, Serialize, Deserialize};


/// Structure that identifies a crate uniquely.
/// Two crates with the same CrateIdentifier are guaranteed to have the same ast.
#[derive(Serialize, Deserialize, PartialEq, Hash, Eq, Clone, Debug)]
pub struct CrateIdentifier {
    pub name: String,
    pub version: (u64, u64, u64),
    pub config_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Crate {
    pub metadata: CrateIdentifier,
    pub mods: Vec<Mod>,
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
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
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Struct {
    pub name: String,
    pub fields: Vec<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct GlobalDefPath {
    pub crate_ident: CrateIdentifier,
    pub def_path: String
}


impl CrateIdentifier {
    /// 
    /// @return a unique identifier containing the name and version of this crate
    /// that can be used as a filename for storing crate bundles.
    /// 
    pub fn get_filename(&self) -> String {
        self.name.clone() + "_"
            + &self.version.0.to_string() + "_" 
            + &self.version.1.to_string() + "_" 
            + &self.version.2.to_string() + "-"
            + &self.config_hash
    }
}

impl Crate {
    pub fn new(name: &str, version: (u64, u64, u64), config_hash: &str) -> Crate {
        Crate {
            metadata: CrateIdentifier {
                name: String::from(name),
                version: version,
                config_hash: config_hash.to_owned(),
            },

            mods: vec![],
            structs: vec![],
            functions: vec![]
        }
    }
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
    pub fn new(def: &hir::map::definitions::DefPath, c: &CrateIdentifier) -> Self {
        GlobalDefPath {
            //path: def.data.iter().map(GlobalDisambiguatedDefPathData::from).collect::<Vec<GlobalDisambiguatedDefPathData>>(),
            crate_ident: c.clone(),
            def_path: def.to_string_no_crate()
        }
    }
}

