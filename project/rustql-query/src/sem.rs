use std::collections::BTreeMap;

pub struct Context {
    pub types: BTreeMap<String, Type>,
}

pub struct Type {
    name: String,
    is_data: bool,
    /// maps function names to return types
    functions: BTreeMap<String, Function>,
}

pub struct Function {
    name: String,
    arg_types: Vec<String>,
    return_type: String
}


impl Context {
    pub fn new() -> Self {
        Context {
            types: Self::generate_types(),
        }
    }

    fn generate_types() -> BTreeMap<String, Type> {
        let mut ret = BTreeMap::new();
        ret.insert("int".to_owned(), Type{ name: "int".to_owned(), is_data: false, functions: BTreeMap::new()});
        let mut string = Type {
            name: "String".to_owned(),
            is_data: false,
            functions: BTreeMap::new()
        };
        string.functions.insert("len".to_owned(), Function{ name: "len".to_owned(), arg_types: vec![], return_type: "int".to_owned()});
        ret
    }
}


