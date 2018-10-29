use std::collections::BTreeMap;
use std::cmp::Ordering;
use super::ast::*;

pub fn compile_query(query: Vec<Rule>) -> String {
    let mut code: String = String::new();

    // add preamble
    
    code += r#"#![feature(rustc_private)]
extern crate datafrog;
extern crate rustql_common;

use datafrog::{Variable, Relation, Iteration};
use rustql_common::tuples::{Function, Mod, Crate, Database};

"#;


    // group rules by name
    let mut rule_map: BTreeMap<&str, Vec<&Rule>> = BTreeMap::new();
    for rule in &query {
        rule_map.entry(&rule.name).or_insert(Vec::new()).push(&rule);
    }

    for (name, rules) in &rule_map {
        let rule_code = compile_rules(name, &rules);
        code += &rule_code;
    }
    println!("{}", code);
    code 
}

#[derive(Debug)]
enum QueryNode<'a> {
    Input(&'a Fact),
    Join(Box<QueryNode<'a>>, Box<QueryNode<'a>>)
}

fn compile_rules(name: &str, rules: &Vec<&Rule>) -> String {
    let mut code: String = String::new();

    for (id, rule) in rules.iter().enumerate() {
        code += &compile_rule(rule, id);
    }
    let mut fn_code: String = "fn rules_".to_owned() + name + "(db: &Database) {\n";

    assert!(rules.len() > 0);

    let natives = generate_native_facts();

    let (id, rule) = rules.iter().enumerate().next().unwrap();
    let args = rule_argument_datanames(&rule, &natives, );
    fn_code += &format!("    let rel = rule_{}{}({});\n", name, id, "");

    for (id, rule) in rules.iter().enumerate().skip(1) {
        fn_code += &format!("    let rel = rel.merge(rule_{}{}({}));\n", name, id, "");
    }
    fn_code += "    rel\n}\n";


    fn_code + &code
}

fn generate_native_facts() -> BTreeMap<&'static str, (Vec<&'static str>, &'static str)> {
    let mut natives = BTreeMap::new();

    natives.insert("calls", (vec!["Function", "Function"], "calls"));
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
        args += ": Relation<(";
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

fn compile_rule(rule: &Rule, index: usize) -> String {
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

    let node = build_join_tree(rule);
    let (rule_code, fact) = compile_join_tree(node, rule);
    println!("new fact: {:?}", fact);

    format!(r#"fn rule_{}{}({}) {{
    let iteration = Iteration::new();
{}
}}"#,
        rule.name, index, args,
        rule_code
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
    
    let map1 = format!("|({})| (({}), ({}))",
        fact1.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap.iter().fold("".to_owned(), |s, x| s + x + ", "),
        fact1.args.iter().filter(|s| !overlap.contains(&s)).fold("".to_owned(), |s, x| s + x + ", ")
    );

    let map2 = format!("|({})| (({}), ({}))",
        fact2.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap.iter().fold("".to_owned(), |s, x| s + x + ", "),
        fact2.args.iter().filter(|s| !overlap.contains(s)).fold("".to_owned(), |s, x| s + x + ", ")
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
        let var1 = iteration.variable::<()>("left");
        let var2 = iteration.variable::<()>("right");
        var1.insert({}.iter().map({}));
        var2.insert({}.iter().map({}));

        let variable = iteration.variable::<()>("join");
        variable.from_join(&var1, &var2, |&key, &val1, &val2| (key, val1, val2)).complete()
    }};
    "#,
        new_fact.name,
        fact1.name, map1,
        fact2.name, map2,
    );

    (code, new_fact)
}





