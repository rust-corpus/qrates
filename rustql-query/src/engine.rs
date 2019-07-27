// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use super::ast::*;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug)]
struct RelationInfo {
    arg_types: Vec<String>,
    variable_name: String,
    is_native: bool,
}

fn generate_native_facts() -> BTreeMap<String, RelationInfo> {
    let mut natives = BTreeMap::new();

    natives.insert(
        "calls".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned(), "Function".to_owned()],
            variable_name: "function_calls".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "function".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned()],
            variable_name: "functions".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "in_module".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned(), "Mod".to_owned()],
            variable_name: "functions_in_modules".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "modules_in_crates".to_owned(),
        RelationInfo {
            arg_types: vec!["Mod".to_owned(), "Crate".to_owned()],
            variable_name: "modules_in_crates".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_unsafe".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned()],
            variable_name: "is_unsafe".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_struct".to_owned(),
        RelationInfo {
            arg_types: vec!["Struct".to_owned()],
            variable_name: "structs".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_type".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned()],
            variable_name: "is_type".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_native".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned()],
            variable_name: "is_native".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_reference_to".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned(), "Type".to_owned()],
            variable_name: "is_reference_to".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_mutable_reference".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned()],
            variable_name: "is_mutable_reference".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_shared_reference".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned()],
            variable_name: "is_shared_reference".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "tuple".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned(), "Type".to_owned()],
            variable_name: "tuple".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "slice".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned(), "Type".to_owned()],
            variable_name: "slice".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "argument_types".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned(), "Type".to_owned()],
            variable_name: "argument_types".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "is_struct_type".to_owned(),
        RelationInfo {
            arg_types: vec!["Type".to_owned(), "Struct".to_owned()],
            variable_name: "is_struct_type".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "field_types".to_owned(),
        RelationInfo {
            arg_types: vec!["Struct".to_owned(), "Type".to_owned()],
            variable_name: "field_types".to_owned(),
            is_native: true,
        },
    );
    natives.insert(
        "return_type".to_owned(),
        RelationInfo {
            arg_types: vec!["Function".to_owned(), "Type".to_owned()],
            variable_name: "return_type".to_owned(),
            is_native: true,
        },
    );
    //natives.insert("function".to_owned(), (vec!["Function"], "functions"));
    //natives.insert("in_module".to_owned(), (vec!["Function", "Mod"], "functions_in_modules"));
    //natives.insert("is_unsafe".to_owned(), (vec!["Function"], "is_unsafe"));

    natives
}

pub fn compile_query(query: Vec<Rule>, decls: Vec<Decl>, actions: &Vec<Action>) -> String {
    let mut code: String = String::new();

    // add preamble

    // TODO add some nice templating mechanism
    code += r#"#![feature(rustc_private)]
extern crate csv;
extern crate datafrog;
extern crate rustql_common;

use datafrog::{Variable, Relation, Iteration};
use rustql_common::tuples::*;

"#;

    let mut existing_rules = generate_native_facts();

    for decl in &decls {
        //println!("inserting decl for {:?}", decl);
        existing_rules.insert(
            decl.name.clone(),
            RelationInfo {
                arg_types: decl.arg_types.clone(),
                variable_name: decl.name.clone(),
                is_native: false,
            },
        );
    }

    // group rules by name
    let mut rule_map: BTreeMap<&str, Vec<&Rule>> = BTreeMap::new();
    for rule in &query {
        rule_map.entry(&rule.name).or_insert(Vec::new()).push(&rule);
    }

    for (&name, rules) in &rule_map {
        let decl = decls
            .iter()
            .filter(|&d| d.name == name)
            .next()
            .expect("found rule without declaration");
        code += &generate_print_code(decl);
        let rule_code = compile_rules(name, &rules, decl, &existing_rules);
        code += &rule_code;
    }

    for action in actions {
        if action.name == "for_each" {
            code += &format!(
                r#"#[no_mangle] pub extern "C" fn {}_{}(db: &RawDatabase, orig_db: &Database) {{
    rules_{}(db).iter().for_each({});
}}
"#,
                action.name, action.target, action.target, action.rust_code
            );
        } else if action.name == "csv" {
            code += &format!(
                r#"#[no_mangle] pub extern "C" fn {}_{}(writer: &mut csv::Writer<std::fs::File>, db: &RawDatabase, orig_db: &Database) {{
    rules_{}(db).iter().for_each({});
}}
"#,
                action.name, action.target, action.target, action.rust_code
            );
        } else {
            panic!("unknown action: {}", action.name);
        }
    }

    //println!("{}", code);
    code
}

fn generate_print_code(decl: &Decl) -> String {
    let code = format!(
        r#"#[no_mangle]
pub extern "C" fn print_{}(db: &RawDatabase) {{
    rules_{}(db).iter().for_each(|element| println!("tuple: {{:?}}", element));
}}
"#,
        decl.name, decl.name
    );
    code
}

#[derive(Debug)]
enum QueryNode<'a> {
    Input(&'a Fact),
    Join(Box<QueryNode<'a>>, Box<QueryNode<'a>>, bool),
    RecursiveJoin(Box<QueryNode<'a>>, Box<QueryNode<'a>>),
}

fn compile_rules(
    name: &str,
    rules: &Vec<&Rule>,
    decl: &Decl,
    existing_rules: &BTreeMap<String, RelationInfo>,
) -> String {
    let mut code: String = String::new();

    for (id, rule) in rules.iter().enumerate() {
        code += &compile_rule(rule, decl, id, existing_rules);
    }

    let return_type: String = decl
        .arg_types
        .iter()
        .fold("Relation<(".to_owned(), |s, t| s + t + ", ")
        + ")>";

    let mut fn_code: String = "#[no_mangle]\npub extern \"C\" fn rules_".to_owned()
        + name
        + "(db: &RawDatabase) -> "
        + &return_type
        + " {\n";

    assert!(rules.len() > 0);

    fn_code += "    let iteration = Iteration::new();\n";

    let mut variable_map: BTreeMap<String, String> = BTreeMap::new();
    for (rel_name, rel_info) in existing_rules {
        let typelist = rel_info
            .arg_types
            .iter()
            .fold("".to_owned(), |s, t| s + t + ", ");
        //if types.len() <= 1 { typelist += "()"; }
        if rel_info.is_native {
            fn_code += &format!(
                "    let {}: &Relation<({})> = &db.{};\n",
                rel_info.variable_name, typelist, rel_info.variable_name
            );
        //variable_map.insert(rel_name.to_owned(), format!("    let {}: &Relation<({})> = &db.{};\n",
        //    rel_info.variable_name, typelist, rel_info.variable_name));
        } else {
            //fn_code += &format!("    let {}: &Relation<({})> = &rules_{}(&db);\n",
            //    rel_info.variable_name, typelist, rel_info.variable_name);
            variable_map.insert(
                rel_name.to_owned(),
                format!(
                    "    let {}: &Relation<({})> = &rules_{}(&db);\n",
                    rel_info.variable_name, typelist, rel_info.variable_name
                ),
            );
        }
    }

    let (id, rule) = rules.iter().enumerate().next().unwrap();
    let arg_datanames = rule_argument_datanames(&rule, &existing_rules);

    for dataname in &arg_datanames {
        if let Some(code) = variable_map.get(dataname) {
            fn_code += code;
        }
    }

    let args = arg_datanames
        .iter()
        .fold("".to_owned(), |s, a| s + a + ", ");
    fn_code += &format!("    let rel = rule_{}{}({});\n", name, id, args);

    for (id, rule) in rules.iter().enumerate().skip(1) {
        /*if rule.is_recursive() {
            let args = rule_argument_datanames(&rule, &existing_rules)
                .into_iter()
            //    .chain(vec![("&rel".to_owned())].into_iter())
                .fold("".to_owned(), |s, a| s + &a + ", ");
            fn_code += &format!("    let variable = iteration.variable(\"variable\");\n");
            fn_code += &format!("    variable.insert(rel.into());\n");
            fn_code += &format!("    while iteration.changed() {{\n");
            fn_code += &format!("        variable.insert(rel.merge(rule_{}{}({})));\n", name, id, args);
            fn_code += &format!("    }}\n");
            fn_code += &format!("    let rel = rel.merge(rule_{}{}({}));\n", name, id, args);
        }
        else {*/
        let arg_names = rule_argument_datanames(&rule, &existing_rules);
        let temp_var_decl = if arg_names.contains(&name.to_string() /* why */) {
            format!("    let {}: Relation<_> = rel.iter().map(|x| x.clone()).collect::<Vec<_>>().into();", name)
        } else {
            "".to_owned()
        };
        let args = arg_names
            .into_iter()
            .map(|n| if n == name { "&".to_owned() + &n } else { n })
            .fold("".to_owned(), |s, a| s + &a + ", ");
        //fn_code += &format!("    let temp = rel.merge(rule_{}{}({}));\n", name, id, args);
        fn_code += &temp_var_decl;
        fn_code += &format!("    let rel = rel.merge(rule_{}{}({}));\n", name, id, args);
        /*}*/
    }
    fn_code += "    rel\n}\n";

    fn_code + &code
}

fn rule_argument_datanames(rule: &Rule, natives: &BTreeMap<String, RelationInfo>) -> Vec<String> {
    let mut datanames: Vec<String> = Vec::new();
    let mut dedup_facts = rule.facts.clone();
    dedup_facts.sort_unstable_by(|(l, _negated_l), (r, _negated_r)| {
        if l.name < r.name {
            Ordering::Less
        } else if l.name == r.name {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    dedup_facts.dedup_by(|(l, _), (r, _)| l.name == r.name);
    for fact in dedup_facts {
        let types = natives.get(&*fact.0.name);
        if let Some(rel_info) = types {
            datanames.push(rel_info.variable_name.to_string());
        } else {
            //println!("{:?}", natives);
            panic!("unknown relation: {}", fact.0.name);
            //datanames.push("rule_".to_owned() + &fact.name);
        }
    }
    datanames
}

fn rule_arguments(rule: &Rule, _decl: &Decl, natives: &BTreeMap<String, RelationInfo>) -> String {
    let mut args = String::new();
    let mut dedup_facts = rule.facts.clone();
    dedup_facts.sort_unstable_by(|(l, _), (r, _)| {
        if l.name < r.name {
            Ordering::Less
        } else if l.name == r.name {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });
    dedup_facts.dedup_by(|(l, _), (r, _)| l.name == r.name);
    for (fact, _negated) in dedup_facts {
        args += &fact.name;
        args += ": &Relation<(";
        let types = natives.get(&*fact.name);
        if let Some(ref rel_info) = types {
            for t in &rel_info.arg_types {
                args += &t;
                args += ", ";
            }
        } else {
            //println!("{:?}", natives);
            panic!("unknown relation: {}", fact.name);
        }
        args += ")>, ";
    }

    // if the rule is a recursive rule, then add a self argument
    /*if rule.is_recursive() {
        args += "self_rel: &Relation<(";
        for t in &decl.arg_types {
            args += &t;
            args += ", ";
        }
        args += ")>";
    }*/

    args
}

fn compile_rule(
    rule: &Rule,
    decl: &Decl,
    index: usize,
    existing_rules: &BTreeMap<String, RelationInfo>,
) -> String {
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
    let args = rule_arguments(rule, decl, &existing_rules); //.into_iter().fold("".to_owned(), |s, n| s + n + ", ");
    let return_type = decl
        .arg_types
        .iter()
        .fold("Relation<(".to_owned(), |s, t| s + t + ", ")
        + ")>";

    let node = build_join_tree(rule);
    let (rule_code, fact) = compile_join_tree(&node, rule);

    let final_map = if let QueryNode::RecursiveJoin(_, _) = &node {
        format!(
            "|({})| {}",
            fact.args.iter().fold("".to_owned(), |s, t| s + t + ", "),
            rule.args
                .iter()
                .fold("".to_owned(), |s, t| s + "*" + t + ", "),
        )
    } else {
        format!(
            "|({})| ({})",
            fact.args.iter().fold("".to_owned(), |s, t| s + t + ", "),
            rule.args
                .iter()
                .fold("".to_owned(), |s, t| s + "*" + t + ", "),
        )
    };

    //println!("new fact: {:?}", fact);

    format!(
        r#"fn rule_{}{}({}) -> {} {{
    let mut iteration = Iteration::new();
{}
    {}.into_iter().map({}).collect::<Vec<_>>().into()
}}
"#,
        // {}.into_iter().map(|((x,), ())| (*x,)).into()
        rule.name,
        index,
        args,
        return_type,
        rule_code,
        fact.name,
        final_map
    )
}

fn build_join_tree(rule: &Rule) -> QueryNode {
    assert!(rule.facts.len() > 0);

    let mut node = QueryNode::Input(&rule.facts[0].0);

    if rule.facts[0].1 {
        panic!("can't have negated fact as first part of a rule");
    }

    for (fact, negated) in rule.facts.iter().skip(1) {
        if fact.name == rule.name {
            node = QueryNode::RecursiveJoin(box node, box QueryNode::Input(&fact));
        } else {
            node = QueryNode::Join(box node, box QueryNode::Input(&fact), *negated);
        }
    }
    node
}

fn compile_join_tree(node: &QueryNode, rule: &Rule) -> (String, Fact) {
    match node {
        QueryNode::Input(ref name) => (
            "".to_owned(),
            Fact {
                name: name.name.clone(),
                args: name.args.clone(),
            },
        ),
        QueryNode::Join(box left, box right, is_antijoin) => {
            let (left_code, lfact) = compile_join_tree(left, rule);
            let (right_code, rfact) = compile_join_tree(right, rule);

            let (join, joinfact) = if *is_antijoin {
                compile_antijoin(&lfact, &rfact)
            } else {
                compile_join(&lfact, &rfact)
            };

            (left_code + &right_code + &join, joinfact)
        }
        QueryNode::RecursiveJoin(box left, box right) => {
            let (left_code, lfact) = compile_join_tree(left, rule);
            let (right_code, rfact) = compile_join_tree(right, rule);

            let target_fact = Fact {
                name: rule.name.clone(),
                args: rule.args.clone(),
            };

            let (join, joinfact) = compile_recursive_join(&lfact, &rfact, &target_fact);
            //joinfact.name = "var.complete()".to_owned();
            (left_code + &right_code + &join, joinfact)

            /*(format!(r#"
            let var = iteration.variable("var");
            var.insert({}.iter().into());
            while iteration.changed() {{
                {}{}{}
                var.insert(temp_fact);
                println!("one loop iteration");
            }}
            "#, rfact.name, left_code, &right_code, &join),

                     joinfact)*/
        }
    }
}

fn compile_recursive_join(fact1: &Fact, fact2: &Fact, target: &Fact) -> (String, Fact) {
    let overlap = fact1.get_overlapping(fact2);

    let vals1 = fact1
        .args
        .iter()
        .filter(|s| !overlap.contains(&s))
        .collect::<Vec<&String>>();
    let vals2 = fact2
        .args
        .iter()
        .filter(|s| !overlap.contains(s))
        .collect::<Vec<&String>>();

    let map1 = format!(
        "|({})| (({}), ({}))",
        fact1.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        vals1.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let map2 = format!(
        "|({})| (({}), ({}{}))",
        fact2.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        fact2
            .args
            .iter()
            .filter(|s| !overlap.contains(s))
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        vals2.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    /*let new_fact = Fact {
        name: "temp_fact".to_owned(),
        args: overlap.iter()
            .chain(fact1.args.iter().filter(|s| !overlap.contains(s)))
            .chain(fact2.args.iter().filter(|s| !overlap.contains(s)))
            .map(|s| s.clone())
            .collect()
    };*/
    let mut new_fact = target.clone();

    let code = format!(
        r#"
    let var1 = iteration.variable("left");
    var1.insert({}.iter().map({}).collect::<Vec<_>>().into());
    let recursive = iteration.variable("rec");
    recursive.insert({}.iter().map({}).collect::<Vec<_>>().into());
    while iteration.changed() {{
        recursive.from_join(&var1, &recursive, |&({}), &({}), &({})| (({}), ({}{})));
    }}
    let {} = recursive.complete();
    "#,
        fact1.name,
        map1,
        fact2.name,
        map2,
        overlap.iter().fold("".to_owned(), |s, t| s + &t + ", "),
        vals1.iter().fold("".to_owned(), |s, x| s + x + ", "),
        vals2.iter().fold("".to_owned(), |s, x| s + x + ", "),
        //new_fact.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        //target.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        target.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        "",
        "",
        //overlap.iter().fold("".to_owned(), |s, x| s + x + ", "),
        //fact2.args.iter().filter(|s| !overlap.contains(s)).fold("".to_owned(), |s, x| s + x + ", "),
        //target.args.iter().fold("".to_owned(), |s, x| s + x + ", "),

        //(0..overlap.len()).map(|i| "key.".to_owned() + &i.to_string()).fold("(".to_owned(), |s, t| s + &t + ", ") + ")",
        //if vals1.len() > 0 { (0..vals1.len()).map(|i| "val1.".to_owned() + &i.to_string()).fold("(".to_owned(), |s, t| s + &t + ", ") + ")" } else { "".to_owned() },
        //if vals2.len() > 0 { (0..vals2.len()).map(|i| "val2.".to_owned() + &i.to_string()).fold("(".to_owned(), |s, t| s + &t + ", ") + ")" } else { "".to_owned() },
        new_fact.name,
    );

    new_fact.args.push("()".to_owned());
    (code, new_fact)
}

fn compile_join(fact1: &Fact, fact2: &Fact) -> (String, Fact) {
    let overlap = fact1.get_overlapping(fact2);

    let vals1 = fact1
        .args
        .iter()
        .filter(|s| !overlap.contains(&s))
        .collect::<Vec<&String>>();
    let vals2 = fact2
        .args
        .iter()
        .filter(|s| !overlap.contains(s))
        .collect::<Vec<&String>>();

    let map1 = format!(
        "|({})| (({}), ({}))",
        fact1.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        vals1.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let map2 = format!(
        "|({})| (({}), ({}))",
        fact2.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        // fact2.args.iter().filter(|s| !overlap.contains(s)).fold("".to_owned(), |s, x| s + "*" + x + ", ")
        vals2.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let new_fact = Fact {
        name: "temp_fact".to_owned(),
        args: overlap
            .iter()
            .chain(fact1.args.iter().filter(|s| !overlap.contains(s)))
            .chain(fact2.args.iter().filter(|s| !overlap.contains(s)))
            .map(|s| s.clone())
            .collect(),
    };

    let code = format!(
        r#"
    let {} = {{
        let var1 = iteration.variable("left");
        let var2 = iteration.variable("right");
        var1.insert({}.iter().map({}).collect::<Vec<_>>().into());
        var2.insert({}.iter().map({}).collect::<Vec<_>>().into());
        iteration.changed();

        let variable = iteration.variable("join");
        variable.from_join(&var1, &var2, |&key, &val1, &val2| ({}{}{}));
        while iteration.changed() {{}}
        variable.complete()
    }};
    "#,
        new_fact.name,
        fact1.name,
        map1,
        fact2.name,
        map2,
        (0..overlap.len())
            .map(|i| "key.".to_owned() + &i.to_string())
            .fold("".to_owned(), |s, t| s + &t + ", "),
        if vals1.len() > 0 {
            (0..vals1.len())
                .map(|i| "val1.".to_owned() + &i.to_string())
                .fold("".to_owned(), |s, t| s + &t + ", ")
        } else {
            "".to_owned()
        },
        if vals2.len() > 0 {
            (0..vals2.len())
                .map(|i| "val2.".to_owned() + &i.to_string())
                .fold("".to_owned(), |s, t| s + &t + ", ")
        } else {
            "".to_owned()
        },
    );

    (code, new_fact)
}

fn compile_antijoin(fact1: &Fact, fact2: &Fact) -> (String, Fact) {
    let overlap = fact1.get_overlapping(fact2);

    let vals1 = fact1
        .args
        .iter()
        .filter(|s| !overlap.contains(&s))
        .collect::<Vec<&String>>();

    let map1 = format!(
        "|({})| (({}), ({}))",
        fact1.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        vals1.iter().fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let map2 = format!(
        "|({})| ({})",
        fact2.args.iter().fold("".to_owned(), |s, x| s + x + ", "),
        overlap
            .iter()
            .fold("".to_owned(), |s, x| s + "*" + x + ", "),
        // fact2.args.iter().filter(|s| !overlap.contains(s)).fold("".to_owned(), |s, x| s + "*" + x + ", ")
    );

    let new_fact = Fact {
        name: "temp_fact".to_owned(),
        args: overlap
            .iter()
            .chain(fact1.args.iter().filter(|s| !overlap.contains(s)))
            .map(|s| s.clone())
            .collect(),
    };

    let code = format!(
        r#"
    let {} = {{
        let var1 = iteration.variable("left");
        var1.insert({}.iter().map({}).into());
        let var2: Relation<_> = {}.iter().map({}).into();
        iteration.changed();

        let variable = iteration.variable("antijoin");
        variable.from_antijoin(&var1, &var2, |&key, &val1| ({}{}));
        while iteration.changed() {{}}
        variable.complete()
    }};
    "#,
        new_fact.name,
        fact1.name,
        map1,
        fact2.name,
        map2,
        (0..overlap.len())
            .map(|i| "key.".to_owned() + &i.to_string())
            .fold("".to_owned(), |s, t| s + &t + ", "),
        if vals1.len() > 0 {
            (0..vals1.len())
                .map(|i| "val1.".to_owned() + &i.to_string())
                .fold("".to_owned(), |s, t| s + &t + ", ")
        } else {
            "".to_owned()
        },
    );

    (code, new_fact)
}
