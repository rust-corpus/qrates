use log::info;
use std::path::Path;

mod build_files;
mod build_meta;
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
    query_name: &str,
    database_root: &Path,
    report_path: &Path,
    workspace_path: &Path,
    sources_list_path: &Path,
) {
    info!("Running query: {}", query_name);
    let loader = corpus_database::tables::Loader::new(database_root.to_path_buf());
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
            counters::query(&loader, &report_path.join("q-counters"));
            counters::new_query(&loader, &report_path.join("q-counters"));
        }
        "size" => {
            size::query(&loader, &report_path.join("q-size"));
            size::new_query(&loader, &report_path.join("q-size"));
        }
        "function-size" => {
            function_size::query(&loader, &report_path.join("function-size"));
            function_size::new_query(&loader, &report_path.join("function-size"));
        }
        "build-files" => build_files::query(&loader, &report_path.join("build-files")),
        "traits" => traits::query(&loader, &report_path.join("traits")),
        "types" => types::query(&loader, &report_path.join("types")),
        "resolved-calls" => resolved_calls::query(&loader, &report_path.join("resolved-calls")),
        "unsafe-types" => unsafe_types::query(&loader, &report_path.join("unsafe-types")),
        "unsafe-block-groups" => {
            unsafe_block_groups::query(&loader, &report_path.join("unsafe-block-groups"));
            unsafe_block_groups::new_query(&loader, &report_path.join("unsafe-block-groups"));
        }
        "unsafe-reasons" => unsafe_reasons::query(&loader, &report_path.join("unsafe-reasons")),
        "unsafe-block-calls" => {
            unsafe_block_calls::query(&loader, &report_path.join("unsafe-block-calls"));
            unsafe_block_calls::new_query(&loader, &report_path.join("unsafe-block-calls"));
        }
        "unsafe-spans" => unsafe_spans::query(&loader, &report_path.join("unsafe-spans")),
        "build-meta" => build_meta::query(&loader, &report_path.join("build-meta")),
        "non-tree-types" => non_tree_types::query(&loader, &report_path.join("non-tree-types")),
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
        _ => unreachable!("Unknown query: {}", query_name),
    }
}
