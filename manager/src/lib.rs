// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Library for managing crate sources.

mod compilation;
mod compilation_utils;
mod database;
mod queries;
mod sources_list;
mod top_crates;

use self::compilation::CompileManager;
use self::database::DatabaseManager;
use self::sources_list::CratesList;
use log::info;
use log_derive::logfn;
use std::path::Path;
use std::time::Duration;

/// Initialise the list of crates with ``top_count`` most downloaded crates.
#[logfn(Trace)]
pub fn initialise_with_top(crate_list_path: &Path, top_count: usize, all_versions: bool) {
    let crates_list = CratesList::top_crates_by_download_count(top_count, all_versions);
    crates_list.save(crate_list_path);
}

pub fn initialise_with_all(crate_list_path: &Path, all_versions: bool) {
    let crates_list = CratesList::all_crates(all_versions);
    crates_list.save(crate_list_path);
}

/// Compile the downloaded crates.
#[logfn(Trace)]
pub fn compile(crate_list_path: &Path, workspace: &Path, settings: &CompilationSettings) {
    let crates_list = CratesList::load(crate_list_path);
    compile_crates_list(crates_list, workspace, settings);
}

fn compile_crates_list(crates_list: CratesList, workspace: &Path, settings: &CompilationSettings) {
    make_manager(crates_list, workspace, settings)
        .compile_all()
        .unwrap();
}

fn make_manager(
    crates_list: CratesList,
    workspace: &Path,
    settings: &CompilationSettings,
) -> CompileManager {
    CompileManager::new(
        crates_list,
        workspace,
        settings.toolchain.clone(),
        settings.max_log_size,
        settings.memory_limit,
        settings.timeout,
        settings.enable_networking,
        settings.output_json,
        settings.use_original_rustc,
        settings.purge_build_dir,
        settings.custom_registry.clone(),
    )
}

#[logfn(Trace)]
pub fn compile_batched(
    batch_size: usize,
    batches_to_skip: usize,
    crate_list_path: &Path,
    workspace: &Path,
    output_folder: &Path,
    options: &CompilationSettings,
) {
    let crates_list = CratesList::load(crate_list_path);
    let batch_progress_path = crate_list_path.with_file_name("current_batch.txt");
    let batches = crates_list.batched(batch_size);
    let batch_count = batches.len();
    for (index, batch) in batches.into_iter().enumerate().skip(batches_to_skip) {
        std::fs::write(
            &batch_progress_path,
            format!("processing batch {} of {}\n", index + 1, batch_count),
        )
        .expect(&format!(
            "Could not write batch progress to {}",
            batch_progress_path.display(),
        ));

        let prefix = format!("[Batch {} of {}]", index + 1, batch_count);
        info!("{} Compiling", prefix);
        let manager = make_manager(batch, workspace, options);
        manager.compile_all().unwrap();
        info!("{} Moving output", prefix);
        compilation_utils::move_extracted(workspace, &output_folder);
        info!("{} Clearing build output", prefix);
        manager.clear_build_output().unwrap();
    }
    if batch_progress_path.exists() {
        std::fs::remove_file(&batch_progress_path)
            .expect("Could not remove batch progress file after finishing.");
    }
}

/// Classify the compilation errors.
#[logfn(Trace)]
pub fn check_compilation(workspace: &Path, delete_failures: bool) {
    self::compilation_utils::check_compilation(workspace, delete_failures);
}

/// Move deduplicated extracted facts to the specified directory.
#[logfn(Trace)]
pub fn move_extracted(workspace: &Path, target_dir: &Path) {
    self::compilation_utils::move_extracted(workspace, target_dir);
}

/// Update the database with the new information from the downloaded crates.
#[logfn(Trace)]
pub fn update_database(workspace: &Path, database_root: &Path) {
    let mut manager = DatabaseManager::new(database_root);
    manager.update_database(workspace);
}

/// Run the specified query.
#[logfn(Trace)]
pub fn run_query(
    query_name: &str,
    database_root: &Path,
    report_path: &Path,
    workspace_path: &Path,
    crate_list_path: &Path,
) {
    if !report_path.exists() {
        std::fs::create_dir_all(&report_path).unwrap();
    }
    queries::run_query(
        query_name,
        database_root,
        report_path,
        workspace_path,
        crate_list_path,
    );
}

#[derive(Clone, Debug)]
pub struct CompilationSettings {
    toolchain: String,
    max_log_size: usize,
    memory_limit: Option<usize>,
    timeout: Option<Duration>,
    enable_networking: bool,
    output_json: bool,
    use_original_rustc: bool,
    purge_build_dir: bool,
    custom_registry: Option<String>,
}

impl CompilationSettings {
    pub fn new(
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
        Self {
            toolchain,
            max_log_size,
            memory_limit,
            timeout,
            enable_networking,
            output_json,
            use_original_rustc,
            purge_build_dir,
            custom_registry,
        }
    }
}
