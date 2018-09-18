#![feature(box_syntax)]
#![feature(rustc_private)]
//#![feature(rust_2018_preview)]

//extern crate serde;
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


//use rustc_driver::driver::{CompileController, PhaseController, CompileState};
use std::process::{exit, Command};
use crate::syntax::ast::NodeId;
use crate::rustc::hir::map::{Map};
use crate::rustc::hir;
use crate::rustc::hir::intravisit::{NestedVisitorMap, Visitor, walk_crate};
use crate::rustc::hir::intravisit::*;
use crate::rustc::hir::itemlikevisit::ItemLikeVisitor;
use crate::syntax::ast::Name;
use crate::syntax::source_map::Span;
use std::env;


pub mod data;


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
        // may print stuff
    }


    let mut controller = CompileController::basic();
    controller.keep_ast = true;

    controller.after_analysis.callback = box |cs: &mut CompileState| {
        let crate_name_env = env::var("CARGO_PKG_NAME").unwrap_or_default();
        let crate_version_env = env::var("CARGO_PKG_VERSION").unwrap_or_default();
        let crate_name = cs.crate_name.unwrap();
        if crate_name_env != crate_name {
            println!("\x1b[31mdifferent names!: {}, {}\x1b[0m", crate_name_env, crate_name);
        }
        let tcx = &cs.tcx.expect("no valid tcx");
        let hir_map = &tcx.hir;
        let ref krate = hir_map.krate();
        let mut cv = CrateVisitor {
            crate_data: data::Crate::new(crate_name, (0, 0, 0)),
            crate_name: crate_name,
            map: hir_map
        };

        walk_crate(&mut cv, krate);

        println!("{:?}", cv.crate_data);

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
    crate_name: &'a str,
    map: &'tcx Map<'tcx>
}

impl<'tcx, 'a> Visitor<'tcx> for CrateVisitor<'tcx, 'a> {
    fn visit_name(&mut self, _: Span, name: Name) {
    }

    fn visit_mod(&mut self, m: &'tcx hir::Mod, _s: Span, id: NodeId) {
        let maybe_node = self.map.find(id);
        if let Some(hir::Node::Item(item)) = maybe_node {
            let name: &str = &item.name.as_str();
            let path = self.map.def_path(self.map.local_def_id(id));
            self.crate_data.mods.push(data::Mod {
                name: String::from(name),
                functions: vec![],
                parent_mod_id: Some(0) // TODO get this index
            });
            println!("{:?}", data::UniqueIdentifier::from_def_path_of_mod(&path));
        }
        else if let Some(hir::Node::ForeignItem(item)) = maybe_node {
            //let name: &str = &item.name.as_str();
            //println!("visited foreign mod: {:?}", maybe_node);
        }
        else {
            //println!("visited mod that is not an item: {:?}", maybe_node);
        }
        
        walk_mod(self, m, id);
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        /*match &item.node {
            hir::ItemKind::Mod(_m) => {
                println!("module: {}", item.name);
            },
            _ => {}
        }*/
        walk_item(self, item);
    }

    fn visit_fn(&mut self, fk: FnKind<'tcx>, fd: &'tcx hir::FnDecl, b: hir::BodyId, s: Span, id: NodeId) {
        //println!("visited function in crate {} {} {:?} {:?}", self.crate_name, self.map.def_path_from_id(id).expect("invalid node traversed").to_string_no_crate(), fd, id);
        //println!("visited function in crate {} {:?} {:?} {:?}", self.crate_name, self.map.def_path_from_id(id).expect("invalid node traversed").data, fd, id);
        walk_fn(self, fk, fd, b, s, id);
    }

    fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
        NestedVisitorMap::All(self.map)
        //NestedVisitorMap::None
    }
}
