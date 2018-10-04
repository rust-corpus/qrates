use crate::rustc::hir::map::definitions::{DefPath, DefPathData};
use crate::rustc::hir;
use crate::rustc::hir::def_id::DefIndex;
use serde::{Serializer, Deserializer, Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrateIdentifier {
    pub name: String,
    pub version: (u64, u64, u64),
    pub config_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
    pub metadata: CrateIdentifier,
    pub mods: Vec<Mod>,
    pub functions: Vec<Function>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    pub parent_mod: Option<GlobalDefPath>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub is_unsafe: bool,
    pub calls: Vec<GlobalDefPath>,
    pub containing_mod: Option<GlobalDefPath>,
    //#[serde(skip_serializing, skip_deserializing)]
    //pub def_id: DefIdWrapper
}

#[derive(Debug)]
pub struct GlobalDefPath {
    pub path: Vec<hir::map::definitions::DisambiguatedDefPathData>,
    pub crate_ident: CrateIdentifier
}
/*pub struct GlobalDefPath {
    pub path: Vec<String>,
    pub crate_name: String,
    pub config_hash: u64
}*/

/*#[derive(Debug, PartialEq)]
pub struct DefIdWrapper(pub hir::def_id::DefId);
*/

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
            functions: vec![]
        }
    }

    /*fn find_mod_ids(&self, name: &str) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();
        for (module, id) in self.mods.iter().zip(0..self.mods.len()) {
            if module.name == name {
                ids.push(id);
            }
        }
        ids
    }*/

    /*pub fn get_mod_id(&self, path: &GlobalDefPath) -> Option<usize> {
        let mut current_mod: Option<usize> = None;
        for segment in path.0.data {
            let mods = self.find_mod_ids(&segment);
            if let Some(new_parent) = mods.into_iter().find(|x| self.mods[*x].parent_mod_id == current_mod) {
                current_mod = Some(new_parent)
            }
            else {
                return None
            }
        }
        current_mod
    }*/
    
    /*
    pub fn get_function(&self, def_id: hir::def_id::DefId) -> Option<&Function> {
        self.functions.iter().find(|f| f.def_id == DefIdWrapper(def_id) )
    }
    */
}

/*
impl Serialize for DefIdWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for DefIdWrapper {
    fn deserialize<D>(deserializer: D) -> Result<DefIdWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(DefIdWrapper::default())
        //deserializer.deserialize_i32(I32Visitor)
    }
}
*/

impl Serialize for GlobalDefPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

impl<'de> Deserialize<'de> for GlobalDefPath {
    fn deserialize<D>(deserializer: D) -> Result<GlobalDefPath, D::Error>
    where
        D: Deserializer<'de>,
    {
        //Ok(Self(DefPath::make()))
        Err(serde::de::Error::custom("not yet implemented"))

        //deserializer.deserialize_i32(I32Visitor)
    }
}

impl Default for Function {
    fn default() -> Self {
        Function {
            name: "".to_owned(),
            is_unsafe: false,
            calls: vec![],
            containing_mod: None,
            //def_id: DefIdWrapper::default(),
        }
    }
}

/*
impl Default for DefIdWrapper {
    fn default() -> Self {
        DefIdWrapper(hir::def_id::DefId::local(hir::def_id::DefIndex::from_raw_u32(0)))
    }
}
*/

/*impl GlobalDefPath {
    /// create an identifier to a local item
    pub fn new(data: Vec<String>) -> GlobalDefPath {
        GlobalDefPath {
            path: data,
            crate_name: "".to_owned(),
            config_hash: 0
        }
    }

    /*
    pub fn from_path(path: &hir::Path) -> GlobalDefPath {
        let mut new = GlobalDefPath {
            path: vec![],
            crate_name: "".to_owned(),
            config_hash: 0
        };

        for seg in &path.segments {
            let ident = seg.ident;
            new.path.push(ident.to_string());
        }

        new
    }
    */

    ///
    /// @warning does not yet support disambiguators
    ///
    pub fn from_def_path_of_mod(def_path: &DefPath) -> GlobalDefPath {
        let mut segments: Vec<String> = Vec::new();
        for elem in &def_path.data {
            if let DefPathData::Module(m) = elem.data {
                segments.push(m.as_str().to_string());
            }
        }
        return Self::new(segments);
    }

    pub fn remove_last_segment(mut self) -> Self {
        self.path.pop();
        self
    }
}*/

/*

impl From<DefId> for hir::def_id::DefId {
    fn from(def: DefId) -> Self {
        let indx: DefIndex = DefIndex::from_raw_u32(def.index);
        //hir::def_id::DefId::local(indx)
        hir::def_id::DefId{krate: hir::def_id::CrateNum::from_u32(def.crate_id), index: indx}
        /*{
            krate: hir::def_id::LOCAL_CRATE,
            index: DefIndex::from_u32(def.index)
        }*/
    }
}
*/
