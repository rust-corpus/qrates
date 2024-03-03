# Grepping crates.io

## Download all crates

```bash
git clone https://github.com/the-lean-crate/criner
cd criner
cargo run —release — mine
```

The crates will be stored in `criner.db` directory.

## Extract all crates

Directory `criner` is where Criner repository was cloned. Directory `qrates` is where Qrates repository was cloned.

```bash
python3 qrates/grep/ungzip.py criner/criner.db/assets/ extracted-crates >> log
```

## Grep

Find all files that mention `unsafe `:

```bash
rg --files-with-matches 'unsafe ' extracted-crates/ > unsafe-files.csv
```

Analyse these files using the syn-based analysis:

```bash
qrates/grep/syn-grep/target/release/syn-grep unsafe-files.csv unsafe-report 
```

This command produces the following files:

* `unsafe-report-functions.csv` – unsafe blocks inside functions.
* `unsafe-report-global-blocks.csv` – global unsafe blocks.
* `unsafe-report-run-status.csv` – whether analysing a file was successful or led to an error.
