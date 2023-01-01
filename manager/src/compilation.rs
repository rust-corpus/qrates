//! Module responsible for compiling crates.

use super::sources_list::Crate as CrateInfo;
use crate::sources_list::CratesList;
use log::LevelFilter;
use log::{error, info};
use log_derive::logfn;
use rustwide::logging::{self, LogStorage};
use rustwide::BuildDirectory;
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
        let out_dir: PathBuf = env!("OUT_DIR").into();
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
    #[logfn(Trace)]
    fn prepare_custom_registry(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(registry_url) = &self.custom_registry {
            let cargo_config_path = self.workspace.join("cargo-home/config.toml");
            let mut cargo_config = toml::toml! {
                [source.crates-io]
                registry = "TODO"
            };
            match &mut cargo_config {
                toml::Value::Table(section) => match &mut section["source"] {
                    toml::Value::Table(source) => match &mut source["crates-io"] {
                        toml::Value::Table(crates_io) => {
                            crates_io["registry"] = toml::Value::String(registry_url.to_string());
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
            std::fs::write(cargo_config_path, &cargo_config.to_string())?;
        }
        Ok(())
    }
    #[logfn(Trace)]
    fn compile_stdlib(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dest_parent_path = self
            .workspace
            .join("cargo-home/sysroot/lib/rustlib/x86_64-unknown-linux-gnu");
        if dest_parent_path.exists() {
            info!("The standard library is already built.");
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
        cmd.args(&[
            "build",
            "--release",
            "-Z",
            "build-std",
            "--target",
            "x86_64-unknown-linux-gnu",
        ]);
        cmd.arg("--manifest-path");
        cmd.arg(cargo_toml);
        cmd.env("RUST_BACKTRACE", "1");
        if !self.use_original_rustc {
            let sysroot = format!(
                "{}/toolchains/{}",
                env!("RUSTUP_HOME"),
                env!("RUSTUP_TOOLCHAIN")
            );
            cmd.env("SYSROOT", sysroot)
                .env("RUSTC", &self.extractor_path)
                .env(
                    "CORPUS_RESULTS_DIR",
                    self.extracted_files_path.join("stdlib"),
                );
        }

        let status = cmd.status().expect("failed to execute process");
        assert!(status.success(), "Failed to compile stdlib.");

        let lib_path = tmp_dir.join("target/x86_64-unknown-linux-gnu/release/deps");
        let dest_path = dest_parent_path.join("lib");
        std::fs::create_dir_all(&dest_parent_path)?;
        let mut cp = Command::new("cp");
        cp.arg("-r");
        cp.arg(&lib_path);
        cp.arg(&dest_path);
        cp.status().expect("failed to execute cp");
        assert!(
            status.success(),
            "couldn't copy '{}' to '{}'",
            lib_path.display(),
            dest_path.display(),
        );
        info!("The standard library successfully built.");
        Ok(())
    }
    #[logfn(Trace)]
    pub fn compile_all(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cargo_config_path = self.workspace.join("cargo-home/config.toml");
        if cargo_config_path.exists() {
            std::fs::remove_file(cargo_config_path)?;
        }
        self.compile_stdlib()?;
        let workspace = self.workspace_builder().init()?;
        let toolchain = Toolchain::dist(&self.toolchain);
        toolchain.install(&workspace)?;
        toolchain.add_component(&workspace, "rustc-dev")?;
        if !self.use_original_rustc {
            self.copy_extractor()?;
        }
        self.prepare_custom_registry()?;
        for (index, krate) in self.crates_list.iter().enumerate() {
            info!("");
            info!(
                "Compiling crate {} of {}",
                index + 1,
                self.crates_list.len()
            );
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
            let crate_extracted_files =
                self.extracted_files_path
                    .join(format!("{}-{}", krate.name(), krate.version()));
            match compiler.build(krate, &crate_extracted_files) {
                Ok(_) => info!("Compilation succeeded."),
                Err(error) => {
                    error!("Compilation failed: {}", error);
                    if !crate_extracted_files.exists() {
                        std::fs::create_dir_all(&crate_extracted_files)?;
                    }
                    let build_logs = crate_extracted_files.join("logs");
                    std::fs::write(build_logs, format!("Compilation failed: {}", error))?;
                }
            }
        }
        Ok(())
    }
    /// Copies extractor to the workspace.
    fn copy_extractor(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dest_path = self.workspace.join("cargo-home/rustc");
        std::fs::copy(&self.extractor_path, &dest_path).unwrap_or_else(|_| {
            panic!(
                "couldn't copy '{}' to '{}'",
                self.extractor_path.display(),
                dest_path.display()
            )
        });
        Ok(())
    }

    /// Clears all output generated by a compilation, including the rust-corpus folder.
    pub fn clear_build_output(&self) -> Result<(), Box<dyn std::error::Error>> {
        let workspace = self.workspace_builder().init()?;
        corpus_build_dir(&workspace).purge()?;
        if self.extracted_files_path.exists() {
            std::fs::remove_dir_all(&self.extracted_files_path)?;
        }
        Ok(())
    }

    fn workspace_builder(&self) -> WorkspaceBuilder {
        WorkspaceBuilder::new(&self.workspace, "rust-corpus")
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
            info!("Already compiled: {}", crate_extracted_files.display());
            return Ok(());
        }
        let krate = Crate::crates_io(krate_info.name(), krate_info.version());
        krate.fetch(self.workspace)?;
        let sandbox = SandboxBuilder::new()
            .memory_limit(self.memory_limit)
            .enable_networking(self.enable_networking);
        let mut build_dir = corpus_build_dir(&self.workspace);
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

                let successful = logging::capture(&storage, || {
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
                });
                let build_logs = crate_extracted_files.join("logs");
                std::fs::write(build_logs, storage.to_string())?;
                let mut target_dir = build.host_target_dir();
                target_dir.push("rust-corpus");
                if target_dir.exists() {
                    if successful {
                        let success_marker = target_dir.join("success");
                        std::fs::write(
                            success_marker,
                            format!("{:?}", chrono::offset::Utc::now()),
                        )?;
                    }
                    for entry in walkdir::WalkDir::new(target_dir) {
                        let entry = entry?;
                        let path = entry.path();
                        if path.is_file() {
                            let file_name = path.file_name().unwrap();
                            std::fs::rename(path, crate_extracted_files.join(file_name))?;
                        }
                    }
                } else {
                    error!("The target directory does not exist: {:?}", target_dir);
                }
                Ok(())
            })?;
        Ok(())
    }
}

fn corpus_build_dir(workspace: &Workspace) -> BuildDirectory {
    workspace.build_dir("corpus")
}
