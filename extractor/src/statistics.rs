//! A binary that prints the statistics about the information stored in the
//! database.
//!
//! To run, use: `cargo run --bin print-stats database.bincode`.

use corpus_database::tables;
use std::env;

fn main() {
    let path = env::args()
        .nth(1)
        .expect("The only command line argument should be the path of the database file.");
    let path: std::path::PathBuf = path.into();
    let tables = tables::Tables::load(&path).unwrap();
    tables.print_statistics();
}
