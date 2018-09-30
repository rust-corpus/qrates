use crate::rustc::hir::map::definitions::{DefPath, DefPathData};
use crate::rustc::hir;
use crate::rustc::hir::def_id::DefIndex;
use serde::{Serializer, Deserializer, Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub version: (u32, u32, u32),
    pub mods: Vec<Mod>,
    pub functions: Vec<Function>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    pub parent_mod_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub is_unsafe: bool,
    pub calls: Vec<UniqueIdentifier>,
    pub containing_mod_id: Option<usize>,
    //#[serde(skip_serializing, skip_deserializing)]
    pub def_id: DefIdWrapper
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueIdentifier {
    pub path: Vec<String>,
}

#[derive(Debug)]
pub struct DefIdWrapper(pub hir::def_id::DefId);


impl Crate {
    pub fn new(name: &str, version: (u32, u32, u32)) -> Crate {
        Crate {
            name: String::from(name),
            version: version,
            mods: vec![],
            functions: vec![]
        }
    }

    fn find_mod_ids(&self, name: &str) -> Vec<usize> {
        let mut ids: Vec<usize> = Vec::new();
        for (module, id) in self.mods.iter().zip(0..self.mods.len()) {
            if module.name == name {
                ids.push(id);
            }
        }
        ids
    }

    pub fn get_mod_id(&self, path: &UniqueIdentifier) -> Option<usize> {
        let mut current_mod: Option<usize> = None;
        for segment in &path.path {
            let mods = self.find_mod_ids(&segment);
            if let Some(new_parent) = mods.into_iter().find(|x| self.mods[*x].parent_mod_id == current_mod) {
                current_mod = Some(new_parent)
            }
            else {
                return None
            }
        }
        current_mod
    }

    /**
     * @return a unique identifier containing the name and version of this crate
     * that can be used as a filename for storing crate bundles.
     */
    pub fn get_filename(&self) -> String {
        self.name.clone() + "_"
            + &self.version.0.to_string() + "_" 
            + &self.version.1.to_string() + "_" 
            + &self.version.2.to_string()
    }
}

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

impl Default for DefIdWrapper {
    fn default() -> DefIdWrapper {
        DefIdWrapper(hir::def_id::DefId::local(hir::def_id::DefIndex::from_raw_u32(0)))
    }
}

impl UniqueIdentifier {
    pub fn new(data: Vec<String>) -> UniqueIdentifier {
        UniqueIdentifier{ path: data }
    }

    pub fn from_def_path_of_mod(def_path: &DefPath) -> UniqueIdentifier {
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
}

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
