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
        .map(|(def_path, reason)| (def_path_resolver.resolve(*def_path), &strings[*reason]))
        .collect();
    write_csv!(report_path, function_unsafe_reasons);
}
