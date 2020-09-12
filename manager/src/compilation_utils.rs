use log::error;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Default)]
struct Failures {
    failure_reasons: HashMap<&'static str, u32>,
    internal_errors: Vec<PathBuf>,
    unknown_failures: Vec<PathBuf>,
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
            .unwrap_or_else(|err| panic!("An error when opening {:?}: {}", logs_file, err));
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
                    .contains("\"fetch\" \"--locked\" \"--manifest-path\" \"Cargo.toml\"` failed")
                {
                    "failed to fetch dependencies"
                } else if line
                    .contains("\"generate-lockfile\" \"--manifest-path\" \"Cargo.toml\"` failed")
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
        } else if line.contains("error: aborting due to") && line.contains("previous errors") {
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
        println!("Failure reasons:");
        for (failure_reason, count) in &self.failure_reasons {
            println!("{}: {}", failure_reason, count);
        }
        if !self.internal_errors.is_empty() {
            println!("Internal extractor errors: {}", self.internal_errors.len());
            println!("Examples:");
            for path in self.internal_errors.iter().take(5) {
                println!("  {:?}", path);
            }
        }
        if !self.unknown_failures.is_empty() {
            println!("Unknown failures: {}", self.unknown_failures.len());
            println!("Examples:");
            for path in self.unknown_failures.iter().take(5) {
                println!("  {:?}", path);
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
            // Let's just assume that we compiled the stdlib correctly.
            continue;
        }
        let success_file = package_dir.join("success");
        if !success_file.exists() {
            // Failed to compile this package.
            let logs_file = package_dir.join("logs");
            failures.update(logs_file);
            if delete_failures {
                std::fs::remove_dir_all(&package_dir)
                    .unwrap_or_else(|err| panic!("Failed to delete {:?}: {}", package_dir, err));
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
        if success_file.exists() || package_dir.file_name() == Some(std::ffi::OsStr::new("stdlib"))
        {
            // We have an assert in the compile action that guarantees that
            // stdlib was compiled successfully.
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
            assert!(logs_file.exists(), "missing logs file: {:?}", logs_file);
            to_move.push(logs_file);
        }
    }

    println!(
        "Collected {} files to move. Sleep for 20 seconds.",
        to_move.len()
    );
    std::thread::sleep(std::time::Duration::from_secs(20));
    println!("Start moving.");

    for (package_dir_name, package_file_names) in &file_names {
        let mut path = target_dir.join(package_dir_name);
        if !path.exists() {
            std::fs::create_dir_all(&path)
                .unwrap_or_else(|err| panic!("failed to create directory {:?}: {}", path, err));
        }
        path.push("files.json");
        let mut file = std::fs::File::create(&path)
            .unwrap_or_else(|e| panic!("Unable to create {:?}: {}", path, e));
        serde_json::to_writer_pretty(&mut file, package_file_names)
            .unwrap_or_else(|e| panic!("Unable to write {:?}: {}", path, e));
    }
    for from_path in to_move {
        let file_name = from_path.file_name().unwrap();
        let package_name = from_path.parent().unwrap().file_name().unwrap();
        let mut to_path = target_dir.join(package_name);
        if !to_path.exists() {
            error!(
                "Creating {:?}. Shouldn't all directories be already created?",
                to_path
            );
            std::fs::create_dir_all(&to_path)
                .unwrap_or_else(|err| panic!("failed to create directory {:?}: {}", to_path, err));
        }
        to_path.push(file_name);
        std::fs::rename(from_path, to_path).unwrap();
    }
}
