// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

//! Helper functions for obtaining the list of most downloaded crates.

use log::info;
use log_derive::logfn;
use serde::{Deserialize, Serialize};
use std::cmp::min;

/// A create on crates.io.
#[derive(Debug, Deserialize, Serialize)]
struct Crate {
    #[serde(rename = "id")]
    name: String,
}

/// The list of crates from crates.io
#[derive(Debug, Deserialize)]
struct CratesList {
    crates: Vec<Crate>,
}

fn get(url: &str) -> reqwest::Result<reqwest::blocking::Response> {
    reqwest::blocking::ClientBuilder::new()
        .user_agent("Rust Corpus - Top Crates Scrapper")
        .build()?
        .get(url)
        .send()
}

/// Create a list of top ``count`` crates.
#[logfn(Trace)]
pub fn top_crates_by_download_count(mut count: usize) -> Vec<String> {
    const PAGE_SIZE: usize = 100;
    let page_count = (count + PAGE_SIZE - 1) / PAGE_SIZE; // round up
    let mut sources = Vec::new();
    for page in 1..=page_count {
        info!("Fetching page {} of {}", page, page_count);
        let url = format!(
            "https://crates.io/api/v1/crates?page={}&per_page={}&sort=downloads",
            page, PAGE_SIZE
        );
        let resp = get(&url).expect("Could not fetch top crates");
        assert!(
            resp.status().is_success(),
            "Response status: {}",
            resp.status()
        );
        let reader = std::io::BufReader::new(resp); // this speeds up json deserialization by orders of magnitude
        let page_crates: CratesList = serde_json::from_reader(reader).expect("Invalid JSON");
        sources.extend(page_crates.crates.into_iter().take(count).map(|c| c.name));
        count -= min(PAGE_SIZE, count);
    }
    sources
}
