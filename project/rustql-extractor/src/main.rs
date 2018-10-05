#![feature(box_syntax, box_patterns, const_string_new)]
#![feature(rustc_private)]
//#![feature(rust_2018_preview)]

extern crate serde;
extern crate serde_json;
extern crate bincode;

extern crate rustql_common;

#[macro_use]
extern crate serde_derive;

#[allow(unused_extern_crates)]
extern crate rustc_driver;

#[allow(unused_extern_crates)]
extern crate rustc;

#[allow(unused_extern_crates)]
extern crate rustc_plugin;

#[allow(unused_extern_crates)]
extern crate syntax;

use self::rustc_driver::{driver::CompileController, driver::PhaseController, Compilation};
use self::rustc_driver::driver::CompileState;

use rustql_common::data;

//use rustc_driver::driver::{CompileController, PhaseController, CompileState};
use std::process::{exit, Command};
use crate::syntax::ast::NodeId;
use crate::rustc::hir::map::{Map};
use crate::rustc::ty::TyCtxt;
use crate::rustc::hir;
use crate::rustc::hir::intravisit::{NestedVisitorMap, Visitor, walk_crate};
use crate::rustc::hir::intravisit::*;
use crate::syntax::ast::Name;
use crate::syntax::source_map::Span;

use std::fs::File;
use std::env;

use std::u64;

const TARGET_DIR_VARNAME: &str = "EXTRACTOR_TARGET_DIR";
const USE_JSON: bool = true;

//pub mod data;

fn main() {
exit(rustc_driver::run(move || {
    // =========================
    // mostly copied from clippy
    // =========================

    let sys_root = option_env!("SYSROOT")
        .map(String::from)
        .or_else(|| std::env::var("SYSROOT").ok())
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
            let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
            home.and_then(|home| toolchain.map(|toolchain| format!("{}/toolchains/{}", home, toolchain)))
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| s.trim().to_owned())
        })
        .expect("need to specify SYSROOT env var during clippy compilation, or use rustup or multirust");
        

    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we ignore this/
    let mut orig_args: Vec<String> = env::args().collect();
    if orig_args.len() <= 1 {
        std::process::exit(1);
    }
    if orig_args[1] == "rustc" {
        // we still want to be able to invoke it normally though
        orig_args.remove(1);
    }

    // this conditional check for the --sysroot flag is there so users can call
    // `clippy_driver` directly
    // without having to pass --sysroot or anything
    let args: Vec<String> = if orig_args.iter().any(|s| s == "--sysroot") {
        orig_args.clone()
    } else {
        orig_args
            .clone()
            .into_iter()
            .chain(Some("--sysroot".to_owned()))
            .chain(Some(sys_root))
            .collect()
    };
    
    if !args.iter().any(|s| s.starts_with("--print=")) {
        // may print arbitrary stuff here
        //println!("{:?}", orig_args);
    }

    let mut local_config_hash: String = "".to_owned();
    //let h = rustc::session::config::
    for arg in &args {
        if arg.starts_with("metadata=") {
            println!("{}", arg);
            // hacky but works
            local_config_hash = arg[arg.char_indices().nth(9).unwrap().0 ..].to_owned();
            //local_config_hash = u64::from_str_radix(hash, 16).unwrap();
        }
    }


    let mut controller = CompileController::basic();
    controller.keep_ast = true;

    controller.after_analysis.callback = box |cs: &mut CompileState| {
        let crate_name_env = env::var("CARGO_PKG_NAME").unwrap_or_default();
        let crate_version = (
            env::var("CARGO_PKG_VERSION_MAJOR").unwrap_or_default().parse::<u64>().unwrap(),
            env::var("CARGO_PKG_VERSION_MINOR").unwrap_or_default().parse::<u64>().unwrap(),
            env::var("CARGO_PKG_VERSION_PATCH").unwrap_or_default().parse::<u64>().unwrap()
        );
        let crate_name = cs.crate_name.unwrap();
        if crate_name_env != crate_name {
            // happens when the crate name contains a '-', this will then get
            // renamed to a '_' to become a rust identifier.

            //println!("\x1b[31mdifferent names!: {}, {}\x1b[0m", crate_name_env, crate_name);
        }
        let tcx = &cs.tcx.expect("no valid tcx");
        let hir_map = &tcx.hir;
        let ref krate = hir_map.krate();
        let mut cv = CrateVisitor {
            crate_data: data::Crate::new(crate_name, crate_version, &local_config_hash),
            current_function: None,
//            crate_name: crate_name,
            map: hir_map,
            tcx: *tcx
        };

        walk_crate(&mut cv, krate);

        //println!("{:?}", cv.crate_data);
        let result = export_crate(&cv.crate_data);
        if let None = result {
            println!("ERROR exporting crate: {}", cv.crate_data.metadata.get_filename());
        }

        /*let mut clv = CrateLikeVisitor {
            map: hir_map
        };
        krate.visit_all_item_likes(&mut clv);*/
    };


    rustc_driver::run_compiler(&args, Box::new(controller), None, None)
}) as i32)
}


struct CrateVisitor<'tcx, 'a>
{
    crate_data: data::Crate,
    current_function: Option<data::Function>,
    map: &'a Map<'tcx>,
    tcx: TyCtxt<'a, 'tcx, 'tcx>
}

impl<'tcx, 'a> Visitor<'tcx> for CrateVisitor<'tcx, 'a> {
    fn visit_mod(&mut self, m: &'tcx hir::Mod, _s: Span, id: NodeId) {
        let maybe_node = self.map.find(id);
        if let Some(hir::Node::Item(item)) = maybe_node {
            let name: &str = &item.name.as_str();
            let path = self.map.def_path(self.map.local_def_id(id));
            //let parent_path = data::GlobalDefPath::from_def_path_of_mod(&path).remove_last_segment();
            //let parent_id = self.crate_data.get_mod_id(&parent_path);

            self.crate_data.mods.push(data::Mod {
                name: String::from(name),
                parent_mod: Some(data::GlobalDefPath::new(&path, &self.crate_data.metadata ))
            });
            /*println!("{:?}", data::UniqueIdentifier::from_def_path_of_mod(&path));
            
            let print_path = self.crate_data.get_mod_id(&data::UniqueIdentifier::from_def_path_of_mod(&path));
            let modul = &self.crate_data.mods[print_path.unwrap_or(0)];
            println!("{:?}, {:?}", print_path, modul);*/
        }
        else {
            //println!("visited mod that is not an item: {:?}", maybe_node);
        }
        
        walk_mod(self, m, id);

        let mut func: Option<data::Function> = None;
        std::mem::swap(&mut self.current_function, &mut func);
        if let Some(f) = func {
            self.crate_data.functions.push(f);
        }
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        /*match &item.node {
            hir::ItemKind::Mod(_m) => {
                println!("module: {}", item.name);
            }
            _ => {}
        }*/
        walk_item(self, item);
    }

    fn visit_fn(&mut self, fk: FnKind<'tcx>, fd: &'tcx hir::FnDecl, b: hir::BodyId, s: Span, id: NodeId) {
        let def_id = self.map.local_def_id(id);
        let def = self.tcx.absolute_item_path_str(def_id);
        //println!("def_id: {}", def);

        let def_path = self.map.def_path_from_id(id).unwrap();
        //let mod_path = data::GlobalDefPath::from_def_path_of_mod(&def_path);
        //let mod_id = self.crate_data.get_mod_id(&mod_path);

        let maybe_node = self.map.find(id);
        if let Some(hir::Node::Item(item)) = maybe_node {
            match fk {
                FnKind::Method(name, method_sig, vis, attr) => {
                    let mut func = Option::Some(data::Function {
                        name: item.name.to_string(),
                        is_unsafe: method_sig.header.unsafety == rustc::hir::Unsafety::Unsafe,
                        calls: vec![], // TODO implement
                        containing_mod: Some(data::GlobalDefPath::new(&def_path, &self.crate_data.metadata)),
                        //def_id: //data::DefIdWrapper(def_id)
                    });

                    std::mem::swap(&mut self.current_function, &mut func);
                    if let Some(f) = func {
                        self.crate_data.functions.push(f);
                    }
                },
                FnKind::Closure(_) => {},
                FnKind::ItemFn(name, generics, header, vis, block) => {
                    let mut func = Option::Some(data::Function {
                        name: item.name.to_string(),
                        is_unsafe: header.unsafety == rustc::hir::Unsafety::Unsafe,
                        calls: vec![], // TODO implement
                        //containing_mod: Some(def_path),
                        containing_mod: Some(data::GlobalDefPath::new(&def_path, &self.crate_data.metadata)),
                        //def_id: //data::DefIdWrapper(def_id)
                    });

                    std::mem::swap(&mut self.current_function, &mut func);
                    if let Some(f) = func {
                        self.crate_data.functions.push(f);
                    }
                }
            };
        }
        walk_fn(self, fk, fd, b, s, id);
    }

    fn visit_body(&mut self, body: &'tcx hir::Body) {
        let id = body.id();
        let owner = self.map.body_owner_def_id(id);
        /*if let Some(function) = self.crate_data.get_function(owner) {
            
            //println!("found body of {:?}: {:?}", function, owner);
        }*/
        //println!("found body of {:?}: {:?}", function, owner);
        walk_body(self, body);
    }

    fn visit_expr(&mut self, expr: &'tcx hir::Expr) {
        if let hir::ExprKind::Call(target, args) = &expr.node {
            //println!("found call to: {:?}", target);
            use self::hir::*;
            use self::hir::def::Def;
            let target_kind = &target.node;
            if let ExprKind::Path(QPath::Resolved(_, p)) = target_kind {
                if let Some(ref mut f) = self.current_function {
                    //let def_id = std::panic::catch_unwind(|| p.def.def_id());
                    let def_id = match p.def {
                        Def::Fn(id) |
                        Def::Variant(id) |
                        Def::StructCtor(id, _) |
                        Def::VariantCtor(id, _) |
                        Def::SelfCtor(id) |
                        Def::Method(id)
                        => Some(id),
                        Def::Local(node_id) |
                        Def::Upvar(node_id, _, _)
                        => { Some(self.map.local_def_id(node_id)) },
                        _ => { println!("unknown call to {}", p.def.kind_name()); None }
                    };

                    if let Some(id) = def_id {
                        if (id.is_local()) {
                            f.calls.push(data::GlobalDefPath::new(&self.map.def_path(id), &self.crate_data.metadata));
                        }
                    }
                    //f.calls.push(data::GlobalDefPath{ path: p.path, crate_ident: self.crate_data.metadata });
                }
                //println!("def id: {:?}", p.def);
                /*if let Some(id) = self.crate_data.get_function(p.def.def_id()) {
                    println!("found func: {:?} ", id);
                }*/
            }
        }
        walk_expr(self, expr);
    }

    fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
        NestedVisitorMap::All(self.map)
        //NestedVisitorMap::None
    }
}

fn export_crate(krate: &data::Crate) -> Option<()> {
    let filename = krate.metadata.get_filename();
    let dirname = env::var(TARGET_DIR_VARNAME).unwrap_or(env::var("HOME").unwrap_or("/".to_owned()) + "/.rustql/crates");
    File::create(dirname + "/" + &filename + ".json").ok().and_then(|file| -> Option<()> {
        if USE_JSON {
            serde_json::to_writer_pretty(file, krate).unwrap();
        }
        else {
            bincode::serialize_into(file, krate).unwrap();
        }
        Some(())
    }).or(None)
}


