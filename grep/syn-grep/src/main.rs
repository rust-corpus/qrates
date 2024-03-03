use rayon::prelude::*; // Import necessary traits and functions from the 'rayon' crate.

mod visitors;

#[derive(Debug)]
struct Report {
    file_path: String,
    function_reports: Vec<visitors::FunctionReport>,
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
    let file_contents = std::fs::read_to_string(file_path).unwrap();
    let ast = syn::parse_file(&file_contents).unwrap();
    let mut visitor = visitors::FunctionVisitor::default();
    syn::visit::visit_file(&mut visitor, &ast);
    assert!(visitor.unsafe_blocks.is_empty());
    Report {
        file_path: file_path.to_string(),
        function_reports: visitor.functions,
    }
}

fn main() {
    let file_list_path = std::env::args().nth(1).unwrap();
    let reports_list_path = std::env::args().nth(2).unwrap();
    let mut writer = csv::Writer::from_path(reports_list_path).unwrap();

    // Read the list of files stored in the file passed as a first argument.
    let file_list_contents = std::fs::read_to_string(file_list_path).unwrap();
    let file_list: Vec<&str> = file_list_contents.lines().collect();
    // Analyse the list of files in parallel.
    let reports: Vec<Report> = file_list
        .par_iter() // Use 'par_iter' from the 'rayon' crate to iterate in parallel.
        .map(|file_path| analyse_file(file_path))
        .collect();
    // Write the reports into the CSV file passed as a second argument.
    for report in &reports {
        for function_report in &report.function_reports {
            for unsafe_block in &function_report.unsafe_blocks {
                let row = Row {
                    file_path: &report.file_path,
                    function_name: &function_report.function_name,
                    is_unsafe: function_report.is_unsafe,
                    function_expression_count: function_report.expression_count,
                    unsafe_blocks_count: function_report.unsafe_blocks.len(),
                    unsafe_block_expression_count: unsafe_block.expression_count,
                };
                writer.serialize(row).unwrap();
            }
        }
    }
}
