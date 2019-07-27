// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.
//
// The driver code is based on the Clippy driver code
// https://github.com/rust-lang/rust-clippy/blob/master/src/driver.rs.

#![feature(box_syntax, box_patterns, const_string_new)]
#![feature(rustc_private)]

extern crate bincode;
extern crate serde;
extern crate serde_json;

extern crate rustql_common;

#[allow(unused_extern_crates)]
extern crate rustc;

#[allow(unused_extern_crates)]
extern crate rustc_driver;

#[allow(unused_extern_crates)]
extern crate rustc_interface;

#[allow(unused_extern_crates)]
extern crate rustc_plugin;

#[allow(unused_extern_crates)]
extern crate syntax;

pub mod visitor;

use rustql_common::data;

//use rustc_driver::driver::{CompileController, PhaseController, CompileState};
use crate::rustc::hir;
use crate::rustc::hir::intravisit::walk_crate;
use rustc_driver::Compilation;
use rustc_interface::interface;
use rustc_tools_util::*;
use std::path::{Path, PathBuf};
use std::process::{exit, Command};

use std::collections::BTreeMap;

use std::fs::File;

use std::u64;
use std::env;

use self::visitor::CrateVisitor;

const TARGET_DIR_VARNAME: &str = "EXTRACTOR_TARGET_DIR";
const USE_JSON: bool = false;

struct Callbacks;

/// If a command-line option matches `find_arg`, then apply the predicate `pred` on its value. If
/// true, then return it. The parameter is assumed to be either `--arg=value` or `--arg value`.
fn arg_value<'a>(
    args: impl IntoIterator<Item = &'a String>,
    find_arg: &str,
    pred: impl Fn(&str) -> bool,
) -> Option<&'a str> {
    let mut args = args.into_iter().map(String::as_str);

    while let Some(arg) = args.next() {
        let arg: Vec<_> = arg.splitn(2, '=').collect();
        if arg.get(0) != Some(&find_arg) {
            continue;
        }

        let value = arg.get(1).cloned().or_else(|| args.next());
        if value.as_ref().map_or(false, |p| pred(p)) {
            return value;
        }
    }
    None
}

impl rustc_driver::Callbacks for Callbacks {
    fn after_analysis(&mut self, compiler: &interface::Compiler) -> Compilation {
        compiler
            .global_ctxt()
            .unwrap()
            .peek_mut()
            .enter(|tcx| {
                let crate_name_env = env::var("CARGO_PKG_NAME").unwrap_or(String::from("main"));
                let crate_version = (
                    env::var("CARGO_PKG_VERSION_MAJOR")
                        .unwrap_or(String::from("0"))
                        .parse::<u64>()
                        .unwrap(),
                    env::var("CARGO_PKG_VERSION_MINOR")
                        .unwrap_or(String::from("0"))
                        .parse::<u64>()
                        .unwrap(),
                    env::var("CARGO_PKG_VERSION_PATCH")
                        .unwrap_or(String::from("0"))
                        .parse::<u64>()
                        .unwrap(),
                );

                let crate_name = compiler.crate_name().unwrap().take();
                if crate_name_env != crate_name {
                    // happens when the crate name contains a '-', this will then get
                    // renamed to a '_' to become a rust identifier.

                    //println!("\x1b[31mdifferent names!: {}, {}\x1b[0m", crate_name_env, crate_name);
                }

                let hir_map = &tcx.hir();
                let ref krate = hir_map.krate();

                //
                // assume, crate num of 0 means current crate
                //
                let config_hash: String = tcx.crate_hash(hir::def_id::CrateNum::new(0)).to_string();
                let mut cv = CrateVisitor {
                    crate_data: data::Crate::new(&crate_name, crate_version, &config_hash),
                    current_function: None,
                    // crate_name: crate_name,
                    map: hir_map,
                    tcx: tcx,
                    local_modules: BTreeMap::new(),
                };

                // add root module
                // cv.visit_mod(&krate.module, krate.span, CRATE_NODE_ID);
                cv.crate_data.mods.push(data::Mod {
                    name: crate_name.to_owned(),
                    parent_mod: None,
                });

                walk_crate(&mut cv, krate);

                //println!("{:?}", cv.crate_data);
                let result = export_crate(&cv.crate_data);
                if let None = result {
                    println!(
                        "ERROR exporting crate: {}",
                        cv.crate_data.metadata.get_filename()
                    );
                }
            });
        Compilation::Stop
    }
}

fn main() {
    rustc_driver::init_rustc_env_logger();
    if std::env::args().any(|a| a == "--version" || a == "-V") {
        let version_info = rustc_tools_util::get_version_info!();
        println!("{}", version_info);
        exit(0);
    }

    let mut orig_args: Vec<String> = env::args().collect();

    // Get the sysroot, looking from most specific to this invocation to the least:
    // - command line
    // - runtime environment
    //    - SYSROOT
    //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
    // - sysroot from rustc in the path
    // - compile-time environment
    let sys_root_arg = arg_value(&orig_args, "--sysroot", |_| true);
    let have_sys_root_arg = sys_root_arg.is_some();
    let sys_root = sys_root_arg
        .map(PathBuf::from)
        .or_else(|| std::env::var("SYSROOT").ok().map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
            let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
            home.and_then(|home| {
                toolchain.map(|toolchain| {
                    let mut path = PathBuf::from(home);
                    path.push("toolchains");
                    path.push(toolchain);
                    path
                })
            })
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .map(|pb| pb.to_string_lossy().to_string())
        .expect("need to specify SYSROOT env var during clippy compilation, or use rustup or multirust");

    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we ignore this
    if orig_args.len() > 1
        && Path::new(&orig_args[1]).file_stem() == Some("rustc".as_ref())
    {
        // we still want to be able to invoke it normally though
        orig_args.remove(1);
    }

    // this conditional check for the --sysroot flag is there so users can call
    // `clippy_driver` directly
    // without having to pass --sysroot or anything
    let args: Vec<String> = if have_sys_root_arg {
        orig_args.clone()
    } else {
        orig_args
            .clone()
            .into_iter()
            .chain(Some("--sysroot".to_owned()))
            .chain(Some(sys_root))
            .collect()
    };

    let result = rustc_driver::report_ices_to_stderr_if_any(move || {
        let mut callbacks = Callbacks;
        rustc_driver::run_compiler(&args, &mut callbacks, None, None)
    })
    .and_then(|result| result);
    exit(result.is_err() as i32);
}

fn export_crate(krate: &data::Crate) -> Option<()> {
    let filename = krate.metadata.get_filename();
    let dirname = env::var(TARGET_DIR_VARNAME)
        .unwrap_or(env::var("HOME").unwrap_or("/".to_owned()) + "/.rustql/crates");
    std::fs::create_dir_all(&dirname).ok();     // Discard the result.

    if USE_JSON {
        File::create(dirname.clone() + "/" + &filename + ".json")
            .ok()
            .and_then(|file| -> Option<()> {
                serde_json::to_writer_pretty(file, krate).unwrap();
                Some(())
            })
            .or(None)
    } else {
        File::create(dirname + "/" + &filename + ".bin")
            .ok()
            .and_then(|file| -> Option<()> {
                bincode::serialize_into(file, krate).unwrap();
                Some(())
            })
            .or(None)
    }
}
