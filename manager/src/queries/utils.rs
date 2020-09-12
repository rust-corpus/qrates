use corpus_database::InterningTable;
use corpus_database::{tables::Loader, types};
use itertools::Itertools;
use std::cell::Ref;
use std::collections::HashMap;

#[macro_export]
macro_rules! write_csv {
    ($reports_dir_path:ident, $data:expr) => {
        if !$reports_dir_path.exists() {
            std::fs::create_dir($reports_dir_path).unwrap();
        }
        let file_path = $reports_dir_path.join(format!("{}.csv", stringify!($data)));
        let mut wtr = csv::Writer::from_path(file_path).unwrap();
        for row in $data {
            wtr.serialize(row).unwrap();
        }
        wtr.flush().unwrap();
    };
}

pub trait GroupByIterator: Itertools {
    /// Itertools::group_by groups consecutive elements. This version groups
    /// also non-consecutive elements.
    fn safe_group_by<K, F>(
        self,
        key: F,
    ) -> itertools::GroupBy<K, std::vec::IntoIter<<Self as std::iter::Iterator>::Item>, F>
    where
        Self: Sized,
        F: Copy + FnMut(&Self::Item) -> K,
        K: PartialEq + Ord,
    {
        self.sorted_by_key(key).group_by(key)
    }
}

impl<T: ?Sized> GroupByIterator for T where T: Itertools {}

/// A helper struct for converting an interned `DefPath` into human readable
/// tuple of strings.
pub struct BuildResolver<'b> {
    builds: Ref<
        'b,
        InterningTable<
            types::Build,
            (
                types::Package,
                types::PackageVersion,
                types::Krate,
                types::CrateHash,
                types::Edition,
            ),
        >,
    >,
    package_names: Ref<'b, InterningTable<types::Package, types::InternedString>>,
    package_versions: Ref<'b, InterningTable<types::PackageVersion, types::InternedString>>,
    crate_names: Ref<'b, InterningTable<types::Krate, types::InternedString>>,
    editions: Ref<'b, InterningTable<types::Edition, types::InternedString>>,
    strings: Ref<'b, InterningTable<types::InternedString, String>>,
}

impl<'b> BuildResolver<'b> {
    pub fn new(loader: &'b Loader) -> Self {
        Self {
            builds: loader.load_builds(),
            package_names: loader.load_package_names(),
            package_versions: loader.load_package_versions(),
            crate_names: loader.load_crate_names(),
            editions: loader.load_editions(),
            strings: loader.load_strings(),
        }
    }
    pub fn resolve(&self, build: types::Build) -> (&str, &str, &str, String, &str) {
        let (package_name, package_version, crate_name, crate_hash, edition) = self.builds[build];
        (
            &self.strings[self.package_names[package_name]],
            &self.strings[self.package_versions[package_version]],
            &self.strings[self.crate_names[crate_name]],
            format!("{:x}", crate_hash),
            &self.strings[self.editions[edition]],
        )
    }
}

/// A helper struct for converting an interned `DefPath` into human readable
/// tuple of strings.
pub struct DefPathResolver<'b> {
    def_paths: Ref<
        'b,
        InterningTable<
            types::DefPath,
            (
                types::Krate,
                types::CrateHash,
                types::RelativeDefId,
                types::DefPathHash,
                types::SummaryId,
            ),
        >,
    >,
    crate_names: Ref<'b, InterningTable<types::Krate, types::InternedString>>,
    relative_def_paths: Ref<'b, InterningTable<types::RelativeDefId, types::InternedString>>,
    summary_keys: Ref<'b, InterningTable<types::SummaryId, types::InternedString>>,
    strings: Ref<'b, InterningTable<types::InternedString, String>>,
}

impl<'b> DefPathResolver<'b> {
    pub fn new(loader: &'b Loader) -> Self {
        Self {
            def_paths: loader.load_def_paths(),
            crate_names: loader.load_crate_names(),
            relative_def_paths: loader.load_relative_def_paths(),
            summary_keys: loader.load_summary_keys(),
            strings: loader.load_strings(),
        }
    }
    pub fn resolve(&self, def_path: types::DefPath) -> (&str, String, &str, String, &str) {
        let (crate_name, crate_hash, relative_def_path, def_path_hash, summary_key) =
            self.def_paths[def_path];
        (
            &self.strings[self.crate_names[crate_name]],
            format!("{:x}", crate_hash),
            &self.strings[self.relative_def_paths[relative_def_path]],
            format!("{:x}", def_path_hash),
            &self.strings[self.summary_keys[summary_key]],
        )
    }
}

/// A helper struct for converting an interned `span` into human readable
/// tuple of strings.
pub struct SpanResolver<'b> {
    spans: HashMap<
        types::Span,
        (
            types::SpanExpansionKind,
            types::InternedString,
            types::SpanFileName,
            u16,
            u16,
        ),
    >,
    span_file_names: Ref<'b, InterningTable<types::SpanFileName, types::InternedString>>,
    strings: Ref<'b, InterningTable<types::InternedString, String>>,
}

impl<'b> SpanResolver<'b> {
    pub fn new(loader: &'b Loader) -> Self {
        let spans = loader
            .load_spans()
            .iter()
            .map(
                |&(
                    span,
                    _call_site_span,
                    expansion_kind,
                    expansion_kind_descr,
                    file_name,
                    line,
                    col,
                )| {
                    (
                        span,
                        (expansion_kind, expansion_kind_descr, file_name, line, col),
                    )
                },
            )
            .collect();
        Self {
            spans: spans,
            span_file_names: loader.load_span_file_names(),
            strings: loader.load_strings(),
        }
    }
    pub fn resolve(&self, span: types::Span) -> (types::Span, String, &str, &str, u16, u16) {
        let (expansion_kind, expansion_kind_descr, file_name, line, col) = self.spans[&span];
        (
            span,
            format!("{:?}", expansion_kind),
            &self.strings[expansion_kind_descr],
            &self.strings[self.span_file_names[file_name]],
            line,
            col,
        )
    }
    pub fn get_expansion_kind(&self, span: types::Span) -> types::SpanExpansionKind {
        let (expansion_kind, _expansion_kind_descr, _file_name, _line, _col) = self.spans[&span];
        expansion_kind
    }
}

/// From relation `iter` filters the facts that belong only to `selected_builds`.
pub fn filter_selected<F1, F2, I, O>(
    iter: impl Iterator<Item = I>,
    selected_builds: &[(
        types::Build,
        types::Package,
        types::PackageVersion,
        types::Krate,
        types::CrateHash,
        types::Edition,
    )],
    def_paths: &InterningTable<
        types::DefPath,
        (
            types::Krate,
            types::CrateHash,
            types::RelativeDefId,
            types::DefPathHash,
            types::SummaryId,
        ),
    >,
    extract_def_path: F1,
    construct_result: F2,
) -> Vec<O>
where
    F1: Fn(I) -> types::DefPath,
    F2: Fn(types::Build, I) -> O,
    I: Clone,
{
    let selected_builds_set: HashMap<_, _> = selected_builds
        .iter()
        .map(
            |&(build, _package, _version, krate, crate_hash, _edition)| {
                ((krate, crate_hash), build)
            },
        )
        .collect();
    iter.flat_map(|element| {
        let def_path = extract_def_path(element.clone());
        let (krate, crate_hash, _, _, _) = def_paths[def_path];
        selected_builds_set
            .get(&(krate, crate_hash))
            .map(|build| construct_result(*build, element))
    })
    .collect()
}
