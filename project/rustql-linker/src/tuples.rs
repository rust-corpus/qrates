
use rustql_common::data;
use datafrog::{Variable, Iteration};

pub struct Database {
    pub crates: Vec<(Crate, data::CrateIdentifier)>,
    pub modules: Vec<(Mod, data::Mod)>,
    pub functions: Vec<(Function, data::Function)>,

    pub modules_in_crates: Variable<(Crate, Mod)>,
    pub modules_in_modules: Variable<(Mod, Mod)>,
    pub functions_in_modules: Variable<(Function, Mod)>,
    pub function_calls: Variable<(Function, Function)>,

    pub iteration: Iteration,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Crate(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Mod(pub u64);
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Function(pub u64);


impl Database {
    pub fn new() -> Self {
        let mut iteration = Iteration::new();
        Database {
            crates: vec![],
            modules: vec![],
            functions: vec![],
            modules_in_crates: iteration.variable::<(Crate, Mod)>("modules_in_crates"),
            modules_in_modules: iteration.variable::<(Mod, Mod)>("modules_in_modules"),
            functions_in_modules: iteration.variable::<(Function, Mod)>("functions_in_modules"),
            function_calls: iteration.variable::<(Function, Function)>("function_calls"),
            iteration: iteration,
        }
    }

    pub fn search_module(name: &str) -> Option<Mod> {

    }
}


