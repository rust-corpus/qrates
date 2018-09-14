#![feature(box_syntax)]
#![feature(rustc_private)]
//#![feature(rust_2018_preview)]


use rustc_driver::{self, Compilation};
use rustc_driver::driver::{CompileController, PhaseController, CompileState};
use std::process::{exit, Command};
use syntax::ast::NodeId;
use rustc::hir::map::{Map};
use rustc::hir;
use rustc::hir::intravisit::{NestedVisitorMap, Visitor, walk_crate};
use rustc::hir::intravisit::*;
use rustc::hir::itemlikevisit::ItemLikeVisitor;
use syntax::ast::Name;
use syntax::source_map::Span;
use std::env;


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
    


    let mut controller = CompileController::basic();
    controller.keep_ast = true;

    controller.after_analysis.callback = box |cs: &mut CompileState| {
        let crate_name_env = env::var("CARGO_PKG_NAME").unwrap_or_default();
        let crate_version_env = env::var("CARGO_PKG_VERSION").unwrap_or_default();
        let crate_name = cs.crate_name.unwrap();
        let tcx = &cs.tcx.expect("no valid tcx");
        let hir_map = &tcx.hir;
        let ref krate = hir_map.krate();
        let mut cv = CrateVisitor {
            crate_name: crate_name,
            map: hir_map
        };

        walk_crate(&mut cv, krate);

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
    crate_name: &'a str,
    map: &'tcx Map<'tcx>
}

impl<'tcx, 'a> Visitor<'tcx> for CrateVisitor<'tcx, 'a> {
    fn visit_name(&mut self, _: Span, name: Name) {
    }

    fn visit_item(&mut self, item: &'tcx hir::Item) {
        walk_item(self, item);
    }

    fn visit_fn(&mut self, fk: FnKind<'tcx>, fd: &'tcx hir::FnDecl, b: hir::BodyId, s: Span, id: NodeId) {
        //println!("visited function in crate {} {} {:?} {:?}", self.crate_name, self.map.def_path_from_id(id).expect("invalid node traversed").to_string_no_crate(), fd, id);
        println!("visited function in crate {} {:?} {:?} {:?}", self.crate_name, self.map.def_path_from_id(id).expect("invalid node traversed").data, fd, id);
        walk_fn(self, fk, fd, b, s, id);
    }

    fn nested_visit_map<'this>(&'this mut self) -> NestedVisitorMap<'this, 'tcx> {
        NestedVisitorMap::All(self.map)
        //NestedVisitorMap::None
    }
}
