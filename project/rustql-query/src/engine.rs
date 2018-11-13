use std::collections::BTreeMap;
use std::cmp::Ordering;
use super::ast::*;


pub fn compile_query(query: Vec<Rule>, decls: Vec<Decl>) -> String {
    let mut code: String = String::new();

    // add preamble
    
    code += r#"#![feature(rustc_private)]
extern crate datafrog;
extern crate rustql_common;

use datafrog::{Variable, Relation, Iteration};
use rustql_common::tuples::{Function, Mod, Crate, RawDatabase};

"#;


    // group rules by name
    let mut rule_map: BTreeMap<&str, Vec<&Rule>> = BTreeMap::new();
    for rule in &query {
        rule_map.entry(&rule.name).or_insert(Vec::new()).push(&rule);
    }

    for (&name, rules) in &rule_map {
        let decl = decls.iter().filter(|&d| d.name == name).next().expect("found rule without declaration");
        code += &generate_print_code(decl);
        let rule_code = compile_rules(name, &rules, decl);
        code += &rule_code;
    }
    println!("{}", code);
    code 
}

fn generate_print_code(decl: &Decl) -> String {
    let code = format!(r#"#[no_mangle]
pub extern "C" fn print_{}(db: &RawDatabase) {{
    rules_{}(db).iter().for_each(|element| println!("tuple: {{:?}}", element));
}}
"#, decl.name, decl.name);
    code
}

#[derive(Debug)]
enum QueryNode<'a> {
    Input(&'a Fact),
    Join(Box<QueryNode<'a>>, Box<QueryNode<'a>>)
}

fn compile_rules(name: &str, rules: &Vec<&Rule>, decl: &Decl) -> String {
    let natives = generate_native_facts();

    let mut code: String = String::new();

    for (id, rule) in rules.iter().enumerate() {
        code += &compile_rule(rule, decl, id);
    }

    /*let rule = &rules[0];
    let result_names = &rule.args;
    for name in result_names {
        for rule in rules {
            for fact in &rule.facts {
                if let Some(id) = fact.args.iter().position(|t| t == name) {
                    if let Some((types, _n)) = natives.get(&fact.name) {

                    }
                }
            }
        }
    }*/
    let return_type:String = decl.arg_types.iter().fold("Relation<(".to_owned(), |s, t| s + t + ", ") + ")>";

    let mut fn_code: String = "#[no_mangle]\npub extern \"C\" fn rules_".to_owned() + name + "(db: &RawDatabase) -> "
        + &return_type + " {\n";

    assert!(rules.len() > 0);

    for (rel_name, (types, native_name)) in &natives {
        let mut typelist = types.iter().fold("".to_owned(), |s, t| s + t + ", ");
        //if types.len() <= 1 { typelist += "()"; }
        fn_code += &format!("    let {}: &Relation<({})> = &db.{};\n",
            native_name, typelist, native_name);
    }

    let (id, rule) = rules.iter().enumerate().next().unwrap();
    let args = rule_argument_datanames(&rule, &natives)
        .into_iter()
        .fold("".to_owned(), |s, a| s + a + ", ");
    fn_code += &format!("    let rel = rule_{}{}({});\n", name, id, args); 

    for (id, rule) in rules.iter().enumerate().skip(1) {
        let args = rule_argument_datanames(&rule, &natives)
            .into_iter()
            .fold("".to_owned(), |s, a| s + a + ", ");
        fn_code += &format!("    let rel = rel.merge(rule_{}{}({}));\n", name, id, args);
    }
    fn_code += "    rel\n}\n";


    fn_code + &code
}

fn generate_native_facts() -> BTreeMap<&'static str, (Vec<&'static str>, &'static str)> {
    let mut natives = BTreeMap::new();

    natives.insert("calls", (vec!["Function", "Function"], "function_calls"));
    natives.insert("function", (vec!["Function"], "functions"));
    natives.insert("in_module", (vec!["Function", "Mod"], "functions_in_modules"));

    natives
}

fn rule_argument_datanames(rule: &Rule, natives: &BTreeMap<&'static str, (Vec<&'static str>, &'static str)>) -> Vec<&'static str>{
    let mut datanames = Vec::new();
    let mut dedup_facts = rule.facts.clone();
    dedup_facts.sort_unstable_by(|l, r|
        if l.name < r.name { Ordering::Less }
        else if l.name == r.name { Ordering::Equal }
        else { Ordering::Greater });
    dedup_facts.dedup_by(|l, r| l.name == r.name);
    for fact in dedup_facts {
        let types = natives.get(&*fact.name);
        if let Some((_t, name)) = types {
            datanames.push(*name);
        }
        else {
            panic!("unknown relation: {}", fact.name);
        }
    }
    datanames
}

fn rule_arguments(rule: &Rule, natives: &BTreeMap<&'static str, (Vec<&'static str>, &'static str)>) -> String {
    let mut args = String::new();
    let mut dedup_facts = rule.facts.clone();
    dedup_facts.sort_unstable_by(|l, r|
        if l.name < r.name { Ordering::Less }
        else if l.name == r.name { Ordering::Equal }
        else { Ordering::Greater });
    dedup_facts.dedup_by(|l, r| l.name == r.name);
    for fact in dedup_facts {
        args += &fact.name;
        args += ": &Relation<(";
        let types = natives.get(&*fact.name);
        if let Some((types, _name)) = types {
            for t in types {
                args += t;
                args += ", ";
            }
        }
        else {
            panic!("unknown relation: {}", fact.name);
        }
        args += ")>, ";
    }
    args
}

fn compile_rule(rule: &Rule, decl: &Decl, index: usize) -> String {
    let natives = generate_native_facts();
    /*
    let mut args: String = String::new();
    let mut dedup_facts = rule.facts.clone();
    dedup_facts.sort_unstable_by(|l, r|
        if l.name < r.name { Ordering::Less }
        else if l.name == r.name { Ordering::Equal }
        else { Ordering::Greater });
    dedup_facts.dedup_by(|l, r| l.name == r.name);
    for fact in dedup_facts {
        args += &fact.name;
        args += ": Relation<(";
        let types = natives.get(&*fact.name);
        if let Some(types) = types {
            for t in types {
                args += t;
                args += ", ";
            }
        }
        else {
            panic!("unknown relation: {}", fact.name);
        }
        args += ")>, ";
    }
    */
    let args = rule_arguments(rule, &natives);//.into_iter().fold("".to_owned(), |s, n| s + n + ", ");
    let return_type = decl.arg_types.iter().fold("Relation<(".to_owned(), |s, t| s + t + ", ") + ")>";

    let node = build_join_tree(rule);
    let (rule_code, fact) = compile_join_tree(node, rule);

    let final_map = format!("|({})| ({})",
        fact.args.iter().fold("".to_owned(), |s, t| s + t + ", "),
        rule.args.iter().fold("".to_owned(), |s, t| s + "*" + t + ", "),
        );

    println!("new fact: {:?}", fact);

    format!(r#"fn rule_{}{}({}) -> {} {{
    let mut iteration = Iteration::new();
{}
    {}.into_iter().map({}).into()
}}"#,
// {}.into_iter().map(|((x,), ())| (*x,)).into()
        rule.name, index, args,
        return_type,
        rule_code,
        fact.name,
        final_map
    )
}

fn build_join_tree(rule: &Rule) -> QueryNode {
    assert!(rule.facts.len() > 0);

    let mut node = QueryNode::Input(&rule.facts[0]);

    for fact in rule.facts.iter().skip(1) {
        node = QueryNode::Join(box node, box QueryNode::Input(&fact));
    }

    node
}

fn compile_join_tree(node: QueryNode, rule: &Rule) -> (String, Fact) {
    match node {
        QueryNode::Input(name) => { ("".to_owned(), name.clone()) },
        QueryNode::Join(box left, box right) => {
            let (left_code, lfact) = compile_join_tree(left, rule);
            let (right_code, rfact) = compile_join_tree(right, rule);

            let (join, joinfact) = compile_join(&lfact, &rfact);

            (left_code + &right_code + &join, joinfact)
        }
    }
}

fn compile_join(fact1: &Fact, fact2: &Fact) -> (String, Fact) {
    let overlap = fact1.get_overlapping(fact2);

    let vals1 = fact1.args.iter().filter(|s| !overlap.contains(&s)).collect::<Vec<&String>>(); 
    let vals2 = fact2.args.iter().filter(|s| !overlap.contains(s)).collect::<Vec<&String>>(); 
    
    let map1 = format!("|({})| (({}), ({}))",
        fact1.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap.iter().fold("".to_owned(), |s, x| s + "*" + x + ", "),
        vals1.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let map2 = format!("|({})| (({}), ({}))",
        fact2.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap.iter().fold("".to_owned(), |s, x| s + "*" + x + ", "),
        // fact2.args.iter().filter(|s| !overlap.contains(s)).fold("".to_owned(), |s, x| s + "*" + x + ", ")
        vals2.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let new_fact = Fact{
        name: "temp_fact".to_owned(),
        args: overlap.iter()
            .chain(fact1.args.iter().filter(|s| !overlap.contains(s)))
            .chain(fact2.args.iter().filter(|s| !overlap.contains(s)))
            .map(|s| s.clone())
            .collect()
    };

    let code = format!(r#"
    let {} = {{
        let var1 = iteration.variable("left");
        let var2 = iteration.variable("right");
        var1.insert({}.iter().map({}).into());
        var2.insert({}.iter().map({}).into());
        iteration.changed();

        let variable = iteration.variable("join");
        variable.from_join(&var1, &var2, |&key, &val1, &val2| ({}{}{}));
        while iteration.changed() {{}}
        variable.complete()
    }};
    "#,
        new_fact.name,
        fact1.name, map1,
        fact2.name, map2,
        (0..overlap.len()).map(|i| "key.".to_owned() + &i.to_string()).fold("".to_owned(), |s, t| s + &t + ", "),
        if vals1.len() > 0 { (0..vals1.len()).map(|i| "val1.".to_owned() + &i.to_string()).fold("".to_owned(), |s, t| s + &t + ", ") } else { "".to_owned() },
        if vals2.len() > 0 { (0..vals2.len()).map(|i| "val2.".to_owned() + &i.to_string()).fold("".to_owned(), |s, t| s + &t + ", ") } else { "".to_owned() },
    );

    (code, new_fact)
}





