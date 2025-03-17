use corpus_database::set_disk_map_temp_dir_root;
use log::info;
use std::path::Path;

mod build_files;
mod build_meta;
mod closure_kinds;
mod counters;
mod function_size;
mod non_tree_types;
mod prepare_builds;
mod prepare_items;
mod resolved_calls;
mod size;
mod traits;
mod types;
mod unsafe_block_calls;
mod unsafe_block_groups;
mod unsafe_reasons;
mod unsafe_spans;
mod unsafe_types;
mod utils;

pub fn run_query(
    loader: &corpus_database::tables::Loader,
    query_name: &str,
    database_root: &Path,
    report_path: &Path,
    workspace_path: &Path,
    sources_list_path: &Path,
) {
    info!("Running query: {}", query_name);
    match query_name {
        "prepare-builds" => prepare_builds::query(
            &loader,
            &report_path.join("prepare-builds"),
            workspace_path,
            sources_list_path,
        ),
        "prepare-items" => prepare_items::query(&loader),
        "prepare-all" => {
            run_query(
                &loader,
                "prepare-builds",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "prepare-items",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
        }
        "closure-kinds" => closure_kinds::query(&loader, &report_path.join("closure-kinds")),
        "counters" => {
            counters::query(&loader, &report_path.join("q-counters"));
        }
        "size" => {
            size::query(&loader, &report_path.join("q-size"));
        }
        "function-size" => {
            function_size::query(&loader, &report_path.join("function-size"));
        }
        "build-files" => build_files::query(&loader, &report_path.join("build-files")),
        "traits" => traits::query(&loader, &report_path.join("traits")),
        "types" => types::query(&loader, &report_path.join("types")),
        "resolved-calls" => resolved_calls::query(&loader, &report_path.join("resolved-calls")),
        "unsafe-types" => unsafe_types::query(&loader, &report_path.join("unsafe-types")),
        "unsafe-block-groups" => {
            unsafe_block_groups::query(&loader, &report_path.join("unsafe-block-groups"));
        }
        "unsafe-reasons" => unsafe_reasons::query(&loader, &report_path.join("unsafe-reasons")),
        "unsafe-block-calls" => {
            unsafe_block_calls::query(&loader, &report_path.join("unsafe-block-calls"));
        }
        "unsafe-spans" => unsafe_spans::query(&loader, &report_path.join("unsafe-spans")),
        "build-meta" => build_meta::query(&loader, &report_path.join("build-meta")),
        "non-tree-types" => non_tree_types::query(&loader, &report_path.join("non-tree-types")),
        "all" => {
            run_query(
                &loader,
                "unsafe-reasons",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "prepare-all",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "counters",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "size",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "function-size",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "build-files",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "build-meta",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "traits",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "types",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "unsafe-types",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "unsafe-block-groups",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "unsafe-block-calls",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "unsafe-spans",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
            run_query(
                &loader,
                "non-tree-types",
                database_root,
                report_path,
                workspace_path,
                sources_list_path,
            );
        }
        _ => unreachable!("Unknown query: {}", query_name),
    }
}
