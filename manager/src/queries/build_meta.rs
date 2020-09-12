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
        .map(|&(build, category)| (build, build_resolver.resolve(build), &strings[category]));

    write_csv!(report_path, categories);
}

fn report_build_keywords(loader: &Loader, report_path: &Path) {
    let build_resolver = BuildResolver::new(loader);
    let strings = loader.load_strings();

    let keywords = loader.load_crate_keywords();
    let keywords = keywords
        .iter()
        .map(|&(build, keyword)| (build, build_resolver.resolve(build), &strings[keyword]));

    write_csv!(report_path, keywords);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_build_categories(loader, report_path);
    report_build_keywords(loader, report_path);
}
