use std::path::Path;

use corpus_database::{tables::Loader, types};
use log::info;

pub fn query(loader: &Loader, report_path: &Path) {
    let closure_kinds = loader.load_thir_exprs_closure_kind();
    let mut fn_count = 0;
    let mut fnmut_count = 0;
    let mut fnonce_count = 0;
    for (_, kind) in closure_kinds.iter() {
        match kind {
            types::ClosureKind::Fn => fn_count += 1,
            types::ClosureKind::FnMut => fnmut_count += 1,
            types::ClosureKind::FnOnce => fnonce_count += 1,
            types::ClosureKind::Unknown => {}
        }
    }
    // Note: This query counts over _all_ builds, not just the selected builds.
    info!(
        "[all builds] fn_count = {}, fnmut_count = {}, fnonce_count = {}",
        fn_count, fnmut_count, fnonce_count
    );
}
