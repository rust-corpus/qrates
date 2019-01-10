#![feature(box_syntax, box_patterns, const_string_new)]
#![feature(rustc_private)]
//#![feature(rust_2018_preview)]

extern crate serde;
extern crate serde_json;
extern crate bincode;

extern crate rustql_common;

#[allow(unused_extern_crates)]
extern crate rustc_driver;

#[allow(unused_extern_crates)]
extern crate rustc;

#[allow(unused_extern_crates)]
extern crate rustc_plugin;

#[allow(unused_extern_crates)]
extern crate syntax;

pub mod visitor;

use self::rustc_driver::{driver::CompileController, driver::PhaseController, Compilation};
use self::rustc_driver::driver::CompileState;

use rustql_common::data;

//use rustc_driver::driver::{CompileController, PhaseController, CompileState};
use std::process::{exit, Command};
use crate::syntax::ast::NodeId;
use crate::rustc::hir::map::{Map};
use crate::rustc::hir::def_id::DefId;
use crate::rustc::ty::TyCtxt;
use crate::rustc::hir;
use crate::rustc::hir::intravisit::{NestedVisitorMap, Visitor, walk_crate};
use crate::rustc::hir::intravisit::*;
use crate::syntax::ast::Name;
use crate::syntax::source_map::Span;
use crate::syntax::ast::CRATE_NODE_ID;

use std::collections::BTreeMap;

use std::fs::File;
use std::env;

use std::u64;

use self::visitor::CrateVisitor;

const TARGET_DIR_VARNAME: &str = "EXTRACTOR_TARGET_DIR";
const USE_JSON: bool = false;


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

    //let mut local_config_hash: String = "".to_owned();
    //let h = rustc::session::config::
    for arg in &args {
        if arg.starts_with("metadata=") {
            //println!("{}", arg);
            // hacky but works
            //println!("{}", arg);
            //local_config_hash = arg[arg.char_indices().nth(9).unwrap().0 ..].to_owned();
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
        let hir_map = &tcx.hir();
        let ref krate = hir_map.krate();
        
        // 
        // assume, crate num of 0 means current crate
        //
        let local_config_hash = tcx.crate_hash(hir::def_id::CrateNum::new(0)).to_string();
        let mut cv = CrateVisitor {
            crate_data: data::Crate::new(crate_name, crate_version, &local_config_hash),
            current_function: None,
//            crate_name: crate_name,
            map: hir_map,
            tcx: *tcx,
            local_modules: BTreeMap::new()
        };

        // add root module
        //cv.visit_mod(&krate.module, krate.span, CRATE_NODE_ID);
        cv.crate_data.mods.push(
            data::Mod {
                name: crate_name.to_owned(),
                parent_mod: None
            }
        );

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


fn export_crate(krate: &data::Crate) -> Option<()> {
    let filename = krate.metadata.get_filename();
    let dirname = env::var(TARGET_DIR_VARNAME).unwrap_or(env::var("HOME").unwrap_or("/".to_owned()) + "/.rustql/crates");
    std::fs::create_dir_all(&dirname);

    if USE_JSON { 
        File::create(dirname.clone() + "/" + &filename + ".json").ok().and_then(|file| -> Option<()> {
            serde_json::to_writer_pretty(file, krate).unwrap();
            Some(())
        }).or(None)
    }
    else {
        File::create(dirname + "/" + &filename + ".bin").ok().and_then(|file| -> Option<()> {
            bincode::serialize_into(file, krate).unwrap();
            Some(())
        }).or(None)
    }
}


