use std::collections::HashMap;
use super::data;
use datafrog::Relation;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub crates: Vec<(Crate, data::CrateIdentifier)>,
    pub modules: Vec<(Mod, data::Mod)>,
    pub functions: Vec<(Function, data::Function)>,

    /// An entry here means that the module is contained in the crate
    pub modules_in_crates: Vec<(Mod, Crate)>,

    /// An entry here means that the second module is a direct parent module
    /// of the first one
    pub modules_in_modules: Vec<(Mod, Mod)>,
    pub functions_in_modules: Vec<(Function, Mod)>,
    pub function_calls: Vec<(Function, Function)>,

    /// hashmap used for reverse lookups of functions
    /// TODO refactor and probably only use one storage for functions
    #[serde(skip_serializing, skip_deserializing)]
    pub function_finder: HashMap<(data::CrateIdentifier, String), Function>,
}

pub struct RawDatabase {
    pub functions: Relation<(Function, )>,
    pub function_calls: Relation<(Function, Function)>,
    pub functions_in_modules: Relation<(Function, Mod)>,
    pub is_unsafe: Relation<(Function, )>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Crate(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Mod(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Function(pub u64);


impl Database {
    pub fn new() -> Self {
        Database {
            crates: vec![],
            modules: vec![],
            functions: vec![],
            modules_in_crates: vec![],// iteration.variable::<(Mod, Crate)>("modules_in_crates"),
            modules_in_modules: vec![],// iteration.variable::<(Mod, Mod)>("modules_in_modules"),
            functions_in_modules: vec![],// iteration.variable::<(Function, Mod)>("functions_in_modules"),
            function_calls: vec![],// iteration.variable::<(Function, Function)>("function_calls"),
            function_finder: HashMap::new(),
        }
    }

    pub fn get_crate(&self, ci: &data::CrateIdentifier) -> Option<Crate> {
        self.crates.iter().find(|(_, c)| c == ci).map(|x| x.0)
    }

    pub fn get_module_in_crate(&self, c_id: Crate, mod_name: &str) -> Option<Mod> {
        self.modules_in_crates.iter().filter(|(m, c)| *c == c_id).next().map(|x| x.0)
    }

    pub fn get_crate_of_function(&self, f_id: Function) -> Option<Crate> {
        let parent = self.functions_in_modules.iter().filter(|(f, m)| *f == f_id).map(|x| x.1).next().unwrap();
        self.modules_in_crates.iter().filter(|(m, c)| *m == parent).map(|x| x.1).next()
        /*while let Some(p) = parent {
            parent = self.modules_in_modules.iter().filter(|(m1, m2)| m1 == parent).map(|x| x.1).next();
        }*/
    }
    
    // maybe rewrite in SQL or Datafrog
    pub fn get_function_in_crate(&self, c_id: Crate, f_def: &str) -> Option<Function> {
        self.functions.iter().filter(|(f_id, f)| {
               self.get_crate_of_function(*f_id) == Some(c_id)
            && f.def_path == f_def
        }).next().map(|x| x.0)
    }

    // maybe rewrite in SQL or Datafrog
    pub fn get_module_in_module(&self, m_id: Mod, mod_name: &str) -> Option<Mod> {
        self.modules_in_modules.iter().filter(|(m, m2)| *m2 == m_id).next().map(|x| x.0)
    }

    pub fn find_function(&self, path: &data::GlobalDefPath) -> Option<Function> {
        let krate = self.get_crate(&path.crate_ident);

        if let Some(krate) = krate {
            /*for data::GlobalDisambiguatedDefPathData {data: path, disambiguator: dis } in &path.path {
                //if let data::GlobalDefPathData::Module(name) = path.data {
                //    //if get
                //}
            }*/
        }
        None
    }

    pub fn search_module(&self, name: &str) -> Option<Mod> {
        self.modules.iter().filter(|m| m.1.name == name).next().map(|(m, _)| *m)
    }

    pub fn get_module(&self, m: Mod) -> &data::Mod {
        &self.modules[m.0 as usize].1
    }

    pub fn get_raw_database(self) -> RawDatabase {
        RawDatabase {
            functions: self.functions.iter().map(|(c, _cd)| (*c, )).into(),
            function_calls: self.function_calls.into_iter().into(),
            functions_in_modules: self.functions_in_modules.into_iter().into(),
            is_unsafe: self.functions.iter().filter(|(f, info)| info.is_unsafe).map(|(c, _cd)| (*c, )).into()
        }
    }
}


