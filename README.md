# Qrates: Analysing Rust Code Corpus

**Qrates** is a tool for running large scale analysis of Rust code. In case you are curious what you can do with **Qrates**, you can find the Jupyter notebooks we produced for our paper “How Do Programmers Use Unsafe Rust?” [here](./reports) (the CSV files are available [here](https://doi.org/10.5281/zenodo.4026639)).

[The documentation](https://rust-corpus.github.io/qrates/) explains how **Qrates** works and shows examples of how to use it on both public and private code bases. If you would like to use some part of **Qrates** as a library, you can find the API documentation [here](https://rust-corpus.github.io/qrates/doc/corpus_manager/index.html).

## Running Queries on Entire crates.io

Running queries on entire crates.io requires a machine that has at least 150 GB of RAM (contributions that reduce this number are very welcome!). To help others to run interesting analyses, we set up a CI job that runs the queries and publishes the generated CSV files [here](https://pmserver.inf.ethz.ch/rust-corpus/query-results.tar.gz). So, if you would like us to run your custom query, follow [the instructions](https://rust-corpus.github.io/qrates/queries_add_new.html) how to write your custom query and open a PR with it.

## History

The initial version of the framework (originally called RustQL) was developed by Nicolas Winkler as part of his [Bachelor thesis](https://ethz.ch/content/dam/ethz/special-interest/infk/chair-program-method/pm/documents/Education/Theses/Nicolas_Winkler_BA_report.pdf). Later, the Rust compiler team released [Rustwide](https://github.com/rust-lang/rustwide/) and stabilized the Rust procedural macros, which led the framework to be rewritten into its current form.

