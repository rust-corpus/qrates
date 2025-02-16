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
    let def_path_spans = loader.load_def_path_span();
    let def_path_spans: HashMap<_, _> = def_path_spans.iter().copied().collect();

    let selected_function_definitions = loader.load_selected_function_definitions();
    let unsafe_function_spans = selected_function_definitions.iter().flat_map(
        |&(build, _item, def_path, _module, visibility, unsafety, abi, _return_ty, uses_unsafe)| {
            if unsafety == types::Safety::Unsafe {
                Some((
                    build,
                    build_resolver.resolve(build),
                    def_path_resolver.resolve(def_path),
                    visibility.to_string(),
                    &strings[abis[abi]],
                    uses_unsafe,
                    span_resolver.resolve(def_path_spans[&def_path]),
                ))
            } else {
                None
            }
        },
    );

    write_csv!(report_path, unsafe_function_spans);
}

pub fn query(loader: &Loader, report_path: &Path) {
    report_unsafe_function_spans(loader, report_path);
}
