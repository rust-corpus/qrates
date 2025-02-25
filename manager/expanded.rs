#![feature(prelude_import)]
//! Library for managing crate sources.
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod compilation {
    //! Module responsible for compiling crates.
    use super::sources_list::Crate as CrateInfo;
    use crate::sources_list::CratesList;
    use log::LevelFilter;
    use log::{error, info};
    use log_derive::logfn;
    use rustwide::logging::{self, LogStorage};
    use rustwide::{cmd::SandboxBuilder, Crate, Toolchain, Workspace, WorkspaceBuilder};
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::Duration;
    use tempfile::TempDir;
    pub struct CompileManager {
        /// The list of crates we want to compile.
        crates_list: CratesList,
        /// The rustwide workspace.
        workspace: PathBuf,
        /// The Rust toolchain to use for building.
        toolchain: String,
        /// Maximum log size for a build before it gets truncated.
        max_log_size: usize,
        /// The memory limit that is set while building a crate.
        memory_limit: Option<usize>,
        /// The timeout for the build.
        timeout: Option<Duration>,
        /// Should the network be enabled while building a crate?
        enable_networking: bool,
        /// Should the extractor output also json, or only bincode?
        output_json: bool,
        /// Should we use the normal rustc for compilation instead of the extractor?
        use_original_rustc: bool,
        /// Should we purge the build directory before trying to compile a crate?
        ///
        /// This may significantly slowdown the compilation because it does not use
        /// the cache. However, it is sometimes necessary when recompiling crates.
        purge_build_dir: bool,
        /// Path to the extractor.
        extractor_path: PathBuf,
        /// The path where to put all extracted files.
        extracted_files_path: PathBuf,
        /// Should we use the custom cargo registry?
        custom_registry: Option<String>,
    }
    impl CompileManager {
        pub fn new(
            crates_list: CratesList,
            workspace: &Path,
            toolchain: String,
            max_log_size: usize,
            memory_limit: Option<usize>,
            timeout: Option<Duration>,
            enable_networking: bool,
            output_json: bool,
            use_original_rustc: bool,
            purge_build_dir: bool,
            custom_registry: Option<String>,
        ) -> Self {
            let out_dir: PathBuf = "/home/nius/eth/as2024/sem-project/v2-qrates/qrates/target/debug/build/corpus-manager-c9b0d249b7d04f79/out"
                .into();
            let extractor_path = out_dir
                .join("../../../rustc")
                .canonicalize()
                .expect("Could not find the extractor.");
            let workspace_canonical = workspace
                .canonicalize()
                .expect("Failed to convert the workspace path to absolute.");
            let extracted_files_path = workspace_canonical.join("rust-corpus");
            Self {
                crates_list,
                workspace: workspace_canonical,
                toolchain,
                max_log_size,
                memory_limit,
                timeout,
                enable_networking,
                output_json,
                use_original_rustc,
                purge_build_dir,
                extractor_path,
                extracted_files_path,
                custom_registry,
            }
        }
        fn prepare_custom_registry(&self) -> Result<(), Box<dyn std::error::Error>> {
            let result = (move || {
                if let Some(registry_url) = &self.custom_registry {
                    let cargo_config_path = self
                        .workspace
                        .join("cargo-home/config.toml");
                    let mut cargo_config = {
                        let table = ::toml::value::Table::new();
                        let mut root = ::toml::Value::Table(table);
                        ::toml::macros::insert_toml(
                            &mut root,
                            &[&"-source"[1..], &"-crates-io"[1..]],
                            ::toml::Value::Table(::toml::value::Table::new()),
                        );
                        {
                            ::toml::macros::insert_toml(
                                &mut root,
                                &[&"-source"[1..], &"-crates-io"[1..], &"-registry"[1..]],
                                {
                                    let de = ::toml::macros::IntoDeserializer::<
                                        ::toml::de::Error,
                                    >::into_deserializer("TODO");
                                    <::toml::Value as ::toml::macros::Deserialize>::deserialize(
                                            de,
                                        )
                                        .unwrap()
                                },
                            );
                        };
                        match root {
                            ::toml::Value::Table(table) => table,
                            _ => {
                                ::core::panicking::panic(
                                    "internal error: entered unreachable code",
                                )
                            }
                        }
                    };
                    let section = cargo_config.get_mut("source").unwrap();
                    match section {
                        toml::Value::Table(source) => {
                            match &mut source["crates-io"] {
                                toml::Value::Table(crates_io) => {
                                    crates_io["registry"] = toml::Value::String(
                                        registry_url.to_string(),
                                    );
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                        _ => {
                            ::core::panicking::panic(
                                "internal error: entered unreachable code",
                            )
                        }
                    }
                    std::fs::write(cargo_config_path, &cargo_config.to_string())?;
                }
                Ok(())
            })();
            result
                .map(|result| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("prepare_custom_registry() => {0:?}", result),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    result
                })
                .map_err(|err| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("prepare_custom_registry() => {0:?}", err),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    err
                })
        }
        fn compile_stdlib(&self) -> Result<(), Box<dyn std::error::Error>> {
            let result = (move || {
                let dest_parent_path = self
                    .workspace
                    .join("cargo-home/sysroot/lib/rustlib/x86_64-unknown-linux-gnu");
                if dest_parent_path.exists() {
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("The standard library is already built."),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    return Ok(());
                }
                let tmp_dir = TempDir::new()?;
                let tmp_dir = tmp_dir.path();
                let cargo_toml = tmp_dir.join("Cargo.toml");
                std::fs::write(
                    &cargo_toml,
                    r#"
        [package]
        name = "corpus-stdlib"
        version = "0.0.0"
        "#,
                )?;
                std::fs::create_dir(tmp_dir.join("src"))?;
                std::fs::write(tmp_dir.join("src/lib.rs"), "")?;
                let cargo = std::env::var("CARGO").unwrap_or("cargo".to_string());
                let mut cmd = Command::new(cargo);
                cmd.args(
                    &[
                        "build",
                        "--release",
                        "-Z",
                        "build-std",
                        "--target",
                        "x86_64-unknown-linux-gnu",
                    ],
                );
                cmd.arg("--manifest-path");
                cmd.arg(cargo_toml);
                cmd.env("RUST_BACKTRACE", "1");
                if !self.use_original_rustc {
                    let sysroot = ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}/toolchains/{1}",
                                "/home/nius/.rustup",
                                "nightly-2024-11-11-x86_64-unknown-linux-gnu",
                            ),
                        );
                        res
                    });
                    cmd.env("SYSROOT", sysroot)
                        .env("RUSTC", &self.extractor_path)
                        .env(
                            "CORPUS_RESULTS_DIR",
                            self.extracted_files_path.join("stdlib"),
                        );
                }
                let status = cmd.status().expect("failed to execute process");
                if !status.success() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Failed to compile stdlib."),
                        );
                    }
                }
                let lib_path = tmp_dir
                    .join("target/x86_64-unknown-linux-gnu/release/deps");
                let dest_path = dest_parent_path.join("lib");
                std::fs::create_dir_all(&dest_parent_path)?;
                let mut cp = Command::new("cp");
                cp.arg("-r");
                cp.arg(&lib_path);
                cp.arg(&dest_path);
                cp.status().expect("failed to execute cp");
                if !status.success() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "couldn\'t copy \'{0}\' to \'{1}\'",
                                lib_path.display(),
                                dest_path.display(),
                            ),
                        );
                    }
                }
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("The standard library successfully built."),
                            lvl,
                            &(
                                "corpus_manager::compilation",
                                "corpus_manager::compilation",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                Ok(())
            })();
            result
                .map(|result| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("compile_stdlib() => {0:?}", result),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    result
                })
                .map_err(|err| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("compile_stdlib() => {0:?}", err),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    err
                })
        }
        pub fn compile_all(&self) -> Result<(), Box<dyn std::error::Error>> {
            let result = (move || {
                let cargo_config_path = self.workspace.join("cargo-home/config.toml");
                if cargo_config_path.exists() {
                    std::fs::remove_file(cargo_config_path)?;
                }
                self.compile_stdlib()?;
                let workspace = WorkspaceBuilder::new(&self.workspace, "rust-corpus")
                    .init()?;
                let toolchain = Toolchain::dist(&self.toolchain);
                toolchain.install(&workspace)?;
                toolchain.add_component(&workspace, "rustc-dev")?;
                if !self.use_original_rustc {
                    self.copy_extractor()?;
                }
                self.prepare_custom_registry()?;
                for krate in self.crates_list.iter() {
                    let compiler = CrateCompiler::new(
                        &toolchain,
                        &workspace,
                        self.max_log_size,
                        self.memory_limit,
                        self.timeout,
                        self.enable_networking,
                        self.output_json,
                        self.use_original_rustc,
                        self.purge_build_dir,
                    );
                    let crate_extracted_files = self
                        .extracted_files_path
                        .join(
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0}-{1}", krate.name(), krate.version()),
                                );
                                res
                            }),
                        );
                    match compiler.build(krate, &crate_extracted_files) {
                        Ok(_) => {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!("Compilation succeeded."),
                                    lvl,
                                    &(
                                        "corpus_manager::compilation",
                                        "corpus_manager::compilation",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        }
                        Err(error) => {
                            {
                                let lvl = ::log::Level::Error;
                                if lvl <= ::log::STATIC_MAX_LEVEL
                                    && lvl <= ::log::max_level()
                                {
                                    ::log::__private_api::log(
                                        format_args!("Compilation failed: {0}", error),
                                        lvl,
                                        &(
                                            "corpus_manager::compilation",
                                            "corpus_manager::compilation",
                                            ::log::__private_api::loc(),
                                        ),
                                        (),
                                    );
                                }
                            };
                            if !crate_extracted_files.exists() {
                                std::fs::create_dir_all(&crate_extracted_files)?;
                            }
                            let build_logs = crate_extracted_files.join("logs");
                            std::fs::write(
                                build_logs,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Compilation failed: {0}", error),
                                    );
                                    res
                                }),
                            )?;
                        }
                    }
                }
                Ok(())
            })();
            result
                .map(|result| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("compile_all() => {0:?}", result),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    result
                })
                .map_err(|err| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("compile_all() => {0:?}", err),
                                lvl,
                                &(
                                    "corpus_manager::compilation",
                                    "corpus_manager::compilation",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    err
                })
        }
        /// Copies extractor to the workspace.
        fn copy_extractor(&self) -> Result<(), Box<dyn std::error::Error>> {
            let dest_path = self.workspace.join("cargo-home/rustc");
            std::fs::copy(&self.extractor_path, &dest_path)
                .unwrap_or_else(|_| {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "couldn\'t copy \'{0}\' to \'{1}\'",
                                self.extractor_path.display(),
                                dest_path.display(),
                            ),
                        );
                    }
                });
            Ok(())
        }
    }
    struct CrateCompiler<'a> {
        toolchain: &'a Toolchain,
        workspace: &'a Workspace,
        max_log_size: usize,
        memory_limit: Option<usize>,
        timeout: Option<Duration>,
        enable_networking: bool,
        output_json: bool,
        use_original_rustc: bool,
        purge_build_dir: bool,
    }
    impl<'a> CrateCompiler<'a> {
        fn new(
            toolchain: &'a Toolchain,
            workspace: &'a Workspace,
            max_log_size: usize,
            memory_limit: Option<usize>,
            timeout: Option<Duration>,
            enable_networking: bool,
            output_json: bool,
            use_original_rustc: bool,
            purge_build_dir: bool,
        ) -> Self {
            Self {
                toolchain,
                workspace,
                max_log_size,
                memory_limit,
                timeout,
                enable_networking,
                output_json,
                use_original_rustc,
                purge_build_dir,
            }
        }
        fn build(
            &self,
            krate_info: &'a CrateInfo,
            crate_extracted_files: &Path,
        ) -> Result<(), Box<dyn std::error::Error>> {
            if crate_extracted_files.exists() {
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "Already compiled: {0}",
                                crate_extracted_files.display(),
                            ),
                            lvl,
                            &(
                                "corpus_manager::compilation",
                                "corpus_manager::compilation",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                return Ok(());
            }
            let krate = Crate::crates_io(krate_info.name(), krate_info.version());
            krate.fetch(self.workspace)?;
            let sandbox = SandboxBuilder::new()
                .memory_limit(self.memory_limit)
                .enable_networking(self.enable_networking);
            let mut build_dir = self.workspace.build_dir("corpus");
            if self.purge_build_dir {
                build_dir.purge()?;
            }
            let sysroot = "/opt/rustwide/cargo-home/sysroot";
            std::fs::create_dir_all(&crate_extracted_files)?;
            build_dir
                .build(self.toolchain, &krate, sandbox)
                .run(|build| {
                    let mut storage = LogStorage::new(LevelFilter::Info);
                    storage.set_max_size(self.max_log_size);
                    let successful = logging::capture(
                        &storage,
                        || {
                            let mut builder = build
                                .cargo()
                                .timeout(self.timeout)
                                .args(&["check", "--all", "--frozen"])
                                .env("RUST_BACKTRACE", "1");
                            if !self.use_original_rustc {
                                builder = builder
                                    .env("SYSROOT", sysroot)
                                    .env("RUSTC", "/opt/rustwide/cargo-home/rustc");
                            }
                            if self.output_json {
                                builder = builder.env("CORPUS_OUTPUT_JSON", "true");
                            }
                            builder.run().is_ok()
                        },
                    );
                    let build_logs = crate_extracted_files.join("logs");
                    std::fs::write(build_logs, storage.to_string())?;
                    let mut target_dir = build.host_target_dir();
                    target_dir.push("rust-corpus");
                    if target_dir.exists() {
                        if successful {
                            let success_marker = target_dir.join("success");
                            std::fs::write(
                                success_marker,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:?}", chrono::offset::Utc::now()),
                                    );
                                    res
                                }),
                            )?;
                        }
                        for entry in walkdir::WalkDir::new(target_dir) {
                            let entry = entry?;
                            let path = entry.path();
                            if path.is_file() {
                                let file_name = path.file_name().unwrap();
                                std::fs::rename(
                                    path,
                                    crate_extracted_files.join(file_name),
                                )?;
                            }
                        }
                    } else {
                        {
                            let lvl = ::log::Level::Error;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "The target directory does not exist: {0:?}",
                                        target_dir,
                                    ),
                                    lvl,
                                    &(
                                        "corpus_manager::compilation",
                                        "corpus_manager::compilation",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    }
                    Ok(())
                })?;
            Ok(())
        }
    }
}
mod compilation_utils {
    use log::error;
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::{Path, PathBuf};
    struct Failures {
        failure_reasons: HashMap<&'static str, u32>,
        internal_errors: Vec<PathBuf>,
        unknown_failures: Vec<PathBuf>,
    }
    #[automatically_derived]
    impl ::core::default::Default for Failures {
        #[inline]
        fn default() -> Failures {
            Failures {
                failure_reasons: ::core::default::Default::default(),
                internal_errors: ::core::default::Default::default(),
                unknown_failures: ::core::default::Default::default(),
            }
        }
    }
    enum CheckResult {
        Error(&'static str),
        InternalCompilerError,
        Ok,
    }
    impl Failures {
        fn new() -> Self {
            Self::default()
        }
        fn update(&mut self, logs_file: PathBuf) {
            if !logs_file.exists() {
                return;
            }
            let file = File::open(&logs_file)
                .unwrap_or_else(|err| {
                    ::core::panicking::panic_fmt(
                        format_args!("An error when opening {0:?}: {1}", logs_file, err),
                    );
                });
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match self.check_line(line.unwrap()) {
                    CheckResult::Error(reason) => {
                        let counter = self.failure_reasons.entry(reason).or_insert(0);
                        *counter += 1;
                        return;
                    }
                    CheckResult::InternalCompilerError => {
                        self.internal_errors.push(logs_file);
                        return;
                    }
                    CheckResult::Ok => {}
                }
            }
            self.unknown_failures.push(logs_file);
        }
        fn check_line(&mut self, line: String) -> CheckResult {
            use CheckResult::*;
            if line.starts_with("Compilation failed: ") {
                Error(
                    if line
                        .contains(
                            "\"fetch\" \"--locked\" \"--manifest-path\" \"Cargo.toml\"` failed",
                        )
                    {
                        "failed to fetch dependencies"
                    } else if line
                        .contains(
                            "\"generate-lockfile\" \"--manifest-path\" \"Cargo.toml\"` failed",
                        )
                    {
                        "failed to generate lockfile"
                    } else if line.contains("the crate depends on yanked dependencies") {
                        "crate depends on yanked dependencies"
                    } else if line.contains("invalid Cargo.toml syntax") {
                        "invalid Cargo.toml syntax"
                    } else if line.contains("missing Cargo.toml") {
                        "missing Cargo.toml"
                    } else if line.contains("unable to download") {
                        "unable to download package"
                    } else if line.contains("Client Error: 403 Forbidden") {
                        "unable to download package (403)"
                    } else if line.contains("Connection reset by peer (os error 104") {
                        "connection error: reset by peer"
                    } else {
                        "unknown compilation failure"
                    },
                )
            } else if line.contains("error: aborting due to")
                && line.contains("previous errors")
            {
                Error("compilation error")
            } else if line.contains("error: aborting due to previous error") {
                Error("compilation error")
            } else if line.contains("error: failed to run custom build command for") {
                Error("failed custom build command")
            } else if line.contains("error: unknown crate type: `dynlib`") {
                Error("unknown crate type")
            } else if line.contains("(signal: 9, SIGKILL: kill)") {
                Error("compilation killed")
            } else if line.contains("error: multiple packages link to native library") {
                Error("multiple package links")
            } else if line.contains("failed to read directory") {
                Error("failed to read directory")
            } else if line.contains("too much data in the log, truncating it") {
                Error("truncated logs")
            } else if line.contains("error: failed to download") {
                Error("failed to download")
            } else if line.contains("error[E0557]: feature has been removed") {
                Error("uses removed features")
            } else if line.contains("thread 'rustc' has overflowed its stack") {
                Error("rustc stack overflow")
            } else if line.contains("error: internal compiler error")
                || line.contains("thread 'rustc' panicked at")
                || line.contains("corpus_extractor")
            {
                InternalCompilerError
            } else {
                Ok
            }
        }
        fn print_report(&self) {
            {
                ::std::io::_print(format_args!("Failure reasons:\n"));
            };
            for (failure_reason, count) in &self.failure_reasons {
                {
                    ::std::io::_print(format_args!("{0}: {1}\n", failure_reason, count));
                };
            }
            if !self.internal_errors.is_empty() {
                {
                    ::std::io::_print(
                        format_args!(
                            "Internal extractor errors: {0}\n",
                            self.internal_errors.len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Examples:\n"));
                };
                for path in self.internal_errors.iter().take(5) {
                    {
                        ::std::io::_print(format_args!("  {0:?}\n", path));
                    };
                }
            }
            if !self.unknown_failures.is_empty() {
                {
                    ::std::io::_print(
                        format_args!(
                            "Unknown failures: {0}\n",
                            self.unknown_failures.len(),
                        ),
                    );
                };
                {
                    ::std::io::_print(format_args!("Examples:\n"));
                };
                for path in self.unknown_failures.iter().take(5) {
                    {
                        ::std::io::_print(format_args!("  {0:?}\n", path));
                    };
                }
            }
        }
    }
    pub fn check_compilation(workspace: &Path, delete_failures: bool) {
        let rust_corpus_dir = workspace.join("rust-corpus");
        let mut failures = Failures::new();
        for package_dir in std::fs::read_dir(rust_corpus_dir).unwrap() {
            let package_dir = package_dir.unwrap().path();
            if package_dir.file_name() == Some(std::ffi::OsStr::new("stdlib")) {
                continue;
            }
            let success_file = package_dir.join("success");
            if !success_file.exists() {
                let logs_file = package_dir.join("logs");
                failures.update(logs_file);
                if delete_failures {
                    std::fs::remove_dir_all(&package_dir)
                        .unwrap_or_else(|err| {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Failed to delete {0:?}: {1}",
                                    package_dir,
                                    err,
                                ),
                            );
                        });
                }
            }
        }
        failures.print_report();
    }
    pub fn move_extracted(workspace: &Path, target_dir: &Path) {
        let target_dir = target_dir.join("rust-corpus");
        if !target_dir.exists() {
            std::fs::create_dir_all(&target_dir).unwrap();
        }
        let mut added = HashSet::new();
        for package_dir in std::fs::read_dir(&target_dir).unwrap() {
            let package_dir = package_dir.unwrap().path();
            for file in std::fs::read_dir(package_dir).unwrap() {
                let file = file.unwrap().path();
                if file.extension() == Some(std::ffi::OsStr::new("bincode")) {
                    added.insert(file.file_name().unwrap().to_owned());
                }
            }
        }
        let mut to_move = Vec::new();
        let mut file_names = HashMap::new();
        let rust_corpus_dir = workspace.join("rust-corpus");
        for package_dir in std::fs::read_dir(rust_corpus_dir).unwrap() {
            let package_dir = package_dir.unwrap().path();
            let success_file = package_dir.join("success");
            if success_file.exists()
                || package_dir.file_name() == Some(std::ffi::OsStr::new("stdlib"))
            {
                if success_file.exists() {
                    to_move.push(success_file);
                }
                let package_dir_name = package_dir.file_name().unwrap().to_owned();
                let mut package_file_names = Vec::new();
                for file in std::fs::read_dir(package_dir).unwrap() {
                    let file = file.unwrap().path();
                    let file_name = file.file_name().unwrap().to_owned();
                    if file.extension() == Some(std::ffi::OsStr::new("bincode"))
                        && !added.contains(&file_name)
                    {
                        added.insert(file_name.clone());
                        to_move.push(file);
                    }
                    package_file_names.push(file_name);
                }
                file_names.insert(package_dir_name, package_file_names);
            } else {
                let logs_file = package_dir.join("logs");
                if !logs_file.exists() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("missing logs file: {0:?}", logs_file),
                        );
                    }
                }
                to_move.push(logs_file);
            }
        }
        {
            ::std::io::_print(
                format_args!(
                    "Collected {0} files to move. Sleep for 20 seconds.\n",
                    to_move.len(),
                ),
            );
        };
        std::thread::sleep(std::time::Duration::from_secs(20));
        {
            ::std::io::_print(format_args!("Start moving.\n"));
        };
        for (package_dir_name, package_file_names) in &file_names {
            let mut path = target_dir.join(package_dir_name);
            if !path.exists() {
                std::fs::create_dir_all(&path)
                    .unwrap_or_else(|err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "failed to create directory {0:?}: {1}",
                                path,
                                err,
                            ),
                        );
                    });
            }
            path.push("files.json");
            let mut file = std::fs::File::create(&path)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Unable to create {0:?}: {1}", path, e),
                    );
                });
            serde_json::to_writer_pretty(&mut file, package_file_names)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Unable to write {0:?}: {1}", path, e),
                    );
                });
        }
        for from_path in to_move {
            let file_name = from_path.file_name().unwrap();
            let package_name = from_path.parent().unwrap().file_name().unwrap();
            let mut to_path = target_dir.join(package_name);
            if !to_path.exists() {
                {
                    let lvl = ::log::Level::Error;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "Creating {0:?}. Shouldn\'t all directories be already created?",
                                to_path,
                            ),
                            lvl,
                            &(
                                "corpus_manager::compilation_utils",
                                "corpus_manager::compilation_utils",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                std::fs::create_dir_all(&to_path)
                    .unwrap_or_else(|err| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "failed to create directory {0:?}: {1}",
                                to_path,
                                err,
                            ),
                        );
                    });
            }
            to_path.push(file_name);
            std::fs::rename(from_path, to_path).unwrap();
        }
    }
}
mod database {
    //! Module responsible for managing the database.
    use anyhow::Result;
    use corpus_database::tables;
    use log::{debug, error, info, trace};
    use log_derive::logfn;
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};
    use std::{ffi, fs, io};
    pub struct DatabaseManager {
        loaded_crates_path: PathBuf,
        loaded_crates: HashSet<String>,
        database_root: PathBuf,
        database: tables::TableMerger,
    }
    impl DatabaseManager {
        pub fn new(database_root: &Path) -> Self {
            let database_root = database_root.to_path_buf();
            let loaded_crates_path = database_root.join("loaded_crates.json");
            let (loaded_crates, database) = if database_root.exists() {
                let file = fs::File::open(&loaded_crates_path)
                    .unwrap_or_else(|e| {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "The database state is corrupted. Failed to read the list of loaded crates {0:?}: {1}",
                                    loaded_crates_path,
                                    e,
                                ),
                            );
                        }
                    });
                let loaded_crates = serde_json::from_reader(file)
                    .unwrap_or_else(|e| {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "The database state is corrupted. The crates list is invalid JSON {0:?}: {1}",
                                    loaded_crates_path,
                                    e,
                                ),
                            );
                        }
                    });
                (loaded_crates, tables::Tables::load_multifile(&database_root).unwrap())
            } else {
                fs::create_dir_all(&database_root)
                    .expect("Failed to create the directory for the database");
                (HashSet::new(), tables::Tables::default())
            };
            Self {
                loaded_crates_path,
                loaded_crates,
                database_root,
                database: tables::TableMerger::new(database),
            }
        }
        pub fn update_database(&mut self, workspace_root: &Path) {
            let result = (move || {
                let crates = self.scan_crates(&workspace_root.join("rust-corpus"));
                let mut success_counter = 0;
                let mut fail_counter = 0;
                for path in crates {
                    {
                        let lvl = ::log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Checking crate: {0:?}", path),
                                lvl,
                                &(
                                    "corpus_manager::database",
                                    "corpus_manager::database",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    let file_name = path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();
                    if self.loaded_crates.contains(&file_name) {
                        {
                            let lvl = ::log::Level::Debug;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "Crate already loaded: {0:?} {1}",
                                        path,
                                        file_name,
                                    ),
                                    lvl,
                                    &(
                                        "corpus_manager::database",
                                        "corpus_manager::database",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                    } else {
                        {
                            let lvl = ::log::Level::Info;
                            if lvl <= ::log::STATIC_MAX_LEVEL
                                && lvl <= ::log::max_level()
                            {
                                ::log::__private_api::log(
                                    format_args!(
                                        "Loading crate ({0}): {1:?}",
                                        success_counter,
                                        path,
                                    ),
                                    lvl,
                                    &(
                                        "corpus_manager::database",
                                        "corpus_manager::database",
                                        ::log::__private_api::loc(),
                                    ),
                                    (),
                                );
                            }
                        };
                        match self.load_crate(file_name, path) {
                            Ok(()) => success_counter += 1,
                            Err(e) => {
                                fail_counter += 1;
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!("  Error occurred: {0}", e),
                                            lvl,
                                            &(
                                                "corpus_manager::database",
                                                "corpus_manager::database",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                }
                            }
                        };
                    }
                }
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "Successfully loaded {0} crates",
                                success_counter,
                            ),
                            lvl,
                            &(
                                "corpus_manager::database",
                                "corpus_manager::database",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                if fail_counter > 0 {
                    {
                        let lvl = ::log::Level::Error;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("Failed to load {0} crates", fail_counter),
                                lvl,
                                &(
                                    "corpus_manager::database",
                                    "corpus_manager::database",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                }
                match fs::remove_file(&self.loaded_crates_path) {
                    Ok(_) => {}
                    Err(error) => {
                        if error.kind() != io::ErrorKind::NotFound {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("Failed to remove the loaded crates file."),
                                );
                            }
                        }
                    }
                }
                self.database.tables().store_multifile(&self.database_root).unwrap();
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("Successfully updated the database"),
                            lvl,
                            &(
                                "corpus_manager::database",
                                "corpus_manager::database",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                let mut file = fs::File::create(&self.loaded_crates_path)
                    .unwrap_or_else(|e| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to create {0:?}: {1}",
                                self.loaded_crates_path,
                                e,
                            ),
                        );
                    });
                serde_json::to_writer_pretty(&mut file, &self.loaded_crates)
                    .unwrap_or_else(|e| {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "Unable to write {0:?}: {1}",
                                self.loaded_crates_path,
                                e,
                            ),
                        );
                    });
                {
                    let lvl = ::log::Level::Info;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!("Successfully saved the loaded crates list"),
                            lvl,
                            &(
                                "corpus_manager::database",
                                "corpus_manager::database",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
            })();
            {
                let lvl = log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("update_database() => {0:?}", result),
                        lvl,
                        &(
                            "corpus_manager::database",
                            "corpus_manager::database",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            result
        }
        fn scan_crates(&self, workspace_root: &Path) -> impl Iterator<Item = PathBuf> {
            walkdir::WalkDir::new(workspace_root.canonicalize().unwrap())
                .into_iter()
                .filter_entry(|entry| entry.file_name() != "source")
                .map(|entry| entry.unwrap().into_path())
                .filter(|path| path.extension() == Some(ffi::OsStr::new("bincode")))
        }
        fn load_crate(&mut self, file_name: String, crate_path: PathBuf) -> Result<()> {
            let result = (move || {
                let crate_tables = tables::Tables::load(&crate_path)?;
                self.database.merge(crate_tables);
                self.loaded_crates.insert(file_name);
                Ok(())
            })();
            result
                .map(|result| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("load_crate() => {0:?}", result),
                                lvl,
                                &(
                                    "corpus_manager::database",
                                    "corpus_manager::database",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    result
                })
                .map_err(|err| {
                    {
                        let lvl = log::Level::Trace;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!("load_crate() => {0:?}", err),
                                lvl,
                                &(
                                    "corpus_manager::database",
                                    "corpus_manager::database",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                    err
                })
        }
    }
}
mod queries {
    use log::info;
    use std::path::Path;
    mod build_files {
        //! Report information about custom build files (`build.rs`).
        use crate::write_csv;
        use cargo::core::Package;
        use corpus_database::{
            tables::Loader, types::{Build, InternedString, PackageVersion},
        };
        use corpus_queries_derive::datapond_query;
        use std::path::Path;
        pub fn query(loader: &Loader, report_path: &Path) {
            let strings = loader.load_strings();
            let crate_names = loader.load_crate_names();
            let builds = loader.load_builds_as_vec();
            let package_names = loader.load_package_names();
            let package_versions = loader.load_package_versions();
            let Some(crate_name) = strings.lookup_str("build_script_build") else {
                let build_script_crates: Vec<
                    (Build, InternedString, InternedString, InternedString, String),
                > = Vec::new();
                if !report_path.exists() {
                    std::fs::create_dir(report_path).unwrap();
                }
                let file_path = report_path
                    .join(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}.csv", "build_script_crates"),
                            );
                            res
                        }),
                    );
                let mut wtr = csv::Writer::from_path(file_path).unwrap();
                for row in build_script_crates {
                    wtr.serialize(row).unwrap();
                }
                wtr.flush().unwrap();
                return;
            };
            let krate = crate_names.lookup(&crate_name).unwrap();
            let build_script_builds: Vec<_> = builds
                .iter()
                .filter(|
                    (_build, _package, _version, build_crate, _crate_hash, _edition)|
                { krate == *build_crate })
                .map(|&(build, package, version, krate, crate_hash, _edition)| {
                    (build, package, version, krate, crate_hash)
                })
                .collect();
            let build_script_crates: Vec<_> = build_script_builds
                .iter()
                .map(|&(build, package, version, krate, crate_hash)| {
                    (
                        build,
                        package_names.r(package),
                        package_versions.r(version),
                        crate_names.r(krate),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0:x}", crate_hash),
                            );
                            res
                        }),
                    )
                })
                .collect();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "build_script_crates"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in build_script_crates {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod build_meta {
        //! For each build, report the categories and keywords that were specified in
        //! `Cargo.toml`.
        use super::utils::BuildResolver;
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use std::path::Path;
        fn report_build_categories(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let categories = loader.load_crate_categories();
            let categories = categories
                .iter()
                .map(|&(build, category)| (
                    build,
                    build_resolver.resolve(build),
                    strings.r(category),
                ));
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "categories"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in categories {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        fn report_build_keywords(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let keywords = loader.load_crate_keywords();
            let keywords = keywords
                .iter()
                .map(|&(build, keyword)| (
                    build,
                    build_resolver.resolve(build),
                    strings.r(keyword),
                ));
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "keywords"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in keywords {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            report_build_categories(loader, report_path);
            report_build_keywords(loader, report_path);
        }
    }
    mod counters {
        //! Compute unsafe blocks and unsafe statements.
        use super::utils::{DefPathResolver, GroupByIterator, SpanResolver};
        use crate::queries::utils::BuildResolver;
        use crate::write_csv;
        use corpus_database::{tables::Loader, types};
        use corpus_queries_derive::datapond_query;
        use log::info;
        use std::collections::{HashMap, HashSet};
        use std::path::Path;
        pub fn query(loader: &Loader, report_path: &Path) {
            let selected_scopes;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(selected_mir_cfgs, subscopes), } output\nselected_scopes(build: Build, mir_body_def_path: DefPath, scope: Scope,\nparent: Scope, safety: ScopeSafety, explicit_unsafe_group: u32, check_mode:\nBlockCheckMode, span: Span,)\nselected_scopes(build, mir_body_def_path, scope, parent, safety,\nexplicit_unsafe_group, check_mode, span) :-\nselected_mir_cfgs(build, _, mir_body_def_path, parent),\nsubscopes(.parent=parent, .child=scope, .safety=safety,\n.explicit_unsafe_group=explicit_unsafe_group, .check_mode=check_mode,\n.span=span).selected_scopes(build, mir_body_def_path, scope, parent, safety,\nexplicit_unsafe_group, check_mode, span) :-\nselected_scopes(.build=build, .mir_body_def_path=mir_body_def_path,\n.scope=parent),\nsubscopes(.parent=parent, .child=scope, .safety=safety,\n.explicit_unsafe_group=explicit_unsafe_group, .check_mode=check_mode,\n.span=span).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let selected_mir_cfgs = loader.load_selected_mir_cfgs().clone();
                    let subscopes = loader.load_subscopes().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_selected_mir_cfgs = datafrog::Relation::<
                            (Build, Item, DefPath, Scope),
                        >::from_vec(selected_mir_cfgs);
                        let var_subscopes = datafrog::Relation::<
                            (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span),
                        >::from_vec(subscopes);
                        let var_selected_scopes = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    Scope,
                                    Scope,
                                    ScopeSafety,
                                    u32,
                                    BlockCheckMode,
                                    Span,
                                ),
                            >("selected_scopes");
                        let var_selected_mir_cfgs_1 = iteration
                            .variable::<
                                (Build, Item, DefPath, Scope),
                            >("selected_mir_cfgs_1");
                        let var_subscopes_2 = iteration
                            .variable::<
                                (Scope, Scope, ScopeSafety, BlockCheckMode, u32, Span),
                            >("subscopes_2");
                        let var_selected_mir_cfgs_1_3 = iteration
                            .variable::<
                                ((Scope,), (Build, DefPath)),
                            >("selected_mir_cfgs_1_3");
                        let var_subscopes_2_4 = iteration
                            .variable::<
                                ((Scope,), (Scope, ScopeSafety, BlockCheckMode, u32, Span)),
                            >("subscopes_2_4");
                        let var_selected_scopes_5 = iteration
                            .variable::<
                                (
                                    Scope,
                                    Build,
                                    DefPath,
                                    Scope,
                                    ScopeSafety,
                                    BlockCheckMode,
                                    u32,
                                    Span,
                                ),
                            >("selected_scopes_5");
                        let var_selected_scopes_6 = iteration
                            .variable::<
                                ((Scope,), (Build, DefPath)),
                            >("selected_scopes_6");
                        let var_subscopes_2_7 = iteration
                            .variable::<
                                ((Scope,), (Scope, ScopeSafety, BlockCheckMode, u32, Span)),
                            >("subscopes_2_7");
                        let var_selected_scopes_8 = iteration
                            .variable::<
                                (
                                    Scope,
                                    Build,
                                    DefPath,
                                    Scope,
                                    ScopeSafety,
                                    BlockCheckMode,
                                    u32,
                                    Span,
                                ),
                            >("selected_scopes_8");
                        var_selected_mir_cfgs_1.insert(var_selected_mir_cfgs);
                        var_subscopes_2.insert(var_subscopes);
                        while iteration.changed() {
                            var_selected_mir_cfgs_1_3
                                .from_map(
                                    &var_selected_mir_cfgs_1,
                                    |&(build, _, mir_body_def_path, parent)| (
                                        (parent,),
                                        (build, mir_body_def_path),
                                    ),
                                );
                            var_subscopes_2_4
                                .from_map(
                                    &var_subscopes_2,
                                    |
                                        &(
                                            parent,
                                            scope,
                                            safety,
                                            check_mode,
                                            explicit_unsafe_group,
                                            span,
                                        )|
                                    (
                                        (parent,),
                                        (scope, safety, check_mode, explicit_unsafe_group, span),
                                    ),
                                );
                            var_selected_scopes_5
                                .from_join(
                                    &var_selected_mir_cfgs_1_3,
                                    &var_subscopes_2_4,
                                    |
                                        &(parent,),
                                        &(build, mir_body_def_path),
                                        &(scope, safety, check_mode, explicit_unsafe_group, span)|
                                    (
                                        parent,
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    ),
                                );
                            var_selected_scopes
                                .from_map(
                                    &var_selected_scopes_5,
                                    |
                                        &(
                                            parent,
                                            build,
                                            mir_body_def_path,
                                            scope,
                                            safety,
                                            check_mode,
                                            explicit_unsafe_group,
                                            span,
                                        )|
                                    (
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        parent,
                                        safety,
                                        explicit_unsafe_group,
                                        check_mode,
                                        span,
                                    ),
                                );
                            var_selected_scopes_6
                                .from_map(
                                    &var_selected_scopes,
                                    |&(build, mir_body_def_path, parent, _, _, _, _, _)| (
                                        (parent,),
                                        (build, mir_body_def_path),
                                    ),
                                );
                            var_subscopes_2_7
                                .from_map(
                                    &var_subscopes_2,
                                    |
                                        &(
                                            parent,
                                            scope,
                                            safety,
                                            check_mode,
                                            explicit_unsafe_group,
                                            span,
                                        )|
                                    (
                                        (parent,),
                                        (scope, safety, check_mode, explicit_unsafe_group, span),
                                    ),
                                );
                            var_selected_scopes_8
                                .from_join(
                                    &var_selected_scopes_6,
                                    &var_subscopes_2_7,
                                    |
                                        &(parent,),
                                        &(build, mir_body_def_path),
                                        &(scope, safety, check_mode, explicit_unsafe_group, span)|
                                    (
                                        parent,
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        safety,
                                        check_mode,
                                        explicit_unsafe_group,
                                        span,
                                    ),
                                );
                            var_selected_scopes
                                .from_map(
                                    &var_selected_scopes_8,
                                    |
                                        &(
                                            parent,
                                            build,
                                            mir_body_def_path,
                                            scope,
                                            safety,
                                            check_mode,
                                            explicit_unsafe_group,
                                            span,
                                        )|
                                    (
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        parent,
                                        safety,
                                        explicit_unsafe_group,
                                        check_mode,
                                        span,
                                    ),
                                );
                        }
                        selected_scopes = var_selected_scopes.complete();
                    }
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("selected_scopes.len = {0}", selected_scopes.len()),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_scopes(selected_scopes.elements);
            let selected_scopes = loader.load_selected_scopes();
            let strings = loader.load_strings();
            let def_path_resolver = DefPathResolver::new(loader);
            let span_resolver = SpanResolver::new(loader);
            let selected_builds: HashSet<_> = loader
                .load_selected_builds()
                .iter()
                .map(|(build, _package, _version, _krate, _crate_hash, _edition)| *build)
                .collect();
            let mut unsafe_blocks_relation = Vec::new();
            let mut unsafe_blocks = Vec::new();
            let mut unsafe_root_scopes = HashMap::new();
            let iter = selected_scopes
                .iter()
                .filter(|
                    &(
                        _build,
                        _mir_body_def_path,
                        _scope,
                        _parent,
                        safety,
                        _explicit_unsafe_group,
                        _check_mode,
                        _span,
                    )|
                { *safety == types::ScopeSafety::ExplicitUnsafe })
                .safe_group_by(|
                    (
                        _build,
                        def_path,
                        _scope,
                        _parent,
                        _safety,
                        explicit_unsafe_group,
                        _check_mode,
                        _span,
                    )|
                { (def_path, explicit_unsafe_group) });
            let mut counter = 0;
            for (_, group) in iter.into_iter() {
                counter += 1;
                let mut children = HashSet::new();
                let group: Vec<_> = group.collect();
                for &(
                    _build,
                    _mir_body_def_path,
                    scope,
                    _parent,
                    _safety,
                    _explicit_unsafe_group,
                    _check_mode,
                    _span,
                ) in &group {
                    children.insert(scope);
                }
                let mut found = false;
                for &(
                    build,
                    mir_body_def_path,
                    scope,
                    parent,
                    _safety,
                    _explicit_unsafe_group,
                    check_mode,
                    span,
                ) in group {
                    if !children.contains(&parent) {
                        if !!found {
                            ::core::panicking::panic("assertion failed: !found")
                        }
                        found = true;
                        unsafe_blocks_relation
                            .push((
                                build,
                                mir_body_def_path,
                                scope,
                                span_resolver.get_expansion_kind(span),
                                check_mode,
                                span,
                            ));
                        if !selected_builds.contains(&build) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Unsafe block from non-selected build: {0:?}",
                                        def_path_resolver.resolve(mir_body_def_path),
                                    ),
                                );
                            }
                        }
                        unsafe_blocks
                            .push((
                                build,
                                def_path_resolver.resolve(mir_body_def_path),
                                scope,
                                check_mode.to_string(),
                                span_resolver.resolve(span),
                            ));
                        for &subscope in &children {
                            unsafe_root_scopes
                                .insert(subscope, (scope, build, check_mode));
                        }
                    }
                }
                if !found {
                    ::core::panicking::panic("assertion failed: found")
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("counter: {0}", counter),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Computed unsafe blocks: {0}", unsafe_blocks.len()),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_unsafe_blocks(unsafe_blocks_relation);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe blocks."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_blocks"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_blocks {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe block report."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let statements = loader.load_statements();
            let unsafe_statements: Vec<_> = statements
                .iter()
                .flat_map(|&(stmt, block, index, kind, scope)| {
                    unsafe_root_scopes
                        .get(&scope)
                        .map(|&(unsafe_scope, build, check_mode)| {
                            (build, stmt, block, index, kind, unsafe_scope, check_mode)
                        })
                })
                .collect();
            loader.store_unsafe_statements(unsafe_statements);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe statements."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let terminators = loader.load_terminators();
            let unsafe_terminators: Vec<_> = terminators
                .iter()
                .flat_map(|&(block, kind, scope)| {
                    unsafe_root_scopes
                        .get(&scope)
                        .map(|&(unsafe_scope, build, check_mode)| {
                            (build, block, kind, unsafe_scope, check_mode)
                        })
                })
                .collect();
            loader.store_unsafe_terminators(unsafe_terminators);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe terminators."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let functions_unsafe_blocks;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(selected_mir_cfgs, unsafe_blocks), } output\nfunctions_unsafe_blocks(build: Build, function: Item, scope: Scope,\nexpansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)\nfunctions_unsafe_blocks(build, function, scope, expansion_kind, check_mode) :-\nunsafe_blocks(build, mir_body_def_path, scope, expansion_kind, check_mode, _),\nselected_mir_cfgs(build, function, mir_body_def_path, _).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let selected_mir_cfgs = loader.load_selected_mir_cfgs().clone();
                    let unsafe_blocks = loader.load_unsafe_blocks().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_selected_mir_cfgs = datafrog::Relation::<
                            (Build, Item, DefPath, Scope),
                        >::from_vec(selected_mir_cfgs);
                        let var_unsafe_blocks = datafrog::Relation::<
                            (
                                Build,
                                DefPath,
                                Scope,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Span,
                            ),
                        >::from_vec(unsafe_blocks);
                        let var_functions_unsafe_blocks = iteration
                            .variable::<
                                (Build, Item, Scope, SpanExpansionKind, BlockCheckMode),
                            >("functions_unsafe_blocks");
                        let var_unsafe_blocks_1 = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    Scope,
                                    SpanExpansionKind,
                                    BlockCheckMode,
                                    Span,
                                ),
                            >("unsafe_blocks_1");
                        let var_selected_mir_cfgs_2 = iteration
                            .variable::<
                                (Build, Item, DefPath, Scope),
                            >("selected_mir_cfgs_2");
                        let var_unsafe_blocks_1_3 = iteration
                            .variable::<
                                (
                                    (Build, DefPath),
                                    (Scope, SpanExpansionKind, BlockCheckMode),
                                ),
                            >("unsafe_blocks_1_3");
                        let var_selected_mir_cfgs_2_4 = iteration
                            .variable::<
                                ((Build, DefPath), (Item,)),
                            >("selected_mir_cfgs_2_4");
                        let var_functions_unsafe_blocks_5 = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    Scope,
                                    SpanExpansionKind,
                                    BlockCheckMode,
                                    Item,
                                ),
                            >("functions_unsafe_blocks_5");
                        var_unsafe_blocks_1.insert(var_unsafe_blocks);
                        var_selected_mir_cfgs_2.insert(var_selected_mir_cfgs);
                        while iteration.changed() {
                            var_unsafe_blocks_1_3
                                .from_map(
                                    &var_unsafe_blocks_1,
                                    |
                                        &(
                                            build,
                                            mir_body_def_path,
                                            scope,
                                            expansion_kind,
                                            check_mode,
                                            _,
                                        )|
                                    (
                                        (build, mir_body_def_path),
                                        (scope, expansion_kind, check_mode),
                                    ),
                                );
                            var_selected_mir_cfgs_2_4
                                .from_map(
                                    &var_selected_mir_cfgs_2,
                                    |&(build, function, mir_body_def_path, _)| (
                                        (build, mir_body_def_path),
                                        (function,),
                                    ),
                                );
                            var_functions_unsafe_blocks_5
                                .from_join(
                                    &var_unsafe_blocks_1_3,
                                    &var_selected_mir_cfgs_2_4,
                                    |
                                        &(build, mir_body_def_path),
                                        &(scope, expansion_kind, check_mode),
                                        &(function,)|
                                    (
                                        build,
                                        mir_body_def_path,
                                        scope,
                                        expansion_kind,
                                        check_mode,
                                        function,
                                    ),
                                );
                            var_functions_unsafe_blocks
                                .from_map(
                                    &var_functions_unsafe_blocks_5,
                                    |
                                        &(
                                            build,
                                            mir_body_def_path,
                                            scope,
                                            expansion_kind,
                                            check_mode,
                                            function,
                                        )|
                                    (build, function, scope, expansion_kind, check_mode),
                                );
                        }
                        functions_unsafe_blocks = var_functions_unsafe_blocks.complete();
                    }
                }
            }
            let functions_unsafe_blocks = functions_unsafe_blocks.elements;
            let function_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
                .iter()
                .safe_group_by(|
                    (_build, function, _scope, _expansion_kind, _check_mode)|
                *function)
                .into_iter()
                .map(|(function, group)| (function, group.count()))
                .collect();
            let function_user_unsafe_block_counts: HashMap<_, _> = functions_unsafe_blocks
                .iter()
                .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
                    *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                })
                .safe_group_by(|
                    (_build, function, _scope, _expansion_kind, _check_mode)|
                *function)
                .into_iter()
                .map(|(function, group)| (function, group.count()))
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "functions_unsafe_blocks.len = {0}",
                            functions_unsafe_blocks.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "&functions_unsafe_blocks"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in &functions_unsafe_blocks {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            loader.store_functions_unsafe_blocks(functions_unsafe_blocks);
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let selected_function_definitions = loader
                .load_selected_function_definitions();
            let selected_function_definitions = selected_function_definitions
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        def_path,
                        module,
                        visibility,
                        unsafety,
                        abi,
                        _return_ty,
                        uses_unsafe,
                    )|
                {
                    (
                        build,
                        def_path_resolver.resolve(def_path),
                        item,
                        def_path,
                        module,
                        visibility.to_string(),
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        uses_unsafe,
                        function_unsafe_block_counts.get(&item).cloned().unwrap_or(0),
                        function_user_unsafe_block_counts
                            .get(&item)
                            .cloned()
                            .unwrap_or(0),
                        trait_items.contains(&def_path),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_function_definitions"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_function_definitions {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn new_query(loader: &Loader, report_path: &Path) {
            let selected_thir_blocks;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(selected_thir_bodies, thir_blocks), } output\nselected_thir_blocks(build: Build, thir_body_def_path: DefPath, parent:\nThirBlock, block: ThirBlock, safety: ScopeSafety, check_mode: BlockCheckMode,\nspan: Span,)\nselected_thir_blocks(build, thir_body_def_path, parent, block, safety,\ncheck_mode, span) :- thir_blocks(parent, block, safety, check_mode, span),\nselected_thir_bodies(build, _, thir_body_def_path,\nparent).selected_thir_blocks(build, thir_body_def_path, parent, block, safety,\ncheck_mode, span) :-\nselected_thir_blocks(.build=build, .thir_body_def_path=thir_body_def_path,\n.block=parent), thir_blocks(parent, block, safety, check_mode, span).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let selected_thir_bodies = loader
                        .load_selected_thir_bodies()
                        .clone();
                    let thir_blocks = loader.load_thir_blocks().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_selected_thir_bodies = datafrog::Relation::<
                            (Build, Item, DefPath, ThirBlock),
                        >::from_vec(selected_thir_bodies);
                        let var_thir_blocks = datafrog::Relation::<
                            (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span),
                        >::from_vec(thir_blocks);
                        let var_selected_thir_blocks = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    ThirBlock,
                                    ThirBlock,
                                    ScopeSafety,
                                    BlockCheckMode,
                                    Span,
                                ),
                            >("selected_thir_blocks");
                        let var_thir_blocks_1 = iteration
                            .variable::<
                                (ThirBlock, ThirBlock, ScopeSafety, BlockCheckMode, Span),
                            >("thir_blocks_1");
                        let var_selected_thir_bodies_2 = iteration
                            .variable::<
                                (Build, Item, DefPath, ThirBlock),
                            >("selected_thir_bodies_2");
                        let var_thir_blocks_1_3 = iteration
                            .variable::<
                                (
                                    (ThirBlock,),
                                    (ThirBlock, ScopeSafety, BlockCheckMode, Span),
                                ),
                            >("thir_blocks_1_3");
                        let var_selected_thir_bodies_2_4 = iteration
                            .variable::<
                                ((ThirBlock,), (Build, DefPath)),
                            >("selected_thir_bodies_2_4");
                        let var_selected_thir_blocks_5 = iteration
                            .variable::<
                                (
                                    ThirBlock,
                                    ThirBlock,
                                    ScopeSafety,
                                    BlockCheckMode,
                                    Span,
                                    Build,
                                    DefPath,
                                ),
                            >("selected_thir_blocks_5");
                        let var_selected_thir_blocks_6 = iteration
                            .variable::<
                                ((ThirBlock,), (Build, DefPath)),
                            >("selected_thir_blocks_6");
                        let var_thir_blocks_1_7 = iteration
                            .variable::<
                                (
                                    (ThirBlock,),
                                    (ThirBlock, ScopeSafety, BlockCheckMode, Span),
                                ),
                            >("thir_blocks_1_7");
                        let var_selected_thir_blocks_8 = iteration
                            .variable::<
                                (
                                    ThirBlock,
                                    Build,
                                    DefPath,
                                    ThirBlock,
                                    ScopeSafety,
                                    BlockCheckMode,
                                    Span,
                                ),
                            >("selected_thir_blocks_8");
                        var_thir_blocks_1.insert(var_thir_blocks);
                        var_selected_thir_bodies_2.insert(var_selected_thir_bodies);
                        while iteration.changed() {
                            var_thir_blocks_1_3
                                .from_map(
                                    &var_thir_blocks_1,
                                    |&(parent, block, safety, check_mode, span)| (
                                        (parent,),
                                        (block, safety, check_mode, span),
                                    ),
                                );
                            var_selected_thir_bodies_2_4
                                .from_map(
                                    &var_selected_thir_bodies_2,
                                    |&(build, _, thir_body_def_path, parent)| (
                                        (parent,),
                                        (build, thir_body_def_path),
                                    ),
                                );
                            var_selected_thir_blocks_5
                                .from_join(
                                    &var_thir_blocks_1_3,
                                    &var_selected_thir_bodies_2_4,
                                    |
                                        &(parent,),
                                        &(block, safety, check_mode, span),
                                        &(build, thir_body_def_path)|
                                    (
                                        parent,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                        build,
                                        thir_body_def_path,
                                    ),
                                );
                            var_selected_thir_blocks
                                .from_map(
                                    &var_selected_thir_blocks_5,
                                    |
                                        &(
                                            parent,
                                            block,
                                            safety,
                                            check_mode,
                                            span,
                                            build,
                                            thir_body_def_path,
                                        )|
                                    (
                                        build,
                                        thir_body_def_path,
                                        parent,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                    ),
                                );
                            var_selected_thir_blocks_6
                                .from_map(
                                    &var_selected_thir_blocks,
                                    |&(build, thir_body_def_path, _, parent, _, _, _)| (
                                        (parent,),
                                        (build, thir_body_def_path),
                                    ),
                                );
                            var_thir_blocks_1_7
                                .from_map(
                                    &var_thir_blocks_1,
                                    |&(parent, block, safety, check_mode, span)| (
                                        (parent,),
                                        (block, safety, check_mode, span),
                                    ),
                                );
                            var_selected_thir_blocks_8
                                .from_join(
                                    &var_selected_thir_blocks_6,
                                    &var_thir_blocks_1_7,
                                    |
                                        &(parent,),
                                        &(build, thir_body_def_path),
                                        &(block, safety, check_mode, span)|
                                    (
                                        parent,
                                        build,
                                        thir_body_def_path,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                    ),
                                );
                            var_selected_thir_blocks
                                .from_map(
                                    &var_selected_thir_blocks_8,
                                    |
                                        &(
                                            parent,
                                            build,
                                            thir_body_def_path,
                                            block,
                                            safety,
                                            check_mode,
                                            span,
                                        )|
                                    (
                                        build,
                                        thir_body_def_path,
                                        parent,
                                        block,
                                        safety,
                                        check_mode,
                                        span,
                                    ),
                                );
                        }
                        selected_thir_blocks = var_selected_thir_blocks.complete();
                    }
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "selected_thir_blocks.len = {0}",
                            selected_thir_blocks.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_thir_blocks(selected_thir_blocks.elements);
            let selected_thir_blocks = loader.load_selected_thir_blocks();
            let def_path_resolver = DefPathResolver::new(loader);
            let span_resolver = SpanResolver::new(loader);
            let strings = loader.load_strings();
            let mut unsafe_thir_blocks_relation = Vec::new();
            let mut unsafe_thir_blocks = Vec::new();
            for &(
                build,
                thir_body_def_path,
                _parent,
                block,
                _safety,
                check_mode,
                span,
            ) in selected_thir_blocks
                .iter()
                .filter(|
                    &(
                        _build,
                        _thir_body_def_path,
                        _parent,
                        _block,
                        safety,
                        _check_mode,
                        _span,
                    )|
                { *safety == types::ScopeSafety::ExplicitUnsafe })
            {
                unsafe_thir_blocks_relation
                    .push((
                        build,
                        thir_body_def_path,
                        block,
                        span_resolver.get_expansion_kind(span),
                        check_mode,
                        span,
                    ));
                unsafe_thir_blocks
                    .push((
                        build,
                        def_path_resolver.resolve(thir_body_def_path),
                        block,
                        check_mode.to_string(),
                        span_resolver.resolve(span),
                    ));
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Computed unsafe thir blocks: {0}",
                            unsafe_thir_blocks.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_unsafe_thir_blocks(unsafe_thir_blocks_relation);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe thir blocks."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_blocks"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_blocks {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe thir block report."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_thir_blocks_relation = loader.load_unsafe_thir_blocks();
            let unsafe_thir_statements = || {
                let thir_statements = loader.load_iter_thir_stmts();
                thir_statements
                    .flat_map(|(stmt, _block, closest_unsafe_block, index)| {
                        unsafe_thir_blocks_relation
                            .iter()
                            .filter(move |
                                &(
                                    build,
                                    _thir_body_def_path,
                                    unsafe_block,
                                    _expansion_kind,
                                    _check_mode,
                                    _span,
                                )|
                            { *unsafe_block == closest_unsafe_block })
                            .map(move |
                                &(
                                    build,
                                    _thir_body_def_path,
                                    _unsafe_block,
                                    _expansion_kind,
                                    check_mode,
                                    span,
                                )|
                            { (build, stmt, closest_unsafe_block, index, check_mode) })
                    })
            };
            loader.store_iter_unsafe_thir_stmts(unsafe_thir_statements());
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe thir statements."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_thir_statements = unsafe_thir_statements();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_statements"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_statements {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Saved unsafe thir statement report."),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let functions_unsafe_thir_blocks;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(selected_thir_bodies, unsafe_thir_blocks), } output\nfunctions_unsafe_thir_blocks(build: Build, function: Item, block: ThirBlock,\nexpansion_kind: SpanExpansionKind, check_mode: BlockCheckMode)\nfunctions_unsafe_thir_blocks(build, function, block, expansion_kind,\ncheck_mode) :-\nunsafe_thir_blocks(build, thir_body_def_path, block, expansion_kind,\ncheck_mode, _), selected_thir_bodies(build, function, thir_body_def_path, _).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let selected_thir_bodies = loader
                        .load_selected_thir_bodies()
                        .clone();
                    let unsafe_thir_blocks = loader.load_unsafe_thir_blocks().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_selected_thir_bodies = datafrog::Relation::<
                            (Build, Item, DefPath, ThirBlock),
                        >::from_vec(selected_thir_bodies);
                        let var_unsafe_thir_blocks = datafrog::Relation::<
                            (
                                Build,
                                DefPath,
                                ThirBlock,
                                SpanExpansionKind,
                                BlockCheckMode,
                                Span,
                            ),
                        >::from_vec(unsafe_thir_blocks);
                        let var_functions_unsafe_thir_blocks = iteration
                            .variable::<
                                (Build, Item, ThirBlock, SpanExpansionKind, BlockCheckMode),
                            >("functions_unsafe_thir_blocks");
                        let var_unsafe_thir_blocks_1 = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    ThirBlock,
                                    SpanExpansionKind,
                                    BlockCheckMode,
                                    Span,
                                ),
                            >("unsafe_thir_blocks_1");
                        let var_selected_thir_bodies_2 = iteration
                            .variable::<
                                (Build, Item, DefPath, ThirBlock),
                            >("selected_thir_bodies_2");
                        let var_unsafe_thir_blocks_1_3 = iteration
                            .variable::<
                                (
                                    (Build, DefPath),
                                    (ThirBlock, SpanExpansionKind, BlockCheckMode),
                                ),
                            >("unsafe_thir_blocks_1_3");
                        let var_selected_thir_bodies_2_4 = iteration
                            .variable::<
                                ((Build, DefPath), (Item,)),
                            >("selected_thir_bodies_2_4");
                        let var_functions_unsafe_thir_blocks_5 = iteration
                            .variable::<
                                (
                                    Build,
                                    DefPath,
                                    ThirBlock,
                                    SpanExpansionKind,
                                    BlockCheckMode,
                                    Item,
                                ),
                            >("functions_unsafe_thir_blocks_5");
                        var_unsafe_thir_blocks_1.insert(var_unsafe_thir_blocks);
                        var_selected_thir_bodies_2.insert(var_selected_thir_bodies);
                        while iteration.changed() {
                            var_unsafe_thir_blocks_1_3
                                .from_map(
                                    &var_unsafe_thir_blocks_1,
                                    |
                                        &(
                                            build,
                                            thir_body_def_path,
                                            block,
                                            expansion_kind,
                                            check_mode,
                                            _,
                                        )|
                                    (
                                        (build, thir_body_def_path),
                                        (block, expansion_kind, check_mode),
                                    ),
                                );
                            var_selected_thir_bodies_2_4
                                .from_map(
                                    &var_selected_thir_bodies_2,
                                    |&(build, function, thir_body_def_path, _)| (
                                        (build, thir_body_def_path),
                                        (function,),
                                    ),
                                );
                            var_functions_unsafe_thir_blocks_5
                                .from_join(
                                    &var_unsafe_thir_blocks_1_3,
                                    &var_selected_thir_bodies_2_4,
                                    |
                                        &(build, thir_body_def_path),
                                        &(block, expansion_kind, check_mode),
                                        &(function,)|
                                    (
                                        build,
                                        thir_body_def_path,
                                        block,
                                        expansion_kind,
                                        check_mode,
                                        function,
                                    ),
                                );
                            var_functions_unsafe_thir_blocks
                                .from_map(
                                    &var_functions_unsafe_thir_blocks_5,
                                    |
                                        &(
                                            build,
                                            thir_body_def_path,
                                            block,
                                            expansion_kind,
                                            check_mode,
                                            function,
                                        )|
                                    (build, function, block, expansion_kind, check_mode),
                                );
                        }
                        functions_unsafe_thir_blocks = var_functions_unsafe_thir_blocks
                            .complete();
                    }
                }
            }
            let functions_unsafe_thir_blocks = functions_unsafe_thir_blocks.elements;
            let function_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
                .iter()
                .safe_group_by(|
                    (_build, function, _scope, _expansion_kind, _check_mode)|
                *function)
                .into_iter()
                .map(|(function, group)| (function, group.count()))
                .collect();
            let function_user_unsafe_thir_block_counts: HashMap<_, _> = functions_unsafe_thir_blocks
                .iter()
                .filter(|(_build, _function, _scope, _expansion_kind, check_mode)| {
                    *check_mode == types::BlockCheckMode::UnsafeBlockUserProvided
                })
                .safe_group_by(|
                    (_build, function, _scope, _expansion_kind, _check_mode)|
                *function)
                .into_iter()
                .map(|(function, group)| (function, group.count()))
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "functions_unsafe_thir_blocks.len = {0}",
                            functions_unsafe_thir_blocks.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::counters",
                            "corpus_manager::queries::counters",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "&functions_unsafe_thir_blocks"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in &functions_unsafe_thir_blocks {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            loader.store_functions_unsafe_thir_blocks(functions_unsafe_thir_blocks);
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let selected_function_definitions = loader
                .load_selected_function_definitions();
            let selected_function_definitions_thir_counts = selected_function_definitions
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        def_path,
                        module,
                        visibility,
                        unsafety,
                        abi,
                        _return_ty,
                        uses_unsafe,
                    )|
                {
                    (
                        build,
                        def_path_resolver.resolve(def_path),
                        item,
                        def_path,
                        module,
                        visibility.to_string(),
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        uses_unsafe,
                        function_unsafe_thir_block_counts
                            .get(&item)
                            .cloned()
                            .unwrap_or(0),
                        function_user_unsafe_thir_block_counts
                            .get(&item)
                            .cloned()
                            .unwrap_or(0),
                        trait_items.contains(&def_path),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "{0}.csv",
                                "selected_function_definitions_thir_counts",
                            ),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_function_definitions_thir_counts {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod function_size {
        //! Report function sizes in MIR statements.
        use super::utils::{BuildResolver, DefPathResolver};
        use crate::write_csv;
        use corpus_database::types::ThirExpr;
        use corpus_database::{tables::Loader, types};
        use corpus_queries_derive::datapond_query;
        use std::collections::HashMap;
        use std::path::Path;
        fn collect_function_sizes(loader: &Loader) {
            let function_scopes: HashMap<_, _> = {
                let selected_scopes = loader.load_selected_scopes();
                selected_scopes
                    .iter()
                    .map(|
                        &(
                            build,
                            mir_body_def_path,
                            scope,
                            _parent,
                            safety,
                            _explicit_unsafe_group,
                            check_mode,
                            _span,
                        )|
                    { (scope, (build, mir_body_def_path, safety, check_mode)) })
                    .collect()
            };
            let function_definitions: HashMap<_, _> = {
                let selected_function_definitions = loader
                    .load_selected_function_definitions();
                let selected_function_definitions: HashMap<_, _> = selected_function_definitions
                    .iter()
                    .map(|
                        &(
                            build,
                            item,
                            def_path,
                            module,
                            visibility,
                            unsafety,
                            abi,
                            return_ty,
                            uses_unsafe,
                        )|
                    {
                        (
                            item,
                            (
                                build,
                                item,
                                def_path,
                                module,
                                visibility,
                                unsafety,
                                abi,
                                return_ty,
                                uses_unsafe,
                            ),
                        )
                    })
                    .collect();
                let selected_mir_cfgs = loader.load_selected_mir_cfgs();
                selected_mir_cfgs
                    .iter()
                    .flat_map(|&(_build, item, body_def_path, _root_scope)| {
                        selected_function_definitions
                            .get(&item)
                            .map(|&def| (body_def_path, def))
                    })
                    .collect()
            };
            let mut selected_function_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
            let mut selected_build_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
            for (_stmt, _block, _index, _kind, scope) in loader.load_statements().iter()
            {
                if let Some(&(build, mir_body_def_path, safety, check_mode)) = function_scopes
                    .get(scope)
                {
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_build_sizes_map
                            .entry(build)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_function_sizes_map
                            .entry(mir_body_def_path)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                }
            }
            let selected_build_sizes = selected_build_sizes_map
                .into_iter()
                .map(|(build, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                    (build, stmt, unsafe_stmt, user_unsafe_stmt)
                })
                .collect();
            loader.store_selected_build_sizes(selected_build_sizes);
            let selected_function_sizes = selected_function_sizes_map
                .into_iter()
                .flat_map(|(mir_body_def_path, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                    function_definitions
                        .get(&mir_body_def_path)
                        .map(|
                            &(
                                build,
                                item,
                                def_path,
                                _module,
                                visibility,
                                unsafety,
                                abi,
                                _return_ty,
                                uses_unsafe,
                            )|
                        {
                            (
                                build,
                                item,
                                def_path,
                                visibility,
                                unsafety,
                                abi,
                                uses_unsafe,
                                stmt,
                                unsafe_stmt,
                                user_unsafe_stmt,
                            )
                        })
                })
                .collect();
            loader.store_selected_function_sizes(selected_function_sizes);
        }
        fn report_function_sizes(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let def_path_resolver = DefPathResolver::new(loader);
            let abis = loader.load_abis();
            let strings = loader.load_strings();
            let selected_build_sizes = loader.load_selected_build_sizes();
            let selected_build_sizes = selected_build_sizes
                .iter()
                .map(|&(build, stmt, unsafe_stmt, user_unsafe_stmt)| {
                    (build_resolver.resolve(build), stmt, unsafe_stmt, user_unsafe_stmt)
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_build_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_build_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let selected_function_sizes = loader.load_selected_function_sizes();
            let selected_function_sizes = selected_function_sizes
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        def_path,
                        visibility,
                        unsafety,
                        abi,
                        uses_unsafe,
                        stmt,
                        unsafe_stmt,
                        user_unsafe_stmt,
                    )|
                {
                    (
                        build_resolver.resolve(build),
                        item,
                        def_path_resolver.resolve(def_path),
                        visibility.to_string(),
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        uses_unsafe,
                        stmt,
                        unsafe_stmt,
                        user_unsafe_stmt,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_function_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_function_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            collect_function_sizes(loader);
            report_function_sizes(loader, report_path);
        }
        fn new_collect_function_sizes(loader: &Loader) {
            let function_thir_blocks: HashMap<_, _> = {
                let selected_blocks = loader.load_iter_selected_thir_blocks();
                selected_blocks
                    .map(|
                        (
                            build,
                            thir_body_def_path,
                            _parent,
                            block,
                            safety,
                            check_mode,
                            _span,
                        )|
                    { (block, (build, thir_body_def_path, safety, check_mode)) })
                    .collect()
            };
            let function_definitions: HashMap<_, _> = {
                let selected_function_definitions = loader
                    .load_iter_selected_function_definitions();
                let selected_function_definitions: HashMap<_, _> = selected_function_definitions
                    .map(|
                        (
                            build,
                            item,
                            def_path,
                            module,
                            visibility,
                            unsafety,
                            abi,
                            return_ty,
                            uses_unsafe,
                        )|
                    {
                        (
                            item,
                            (
                                build,
                                item,
                                def_path,
                                module,
                                visibility,
                                unsafety,
                                abi,
                                return_ty,
                                uses_unsafe,
                            ),
                        )
                    })
                    .collect();
                let selected_thir_bodies = loader.load_iter_selected_thir_bodies();
                selected_thir_bodies
                    .flat_map(|(_build, item, body_def_path, _root_block)| {
                        selected_function_definitions
                            .get(&item)
                            .map(|&def| (body_def_path, def))
                    })
                    .collect()
            };
            let mut selected_function_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
            let mut selected_build_thir_sizes_map: HashMap<_, (u64, u64, u64)> = HashMap::new();
            for (_stmt, block, closest_unsafe_block, _index) in loader
                .load_iter_thir_stmts()
            {
                if let Some(&(build, thir_body_def_path, safety, check_mode)) = function_thir_blocks
                    .get(&closest_unsafe_block)
                    .or(function_thir_blocks.get(&block))
                {
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_build_thir_sizes_map
                            .entry(build)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_function_thir_sizes_map
                            .entry(thir_body_def_path)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                }
            }
            let no_thir_expr: ThirExpr = 0u64.into();
            let thir_trailing_expr_to_block: HashMap<_, _> = loader
                .load_iter_thir_block_expr()
                .flat_map(|(block, expr)| {
                    if expr == no_thir_expr { None } else { Some((expr, block)) }
                })
                .collect();
            for (expr, _, closest_unsafe_block, _, _) in loader.load_iter_thir_exprs() {
                let Some(&block) = thir_trailing_expr_to_block.get(&expr) else {
                    continue;
                };
                if expr == no_thir_expr {
                    continue;
                }
                if let Some(&(build, thir_body_def_path, safety, check_mode)) = function_thir_blocks
                    .get(&closest_unsafe_block)
                    .or(function_thir_blocks.get(&block))
                {
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_build_thir_sizes_map
                            .entry(build)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                    {
                        let (build_stmt, build_unsafe_stmt, build_user_unsafe_stmt) = selected_function_thir_sizes_map
                            .entry(thir_body_def_path)
                            .or_default();
                        *build_stmt += 1;
                        if safety != types::ScopeSafety::Safe {
                            *build_unsafe_stmt += 1;
                        }
                        if safety == types::ScopeSafety::FnUnsafe
                            || check_mode
                                == types::BlockCheckMode::UnsafeBlockUserProvided
                        {
                            *build_user_unsafe_stmt += 1;
                        }
                    }
                }
            }
            let selected_build_thir_sizes = selected_build_thir_sizes_map
                .into_iter()
                .map(|(build, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                    (build, stmt, unsafe_stmt, user_unsafe_stmt)
                });
            loader.store_iter_selected_build_thir_sizes(selected_build_thir_sizes);
            let selected_function_thir_sizes = selected_function_thir_sizes_map
                .into_iter()
                .flat_map(|(thir_body_def_path, (stmt, unsafe_stmt, user_unsafe_stmt))| {
                    function_definitions
                        .get(&thir_body_def_path)
                        .map(|
                            &(
                                build,
                                item,
                                def_path,
                                _module,
                                visibility,
                                unsafety,
                                abi,
                                _return_ty,
                                uses_unsafe,
                            )|
                        {
                            (
                                build,
                                item,
                                def_path,
                                visibility,
                                unsafety,
                                abi,
                                uses_unsafe,
                                stmt,
                                unsafe_stmt,
                                user_unsafe_stmt,
                            )
                        })
                });
            loader.store_iter_selected_function_thir_sizes(selected_function_thir_sizes);
        }
        fn new_report_function_sizes(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let def_path_resolver = DefPathResolver::new(loader);
            let abis = loader.load_abis();
            let strings = loader.load_strings();
            let selected_build_thir_sizes = loader.load_selected_build_thir_sizes();
            let selected_build_thir_sizes = selected_build_thir_sizes
                .iter()
                .map(|&(build, stmt, unsafe_stmt, user_unsafe_stmt)| {
                    (build_resolver.resolve(build), stmt, unsafe_stmt, user_unsafe_stmt)
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_build_thir_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_build_thir_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let selected_function_thir_sizes = loader
                .load_selected_function_thir_sizes();
            let selected_function_thir_sizes = selected_function_thir_sizes
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        def_path,
                        visibility,
                        unsafety,
                        abi,
                        uses_unsafe,
                        stmt,
                        unsafe_stmt,
                        user_unsafe_stmt,
                    )|
                {
                    (
                        build_resolver.resolve(build),
                        item,
                        def_path_resolver.resolve(def_path),
                        visibility.to_string(),
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        uses_unsafe,
                        stmt,
                        unsafe_stmt,
                        user_unsafe_stmt,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_function_thir_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_function_thir_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn new_query(loader: &Loader, report_path: &Path) {
            new_collect_function_sizes(loader);
            new_report_function_sizes(loader, report_path);
        }
    }
    mod non_tree_types {
        //! A query intended to find definitions of potentially non-tree data structures
        //! implemented by using unsafe code.
        use super::utils::{BuildResolver, DefPathResolver};
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use corpus_queries_derive::datapond_query;
        use std::collections::HashSet;
        use std::path::Path;
        /// Try to find definitions of potentially non-tree data structures:
        /// 1. contain raw pointers as fields;
        /// 2. have no attributes, such as `#[repr(C)]`, indicating that the struct is
        ///    used for FFI.
        fn report_non_tree_types(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let selected_adts = loader.load_selected_adts();
            let strings = loader.load_strings();
            let type_kinds = loader.load_type_kinds();
            let non_tree_types;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(types_adt_field, types_raw_ptr), } output\nnon_tree_types(typ: Type) non_tree_types(adt) :-\ntypes_adt_field(.adt=adt, .typ=typ), types_raw_ptr(.typ=typ).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let types_adt_field = loader.load_types_adt_field().clone();
                    let types_raw_ptr = loader.load_types_raw_ptr().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_types_adt_field = datafrog::Relation::<
                            (
                                Field,
                                Type,
                                AdtVariantIndex,
                                DefPath,
                                InternedString,
                                TyVisibility,
                                Type,
                            ),
                        >::from_vec(types_adt_field);
                        let var_types_raw_ptr = datafrog::Relation::<
                            (Type, Type, Mutability),
                        >::from_vec(types_raw_ptr);
                        let var_non_tree_types = iteration
                            .variable::<(Type,)>("non_tree_types");
                        let var_types_adt_field_1 = iteration
                            .variable::<
                                (
                                    Field,
                                    Type,
                                    AdtVariantIndex,
                                    DefPath,
                                    InternedString,
                                    TyVisibility,
                                    Type,
                                ),
                            >("types_adt_field_1");
                        let var_types_raw_ptr_2 = iteration
                            .variable::<(Type, Type, Mutability)>("types_raw_ptr_2");
                        let var_types_adt_field_1_3 = iteration
                            .variable::<((Type,), (Type,))>("types_adt_field_1_3");
                        let var_types_raw_ptr_2_4 = iteration
                            .variable::<((Type,), ())>("types_raw_ptr_2_4");
                        let var_non_tree_types_5 = iteration
                            .variable::<(Type, Type)>("non_tree_types_5");
                        var_types_adt_field_1.insert(var_types_adt_field);
                        var_types_raw_ptr_2.insert(var_types_raw_ptr);
                        while iteration.changed() {
                            var_types_adt_field_1_3
                                .from_map(
                                    &var_types_adt_field_1,
                                    |&(_, adt, _, _, _, _, typ)| ((typ,), (adt,)),
                                );
                            var_types_raw_ptr_2_4
                                .from_map(
                                    &var_types_raw_ptr_2,
                                    |&(typ, _, _)| ((typ,), ()),
                                );
                            var_non_tree_types_5
                                .from_join(
                                    &var_types_adt_field_1_3,
                                    &var_types_raw_ptr_2_4,
                                    |&(typ,), &(adt,), &()| (typ, adt),
                                );
                            var_non_tree_types
                                .from_map(&var_non_tree_types_5, |&(typ, adt)| (adt,));
                        }
                        non_tree_types = var_non_tree_types.complete();
                    }
                }
            }
            let non_tree_types: HashSet<_> = non_tree_types
                .elements
                .iter()
                .map(|&(typ,)| typ)
                .collect();
            let non_tree_adts = selected_adts
                .iter()
                .flat_map(|
                    &(
                        build,
                        item,
                        typ,
                        def_path,
                        resolved_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                    )|
                {
                    if non_tree_types.contains(&typ) {
                        Some((
                            build,
                            build_resolver.resolve(build),
                            item,
                            typ,
                            def_path_resolver.resolve(def_path),
                            def_path_resolver.resolve(resolved_def_path),
                            strings.r(name),
                            visibility.to_string(),
                            strings.r(type_kinds.r(type_kind)),
                            def_kind.to_string(),
                            kind.to_string(),
                            c_repr,
                            is_phantom,
                        ))
                    } else {
                        None
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "non_tree_adts"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in non_tree_adts {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            report_non_tree_types(loader, report_path);
        }
    }
    mod prepare_builds {
        use super::utils::GroupByIterator;
        use crate::sources_list::CratesList;
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use itertools::Itertools;
        use log::{error, info};
        use std::collections::{HashMap, HashSet};
        use std::path::Path;
        fn collect_files(workspace_path: &Path) -> HashSet<(String, String)> {
            let mut set = HashSet::new();
            for package_dir in std::fs::read_dir(workspace_path.join("rust-corpus"))
                .unwrap()
            {
                let mut path = package_dir.unwrap().path();
                let package_name = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned();
                path.push("files.json");
                if path.exists() {
                    let file = std::fs::File::open(&path)
                        .unwrap_or_else(|err| {
                            ::core::panicking::panic_fmt(
                                format_args!("Failed to open {0:?}: {1}", path, err),
                            );
                        });
                    let package_files: Vec<std::ffi::OsString> = serde_json::from_reader(
                            file,
                        )
                        .unwrap_or_else(|err| {
                            ::core::panicking::panic_fmt(
                                format_args!("Failed to read CSV {0:?}: {1}", path, err),
                            );
                        });
                    for package_file in package_files {
                        set.insert((
                            package_name.clone(),
                            package_file.to_str().unwrap().to_owned(),
                        ));
                    }
                }
            }
            set
        }
        /// Filter the `builds` relation to contain only the builds we are “interested”
        /// in analysing.
        ///
        /// The filter performs the following:
        ///
        /// 1.  Removes build scripts.
        /// 2.  Takes the builds that are builds of the package and appear in the same
        ///     package directory.
        /// 3.  Picks builds for all other packages so that we have a build for each
        ///     package/crate.
        pub fn query(
            loader: &Loader,
            report_path: &Path,
            workspace_path: &Path,
            sources_list_path: &Path,
        ) {
            let crates_list = CratesList::load(sources_list_path);
            let original_crates_set: HashSet<_> = crates_list
                .iter()
                .map(|krate| (krate.name().to_string(), krate.version().to_string()))
                .collect();
            let original_crates_list = original_crates_set.iter();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "original_crates_list"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in original_crates_list {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let files = collect_files(workspace_path);
            let mut chosen_packages: HashMap<_, _> = crates_list
                .iter()
                .map(|krate| (krate.name().to_string(), false))
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "The number of packages in the initial list: {0}.",
                            chosen_packages.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let strings = loader.load_strings();
            let builds = loader.load_builds_as_vec();
            let package_names = loader.load_package_names();
            let package_versions = loader.load_package_versions();
            let crate_names = loader.load_crate_names();
            let editions = loader.load_editions();
            let crate_types = loader.load_build_crate_types();
            let crate_types: HashMap<_, Vec<_>> = crate_types
                .iter()
                .safe_group_by(|&(build, _)| build)
                .into_iter()
                .map(|(build, group)| {
                    (build, group.map(|(_build, crate_type)| *crate_type).collect())
                })
                .collect();
            let mut build_script_builds = Vec::new();
            let mut build_script_builds_relation = Vec::new();
            let mut selected_builds_relation = Vec::new();
            let mut selected_builds = Vec::new();
            let mut candidate_builds_relation = Vec::new();
            let mut candidate_builds = Vec::new();
            let mut included_packages = HashSet::new();
            let mut all_builds = Vec::new();
            let mut drop_count = 0;
            let mut ignored_builds_count = 0;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Number of builds in total: {0}", builds.len()),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            for &(build, package, version, krate, crate_hash, edition) in builds.iter() {
                let krate_str = strings.r(crate_names.r(krate)).to_string();
                let package_str = strings.r(package_names.r(package)).clone();
                let version_str = strings.r(package_versions.r(version)).clone();
                let edition_str = strings.r(editions.r(edition)).to_string();
                all_builds
                    .push((
                        build,
                        package_str.clone(),
                        version_str.clone(),
                        krate_str.clone(),
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0:x}", crate_hash),
                            );
                            res
                        }),
                        edition_str.clone(),
                    ));
                if krate_str.starts_with("build_script_") {
                    build_script_builds_relation
                        .push((build, package, version, krate, crate_hash, edition));
                    build_script_builds
                        .push((
                            build,
                            package_str,
                            version_str,
                            krate_str,
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0:x}", crate_hash),
                                );
                                res
                            }),
                            edition_str,
                        ));
                } else {
                    let crate_types = if let Some(types) = crate_types.get(&build) {
                        types
                            .iter()
                            .map(|&crate_type| strings.r(crate_type))
                            .sorted()
                            .join(", ")
                    } else {
                        String::from("")
                    };
                    let directory_name = ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}-{1}", package_str, version_str),
                        );
                        res
                    });
                    let file_name = ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}_{1:x}.bincode", krate_str, crate_hash),
                        );
                        res
                    });
                    if files.contains(&(directory_name, file_name)) {
                        let is_package_chosen = chosen_packages
                            .get_mut(&package_str)
                            .unwrap();
                        let key = (package_str, version_str);
                        if !original_crates_set.contains(&key) {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "Including unexpected package version: {0:?}",
                                        key,
                                    ),
                                );
                            }
                        }
                        let (package_str, version_str) = key;
                        let included_key = (package_str, krate_str, crate_types);
                        if !included_packages.contains(&included_key) {
                            included_packages.insert(included_key.clone());
                        } else {
                            let (package_str, krate_str, crate_types) = included_key;
                            if crate_types == "proc-macro"
                            {} else if package_str == "charmhelpers"
                                && version_str == "0.1.3"
                            {} else {
                                {
                                    let lvl = ::log::Level::Error;
                                    if lvl <= ::log::STATIC_MAX_LEVEL
                                        && lvl <= ::log::max_level()
                                    {
                                        ::log::__private_api::log(
                                            format_args!(
                                                "Dropping build to avoid duplicate entries: {0} {1} / {2} {3:x} {4}.",
                                                package_str,
                                                version_str,
                                                krate_str,
                                                crate_hash,
                                                crate_types,
                                            ),
                                            lvl,
                                            &(
                                                "corpus_manager::queries::prepare_builds",
                                                "corpus_manager::queries::prepare_builds",
                                                ::log::__private_api::loc(),
                                            ),
                                            (),
                                        );
                                    }
                                };
                                drop_count += 1;
                            }
                            continue;
                        }
                        let (package_str, krate_str, crate_types) = included_key;
                        selected_builds_relation
                            .push((build, package, version, krate, crate_hash, edition));
                        selected_builds
                            .push((
                                build,
                                package_str.clone(),
                                version_str,
                                krate_str,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:x}", crate_hash),
                                    );
                                    res
                                }),
                                edition_str,
                                crate_types,
                            ));
                        *is_package_chosen = true;
                    } else {
                        candidate_builds_relation
                            .push((build, package, version, krate, crate_hash, edition));
                        candidate_builds
                            .push((
                                build,
                                package_str.clone(),
                                version_str,
                                krate_str,
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0:x}", crate_hash),
                                    );
                                    res
                                }),
                                edition_str,
                                crate_types,
                            ));
                        ignored_builds_count += 1;
                    }
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Included builds with the default configuration: {0}",
                            included_packages.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            match (&drop_count, &0) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::Some(
                                format_args!("There were duplicates!"),
                            ),
                        );
                    }
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unmatched builds after choosing only the default ones: {0}",
                            ignored_builds_count,
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of selected builds after choosing only the default ones: {0}",
                            selected_builds_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of build scripts: {0}",
                            build_script_builds_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let mut taken_candidates = HashSet::new();
            for (relation, row) in candidate_builds_relation
                .into_iter()
                .zip(candidate_builds.into_iter())
            {
                let (
                    build,
                    package_str,
                    version_str,
                    krate_str,
                    crate_hash,
                    edition_str,
                    crate_types,
                ) = row;
                let key = (package_str, version_str);
                if !original_crates_set.contains(&key) {
                    continue;
                }
                let (package_str, version_str) = key;
                let key = (package_str, krate_str, crate_types);
                if taken_candidates.contains(&key) {
                    let (package_str, krate_str, crate_types) = key;
                    {
                        let lvl = ::log::Level::Info;
                        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                            ::log::__private_api::log(
                                format_args!(
                                    "Duplicate candidate: {0} {1} / {2} {3} {4}.",
                                    package_str,
                                    version_str,
                                    krate_str,
                                    crate_hash,
                                    crate_types,
                                ),
                                lvl,
                                &(
                                    "corpus_manager::queries::prepare_builds",
                                    "corpus_manager::queries::prepare_builds",
                                    ::log::__private_api::loc(),
                                ),
                                (),
                            );
                        }
                    };
                } else if !included_packages.contains(&key) {
                    included_packages.insert(key.clone());
                    taken_candidates.insert(key.clone());
                    let (package_str, krate_str, crate_types) = key;
                    selected_builds
                        .push((
                            build,
                            package_str,
                            version_str,
                            krate_str,
                            crate_hash,
                            edition_str,
                            crate_types,
                        ));
                    selected_builds_relation.push(relation);
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Included builds with additional ones from the dependencies: {0}",
                            included_packages.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unmatched builds after choosing only the default ones: {0}",
                            ignored_builds_count,
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of selected builds after choosing only the default ones: {0}",
                            selected_builds_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_builds(selected_builds_relation);
            loader.store_build_script_builds(build_script_builds_relation);
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_builds"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_builds {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "build_script_builds"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in build_script_builds {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "all_builds"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in all_builds {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let mut chosen_packages_count = 0;
            let mut not_chosen_packages_count = 0;
            for val in chosen_packages.values() {
                if *val {
                    chosen_packages_count += 1;
                } else {
                    not_chosen_packages_count += 1;
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of packages from which at least one build was selected: {0}",
                            chosen_packages_count,
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of packages from which no build was selected: {0}",
                            not_chosen_packages_count,
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_builds",
                            "corpus_manager::queries::prepare_builds",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
    }
    mod prepare_items {
        use corpus_database::tables::Loader;
        use corpus_queries_derive::datapond_query;
        use log::info;
        use std::collections::HashMap;
        fn compute_selected_modules(loader: &Loader) {
            let selected_modules;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(selected_builds, root_modules, submodules), } output\nselected_modules(build: Build, module: Module) selected_modules(build, module)\n:- selected_builds(.build=build),\nroot_modules(build, module).selected_modules(build, module) :-\nselected_modules(build, parent), submodules(.parent=parent, .child=module).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let selected_builds = loader.load_selected_builds().clone();
                    let root_modules = loader.load_root_modules().clone();
                    let submodules = loader.load_submodules().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_selected_builds = datafrog::Relation::<
                            (Build, Package, PackageVersion, Krate, CrateHash, Edition),
                        >::from_vec(selected_builds);
                        let var_root_modules = datafrog::Relation::<
                            (Build, Module),
                        >::from_vec(root_modules);
                        let var_submodules = datafrog::Relation::<
                            (DefPath, Module, Module, Name, TyVisibility, Abi),
                        >::from_vec(submodules);
                        let var_selected_modules = iteration
                            .variable::<(Build, Module)>("selected_modules");
                        let var_selected_builds_1 = iteration
                            .variable::<
                                (Build, Package, PackageVersion, Krate, CrateHash, Edition),
                            >("selected_builds_1");
                        let var_root_modules_2 = iteration
                            .variable::<(Build, Module)>("root_modules_2");
                        let var_selected_builds_1_3 = iteration
                            .variable::<((Build,), ())>("selected_builds_1_3");
                        let var_root_modules_2_4 = iteration
                            .variable::<((Build,), (Module,))>("root_modules_2_4");
                        let var_selected_modules_5 = iteration
                            .variable::<(Build, Module)>("selected_modules_5");
                        let var_submodules_6 = iteration
                            .variable::<
                                (DefPath, Module, Module, Name, TyVisibility, Abi),
                            >("submodules_6");
                        let var_selected_modules_7 = iteration
                            .variable::<((Module,), (Build,))>("selected_modules_7");
                        let var_submodules_6_8 = iteration
                            .variable::<((Module,), (Module,))>("submodules_6_8");
                        let var_selected_modules_9 = iteration
                            .variable::<(Module, Build, Module)>("selected_modules_9");
                        var_selected_builds_1.insert(var_selected_builds);
                        var_root_modules_2.insert(var_root_modules);
                        var_submodules_6.insert(var_submodules);
                        while iteration.changed() {
                            var_selected_builds_1_3
                                .from_map(
                                    &var_selected_builds_1,
                                    |&(build, _, _, _, _, _)| ((build,), ()),
                                );
                            var_root_modules_2_4
                                .from_map(
                                    &var_root_modules_2,
                                    |&(build, module)| ((build,), (module,)),
                                );
                            var_selected_modules_5
                                .from_join(
                                    &var_selected_builds_1_3,
                                    &var_root_modules_2_4,
                                    |&(build,), &(), &(module,)| (build, module),
                                );
                            var_selected_modules
                                .from_map(
                                    &var_selected_modules_5,
                                    |&(build, module)| (build, module),
                                );
                            var_selected_modules_7
                                .from_map(
                                    &var_selected_modules,
                                    |&(build, parent)| ((parent,), (build,)),
                                );
                            var_submodules_6_8
                                .from_map(
                                    &var_submodules_6,
                                    |&(_, parent, module, _, _, _)| ((parent,), (module,)),
                                );
                            var_selected_modules_9
                                .from_join(
                                    &var_selected_modules_7,
                                    &var_submodules_6_8,
                                    |&(parent,), &(build,), &(module,)| (parent, build, module),
                                );
                            var_selected_modules
                                .from_map(
                                    &var_selected_modules_9,
                                    |&(parent, build, module)| (build, module),
                                );
                        }
                        selected_modules = var_selected_modules.complete();
                    }
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Computed modules."),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_items",
                            "corpus_manager::queries::prepare_items",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_modules(selected_modules.elements);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Stored modules."),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_items",
                            "corpus_manager::queries::prepare_items",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
        /// Check that `(krate, crate_hash)` is a key. In other words, check that
        /// `(krate, crate_hash)` uniquely identifies `build`.
        fn check_crate_with_hash_is_key(loader: &Loader) {
            let builds = loader.load_builds_as_vec();
            let mut keys = HashMap::new();
            for &(build, _package, _version, krate, crate_hash, _edition) in builds
                .iter()
            {
                let key = (krate, crate_hash);
                if !!keys.contains_key(&key) {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Duplicate for: {0:?} {1:?}", krate, crate_hash),
                        );
                    }
                }
                keys.insert(key, build);
            }
        }
        fn compute_selected_functions_and_mir_cfgs(loader: &Loader) {
            check_crate_with_hash_is_key(loader);
            let selected_builds = loader.load_selected_builds();
            let def_paths = loader.load_def_paths();
            let selected_mir_cfgs = super::utils::filter_selected(
                loader.load_iter_mir_cfgs(),
                &selected_builds,
                &def_paths,
                |(_item, body_def_path, _root_scope)| body_def_path,
                |build, (item, body_def_path, root_scope)| (
                    build,
                    item,
                    body_def_path,
                    root_scope,
                ),
            );
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "selected_mir_cfgs.len = {0}",
                            selected_mir_cfgs.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_items",
                            "corpus_manager::queries::prepare_items",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_mir_cfgs(selected_mir_cfgs);
            let selected_thir_bodies: Vec<
                (
                    corpus_database::types::Build,
                    corpus_database::types::Item,
                    corpus_database::types::DefPath,
                    corpus_database::types::ThirBlock,
                ),
            > = super::utils::filter_selected(
                loader.load_iter_thir_bodies(),
                &selected_builds,
                &def_paths,
                |(_item, body_def_path, _root_block)| body_def_path,
                |build, (item, body_def_path, root_block)| (
                    build,
                    item,
                    body_def_path,
                    root_block,
                ),
            );
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "selected_thir_bodies.len = {0}",
                            selected_thir_bodies.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::prepare_items",
                            "corpus_manager::queries::prepare_items",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_selected_thir_bodies(selected_thir_bodies);
            let function_unsafe_use: HashMap<_, _> = loader
                .load_iter_function_unsafe_use()
                .collect();
            let selected_functions = super::utils::filter_selected(
                loader.load_iter_function_definitions(),
                &selected_builds,
                &def_paths,
                |(_item, def_path, _module, _visibility, _unsafety, _abi, _return_ty)| {
                    def_path
                },
                |build, (item, def_path, module, visibility, unsafety, abi, return_ty)| {
                    let uses_unsafe = function_unsafe_use
                        .get(&def_path)
                        .cloned()
                        .unwrap_or(false);
                    (
                        build,
                        item,
                        def_path,
                        module,
                        visibility,
                        unsafety,
                        abi,
                        return_ty,
                        uses_unsafe,
                    )
                },
            );
            loader.store_selected_function_definitions(selected_functions);
        }
        /// Prepare the list of functions, which belong to the builds we want
        /// to analyse (`selected_builds`).
        pub fn query(loader: &Loader) {
            compute_selected_modules(loader);
            compute_selected_functions_and_mir_cfgs(loader);
        }
    }
    mod resolved_calls {
        //! Report information about calls in our codebase.
        //! For trait methods whose receiver is statically known, report this resolved type rather than the trait.
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use itertools::Itertools;
        use std::collections::HashMap;
        use std::path::Path;
        /// Gathers data on all calls made.
        /// This query reports, for each call, descriptions of:
        /// - the receiver of a trait method (if applicable) & its generics
        /// - the call's target method & the generics of its type & function
        /// - the crate of the call site and of the called function
        /// - if applicable, the first macro involved in the call stack that is not from the calling crate
        ///
        /// The produced table is deduplicated, with each row instead saying how many times it occurs.
        ///
        /// As an example, for a call in crate `foo` to `Option::map` mapping an optional string slice (`&str`) to an owned `String`, these would be:
        /// - empty strings for the receiver and its generics, since this is not part of a trait.
        /// - `core::option::Option<T>::map`, `&str`, and `String, $fn`
        /// - `foo` and `core` (the crates involved)
        ///
        /// Note that function references & closures are always reported as `$fn`.
        /// Further, and perhaps unexpectedly, the path to the target includes generic parameters, but they are simply what the corresponding `impl` block calls them, not the actual types used---these are found in the type generics (here, `&str` is the value of `T`).
        pub fn query(loader: &Loader, report_path: &Path) {
            let call_target = loader.load_terminators_call_const_target_as_map();
            let call_target_self = loader
                .load_terminators_call_const_target_self_as_map();
            let call_target_desc: HashMap<_, _> = loader
                .load_terminators_call_const_target_desc()
                .iter()
                .copied()
                .map(|(call, desc, function_generics, type_generics)| {
                    (call, (desc, function_generics, type_generics))
                })
                .collect();
            let call_target_macro = loader
                .load_terminators_call_macro_backtrace_as_map();
            let strings = loader.load_strings();
            let def_paths = loader.load_def_paths();
            let crate_names = loader.load_crate_names();
            let type_descriptions: HashMap<_, _> = loader
                .load_type_description()
                .iter()
                .copied()
                .map(|(ty, desc, generics)| (ty, (desc, generics)))
                .collect();
            let basic_block_def_paths: HashMap<_, _> = loader
                .load_basic_blocks()
                .iter()
                .map(|&(bb, def_path, _kind)| (bb, def_path))
                .collect();
            let all_calls = loader.load_terminators_call();
            let all_calls = all_calls
                .iter()
                .filter_map(|
                    &(
                        block,
                        call,
                        _func,
                        _unsafety,
                        _abi,
                        _return_ty,
                        _destination,
                        _span,
                    )|
                {
                    let target = call_target.get(&call)?;
                    let (target_desc, function_generics, type_generics) = call_target_desc[&call];
                    let (caller_crate, _, _, _, _) = def_paths
                        .r(basic_block_def_paths[&block]);
                    let caller_crate_name = strings.r(crate_names.r(caller_crate));
                    let (target_crate, _, _, _, _) = def_paths.r(*target);
                    let target_crate_name = strings.r(crate_names.r(target_crate));
                    let (receiver_name, receiver_generics) = call_target_self
                        .get(&call)
                        .map_or_else(
                            || ("".to_string(), "".to_string()),
                            |typ| {
                                let (desc, generics) = type_descriptions[typ];
                                (strings.r(desc), strings.r(generics))
                            },
                        );
                    let macro_path = call_target_macro
                        .get(&call)
                        .map_or("".to_string(), |path| strings.r(*path));
                    Some((
                        receiver_name,
                        receiver_generics,
                        strings.r(target_desc),
                        strings.r(type_generics),
                        strings.r(function_generics),
                        caller_crate_name,
                        target_crate_name,
                        macro_path,
                    ))
                });
            let counts: HashMap<_, i32> = all_calls
                .fold(
                    HashMap::new(),
                    |mut counts, row| {
                        *counts.entry(row).or_insert(0) += 1;
                        counts
                    },
                );
            let all_calls = counts
                .iter()
                .map(|((a, b, c, d, e, f, g, h), count)| (
                    a,
                    b,
                    c,
                    d,
                    e,
                    f,
                    g,
                    h,
                    count,
                ));
            let all_calls: Vec<_> = all_calls
                .sorted_by_key(|(_, _, target, ..)| target.clone())
                .collect();
            let cross_crate_calls = all_calls
                .iter()
                .filter(|&(_, _, _, _, _, caller_crate, target_crate, _, _)| {
                    caller_crate != target_crate
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "cross_crate_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in cross_crate_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "all_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in all_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod size {
        //! Report unsafe block sizes by MIR statements.
        use super::utils::BuildResolver;
        use super::utils::GroupByIterator;
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use corpus_database::types::ThirBlock;
        use corpus_database::types::ThirExpr;
        use corpus_queries_derive::datapond_query;
        use std::collections::{HashMap, HashSet};
        use std::path::Path;
        pub fn query(loader: &Loader, report_path: &Path) {
            let mut seen_scopes = HashSet::new();
            let build_resolver = BuildResolver::new(loader);
            let unsafe_statements = loader.load_unsafe_statements();
            let unsafe_blocks_by_stmts = unsafe_statements
                .iter()
                .safe_group_by(|
                    (build, _stmt, _block, _index, _kind, unsafe_scope, check_mode)|
                { (build, unsafe_scope, check_mode) });
            let unsafe_blocks_sizes_by_stmts: Vec<_> = unsafe_blocks_by_stmts
                .into_iter()
                .map(|((build, unsafe_scope, check_mode), group)| {
                    if !!seen_scopes.contains(unsafe_scope) {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "duplicate scope: {0:?} {1:?}",
                                    build,
                                    unsafe_scope,
                                ),
                            );
                        }
                    }
                    seen_scopes.insert(unsafe_scope);
                    (*build, unsafe_scope, check_mode, group.count())
                })
                .collect();
            let unsafe_terminators = loader.load_unsafe_terminators();
            let unsafe_blocks_by_terminators = unsafe_terminators
                .iter()
                .safe_group_by(|(build, _block, _kind, unsafe_scope, check_mode)| (
                    build,
                    unsafe_scope,
                    check_mode,
                ));
            let unsafe_blocks_sizes_by_terminators: HashMap<_, _> = unsafe_blocks_by_terminators
                .into_iter()
                .map(|((build, unsafe_scope, check_mode), group)| {
                    ((*build, unsafe_scope, check_mode), group.count())
                })
                .collect();
            let mut unsafe_block_sizes: Vec<_> = unsafe_blocks_sizes_by_stmts
                .into_iter()
                .map(|(build, unsafe_scope, check_mode, statement_count)| {
                    let terminator_count = unsafe_blocks_sizes_by_terminators
                        .get(&(build, unsafe_scope, check_mode))
                        .cloned()
                        .unwrap_or(0);
                    (build, unsafe_scope, check_mode, statement_count, terminator_count)
                })
                .collect();
            for ((build, unsafe_scope, check_mode), terminator_count) in unsafe_blocks_sizes_by_terminators {
                if !seen_scopes.contains(unsafe_scope) {
                    unsafe_block_sizes
                        .push((build, unsafe_scope, check_mode, 0, terminator_count))
                }
            }
            let unsafe_block_sizes = unsafe_block_sizes
                .into_iter()
                .map(|
                    (build, unsafe_scope, check_mode, statement_count, terminator_count)|
                {
                    (
                        build,
                        build_resolver.resolve(build),
                        unsafe_scope,
                        check_mode.to_string(),
                        statement_count,
                        terminator_count,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_block_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_block_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn new_query(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let mut unsafe_thir_blocks_sizes_by_stmts_map: HashMap<ThirBlock, usize> = HashMap::new();
            for (_build, _stmt, block, _index, _check_mode) in loader
                .load_iter_unsafe_thir_stmts()
            {
                let count = unsafe_thir_blocks_sizes_by_stmts_map
                    .entry(block)
                    .or_insert(0);
                *count += 1;
            }
            let no_thir_expr: ThirExpr = 0u64.into();
            let mut trailing_exprs: HashSet<ThirExpr> = HashSet::new();
            for (_block, expr) in loader.load_iter_thir_block_expr() {
                if expr != no_thir_expr {
                    trailing_exprs.insert(expr);
                }
            }
            let mut thir_exprs_call: HashSet<ThirExpr> = HashSet::new();
            for (expr, _ty, _fun, _safety, _abi, _retty) in loader
                .load_iter_thir_exprs_call()
            {
                thir_exprs_call.insert(expr);
            }
            let mut unsafe_block_to_count_trailing_expr: HashMap<ThirBlock, usize> = HashMap::new();
            let mut unsafe_thir_blocks_to_call_expr_count: HashMap<ThirBlock, usize> = HashMap::new();
            for (expr, _block, closest_unsafe_block, _, _) in loader
                .load_iter_thir_exprs()
            {
                if trailing_exprs.contains(&expr) {
                    let count = unsafe_block_to_count_trailing_expr
                        .entry(closest_unsafe_block)
                        .or_insert(0);
                    *count += 1;
                }
                if thir_exprs_call.contains(&expr) {
                    let count = unsafe_thir_blocks_to_call_expr_count
                        .entry(closest_unsafe_block)
                        .or_insert(0);
                    *count += 1;
                }
            }
            let unsafe_thir_block_sizes = loader
                .load_iter_unsafe_thir_blocks()
                .map(|(build, _def_path, block, _span_expansion, check_mode, _span)| {
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        check_mode.to_string(),
                        unsafe_thir_blocks_sizes_by_stmts_map
                            .get(&block)
                            .copied()
                            .unwrap_or(0),
                        unsafe_thir_blocks_to_call_expr_count
                            .get(&block)
                            .copied()
                            .unwrap_or(0),
                        unsafe_block_to_count_trailing_expr
                            .get(&block)
                            .copied()
                            .unwrap_or(0),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_block_sizes"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_block_sizes {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod traits {
        //! Report information about traits and their implementations.
        use super::utils::DefPathResolver;
        use super::utils::GroupByIterator;
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use log::info;
        use std::collections::HashMap;
        use std::path::Path;
        pub fn query(loader: &Loader, report_path: &Path) {
            let selected_builds = loader.load_selected_builds();
            let def_paths = loader.load_def_paths();
            let all_traits_relation = loader.load_traits();
            let def_path_resolver = DefPathResolver::new(loader);
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("Loaded relations."),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let all_traits = all_traits_relation
                .iter()
                .map(|
                    &(item, def_path, _name, visibility, is_auto, is_marker, unsafety)|
                {
                    (
                        def_path_resolver.resolve(def_path),
                        item,
                        visibility.to_string(),
                        unsafety.to_string(),
                        is_auto,
                        is_marker,
                    )
                });
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Writing CSV of all_traits.len={0}",
                            all_traits.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "all_traits"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in all_traits {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let selected_traits_relation = super::utils::filter_selected(
                all_traits_relation.iter(),
                &selected_builds,
                &def_paths,
                |
                    &(
                        _item,
                        def_path,
                        _name,
                        _visibility,
                        _is_auto,
                        _is_marker,
                        _unsafety,
                    )|
                def_path,
                |
                    build,
                    &(item, def_path, name, visibility, is_auto, is_marker, unsafety)|
                {
                    (
                        build,
                        item,
                        def_path,
                        name,
                        visibility,
                        is_auto,
                        is_marker,
                        unsafety,
                    )
                },
            );
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "selected_traits.len = {0}",
                            selected_traits_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let trait_impls = loader.load_trait_impls();
            let trait_impl_counts: HashMap<_, _> = trait_impls
                .iter()
                .safe_group_by(|(_item, _typ, trait_def_path)| trait_def_path)
                .into_iter()
                .map(|(key, group)| (key, group.count()))
                .collect();
            let selected_traits = selected_traits_relation
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        def_path,
                        _name,
                        visibility,
                        is_auto,
                        is_marker,
                        unsafety,
                    )|
                {
                    (
                        build,
                        def_path_resolver.resolve(def_path),
                        item,
                        visibility.to_string(),
                        unsafety.to_string(),
                        is_auto,
                        is_marker,
                        trait_impl_counts.get(&def_path).cloned().unwrap_or(0),
                    )
                });
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Writing CSV of selected_traits.len={0}",
                            selected_traits.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_traits"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_traits {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let selected_impl_definitions_relation = super::utils::filter_selected(
                loader.load_iter_impl_definitions(),
                &selected_builds,
                &def_paths,
                |
                    (
                        def_path,
                        _item,
                        _module,
                        _name,
                        _visibility,
                        _unsafety,
                        _polarity,
                        _defaultness,
                        _constness,
                        _typ,
                    )|
                def_path,
                |
                    build,
                    (
                        def_path,
                        item,
                        module,
                        name,
                        visibility,
                        unsafety,
                        polarity,
                        defaultness,
                        constness,
                        typ,
                    )|
                {
                    (
                        build,
                        def_path,
                        item,
                        module,
                        name,
                        visibility,
                        unsafety,
                        polarity,
                        defaultness,
                        constness,
                        typ,
                    )
                },
            );
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "selected_impl_definitions.len = {0}",
                            selected_impl_definitions_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let impl_traits: HashMap<_, _> = trait_impls
                .iter()
                .map(|(item, _typ, trait_def_path)| (item, trait_def_path))
                .collect();
            let selected_impl_definitions = selected_impl_definitions_relation
                .into_iter()
                .flat_map(|
                    (
                        build,
                        def_path,
                        item,
                        _module,
                        _name,
                        visibility,
                        unsafety,
                        polarity,
                        defaultness,
                        constness,
                        _typ,
                    )|
                {
                    impl_traits
                        .get(&item)
                        .map(|&trait_def_path| {
                            (
                                build,
                                def_path_resolver.resolve(def_path),
                                item,
                                visibility.to_string(),
                                unsafety.to_string(),
                                polarity.to_string(),
                                defaultness.to_string(),
                                constness.to_string(),
                                def_path_resolver.resolve(*trait_def_path),
                            )
                        })
                });
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Writing CSV of selected impl definitions of traits",
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::traits",
                            "corpus_manager::queries::traits",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_impl_definitions"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_impl_definitions {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod types {
        use super::utils::{BuildResolver, DefPathResolver};
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use log::info;
        use std::collections::HashMap;
        use std::path::Path;
        /// Collect general information about types.
        pub fn query(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let selected_builds = loader.load_selected_builds();
            let def_paths = loader.load_def_paths();
            let strings = loader.load_strings();
            let type_defs = loader.load_type_defs();
            let type_kinds = loader.load_type_kinds();
            let types = loader.load_types_redb_map();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of all type definitions (type_defs): {0}",
                            type_defs.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::types",
                            "corpus_manager::queries::types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let selected_type_defs_relation = super::utils::filter_selected(
                type_defs.iter(),
                &selected_builds,
                &def_paths,
                |&(_item, _typ, def_path, _name, _visibility, _kind)| def_path,
                |build, &(item, typ, def_path, name, visibility, kind)| {
                    (build, item, typ, def_path, name, visibility, types.r(typ), kind)
                },
            );
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of selected type definitions (type_defs): {0}",
                            selected_type_defs_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::types",
                            "corpus_manager::queries::types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let selected_type_defs = selected_type_defs_relation
                .iter()
                .map(|
                    &(build, item, typ, def_path, name, visibility, type_kind, def_kind)|
                {
                    (
                        build_resolver.resolve(build),
                        item,
                        typ,
                        def_path_resolver.resolve(def_path),
                        strings.r(name),
                        visibility.to_string(),
                        strings.r(type_kinds.r(type_kind)),
                        def_kind.to_string(),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_type_defs"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_type_defs {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let adts = loader.load_types_adt_def_redb_map();
            let selected_adts_relation: Vec<_> = selected_type_defs_relation
                .iter()
                .flat_map(|
                    &(build, item, typ, def_path, name, visibility, type_kind, def_kind)|
                {
                    adts.get_redb(typ)
                        .map(|(resolved_def_path, kind, c_repr, is_phantom)| {
                            (
                                build,
                                item,
                                typ,
                                def_path,
                                resolved_def_path,
                                name,
                                visibility,
                                type_kind,
                                def_kind,
                                kind,
                                c_repr,
                                is_phantom,
                            )
                        })
                })
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of selected ADTs: {0}",
                            selected_adts_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::types",
                            "corpus_manager::queries::types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let selected_adts = selected_adts_relation
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        typ,
                        def_path,
                        resolved_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                    )|
                {
                    (
                        build_resolver.resolve(build),
                        item,
                        typ,
                        def_path_resolver.resolve(def_path),
                        def_path_resolver.resolve(resolved_def_path),
                        strings.r(name),
                        visibility.to_string(),
                        strings.r(type_kinds.r(type_kind)),
                        def_kind.to_string(),
                        kind.to_string(),
                        c_repr,
                        is_phantom,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_adts"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_adts {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            let selected_adts_map: HashMap<_, _> = selected_adts_relation
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        typ,
                        def_path,
                        resolved_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                    )|
                {
                    (
                        typ,
                        (
                            build,
                            item,
                            def_path,
                            resolved_def_path,
                            name,
                            visibility,
                            type_kind,
                            def_kind,
                            kind,
                            c_repr,
                            is_phantom,
                        ),
                    )
                })
                .collect();
            let types_adt_field = loader.load_types_adt_field();
            let selected_adt_field_types_relation: Vec<_> = types_adt_field
                .iter()
                .flat_map(|
                    &(
                        _field,
                        adt,
                        adt_variant,
                        field_def_path,
                        field_name,
                        field_visibility,
                        field_type,
                    )|
                {
                    selected_adts_map
                        .get(&adt)
                        .map(|
                            &(
                                build,
                                item,
                                adt_def_path,
                                resolved_adt_def_path,
                                name,
                                visibility,
                                type_kind,
                                def_kind,
                                kind,
                                c_repr,
                                is_phantom,
                            )|
                        {
                            (
                                build,
                                item,
                                adt,
                                adt_variant,
                                adt_def_path,
                                resolved_adt_def_path,
                                field_def_path,
                                name,
                                visibility,
                                type_kind,
                                def_kind,
                                kind,
                                c_repr,
                                is_phantom,
                                field_name,
                                field_visibility,
                                field_type,
                                types.r(field_type),
                            )
                        })
                })
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of selected ADT fields: {0}",
                            selected_adt_field_types_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::types",
                            "corpus_manager::queries::types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let selected_adt_field_types = selected_adt_field_types_relation
                .iter()
                .map(|
                    &(
                        build,
                        item,
                        adt,
                        adt_variant,
                        adt_def_path,
                        resolved_adt_def_path,
                        field_def_path,
                        name,
                        visibility,
                        type_kind,
                        def_kind,
                        kind,
                        c_repr,
                        is_phantom,
                        field_name,
                        field_visibility,
                        field_type,
                        field_type_kind,
                    )|
                {
                    (
                        build_resolver.resolve(build),
                        item,
                        adt,
                        adt_variant,
                        def_path_resolver.resolve(adt_def_path),
                        def_path_resolver.resolve(resolved_adt_def_path),
                        def_path_resolver.resolve(field_def_path),
                        (
                            strings.r(name),
                            visibility.to_string(),
                            strings.r(type_kinds.r(type_kind)),
                            def_kind.to_string(),
                            kind.to_string(),
                            c_repr,
                            is_phantom,
                        ),
                        strings.r(field_name),
                        field_visibility.to_string(),
                        field_type,
                        strings.r(type_kinds.r(field_type_kind)),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "selected_adt_field_types"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in selected_adt_field_types {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            loader.store_selected_adts(selected_adts_relation);
            loader.store_selected_type_defs(selected_type_defs_relation);
            loader.store_selected_adt_field_types(selected_adt_field_types_relation);
        }
    }
    mod unsafe_block_calls {
        //! Report information about calls in our codebase. For calls from unsafe blocks
        //! report additional information.
        use super::utils::{BuildResolver, SpanResolver};
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use std::collections::{HashMap, HashSet};
        use std::path::Path;
        /// Report information about calls from unsafe blocks.
        fn report_unsafe_block_calls(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let span_resolver = SpanResolver::new(loader);
            let def_paths = loader.load_def_paths();
            let terminators_call_const_target = loader
                .load_terminators_call_const_target_as_map();
            let crate_names = loader.load_crate_names();
            let relative_def_paths = loader.load_relative_def_paths();
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let summary_keys = loader.load_summary_keys();
            let subscopes = loader.load_subscopes();
            let scope_spans: HashMap<_, _> = subscopes
                .iter()
                .map(|
                    &(
                        _parent,
                        child,
                        _safety,
                        _check_mode,
                        _explicit_unsafe_group,
                        span,
                    )|
                (child, span))
                .collect();
            let unsafe_block_calls = loader.load_unsafe_block_calls();
            let unsafe_block_calls = unsafe_block_calls
                .iter()
                .map(|
                    &(
                        build,
                        block,
                        unsafe_scope,
                        check_mode,
                        call,
                        unsafety,
                        abi,
                        _return_ty,
                    )|
                {
                    let (
                        target_crate_name,
                        target_crate_hash,
                        call_target_def_path,
                        call_target,
                        is_trait_item,
                    ) = if let Some(target) = terminators_call_const_target.get(&call) {
                        let (
                            crate_name,
                            crate_hash,
                            relative_def_path,
                            _def_path_hash,
                            summary_key,
                        ) = def_paths.r(*target);
                        (
                            strings.r(crate_names.r(crate_name)),
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0:x}", crate_hash),
                                );
                                res
                            }),
                            strings.r(relative_def_paths.r(relative_def_path)),
                            strings.r(summary_keys.r(summary_key)),
                            trait_items.contains(target),
                        )
                    } else {
                        (
                            "non-const".into(),
                            "non-const".into(),
                            "non-const".into(),
                            "non-const".into(),
                            false,
                        )
                    };
                    let unsafe_scope_span = scope_spans[&unsafe_scope];
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        unsafe_scope,
                        span_resolver.resolve(unsafe_scope_span),
                        check_mode.to_string(),
                        call,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        target_crate_name,
                        target_crate_hash,
                        call_target_def_path,
                        call_target,
                        is_trait_item,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_block_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_block_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        /// Report information about all calls in our codebase.
        fn report_all_calls(loader: &Loader, report_path: &Path) {
            let def_paths = loader.load_def_paths();
            let terminators_call_const_target = loader
                .load_terminators_call_const_target_as_map();
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let summary_keys = loader.load_summary_keys();
            let all_calls = loader.load_terminators_call();
            let all_calls = all_calls
                .iter()
                .map(|
                    &(
                        _block,
                        call,
                        func,
                        unsafety,
                        abi,
                        _return_ty,
                        _destination,
                        _span,
                    )|
                {
                    let (call_target, is_trait_item) = if let Some(target) = terminators_call_const_target
                        .get(&call)
                    {
                        let (
                            _crate_name,
                            _crate_hash,
                            _relative_def_path,
                            _def_path_hash,
                            summary_key,
                        ) = def_paths.r(*target);
                        (
                            strings.r(summary_keys.r(summary_key)),
                            trait_items.contains(target),
                        )
                    } else {
                        ("non-const".into(), false)
                    };
                    (
                        call,
                        func,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        call_target,
                        is_trait_item,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "all_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in all_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            report_unsafe_block_calls(loader, report_path);
            report_all_calls(loader, report_path);
        }
        /// Report information about calls from unsafe thir blocks.
        fn new_report_unsafe_block_calls(loader: &Loader, report_path: &Path) {
            let build_resolver = BuildResolver::new(loader);
            let span_resolver = SpanResolver::new(loader);
            let def_paths = loader.load_def_paths();
            let fun_to_const_target_map = loader
                .load_thir_exprs_call_const_target_redb_map();
            let crate_names = loader.load_crate_names();
            let relative_def_paths = loader.load_relative_def_paths();
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let summary_keys = loader.load_summary_keys();
            let unsafe_thir_block_calls = loader.load_iter_unsafe_thir_block_calls();
            let thir_block_map = loader.load_thir_blocks_redb_map();
            let thir_block_to_span = |block| {
                let (_parent, _, _, span) = thir_block_map.get_redb(block).unwrap();
                span
            };
            let unsafe_thir_block_calls = unsafe_thir_block_calls
                .map(|(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
                    let (
                        target_crate_name,
                        target_crate_hash,
                        call_target_def_path,
                        call_target,
                        is_trait_item,
                    ) = if let Some(target) = fun_to_const_target_map.get_redb(fun) {
                        let (
                            crate_name,
                            crate_hash,
                            relative_def_path,
                            _def_path_hash,
                            summary_key,
                        ) = def_paths.r(target);
                        (
                            strings.r(crate_names.r(crate_name)),
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("{0:x}", crate_hash),
                                );
                                res
                            }),
                            strings.r(relative_def_paths.r(relative_def_path)),
                            strings.r(summary_keys.r(summary_key)),
                            trait_items.contains(&target),
                        )
                    } else {
                        (
                            "non-const".into(),
                            "non-const".into(),
                            "non-const".into(),
                            "non-const".into(),
                            false,
                        )
                    };
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        span_resolver.resolve(thir_block_to_span(block)),
                        check_mode.to_string(),
                        call,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                        target_crate_name,
                        target_crate_hash,
                        call_target_def_path,
                        call_target,
                        is_trait_item,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_block_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_block_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        /// Report information about all thir calls in our codebase.
        fn new_report_all_calls(loader: &Loader, report_path: &Path) {
            let def_paths = loader.load_def_paths();
            let fun_to_const_target_map = loader
                .load_thir_exprs_call_const_target_redb_map();
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let trait_items = loader.load_trait_items();
            let trait_items: HashSet<_> = trait_items
                .iter()
                .map(|(_trait_id, def_path, _defaultness)| def_path)
                .collect();
            let summary_keys = loader.load_summary_keys();
            let all_calls = loader.load_iter_thir_exprs_call();
            let all_thir_calls = all_calls
                .map(|(call, _fun_type, fun, unsafety, abi, _return_ty)| {
                    let (call_target, is_trait_item) = if let Some(target) = fun_to_const_target_map
                        .get_redb(fun)
                    {
                        let (
                            _crate_name,
                            _crate_hash,
                            _relative_def_path,
                            _def_path_hash,
                            summary_key,
                        ) = def_paths.r(target);
                        (
                            strings.r(summary_keys.r(summary_key)),
                            trait_items.contains(&target),
                        )
                    } else {
                        ("non-const".into(), false)
                    };
                    (
                        call,
                        fun,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)).to_string(),
                        call_target,
                        is_trait_item,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "all_thir_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in all_thir_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn new_query(loader: &Loader, report_path: &Path) {
            new_report_unsafe_block_calls(loader, report_path);
            new_report_all_calls(loader, report_path);
        }
    }
    mod unsafe_block_groups {
        //! Report information about function calls.
        use super::utils::GroupByIterator;
        use super::utils::{BuildResolver, DefPathResolver};
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use corpus_queries_derive::datapond_query;
        use log::info;
        use std::collections::{HashMap, HashSet};
        use std::convert::TryInto;
        use std::path::Path;
        /// Count how many functions are called from each unsafe block.
        fn count_called_functions(loader: &Loader) {
            let unsafe_block_calls;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader { relations(unsafe_terminators, terminators_call), } output\nunsafe_block_calls(build: Build, block: BasicBlock, unsafe_scope: Scope,\ncheck_mode: BlockCheckMode, call: FunctionCall, unsafety: Safety, abi: Abi,\nreturn_ty: Type)\nunsafe_block_calls(build, block, unsafe_scope, check_mode, call, unsafety,\nabi, return_ty) :-\nunsafe_terminators(.build=build, .block=block, .unsafe_scope=unsafe_scope,\n.check_mode=check_mode),\nterminators_call(.block=block, .call=call, .unsafety=unsafety, .abi=abi,\n.return_ty=return_ty).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let unsafe_terminators = loader.load_unsafe_terminators().clone();
                    let terminators_call = loader.load_terminators_call().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_unsafe_terminators = datafrog::Relation::<
                            (Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode),
                        >::from_vec(unsafe_terminators);
                        let var_terminators_call = datafrog::Relation::<
                            (
                                BasicBlock,
                                FunctionCall,
                                Operand,
                                Safety,
                                Abi,
                                Type,
                                BasicBlock,
                                Span,
                            ),
                        >::from_vec(terminators_call);
                        let var_unsafe_block_calls = iteration
                            .variable::<
                                (
                                    Build,
                                    BasicBlock,
                                    Scope,
                                    BlockCheckMode,
                                    FunctionCall,
                                    Safety,
                                    Abi,
                                    Type,
                                ),
                            >("unsafe_block_calls");
                        let var_unsafe_terminators_1 = iteration
                            .variable::<
                                (Build, BasicBlock, TerminatorKind, Scope, BlockCheckMode),
                            >("unsafe_terminators_1");
                        let var_terminators_call_2 = iteration
                            .variable::<
                                (
                                    BasicBlock,
                                    FunctionCall,
                                    Operand,
                                    Safety,
                                    Abi,
                                    Type,
                                    BasicBlock,
                                    Span,
                                ),
                            >("terminators_call_2");
                        let var_unsafe_terminators_1_3 = iteration
                            .variable::<
                                ((BasicBlock,), (Build, Scope, BlockCheckMode)),
                            >("unsafe_terminators_1_3");
                        let var_terminators_call_2_4 = iteration
                            .variable::<
                                ((BasicBlock,), (FunctionCall, Safety, Abi, Type)),
                            >("terminators_call_2_4");
                        let var_unsafe_block_calls_5 = iteration
                            .variable::<
                                (
                                    BasicBlock,
                                    Build,
                                    Scope,
                                    BlockCheckMode,
                                    FunctionCall,
                                    Safety,
                                    Abi,
                                    Type,
                                ),
                            >("unsafe_block_calls_5");
                        var_unsafe_terminators_1.insert(var_unsafe_terminators);
                        var_terminators_call_2.insert(var_terminators_call);
                        while iteration.changed() {
                            var_unsafe_terminators_1_3
                                .from_map(
                                    &var_unsafe_terminators_1,
                                    |&(build, block, _, unsafe_scope, check_mode)| (
                                        (block,),
                                        (build, unsafe_scope, check_mode),
                                    ),
                                );
                            var_terminators_call_2_4
                                .from_map(
                                    &var_terminators_call_2,
                                    |&(block, call, _, unsafety, abi, return_ty, _, _)| (
                                        (block,),
                                        (call, unsafety, abi, return_ty),
                                    ),
                                );
                            var_unsafe_block_calls_5
                                .from_join(
                                    &var_unsafe_terminators_1_3,
                                    &var_terminators_call_2_4,
                                    |
                                        &(block,),
                                        &(build, unsafe_scope, check_mode),
                                        &(call, unsafety, abi, return_ty)|
                                    (
                                        block,
                                        build,
                                        unsafe_scope,
                                        check_mode,
                                        call,
                                        unsafety,
                                        abi,
                                        return_ty,
                                    ),
                                );
                            var_unsafe_block_calls
                                .from_map(
                                    &var_unsafe_block_calls_5,
                                    |
                                        &(
                                            block,
                                            build,
                                            unsafe_scope,
                                            check_mode,
                                            call,
                                            unsafety,
                                            abi,
                                            return_ty,
                                        )|
                                    (
                                        build,
                                        block,
                                        unsafe_scope,
                                        check_mode,
                                        call,
                                        unsafety,
                                        abi,
                                        return_ty,
                                    ),
                                );
                        }
                        unsafe_block_calls = var_unsafe_block_calls.complete();
                    }
                }
            }
            let unsafe_block_calls_relation = unsafe_block_calls.elements;
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of calls in unsafe blocks: {0}",
                            unsafe_block_calls_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_block_call_counts_relation: Vec<_> = unsafe_block_calls_relation
                .iter()
                .safe_group_by(|
                    &&(
                        build,
                        _block,
                        unsafe_scope,
                        check_mode,
                        _call,
                        _unsafety,
                        _abi,
                        _return_ty,
                    )|
                { (build, unsafe_scope, check_mode) })
                .into_iter()
                .map(|((build, unsafe_scope, check_mode), group)| {
                    (build, unsafe_scope, check_mode, group.count().try_into().unwrap())
                })
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unsafe blocks with calls: {0}",
                            unsafe_block_call_counts_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let blocks_with_calls: HashSet<_> = unsafe_block_call_counts_relation
                .iter()
                .map(|&(_build, unsafe_scope, _check_mode, _call_count)| unsafe_scope)
                .collect();
            let unsafe_block_no_calls_relation: Vec<_> = loader
                .load_unsafe_blocks()
                .iter()
                .filter(|
                    (
                        _build,
                        _mir_body_def_path,
                        scope,
                        _expansion_kind,
                        _check_mode,
                        _span,
                    )|
                { !blocks_with_calls.contains(scope) })
                .cloned()
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unsafe blocks with non-const calls: {0}",
                            unsafe_block_no_calls_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_unsafe_block_calls(unsafe_block_calls_relation);
            loader.store_unsafe_block_call_counts(unsafe_block_call_counts_relation);
            loader.store_unsafe_block_no_calls(unsafe_block_no_calls_relation);
        }
        /// Report how many function calls each unsafe block contains.
        fn report_called_functions(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_block_calls = loader.load_unsafe_block_calls();
            let unsafe_block_calls = unsafe_block_calls
                .iter()
                .map(|
                    &(
                        build,
                        block,
                        unsafe_scope,
                        check_mode,
                        call,
                        unsafety,
                        abi,
                        _return_ty,
                    )|
                {
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        unsafe_scope,
                        check_mode.to_string(),
                        call,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_block_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_block_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_block_calls"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_block_call_counts = loader.load_unsafe_block_call_counts();
            let unsafe_block_call_counts = unsafe_block_call_counts
                .iter()
                .map(|&(build, unsafe_scope, check_mode, call_count)| {
                    (
                        build,
                        build_resolver.resolve(build),
                        unsafe_scope,
                        check_mode.to_string(),
                        call_count,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_block_call_counts"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_block_call_counts {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_block_call_counts"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_block_no_calls = loader.load_unsafe_block_no_calls();
            let unsafe_block_no_calls = unsafe_block_no_calls
                .iter()
                .map(|
                    &(
                        build,
                        mir_body_def_path,
                        scope,
                        expansion_kind,
                        check_mode,
                        _span,
                    )|
                {
                    (
                        build,
                        build_resolver.resolve(build),
                        def_path_resolver.resolve(mir_body_def_path),
                        scope,
                        expansion_kind.to_string(),
                        check_mode.to_string(),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_block_no_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_block_no_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_block_no_calls"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
        /// Find all calls in unsafe functions that call non-constant targets. In other
        /// words, find all calls that call function pointers.
        fn report_non_const_call_targets(loader: &Loader, report_path: &Path) {
            let const_calls: HashSet<_> = loader
                .load_terminators_call_const_target()
                .iter()
                .map(|(call, _def_path)| *call)
                .collect();
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_block_calls = loader.load_unsafe_block_calls();
            let non_const_calls = unsafe_block_calls
                .iter()
                .flat_map(|
                    (
                        build,
                        block,
                        unsafe_scope,
                        check_mode,
                        call,
                        unsafety,
                        abi,
                        _return_ty,
                    )|
                {
                    if const_calls.contains(call) {
                        None
                    } else {
                        Some((
                            build,
                            build_resolver.resolve(*build),
                            block,
                            unsafe_scope,
                            check_mode.to_string(),
                            call,
                            unsafety.to_string(),
                            strings.r(abis.r(*abi)),
                        ))
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "non_const_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in non_const_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported non_const_call_targets"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
        /// Find all calls in unsafe functions that call constant targets. The call
        /// targets that appear as constants:
        ///
        /// 1. Static function calls.
        /// 2. Static method calls.
        /// 3. Dynamic calls on trait objects.
        /// 4. Calls of closures.
        fn report_const_call_targets(loader: &Loader, report_path: &Path) {
            let const_calls_map = loader.load_terminators_call_const_target_as_map();
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_block_calls = loader.load_unsafe_block_calls();
            let const_calls = unsafe_block_calls
                .iter()
                .flat_map(|
                    (
                        build,
                        block,
                        unsafe_scope,
                        check_mode,
                        call,
                        unsafety,
                        abi,
                        _return_ty,
                    )|
                {
                    const_calls_map
                        .get(call)
                        .map(|def_path| {
                            Some((
                                build,
                                build_resolver.resolve(*build),
                                def_path_resolver.resolve(*def_path),
                                block,
                                unsafe_scope,
                                check_mode.to_string(),
                                call,
                                unsafety.to_string(),
                                strings.r(abis.r(*abi)),
                            ))
                        })
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "const_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in const_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported const_call_targets"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            count_called_functions(loader);
            report_called_functions(loader, report_path);
            report_non_const_call_targets(loader, report_path);
            report_const_call_targets(loader, report_path);
        }
        /// Count how many functions are called from each unsafe thir block.
        fn new_count_called_functions(loader: &Loader) {
            let mut unsafe_blocks_to_data = HashMap::new();
            for (build, _, block, _, check_mode, _) in loader
                .load_iter_unsafe_thir_blocks()
            {
                unsafe_blocks_to_data.insert(block, (build, check_mode));
            }
            let mut expr_to_call_data = HashMap::new();
            for (call, ty, fun, unsafety, abi, return_ty) in loader
                .load_iter_thir_exprs_call()
            {
                expr_to_call_data.insert(call, (fun, unsafety, abi, return_ty));
            }
            let mut unsafe_thir_block_call_counts_map = HashMap::new();
            for (expr, _, closest_unsafe_block, _, _) in loader.load_iter_thir_exprs() {
                let Some((fun, unsafety, abi, return_ty)) = expr_to_call_data.get(&expr)
                else {
                    continue;
                };
                let Some((build, check_mode)) = unsafe_blocks_to_data
                    .get(&closest_unsafe_block) else {
                    continue;
                };
                let count = unsafe_thir_block_call_counts_map
                    .entry((*build, closest_unsafe_block, *check_mode))
                    .or_insert(0);
                *count += 1;
            }
            let iter_version = loader
                .load_iter_thir_exprs()
                .flat_map(|(expr, _, closest_unsafe_block, _, _)| {
                    let (fun, unsafety, abi, return_ty) = expr_to_call_data.get(&expr)?;
                    let (build, check_mode) = unsafe_blocks_to_data
                        .get(&closest_unsafe_block)?;
                    Some((
                        *build,
                        closest_unsafe_block,
                        *check_mode,
                        expr,
                        *fun,
                        *unsafety,
                        *abi,
                        *return_ty,
                    ))
                });
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unsafe thir blocks with calls: {0}",
                            unsafe_thir_block_call_counts_map.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_blocks_with_calls: HashSet<_> = unsafe_thir_block_call_counts_map
                .iter()
                .map(|(&(_build, block, _check_mode), _call_count)| block)
                .collect();
            let unsafe_thir_block_no_calls_relation: Vec<_> = loader
                .load_unsafe_thir_blocks()
                .iter()
                .filter(|
                    (
                        _build,
                        _thir_body_def_path,
                        block,
                        _expansion_kind,
                        _check_mode,
                        _span,
                    )|
                { !unsafe_blocks_with_calls.contains(block) })
                .cloned()
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unsafe thir blocks with non-const calls: {0}",
                            unsafe_thir_block_no_calls_relation.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_iter_unsafe_thir_block_calls(iter_version);
            loader
                .store_iter_unsafe_thir_block_call_counts(
                    unsafe_thir_block_call_counts_map
                        .into_iter()
                        .map(|((build, block, check_mode), count)| (
                            build,
                            block,
                            check_mode,
                            count,
                        )),
                );
            loader.store_unsafe_thir_block_no_calls(unsafe_thir_block_no_calls_relation);
        }
        /// Report how many function calls each unsafe thir block contains.
        fn new_report_called_functions(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
            let unsafe_thir_block_calls = unsafe_thir_block_calls
                .iter()
                .map(|&(build, block, check_mode, call, fun, unsafety, abi, _return_ty)| {
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        check_mode.to_string(),
                        call,
                        fun,
                        unsafety.to_string(),
                        strings.r(abis.r(abi)),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_block_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_block_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_thir_block_calls"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_thir_block_call_counts = loader
                .load_unsafe_thir_block_call_counts();
            let unsafe_thir_block_call_counts = unsafe_thir_block_call_counts
                .iter()
                .map(|&(build, block, check_mode, call_count)| {
                    (
                        build,
                        build_resolver.resolve(build),
                        block,
                        check_mode.to_string(),
                        call_count,
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_block_call_counts"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_block_call_counts {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_thir_block_call_counts"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let unsafe_thir_block_no_calls = loader.load_unsafe_thir_block_no_calls();
            let unsafe_thir_block_no_calls = unsafe_thir_block_no_calls
                .iter()
                .map(|
                    &(
                        build,
                        thir_body_def_path,
                        block,
                        expansion_kind,
                        check_mode,
                        _span,
                    )|
                {
                    (
                        build,
                        build_resolver.resolve(build),
                        def_path_resolver.resolve(thir_body_def_path),
                        block,
                        expansion_kind.to_string(),
                        check_mode.to_string(),
                    )
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_thir_block_no_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_thir_block_no_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("reported unsafe_thir_block_no_calls"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_block_groups",
                            "corpus_manager::queries::unsafe_block_groups",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
        }
        /// Find all thir calls in unsafe functions that call non-constant targets. In other
        /// words, find all calls that call function pointers.
        fn new_report_non_const_call_targets(loader: &Loader, report_path: &Path) {
            let const_calls: HashSet<_> = loader
                .load_thir_exprs_call_const_target()
                .iter()
                .map(|(fun, _def_path)| *fun)
                .collect();
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
            let non_const_thir_calls = unsafe_thir_block_calls
                .iter()
                .flat_map(|
                    (build, block, _check_mode, call, fun, unsafety, abi, _return_ty)|
                {
                    if const_calls.contains(fun) {
                        None
                    } else {
                        Some((
                            build,
                            build_resolver.resolve(*build),
                            block,
                            call,
                            fun,
                            unsafety.to_string(),
                            strings.r(abis.r(*abi)),
                        ))
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "non_const_thir_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in non_const_thir_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        /// Find all thir calls in unsafe functions that call constant targets. The call
        /// targets that appear as constants: (TODO: double check this list for THIR)
        ///
        /// 1. Static function calls.
        /// 2. Static method calls.
        /// 3. Dynamic calls on trait objects.
        /// 4. Calls of closures.
        fn new_report_const_call_targets(loader: &Loader, report_path: &Path) {
            let const_calls_map = loader.load_thir_exprs_call_const_target_as_map();
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let unsafe_thir_block_calls = loader.load_unsafe_thir_block_calls();
            let const_thir_calls = unsafe_thir_block_calls
                .iter()
                .flat_map(|
                    (build, block, check_mode, call, fun, unsafety, abi, _return_ty)|
                {
                    const_calls_map
                        .get(fun)
                        .map(|def_path| {
                            Some((
                                build,
                                build_resolver.resolve(*build),
                                def_path_resolver.resolve(*def_path),
                                block,
                                check_mode,
                                call,
                                fun,
                                unsafety.to_string(),
                                strings.r(abis.r(*abi)),
                            ))
                        })
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "const_thir_calls"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in const_thir_calls {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn new_query(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let trait_items = loader.load_trait_items();
            let trait_items_debug: Vec<_> = trait_items
                .iter()
                .map(|(trait_id, def_path, defaultness)| {
                    (trait_id, def_path_resolver.resolve(*def_path), defaultness)
                })
                .collect();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "trait_items_debug"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in trait_items_debug {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            new_count_called_functions(loader);
            new_report_called_functions(loader, report_path);
            new_report_non_const_call_targets(loader, report_path);
            new_report_const_call_targets(loader, report_path);
        }
    }
    mod unsafe_reasons {
        //! Report the reasons collected from the compiler why the specific function
        //! needs to use unsafe blocks.
        use super::utils::DefPathResolver;
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use std::collections::HashSet;
        use std::path::Path;
        pub fn query(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let strings = loader.load_strings();
            let function_unsafe_reasons: Vec<_> = loader
                .load_function_unsafe_reasons()
                .iter()
                .map(|(def_path, _index, reason)| (def_path, reason))
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|(def_path, reason)| (
                    def_path_resolver.resolve(*def_path),
                    strings.r(*reason),
                ))
                .collect();
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "function_unsafe_reasons"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in function_unsafe_reasons {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
    }
    mod unsafe_spans {
        //! Report spans of the selected unsafe functions so that it is possible to
        //! quickly look up their source code.
        use super::utils::{BuildResolver, DefPathResolver, SpanResolver};
        use crate::write_csv;
        use corpus_database::{tables::Loader, types};
        use std::collections::HashMap;
        use std::path::Path;
        /// Report the spans of the selected unsafe functions.
        fn report_unsafe_function_spans(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let span_resolver = SpanResolver::new(loader);
            let strings = loader.load_strings();
            let abis = loader.load_abis();
            let def_path_spans = loader.load_def_path_span_redb_map();
            let selected_function_definitions = loader
                .load_selected_function_definitions();
            let unsafe_function_spans = selected_function_definitions
                .iter()
                .flat_map(|
                    &(
                        build,
                        _item,
                        def_path,
                        _module,
                        visibility,
                        unsafety,
                        abi,
                        _return_ty,
                        uses_unsafe,
                    )|
                {
                    if unsafety == types::Safety::Unsafe {
                        Some((
                            build,
                            build_resolver.resolve(build),
                            def_path_resolver.resolve(def_path),
                            visibility.to_string(),
                            strings.r(abis.r(abi)),
                            uses_unsafe,
                            span_resolver.resolve(def_path_spans.r(def_path)),
                        ))
                    } else {
                        None
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_function_spans"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_function_spans {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        pub fn query(loader: &Loader, report_path: &Path) {
            report_unsafe_function_spans(loader, report_path);
        }
    }
    mod unsafe_types {
        //! Collect information about unsafe types.
        use super::utils::GroupByIterator;
        use super::utils::{BuildResolver, DefPathResolver};
        use crate::write_csv;
        use corpus_database::tables::Loader;
        use corpus_database::types;
        use corpus_queries_derive::datapond_query;
        use log::{info, warn};
        use std::collections::HashSet;
        use std::path::Path;
        fn report_types_foreign(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let types_foreign = loader.load_types_foreign();
            let types_foreign = types_foreign
                .iter()
                .map(|&(typ, def_path)| (typ, def_path_resolver.resolve(def_path)));
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "types_foreign"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in types_foreign {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        fn collect_unsafe_cell_types(loader: &Loader, report_path: &Path) {
            let def_paths = loader.load_def_paths();
            let strings = loader.load_strings();
            let Some(unsafe_cell_summary_key) = strings
                .lookup_str("core.cell.UnsafeCell") else {
                let unsafe_cell_types: Vec<(types::Type, types::DefPath)> = Vec::new();
                let unsafe_cell_types_relation: Vec<(types::Type, types::DefPath)> = Vec::new();
                if !report_path.exists() {
                    std::fs::create_dir(report_path).unwrap();
                }
                let file_path = report_path
                    .join(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}.csv", "unsafe_cell_types"),
                            );
                            res
                        }),
                    );
                let mut wtr = csv::Writer::from_path(file_path).unwrap();
                for row in unsafe_cell_types {
                    wtr.serialize(row).unwrap();
                }
                wtr.flush().unwrap();
                loader.store_types_unsafe_cell(unsafe_cell_types_relation);
                return;
            };
            let unsafe_cell_summary_id = loader
                .load_summary_keys()
                .lookup(&unsafe_cell_summary_key)
                .unwrap();
            let get_unsafe_cell_types_relation = || {
                loader
                    .load_iter_types_adt_def()
                    .filter_map(|(typ, def_path, _, _, _)| {
                        let (_, _, _, _, def_path_summary) = def_paths.r(def_path);
                        if def_path_summary == unsafe_cell_summary_id {
                            Some((typ, def_path))
                        } else {
                            None
                        }
                    })
            };
            let def_path_resolver = DefPathResolver::new(loader);
            let unsafe_cell_types = get_unsafe_cell_types_relation()
                .map(|(typ, def_path)| (typ, def_path_resolver.resolve(def_path)));
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_cell_types"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_cell_types {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
            loader.store_iter_types_unsafe_cell(get_unsafe_cell_types_relation());
        }
        fn collect_union_types(loader: &Loader) {
            let get_union_types = || {
                loader
                    .load_iter_types_adt_def()
                    .flat_map(|(typ, def_path, kind, _, _)| {
                        if kind == types::AdtKind::Union {
                            Some((typ, def_path))
                        } else {
                            None
                        }
                    })
            };
            {
                let lvl = ::log::Level::Warn;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("TODO: inefficient iter.count()"),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_types",
                            "corpus_manager::queries::unsafe_types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of union types: {0}",
                            get_union_types().count(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_types",
                            "corpus_manager::queries::unsafe_types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_iter_types_union(get_union_types());
        }
        fn collect_unsafe_types(loader: &Loader) {
            let public_visibility = <[_]>::into_vec(
                #[rustc_box]
                ::alloc::boxed::Box::new([(types::TyVisibility::Public,)]),
            );
            let unsafe_types;
            {
                #[allow(dead_code)]
                enum ProcMacroHack {
                    Value = (
                        "load loader\n{\n    relations(types_unsafe_cell, types_union, types_raw_ptr, types_foreign,\n    types_adt_field, types_array, types_slice, types_ref,\n    types_tuple_element),\n} input public_visibility(visibility: TyVisibility) output\nunsafe_types(typ: Type) unsafe_types(typ) :-\ntypes_unsafe_cell(.typ=typ).unsafe_types(typ) :-\ntypes_union(.typ=typ).unsafe_types(typ) :-\ntypes_raw_ptr(.typ=typ).unsafe_types(typ) :-\ntypes_foreign(.typ=typ).unsafe_types(typ) :- public_visibility(visibility),\nunsafe_types(field_type),\ntypes_adt_field(.adt=typ, .visibility=visibility,\n.typ=field_type).unsafe_types(typ) :- unsafe_types(element_type),\ntypes_array(typ, element_type).unsafe_types(typ) :-\nunsafe_types(element_type), types_slice(typ, element_type).unsafe_types(typ)\n:- unsafe_types(target_type), types_ref(typ, target_type, _).unsafe_types(typ)\n:- unsafe_types(element_type), types_tuple_element(typ, _, element_type).",
                        0,
                    )
                        .1,
                }
                {
                    use corpus_database::types::*;
                    let types_unsafe_cell = loader.load_types_unsafe_cell().clone();
                    let types_union = loader.load_types_union().clone();
                    let types_raw_ptr = loader.load_types_raw_ptr().clone();
                    let types_foreign = loader.load_types_foreign().clone();
                    let types_adt_field = loader.load_types_adt_field().clone();
                    let types_array = loader.load_types_array().clone();
                    let types_slice = loader.load_types_slice().clone();
                    let types_ref = loader.load_types_ref().clone();
                    let types_tuple_element = loader.load_types_tuple_element().clone();
                    {
                        let mut iteration = datafrog::Iteration::new();
                        let var_types_unsafe_cell = datafrog::Relation::<
                            (Type, DefPath),
                        >::from_vec(types_unsafe_cell);
                        let var_types_union = datafrog::Relation::<
                            (Type, DefPath),
                        >::from_vec(types_union);
                        let var_types_raw_ptr = datafrog::Relation::<
                            (Type, Type, Mutability),
                        >::from_vec(types_raw_ptr);
                        let var_types_foreign = datafrog::Relation::<
                            (Type, DefPath),
                        >::from_vec(types_foreign);
                        let var_types_adt_field = datafrog::Relation::<
                            (
                                Field,
                                Type,
                                AdtVariantIndex,
                                DefPath,
                                InternedString,
                                TyVisibility,
                                Type,
                            ),
                        >::from_vec(types_adt_field);
                        let var_types_array = datafrog::Relation::<
                            (Type, Type),
                        >::from_vec(types_array);
                        let var_types_slice = datafrog::Relation::<
                            (Type, Type),
                        >::from_vec(types_slice);
                        let var_types_ref = datafrog::Relation::<
                            (Type, Type, Mutability),
                        >::from_vec(types_ref);
                        let var_types_tuple_element = datafrog::Relation::<
                            (Type, TupleFieldIndex, Type),
                        >::from_vec(types_tuple_element);
                        let var_public_visibility = datafrog::Relation::<
                            (TyVisibility,),
                        >::from_vec(public_visibility);
                        let var_unsafe_types = iteration
                            .variable::<(Type,)>("unsafe_types");
                        let var_types_unsafe_cell_1 = iteration
                            .variable::<(Type, DefPath)>("types_unsafe_cell_1");
                        let var_types_union_2 = iteration
                            .variable::<(Type, DefPath)>("types_union_2");
                        let var_types_raw_ptr_3 = iteration
                            .variable::<(Type, Type, Mutability)>("types_raw_ptr_3");
                        let var_types_foreign_4 = iteration
                            .variable::<(Type, DefPath)>("types_foreign_4");
                        let var_public_visibility_5 = iteration
                            .variable::<(TyVisibility,)>("public_visibility_5");
                        let var_public_visibility_5_6 = iteration
                            .variable::<((), (TyVisibility,))>("public_visibility_5_6");
                        let var_unsafe_types_7 = iteration
                            .variable::<((), (Type,))>("unsafe_types_7");
                        let var_unsafe_types_8 = iteration
                            .variable::<(TyVisibility, Type)>("unsafe_types_8");
                        let var_types_adt_field_9 = iteration
                            .variable::<
                                (
                                    Field,
                                    Type,
                                    AdtVariantIndex,
                                    DefPath,
                                    InternedString,
                                    TyVisibility,
                                    Type,
                                ),
                            >("types_adt_field_9");
                        let var_unsafe_types_8_10 = iteration
                            .variable::<((TyVisibility, Type), ())>("unsafe_types_8_10");
                        let var_types_adt_field_9_11 = iteration
                            .variable::<
                                ((TyVisibility, Type), (Type,)),
                            >("types_adt_field_9_11");
                        let var_unsafe_types_12 = iteration
                            .variable::<(TyVisibility, Type, Type)>("unsafe_types_12");
                        let var_types_array_13 = iteration
                            .variable::<(Type, Type)>("types_array_13");
                        let var_unsafe_types_14 = iteration
                            .variable::<((Type,), ())>("unsafe_types_14");
                        let var_types_array_13_15 = iteration
                            .variable::<((Type,), (Type,))>("types_array_13_15");
                        let var_unsafe_types_16 = iteration
                            .variable::<(Type, Type)>("unsafe_types_16");
                        let var_types_slice_17 = iteration
                            .variable::<(Type, Type)>("types_slice_17");
                        let var_unsafe_types_18 = iteration
                            .variable::<((Type,), ())>("unsafe_types_18");
                        let var_types_slice_17_19 = iteration
                            .variable::<((Type,), (Type,))>("types_slice_17_19");
                        let var_unsafe_types_20 = iteration
                            .variable::<(Type, Type)>("unsafe_types_20");
                        let var_types_ref_21 = iteration
                            .variable::<(Type, Type, Mutability)>("types_ref_21");
                        let var_unsafe_types_22 = iteration
                            .variable::<((Type,), ())>("unsafe_types_22");
                        let var_types_ref_21_23 = iteration
                            .variable::<((Type,), (Type,))>("types_ref_21_23");
                        let var_unsafe_types_24 = iteration
                            .variable::<(Type, Type)>("unsafe_types_24");
                        let var_types_tuple_element_25 = iteration
                            .variable::<
                                (Type, TupleFieldIndex, Type),
                            >("types_tuple_element_25");
                        let var_unsafe_types_26 = iteration
                            .variable::<((Type,), ())>("unsafe_types_26");
                        let var_types_tuple_element_25_27 = iteration
                            .variable::<((Type,), (Type,))>("types_tuple_element_25_27");
                        let var_unsafe_types_28 = iteration
                            .variable::<(Type, Type)>("unsafe_types_28");
                        var_types_unsafe_cell_1.insert(var_types_unsafe_cell);
                        var_types_union_2.insert(var_types_union);
                        var_types_raw_ptr_3.insert(var_types_raw_ptr);
                        var_types_foreign_4.insert(var_types_foreign);
                        var_public_visibility_5.insert(var_public_visibility);
                        var_types_adt_field_9.insert(var_types_adt_field);
                        var_types_array_13.insert(var_types_array);
                        var_types_slice_17.insert(var_types_slice);
                        var_types_ref_21.insert(var_types_ref);
                        var_types_tuple_element_25.insert(var_types_tuple_element);
                        while iteration.changed() {
                            var_unsafe_types
                                .from_map(&var_types_unsafe_cell_1, |&(typ, _)| (typ,));
                            var_unsafe_types
                                .from_map(&var_types_union_2, |&(typ, _)| (typ,));
                            var_unsafe_types
                                .from_map(&var_types_raw_ptr_3, |&(typ, _, _)| (typ,));
                            var_unsafe_types
                                .from_map(&var_types_foreign_4, |&(typ, _)| (typ,));
                            var_public_visibility_5_6
                                .from_map(
                                    &var_public_visibility_5,
                                    |&(visibility,)| ((), (visibility,)),
                                );
                            var_unsafe_types_7
                                .from_map(
                                    &var_unsafe_types,
                                    |&(field_type,)| ((), (field_type,)),
                                );
                            var_unsafe_types_8
                                .from_join(
                                    &var_public_visibility_5_6,
                                    &var_unsafe_types_7,
                                    |&(), &(visibility,), &(field_type,)| (
                                        visibility,
                                        field_type,
                                    ),
                                );
                            var_unsafe_types_8_10
                                .from_map(
                                    &var_unsafe_types_8,
                                    |&(visibility, field_type)| ((visibility, field_type), ()),
                                );
                            var_types_adt_field_9_11
                                .from_map(
                                    &var_types_adt_field_9,
                                    |&(_, typ, _, _, _, visibility, field_type)| (
                                        (visibility, field_type),
                                        (typ,),
                                    ),
                                );
                            var_unsafe_types_12
                                .from_join(
                                    &var_unsafe_types_8_10,
                                    &var_types_adt_field_9_11,
                                    |&(visibility, field_type), &(), &(typ,)| (
                                        visibility,
                                        field_type,
                                        typ,
                                    ),
                                );
                            var_unsafe_types
                                .from_map(
                                    &var_unsafe_types_12,
                                    |&(visibility, field_type, typ)| (typ,),
                                );
                            var_unsafe_types_14
                                .from_map(
                                    &var_unsafe_types,
                                    |&(element_type,)| ((element_type,), ()),
                                );
                            var_types_array_13_15
                                .from_map(
                                    &var_types_array_13,
                                    |&(typ, element_type)| ((element_type,), (typ,)),
                                );
                            var_unsafe_types_16
                                .from_join(
                                    &var_unsafe_types_14,
                                    &var_types_array_13_15,
                                    |&(element_type,), &(), &(typ,)| (element_type, typ),
                                );
                            var_unsafe_types
                                .from_map(
                                    &var_unsafe_types_16,
                                    |&(element_type, typ)| (typ,),
                                );
                            var_unsafe_types_18
                                .from_map(
                                    &var_unsafe_types,
                                    |&(element_type,)| ((element_type,), ()),
                                );
                            var_types_slice_17_19
                                .from_map(
                                    &var_types_slice_17,
                                    |&(typ, element_type)| ((element_type,), (typ,)),
                                );
                            var_unsafe_types_20
                                .from_join(
                                    &var_unsafe_types_18,
                                    &var_types_slice_17_19,
                                    |&(element_type,), &(), &(typ,)| (element_type, typ),
                                );
                            var_unsafe_types
                                .from_map(
                                    &var_unsafe_types_20,
                                    |&(element_type, typ)| (typ,),
                                );
                            var_unsafe_types_22
                                .from_map(
                                    &var_unsafe_types,
                                    |&(target_type,)| ((target_type,), ()),
                                );
                            var_types_ref_21_23
                                .from_map(
                                    &var_types_ref_21,
                                    |&(typ, target_type, _)| ((target_type,), (typ,)),
                                );
                            var_unsafe_types_24
                                .from_join(
                                    &var_unsafe_types_22,
                                    &var_types_ref_21_23,
                                    |&(target_type,), &(), &(typ,)| (target_type, typ),
                                );
                            var_unsafe_types
                                .from_map(
                                    &var_unsafe_types_24,
                                    |&(target_type, typ)| (typ,),
                                );
                            var_unsafe_types_26
                                .from_map(
                                    &var_unsafe_types,
                                    |&(element_type,)| ((element_type,), ()),
                                );
                            var_types_tuple_element_25_27
                                .from_map(
                                    &var_types_tuple_element_25,
                                    |&(typ, _, element_type)| ((element_type,), (typ,)),
                                );
                            var_unsafe_types_28
                                .from_join(
                                    &var_unsafe_types_26,
                                    &var_types_tuple_element_25_27,
                                    |&(element_type,), &(), &(typ,)| (element_type, typ),
                                );
                            var_unsafe_types
                                .from_map(
                                    &var_unsafe_types_28,
                                    |&(element_type, typ)| (typ,),
                                );
                        }
                        unsafe_types = var_unsafe_types.complete();
                    }
                }
            }
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of unsafe types: {0}",
                            unsafe_types.elements.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_types",
                            "corpus_manager::queries::unsafe_types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_unsafe_types(unsafe_types.elements);
        }
        fn report_unsafe_type_defs(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let type_kinds = loader.load_type_kinds();
            let unsafe_types: HashSet<_> = loader
                .load_unsafe_types()
                .iter()
                .map(|&(typ,)| typ)
                .collect();
            match (&unsafe_types.len(), &loader.load_unsafe_types().len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            let type_defs = loader.load_selected_type_defs();
            let unsafe_type_defs = type_defs
                .iter()
                .flat_map(|
                    &(build, item, typ, def_path, name, visibility, type_kind, def_kind)|
                {
                    if unsafe_types.contains(&typ) {
                        Some((
                            build,
                            build_resolver.resolve(build),
                            item,
                            typ,
                            def_path_resolver.resolve(def_path),
                            strings.r(name),
                            visibility.to_string(),
                            strings.r(type_kinds.r(type_kind)),
                            def_kind.to_string(),
                        ))
                    } else {
                        None
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "unsafe_type_defs"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in unsafe_type_defs {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        fn collect_safe_wrapper_types(loader: &Loader) {
            let unsafe_types: HashSet<_> = loader
                .load_unsafe_types()
                .iter()
                .map(|&(typ,)| typ)
                .collect();
            let safe_wrapper_types: Vec<_> = loader
                .load_types_adt_field()
                .iter()
                .safe_group_by(|
                    &(_field, adt, _index, _def_path, _ident, _visibility, _typ)|
                adt)
                .into_iter()
                .flat_map(|(key, group)| {
                    let mut contains_unsafe_field = false;
                    for &(_field, _adt, _index, _def_path, _ident, visibility, typ) in group {
                        if unsafe_types.contains(&typ) {
                            contains_unsafe_field = true;
                            if visibility == types::TyVisibility::Public {
                                return None;
                            }
                        }
                    }
                    if contains_unsafe_field { Some((*key,)) } else { None }
                })
                .collect();
            {
                let lvl = ::log::Level::Info;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!(
                            "Number of safe wrapper types: {0}",
                            safe_wrapper_types.len(),
                        ),
                        lvl,
                        &(
                            "corpus_manager::queries::unsafe_types",
                            "corpus_manager::queries::unsafe_types",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            loader.store_safe_wrapper_types(safe_wrapper_types);
        }
        fn report_safe_wrapper_type_defs(loader: &Loader, report_path: &Path) {
            let def_path_resolver = DefPathResolver::new(loader);
            let build_resolver = BuildResolver::new(loader);
            let strings = loader.load_strings();
            let type_kinds = loader.load_type_kinds();
            let safe_wrapper_types: HashSet<_> = loader
                .load_safe_wrapper_types()
                .iter()
                .map(|&(typ,)| typ)
                .collect();
            match (&safe_wrapper_types.len(), &loader.load_safe_wrapper_types().len()) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        let kind = ::core::panicking::AssertKind::Eq;
                        ::core::panicking::assert_failed(
                            kind,
                            &*left_val,
                            &*right_val,
                            ::core::option::Option::None,
                        );
                    }
                }
            };
            let type_defs = loader.load_selected_type_defs();
            let safe_wrapper_type_defs = type_defs
                .iter()
                .flat_map(|
                    &(build, item, typ, def_path, name, visibility, type_kind, def_kind)|
                {
                    if safe_wrapper_types.contains(&typ) {
                        Some((
                            build,
                            build_resolver.resolve(build),
                            item,
                            typ,
                            def_path_resolver.resolve(def_path),
                            strings.r(name),
                            visibility.to_string(),
                            strings.r(type_kinds.r(type_kind)),
                            def_kind.to_string(),
                        ))
                    } else {
                        None
                    }
                });
            if !report_path.exists() {
                std::fs::create_dir(report_path).unwrap();
            }
            let file_path = report_path
                .join(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0}.csv", "safe_wrapper_type_defs"),
                        );
                        res
                    }),
                );
            let mut wtr = csv::Writer::from_path(file_path).unwrap();
            for row in safe_wrapper_type_defs {
                wtr.serialize(row).unwrap();
            }
            wtr.flush().unwrap();
        }
        /// Find all types that have fields whose types are “unsafe”: `UnsafeCell`, raw pointers, unions.
        pub fn query(loader: &Loader, report_path: &Path) {
            report_types_foreign(loader, report_path);
            collect_unsafe_cell_types(loader, report_path);
            collect_union_types(loader);
            collect_unsafe_types(loader);
            report_unsafe_type_defs(loader, report_path);
            collect_safe_wrapper_types(loader);
            report_safe_wrapper_type_defs(loader, report_path);
        }
    }
    mod utils {
        use corpus_database::{InterningTable, RelationMap};
        use corpus_database::{tables::Loader, types};
        use itertools::Itertools;
        use std::cell::Ref;
        use std::collections::HashMap;
        pub trait GroupByIterator: Itertools {
            /// Itertools::group_by groups consecutive elements. This version groups
            /// also non-consecutive elements.
            fn safe_group_by<K, F>(
                self,
                key: F,
            ) -> itertools::GroupBy<
                K,
                std::vec::IntoIter<<Self as std::iter::Iterator>::Item>,
                F,
            >
            where
                Self: Sized,
                F: Copy + FnMut(&Self::Item) -> K,
                K: PartialEq + Ord,
            {
                self.sorted_by_key(key).group_by(key)
            }
        }
        impl<T: ?Sized> GroupByIterator for T
        where
            T: Itertools,
        {}
        /// A helper struct for converting an interned `DefPath` into human readable
        /// tuple of strings.
        pub struct BuildResolver<'b> {
            builds: Ref<
                'b,
                InterningTable<
                    types::Build,
                    (
                        types::Package,
                        types::PackageVersion,
                        types::Krate,
                        types::CrateHash,
                        types::Edition,
                    ),
                >,
            >,
            package_names: Ref<
                'b,
                InterningTable<types::Package, types::InternedString>,
            >,
            package_versions: Ref<
                'b,
                InterningTable<types::PackageVersion, types::InternedString>,
            >,
            crate_names: Ref<'b, InterningTable<types::Krate, types::InternedString>>,
            editions: Ref<'b, InterningTable<types::Edition, types::InternedString>>,
            strings: Ref<'b, InterningTable<types::InternedString, String>>,
        }
        impl<'b> BuildResolver<'b> {
            pub fn new(loader: &'b Loader) -> Self {
                Self {
                    builds: loader.load_builds(),
                    package_names: loader.load_package_names(),
                    package_versions: loader.load_package_versions(),
                    crate_names: loader.load_crate_names(),
                    editions: loader.load_editions(),
                    strings: loader.load_strings(),
                }
            }
            pub fn resolve(
                &self,
                build: types::Build,
            ) -> (String, String, String, String, String) {
                let (package_name, package_version, crate_name, crate_hash, edition) = self
                    .builds
                    .r(build);
                (
                    self.strings.r(self.package_names.r(package_name)),
                    self.strings.r(self.package_versions.r(package_version)),
                    self.strings.r(self.crate_names.r(crate_name)),
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0:x}", crate_hash),
                        );
                        res
                    }),
                    self.strings.r(self.editions.r(edition)),
                )
            }
        }
        /// A helper struct for converting an interned `DefPath` into human readable
        /// tuple of strings.
        pub struct DefPathResolver<'b> {
            def_paths: Ref<
                'b,
                InterningTable<
                    types::DefPath,
                    (
                        types::Krate,
                        types::CrateHash,
                        types::RelativeDefId,
                        types::DefPathHash,
                        types::SummaryId,
                    ),
                >,
            >,
            crate_names: Ref<'b, InterningTable<types::Krate, types::InternedString>>,
            relative_def_paths: Ref<
                'b,
                InterningTable<types::RelativeDefId, types::InternedString>,
            >,
            summary_keys: Ref<
                'b,
                InterningTable<types::SummaryId, types::InternedString>,
            >,
            strings: Ref<'b, InterningTable<types::InternedString, String>>,
        }
        impl<'b> DefPathResolver<'b> {
            pub fn new(loader: &'b Loader) -> Self {
                Self {
                    def_paths: loader.load_def_paths(),
                    crate_names: loader.load_crate_names(),
                    relative_def_paths: loader.load_relative_def_paths(),
                    summary_keys: loader.load_summary_keys(),
                    strings: loader.load_strings(),
                }
            }
            pub fn resolve(
                &self,
                def_path: types::DefPath,
            ) -> (String, String, String, String, String) {
                let (
                    crate_name,
                    crate_hash,
                    relative_def_path,
                    def_path_hash,
                    summary_key,
                ) = self.def_paths.r(def_path);
                let crate_name = self.crate_names.get_redb(crate_name).unwrap();
                (
                    self.strings.r(crate_name),
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0:x}", crate_hash),
                        );
                        res
                    }),
                    self.strings.r(self.relative_def_paths.r(relative_def_path)),
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0:x}", def_path_hash),
                        );
                        res
                    }),
                    self.strings.r(self.summary_keys.r(summary_key)),
                )
            }
        }
        /// A helper struct for converting an interned `span` into human readable
        /// tuple of strings.
        pub struct SpanResolver<'b> {
            spans: Ref<
                'b,
                RelationMap<
                    types::Span,
                    (
                        types::Span,
                        types::SpanExpansionKind,
                        types::InternedString,
                        types::SpanFileName,
                        u16,
                        u16,
                    ),
                >,
            >,
            span_file_names: Ref<
                'b,
                InterningTable<types::SpanFileName, types::InternedString>,
            >,
            strings: Ref<'b, InterningTable<types::InternedString, String>>,
        }
        impl<'b> SpanResolver<'b> {
            pub fn new(loader: &'b Loader) -> Self {
                let spans = loader.load_spans_redb_map();
                Self {
                    spans,
                    span_file_names: loader.load_span_file_names(),
                    strings: loader.load_strings(),
                }
            }
            pub fn resolve(
                &self,
                span: types::Span,
            ) -> (types::Span, String, String, String, u16, u16) {
                let (
                    _parent,
                    expansion_kind,
                    expansion_kind_descr,
                    file_name,
                    line,
                    col,
                ) = self.spans.r(span);
                (
                    span,
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("{0:?}", expansion_kind),
                        );
                        res
                    }),
                    self.strings.r(expansion_kind_descr),
                    self.strings.r(self.span_file_names.r(file_name)),
                    line,
                    col,
                )
            }
            pub fn get_expansion_kind(
                &self,
                span: types::Span,
            ) -> types::SpanExpansionKind {
                let (
                    _parent,
                    expansion_kind,
                    _expansion_kind_descr,
                    _file_name,
                    _line,
                    _col,
                ) = self.spans.r(span);
                expansion_kind
            }
        }
        /// From relation `iter` filters the facts that belong only to `selected_builds`.
        pub fn filter_selected<F1, F2, I, O>(
            iter: impl Iterator<Item = I>,
            selected_builds: &[(
                types::Build,
                types::Package,
                types::PackageVersion,
                types::Krate,
                types::CrateHash,
                types::Edition,
            )],
            def_paths: &InterningTable<
                types::DefPath,
                (
                    types::Krate,
                    types::CrateHash,
                    types::RelativeDefId,
                    types::DefPathHash,
                    types::SummaryId,
                ),
            >,
            extract_def_path: F1,
            construct_result: F2,
        ) -> Vec<O>
        where
            F1: Fn(I) -> types::DefPath,
            F2: Fn(types::Build, I) -> O,
            I: Clone,
        {
            let selected_builds_set: HashMap<_, _> = selected_builds
                .iter()
                .map(|&(build, _package, _version, krate, crate_hash, _edition)| {
                    ((krate, crate_hash), build)
                })
                .collect();
            iter.flat_map(|element| {
                    let def_path = extract_def_path(element.clone());
                    let (krate, crate_hash, _, _, _) = def_paths.r(def_path);
                    selected_builds_set
                        .get(&(krate, crate_hash))
                        .map(|build| construct_result(*build, element))
                })
                .collect()
        }
    }
    pub fn run_query(
        query_name: &str,
        database_root: &Path,
        report_path: &Path,
        workspace_path: &Path,
        sources_list_path: &Path,
    ) {
        {
            let lvl = ::log::Level::Info;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("Running query: {0}", query_name),
                    lvl,
                    &(
                        "corpus_manager::queries",
                        "corpus_manager::queries",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        let loader = corpus_database::tables::Loader::new(database_root.to_path_buf());
        match query_name {
            "prepare-builds" => {
                prepare_builds::query(
                    &loader,
                    &report_path.join("prepare-builds"),
                    workspace_path,
                    sources_list_path,
                )
            }
            "prepare-items" => prepare_items::query(&loader),
            "prepare-all" => {
                run_query(
                    "prepare-builds",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "prepare-items",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
            }
            "counters" => {
                counters::new_query(&loader, &report_path.join("q-counters"));
            }
            "size" => {
                size::new_query(&loader, &report_path.join("q-size"));
            }
            "function-size" => {
                function_size::new_query(&loader, &report_path.join("function-size"));
            }
            "build-files" => {
                build_files::query(&loader, &report_path.join("build-files"))
            }
            "traits" => traits::query(&loader, &report_path.join("traits")),
            "types" => types::query(&loader, &report_path.join("types")),
            "resolved-calls" => {
                resolved_calls::query(&loader, &report_path.join("resolved-calls"))
            }
            "unsafe-types" => {
                unsafe_types::query(&loader, &report_path.join("unsafe-types"))
            }
            "unsafe-block-groups" => {
                unsafe_block_groups::new_query(
                    &loader,
                    &report_path.join("unsafe-block-groups"),
                );
            }
            "unsafe-reasons" => {
                unsafe_reasons::query(&loader, &report_path.join("unsafe-reasons"))
            }
            "unsafe-block-calls" => {
                unsafe_block_calls::new_query(
                    &loader,
                    &report_path.join("unsafe-block-calls"),
                );
            }
            "unsafe-spans" => {
                unsafe_spans::query(&loader, &report_path.join("unsafe-spans"))
            }
            "build-meta" => build_meta::query(&loader, &report_path.join("build-meta")),
            "non-tree-types" => {
                non_tree_types::query(&loader, &report_path.join("non-tree-types"))
            }
            "all" => {
                run_query(
                    "unsafe-reasons",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "prepare-all",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "counters",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "size",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "function-size",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "build-files",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "build-meta",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "traits",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "types",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "unsafe-types",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "unsafe-block-groups",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "unsafe-block-calls",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "unsafe-spans",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
                run_query(
                    "non-tree-types",
                    database_root,
                    report_path,
                    workspace_path,
                    sources_list_path,
                );
            }
            _ => {
                ::core::panicking::panic_fmt(
                    format_args!(
                        "internal error: entered unreachable code: {0}",
                        format_args!("Unknown query: {0}", query_name),
                    ),
                );
            }
        }
    }
}
mod sources_list {
    //! Module for managing lists of crate sources.
    use cargo::core::{Dependency, SourceId};
    use cargo::sources::source::{QueryKind, Source};
    use cargo::sources::RegistrySource;
    use cargo::util::cache_lock::CacheLockMode;
    use cargo::util::interning::InternedString;
    use cargo::GlobalContext;
    use log_derive::{logfn, logfn_inputs};
    use serde::{Deserialize, Serialize};
    use std::collections::HashSet;
    use std::fs::File;
    use std::task::Poll;
    use std::time::SystemTime;
    /// A create on crates.io.
    pub struct Package {
        name: String,
        version: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Package {
        #[inline]
        fn clone(&self) -> Package {
            Package {
                name: ::core::clone::Clone::clone(&self.name),
                version: ::core::clone::Clone::clone(&self.version),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Package {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Package",
                "name",
                &self.name,
                "version",
                &&self.version,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Package {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "version" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"version" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Package>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Package;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Package",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Package with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Package with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Package {
                            name: __field0,
                            version: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "version",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("version")?
                            }
                        };
                        _serde::__private::Ok(Package {
                            name: __field0,
                            version: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name", "version"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Package",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Package>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Package {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Package",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "version",
                    &self.version,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    /// A crate source: either crates.io name or a repository URL.
    pub enum Crate {
        Package(Package),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Crate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Crate::Package(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Package",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Crate {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 1",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Package" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Package" => _serde::__private::Ok(__Field::__field0),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Crate>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Crate;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum Crate",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::__private::Result::map(
                                    _serde::de::VariantAccess::newtype_variant::<
                                        Package,
                                    >(__variant),
                                    Crate::Package,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &["Package"];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "Crate",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Crate>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Crate {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    Crate::Package(ref __field0) => {
                        _serde::Serializer::serialize_newtype_variant(
                            __serializer,
                            "Crate",
                            0u32,
                            "Package",
                            __field0,
                        )
                    }
                }
            }
        }
    };
    impl Crate {
        pub fn name(&self) -> &str {
            match self {
                Crate::Package(Package { ref name, .. }) => name,
            }
        }
        pub fn version(&self) -> &str {
            match self {
                Crate::Package(Package { ref version, .. }) => version,
            }
        }
    }
    /// A list of sources from where to download creates.
    pub struct CratesList {
        creation_date: SystemTime,
        crates: Vec<Crate>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CratesList {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CratesList",
                "creation_date",
                &self.creation_date,
                "crates",
                &&self.crates,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CratesList {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "creation_date" => _serde::__private::Ok(__Field::__field0),
                            "crates" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"creation_date" => _serde::__private::Ok(__Field::__field0),
                            b"crates" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CratesList>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CratesList;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CratesList",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            SystemTime,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CratesList with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Vec<Crate>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct CratesList with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CratesList {
                            creation_date: __field0,
                            crates: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<SystemTime> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Vec<Crate>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "creation_date",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<SystemTime>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("crates"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec<Crate>>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("creation_date")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("crates")?
                            }
                        };
                        _serde::__private::Ok(CratesList {
                            creation_date: __field0,
                            crates: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["creation_date", "crates"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CratesList",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CratesList>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CratesList {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CratesList",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "creation_date",
                    &self.creation_date,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "crates",
                    &self.crates,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl CratesList {
        /// Create a list of top ``count`` crates.
        ///
        /// `all_versions` – should get all versions or only the newest one?
        pub fn top_crates_by_download_count(count: usize, all_versions: bool) -> Self {
            let result = (move || {
                let config = GlobalContext::default()
                    .expect("Unable to create default Cargo config");
                let _lock = config
                    .acquire_package_cache_lock(CacheLockMode::MutateExclusive);
                let crates_io = SourceId::crates_io(&config)
                    .expect("Unable to create crates.io source ID");
                let mut source = RegistrySource::remote(
                        crates_io,
                        &HashSet::new(),
                        &config,
                    )
                    .expect("Unable to create registry source");
                source.block_until_ready().expect("Unable to block until ready");
                let creation_date = SystemTime::now();
                let mut crates = Vec::new();
                for crate_name in super::top_crates::top_crates_by_download_count(
                    count,
                ) {
                    let query = Dependency::new_override(
                        InternedString::new(&crate_name),
                        crates_io,
                    );
                    let poll = source.query_vec(&query, QueryKind::Normalized);
                    while poll.is_pending() {}
                    let Poll::Ready(summaries) = poll else {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!("Querying for crate failed"),
                            );
                        };
                    };
                    let summaries = summaries.expect("No summaries found");
                    if all_versions {
                        for summary in summaries {
                            let package = Package {
                                name: crate_name.clone(),
                                version: summary.as_summary().version().to_string(),
                            };
                            crates.push(Crate::Package(package));
                        }
                    } else {
                        let maybe_summary = summaries
                            .into_iter()
                            .max_by_key(|summary| {
                                summary.as_summary().version().clone()
                            });
                        if let Some(summary) = maybe_summary {
                            let package = Package {
                                name: crate_name.clone(),
                                version: summary.as_summary().version().to_string(),
                            };
                            crates.push(Crate::Package(package));
                        }
                    }
                }
                Self { creation_date, crates }
            })();
            {
                let lvl = log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("top_crates_by_download_count() => {0:?}", result),
                        lvl,
                        &(
                            "corpus_manager::sources_list",
                            "corpus_manager::sources_list",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            result
        }
        /// Create a list with all crates.
        ///
        /// `all_versions` – should get all versions or only the newest one?
        pub fn all_crates(all_versions: bool) -> Self {
            {
                let lvl = log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("all_crates(all_versions: {0:?})", all_versions),
                        lvl,
                        &(
                            "corpus_manager::sources_list",
                            "corpus_manager::sources_list",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let creation_date = SystemTime::now();
            let mut index = crates_index::GitIndex::new_cargo_default().unwrap();
            index.update().expect("Unable to update registry");
            let mut crates = Vec::new();
            for krate in index.crates() {
                if all_versions {
                    for version in krate.versions() {
                        let package = Package {
                            name: version.name().to_string(),
                            version: version.version().to_string(),
                        };
                        crates.push(Crate::Package(package));
                    }
                } else {
                    let version = krate.most_recent_version();
                    let package = Package {
                        name: version.name().to_string(),
                        version: version.version().to_string(),
                    };
                    crates.push(Crate::Package(package));
                }
            }
            Self { creation_date, crates }
        }
        /// Save the list into a file.
        pub fn save(&self, path: &std::path::Path) {
            {
                let lvl = log::Level::Trace;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("save(self: {0:?},path: {1:?})", self, path),
                        lvl,
                        &(
                            "corpus_manager::sources_list",
                            "corpus_manager::sources_list",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            let mut file = File::create(path)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Unable to create {0:?}: {1}", path, e),
                    );
                });
            serde_json::to_writer_pretty(&mut file, self)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Unable to write {0:?}: {1}", path, e),
                    );
                });
        }
        /// Load the list from a file.
        pub fn load(path: &std::path::Path) -> Self {
            let file = File::open(path)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Failed to load from {0:?}: {1}", path, e),
                    );
                });
            serde_json::from_reader(file)
                .unwrap_or_else(|e| {
                    ::core::panicking::panic_fmt(
                        format_args!("Invalid JSON {0:?}: {1}", path, e),
                    );
                })
        }
        pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a Crate> {
            self.crates.iter()
        }
    }
}
mod top_crates {
    //! Helper functions for obtaining the list of most downloaded crates.
    use log::debug;
    use log_derive::logfn;
    use serde::{Deserialize, Serialize};
    use std::cmp::min;
    /// A create on crates.io.
    struct Crate {
        #[serde(rename = "id")]
        name: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Crate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "Crate",
                "name",
                &&self.name,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Crate {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Crate>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Crate;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Crate",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Crate with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Crate { name: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        _serde::__private::Ok(Crate { name: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["id"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Crate",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Crate>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Crate {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Crate",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    /// The list of crates from crates.io
    struct CratesList {
        crates: Vec<Crate>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CratesList {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CratesList",
                "crates",
                &&self.crates,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CratesList {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "crates" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"crates" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CratesList>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CratesList;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CratesList",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<Crate>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CratesList with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CratesList { crates: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Vec<Crate>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("crates"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Vec<Crate>>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("crates")?
                            }
                        };
                        _serde::__private::Ok(CratesList { crates: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["crates"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CratesList",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CratesList>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    fn get(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Rust Corpus - Top Crates Scrapper")
            .build()?
            .get(url)
            .send()
    }
    /// Create a list of top ``count`` crates.
    pub fn top_crates_by_download_count(mut count: usize) -> Vec<String> {
        let result = (move || {
            const PAGE_SIZE: usize = 100;
            let page_count = count / PAGE_SIZE + 2;
            let mut sources = Vec::new();
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("page count: {0}", page_count),
                        lvl,
                        &(
                            "corpus_manager::top_crates",
                            "corpus_manager::top_crates",
                            ::log::__private_api::loc(),
                        ),
                        (),
                    );
                }
            };
            for page in 1..page_count {
                {
                    let lvl = ::log::Level::Debug;
                    if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                        ::log::__private_api::log(
                            format_args!(
                                "page: {0} page_size: {1} count: {2}",
                                page,
                                PAGE_SIZE,
                                count,
                            ),
                            lvl,
                            &(
                                "corpus_manager::top_crates",
                                "corpus_manager::top_crates",
                                ::log::__private_api::loc(),
                            ),
                            (),
                        );
                    }
                };
                let url = ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "https://crates.io/api/v1/crates?page={0}&per_page={1}&sort=downloads",
                            page,
                            PAGE_SIZE,
                        ),
                    );
                    res
                });
                let resp = get(&url).expect("Could not fetch top crates");
                if !resp.status().is_success() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Response status: {0}", resp.status()),
                        );
                    }
                }
                let page_crates: CratesList = serde_json::from_reader(resp)
                    .expect("Invalid JSON");
                sources
                    .extend(page_crates.crates.into_iter().take(count).map(|c| c.name));
                count -= min(PAGE_SIZE, count);
            }
            sources
        })();
        {
            let lvl = log::Level::Trace;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api::log(
                    format_args!("top_crates_by_download_count() => {0:?}", result),
                    lvl,
                    &(
                        "corpus_manager::top_crates",
                        "corpus_manager::top_crates",
                        ::log::__private_api::loc(),
                    ),
                    (),
                );
            }
        };
        result
    }
}
use self::compilation::CompileManager;
use self::database::DatabaseManager;
use self::sources_list::CratesList;
use log_derive::logfn;
use std::path::Path;
use std::time::Duration;
/// Initialise the list of crates with ``top_count`` most downloaded crates.
pub fn initialise_with_top(
    sources_list_path: &Path,
    top_count: usize,
    all_versions: bool,
) {
    let result = (move || {
        let crates_list = CratesList::top_crates_by_download_count(
            top_count,
            all_versions,
        );
        crates_list.save(sources_list_path);
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("initialise_with_top() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
pub fn initialise_with_all(sources_list_path: &Path, all_versions: bool) {
    let crates_list = CratesList::all_crates(all_versions);
    crates_list.save(sources_list_path);
}
/// Compile the downloaded crates.
pub fn compile(
    sources_list_path: &Path,
    workspace: &Path,
    toolchain: String,
    max_log_size: usize,
    memory_limit: Option<usize>,
    timeout: Option<Duration>,
    enable_networking: bool,
    output_json: bool,
    use_original_rustc: bool,
    purge_build_dir: bool,
    custom_registry: Option<String>,
) {
    let result = (move || {
        let crates_list = CratesList::load(sources_list_path);
        let manager = CompileManager::new(
            crates_list,
            workspace,
            toolchain,
            max_log_size,
            memory_limit,
            timeout,
            enable_networking,
            output_json,
            use_original_rustc,
            purge_build_dir,
            custom_registry,
        );
        manager
            .compile_all()
            .map_err(|e| {
                ::core::panicking::panic_fmt(format_args!("Error: {0}", e));
            })
            .unwrap();
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("compile() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
/// Classify the compilation errors.
pub fn check_compilation(workspace: &Path, delete_failures: bool) {
    let result = (move || {
        self::compilation_utils::check_compilation(workspace, delete_failures);
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("check_compilation() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
/// Move deduplicated extracted facts to the specified directory.
pub fn move_extracted(workspace: &Path, target_dir: &Path) {
    let result = (move || {
        self::compilation_utils::move_extracted(workspace, target_dir);
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("move_extracted() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
/// Update the database with the new information from the downloaded crates.
pub fn update_database(workspace: &Path, database_root: &Path) {
    let result = (move || {
        let mut manager = DatabaseManager::new(database_root);
        manager.update_database(workspace);
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("update_database() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
/// Run the specified query.
pub fn run_query(
    query_name: &str,
    database_root: &Path,
    report_path: &Path,
    workspace_path: &Path,
    sources_list_path: &Path,
) {
    let result = (move || {
        if !report_path.exists() {
            std::fs::create_dir_all(&report_path).unwrap();
        }
        queries::run_query(
            query_name,
            database_root,
            report_path,
            workspace_path,
            sources_list_path,
        );
    })();
    {
        let lvl = log::Level::Trace;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api::log(
                format_args!("run_query() => {0:?}", result),
                lvl,
                &("corpus_manager", "corpus_manager", ::log::__private_api::loc()),
                (),
            );
        }
    };
    result
}
