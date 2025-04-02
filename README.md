# Qrates: Analysing Rust Code Corpus

**Qrates** is a tool for running large scale analysis of Rust code. In case you are curious what you can do with **Qrates**, you can find the Jupyter notebooks we produced for our paper “How Do Programmers Use Unsafe Rust?” [here](./reports) (the CSV files are available [here](https://doi.org/10.5281/zenodo.4026639)).

[The documentation](https://rust-corpus.github.io/qrates/) explains how **Qrates** works and shows examples of how to use it on both public and private code bases. If you would like to use some part of **Qrates** as a library, you can find the API documentation [here](https://rust-corpus.github.io/qrates/doc/corpus_manager/index.html).

## Running Queries on Entire crates.io

~~Running queries on entire crates.io requires a machine that has at least 150 GB of RAM (contributions that reduce this number are very welcome!). To help others to run interesting analyses, we set up a CI job that runs the queries and publishes the generated CSV files [here](https://pmserver.inf.ethz.ch/rust-corpus/query-results.tar.gz). So, if you would like us to run your custom query, follow [the instructions](https://rust-corpus.github.io/qrates/queries_add_new.html) how to write your custom query and open a PR with it.~~

Qrates was updated in 2025 to use less memory. Analyzing 2000 crates took 3.5GB RAM, but larger runs have not been tested yet.
Please let us know your findings if you run Qrates on a larger dataset!

## Future Work

Please see [FUTURE_WORK.md](./FUTURE_WORK.md).

## History

### 2020

The initial version of the framework (originally called RustQL) was developed by Nicolas Winkler as part of his [Bachelor thesis](https://ethz.ch/content/dam/ethz/special-interest/infk/chair-program-method/pm/documents/Education/Theses/Nicolas_Winkler_BA_report.pdf). Later, the Rust compiler team released [Rustwide](https://github.com/rust-lang/rustwide/) and stabilized the Rust procedural macros, which led the framework to be rewritten into its current form.

### 2025

Qrates was updated to use a modern compiler. This forced a change in the database schema: We had to move the entire `unsafe` analysis from MIR to THIR.

Next, the Qrates backend and queries were rewritten to use less memory.
In 2020, with around 33000 crates, using MIR as the main part of the database schema, a run took around 150GB of RAM.
In 2025, with around 2000 crates, and THIR as the main part of the database schema (which inherently increases the size), a run takes around 3.5GB of RAM.
It is worth to be noted, however, that this is mostly due to caching of data, i.e., linear increases of memory usage are *not expected* for increasing corpus sizes. 