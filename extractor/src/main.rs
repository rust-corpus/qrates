// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_interface;

use corpus_extractor::{analyse, override_queries, save_cfg_configuration};
use rustc_driver::Compilation;
use rustc_interface::{
    interface::{Compiler, Config},
    Queries,
};
use std::process;

struct CorpusCallbacks {}

impl rustc_driver::Callbacks for CorpusCallbacks {
    fn config(&mut self, config: &mut Config) {
        save_cfg_configuration(&config.crate_cfg);
        config.override_queries = Some(override_queries);
    }

    fn after_analysis<'tcx>(
        &mut self,
        compiler: &Compiler,
        queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        analyse(compiler, queries);
        Compilation::Continue
    }
}

fn main() {
    rustc_driver::init_rustc_env_logger();
    let mut callbacks = CorpusCallbacks {};
    let exit_code = rustc_driver::catch_with_exit_code(|| {
        use std::env;
        let mut is_color_arg = false;
        let mut args = env::args()
            .filter(|arg| {
                if arg == "--color" {
                    is_color_arg = true;
                    false
                } else if is_color_arg {
                    is_color_arg = false;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

        args.push("--sysroot".to_owned());
        args.push(std::env::var("SYSROOT").expect("Please specify the SYSROOT env variable."));
        args.splice(
            1..1,
            [
                "-Zalways-encode-mir",
                "-Zmir-opt-level=0",
                "-Cdebug-assertions=on",
            ]
            .iter()
            .map(ToString::to_string),
        );
        rustc_driver::RunCompiler::new(&args, &mut callbacks).run()
    });
    process::exit(exit_code);
}
