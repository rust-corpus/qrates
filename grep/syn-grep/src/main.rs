use rayon::prelude::*;
use serde::ser::{Serialize, Serializer};

mod visitors;

#[derive(Debug)]
enum Status {
    Success,
    FileReadError(String),
    SynParseError(String),
}

impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Status::Success => serializer.serialize_str("Success"),
            Status::FileReadError(ref s) => {
                serializer.serialize_str(&format!("FileReadError: {}", s))
            }
            Status::SynParseError(ref s) => {
                serializer.serialize_str(&format!("SynParseError: {}", s))
            }
        }
    }
}

#[derive(Debug)]
struct Report {
    file_path: String,
    status: Status,
    functions: Vec<visitors::FunctionReport>,
    global_unsafe_blocks: Vec<visitors::UnsafeBlockReport>,
}

#[derive(Debug, serde::Serialize)]
struct StatusRow<'a> {
    file_path: &'a str,
    status: &'a Status,
}

#[derive(Debug, serde::Serialize)]
struct GlobalUnsafeRow<'a> {
    file_path: &'a str,
    expression_count: u64,
}

#[derive(Debug, serde::Serialize)]
struct Row<'a> {
    file_path: &'a str,
    function_name: &'a str,
    is_unsafe: bool,
    function_expression_count: u64,
    unsafe_blocks_count: usize,
    unsafe_block_expression_count: u64,
}

fn analyse_file(file_path: &str) -> Report {
    let mut report = Report {
        file_path: file_path.to_string(),
        status: Status::Success,
        functions: Vec::new(),
        global_unsafe_blocks: Vec::new(),
    };
    let file_contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            report.status = Status::FileReadError(error.to_string());
            return report;
        }
    };
    let ast = match syn::parse_file(&file_contents) {
        Ok(ast) => ast,
        Err(error) => {
            let span = error.span();
            let start = span.start();
            let end = span.end();
            report.status = Status::SynParseError(format!(
                "{} ({}:{}-{}:{})",
                error.to_string(),
                start.line,
                start.column,
                end.line,
                end.column
            ));
            return report;
        }
    };
    let mut visitor = visitors::FunctionVisitor::default();
    syn::visit::visit_file(&mut visitor, &ast);
    report.functions = visitor.functions;
    report.global_unsafe_blocks = visitor.unsafe_blocks;
    report
}

fn main() {
    let file_list_path = std::env::args().nth(1).unwrap();
    let output_prefix = std::env::args().nth(2).unwrap();
    let reports_list_path = format!("{}-functions.csv", output_prefix);
    let status_path = format!("{}-run-status.csv", output_prefix);
    let global_unsafe_path = format!("{}-global-blocks.csv", output_prefix);
    let mut functions_writer = csv::Writer::from_path(reports_list_path).unwrap();
    let mut status_writer = csv::Writer::from_path(status_path).unwrap();
    let mut global_unsafe_writer = csv::Writer::from_path(global_unsafe_path).unwrap();

    // Read the list of files stored in the file passed as a first argument.
    let file_list_contents = std::fs::read_to_string(file_list_path).unwrap();
    let file_list: Vec<&str> = file_list_contents.lines().collect();
    // Analyse the list of files in parallel.
    let reports: Vec<Report> = file_list
        .par_iter()
        .filter(|file_path| file_path.ends_with(".rs"))
        .map(|file_path| analyse_file(file_path))
        .collect();
    // Write the reports into the CSV file passed as a second argument.
    for report in &reports {
        let status_row = StatusRow {
            file_path: &report.file_path,
            status: &report.status,
        };
        status_writer.serialize(status_row).unwrap();
        for unsafe_block in &report.global_unsafe_blocks {
            let row = GlobalUnsafeRow {
                file_path: &report.file_path,
                expression_count: unsafe_block.expression_count,
            };
            global_unsafe_writer.serialize(row).unwrap();
        }
        for function_report in &report.functions {
            for unsafe_block in &function_report.unsafe_blocks {
                let row = Row {
                    file_path: &report.file_path,
                    function_name: &function_report.function_name,
                    is_unsafe: function_report.is_unsafe,
                    function_expression_count: function_report.expression_count,
                    unsafe_blocks_count: function_report.unsafe_blocks.len(),
                    unsafe_block_expression_count: unsafe_block.expression_count,
                };
                functions_writer.serialize(row).unwrap();
            }
        }
    }
}
