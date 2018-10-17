use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Context {
    /// list with all available types in a query
    pub types: BTreeMap<String, Type>,

    /// all available variables allowed in a from clause
    pub variables: Vec<String>,

    /// all relations available in the database
    pub relations: Vec<String>,
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub rust_name: String,
    pub is_data: bool,
    /// maps function names to return types
    pub methods: BTreeMap<String, Function>,
}

///
/// represents one method that can be used in the query language
/// (predicates like like func.is_unsafe(), or something like func.)
///
#[derive(Debug)]
pub struct Function {
    name: String,
    arg_types: Vec<String>,
    return_type: String
}

#[derive(Debug)]
pub struct FilterFunc {
}

#[derive(Debug)]
pub struct Relation {
    pub types: Vec<String>,
}

#[derive(Clone, Copy, Debug)]
pub struct RelationId(pub usize);

#[derive(Debug)]
pub struct Query {
    pub transfromations: Vec<Transformation>,
}

#[derive(Debug)]
pub enum Variable {
    Native(String),
    Transformation(usize)
}

#[derive(Debug)]
pub enum Transformation {
    Filter {
        var: Variable,
        filter: FilterFunc
    },
}


impl Context {
    pub fn new() -> Self {
        let variables = vec!["Crate", "Mod", "Fn"];
        Context {
            types: Self::generate_types(),
            variables: variables.into_iter().map(|s| s.to_owned()).collect(),
            relations: vec![],//Relation{ types: vec!["int".to_owned(), "String".to_owned()] }]
        }
    }

    fn generate_types() -> BTreeMap<String, Type> {
        let mut ret = BTreeMap::new();
        ret.insert("int".to_owned(), Type{ name: "int".to_owned(), rust_name: "i64".to_string(),
        is_data: false, methods: BTreeMap::new()});
        let mut string = Type {
            name: "String".to_owned(),
            rust_name: "String".to_owned(),
            is_data: false,
            methods: BTreeMap::new()
        };
        string.methods.insert("len".to_owned(), Function{ name: "len".to_owned(), arg_types: vec![], return_type: "int".to_owned()});
        ret.insert(string.name.clone(), string);
        ret
    }

    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }
}


