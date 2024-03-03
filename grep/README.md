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
