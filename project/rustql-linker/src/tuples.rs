
use rustql_common::data;
use datafrog::{Variable, Relation, Iteration};

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
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Crate(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Mod(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Function(pub u64);


impl Database {
    pub fn new() -> Self {
        let mut iteration = Iteration::new();
        Database {
            crates: vec![],
            modules: vec![],
            functions: vec![],
            modules_in_crates: vec![],// iteration.variable::<(Mod, Crate)>("modules_in_crates"),
            modules_in_modules: vec![],// iteration.variable::<(Mod, Mod)>("modules_in_modules"),
            functions_in_modules: vec![],// iteration.variable::<(Function, Mod)>("functions_in_modules"),
            function_calls: vec![],// iteration.variable::<(Function, Function)>("function_calls"),
        }
    }

    pub fn get_crate(&self, ci: &data::CrateIdentifier) -> Option<Crate> {
        self.crates.iter().find(|(_, c)| c == ci).map(|x| x.0)
    }

    pub fn get_module_in_crate(&self, c_id: Crate, mod_name: &str) -> Option<Mod> {
        self.modules_in_crates.iter().filter(|(m, c)| *c == c_id).next().map(|x| x.0)
    }

    pub fn get_module_in_module(&self, m_id: Mod, mod_name: &str) -> Option<Mod> {
        self.modules_in_modules.iter().filter(|(m, m2)| *m2 == m_id).next().map(|x| x.0)
    }

    pub fn find_function(&self, path: &data::GlobalDefPath) -> Option<Function> {
        let krate = self.get_crate(&path.crate_ident);

        if let Some(krate) = krate {
            for data::GlobalDisambiguatedDefPathData {data: path, disambiguator: dis } in &path.path {
                if let data::GlobalDefPathData::Module(name) == path.data {
                    if get
                }
            }
        }
        None
    }

    pub fn search_module(&self, name: &str) -> Option<Mod> {
        self.modules.iter().filter(|m| m.1.name == name).next().map(|(m, _)| *m)
    }

    pub fn get_module(&self, m: Mod) -> &data::Mod {
        &self.modules[m.0 as usize].1
    }
}


