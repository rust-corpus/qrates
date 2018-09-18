use crate::rustc::hir::map::definitions::{DefPath, DefPathData};

#[derive(Serialize, Deserialize, Debug)]
pub struct Crate {
    pub name: String,
    pub version: (u32, u32, u32),
    pub mods: Vec<Mod>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mod {
    pub name: String,
    pub functions: Vec<Function>,
    pub parent_mod_id: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Function {
    pub name: String,
    pub is_unsafe: bool,
    pub calls: Vec<UniqueIdentifier>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UniqueIdentifier {
    pub path: Vec<String>
}



impl Crate {
    pub fn new(name: &str, version: (u32, u32, u32)) -> Crate {
        Crate {
            name: String::from(name),
            version: version,
            mods: vec![]
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
