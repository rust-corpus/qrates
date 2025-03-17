# Future Work

## Disable incremental builds

We build crates using the `-Cincremental=incremental` compiler option.
We don't need incremental builds, however, and they tend to cause us more problems than they help. In particular, the incremental build directory needs to be deleted each time a re-compilation is executed (i.e., because `schema.dl` was changed), otherwise `rustc` will skip our extraction hook because it sees the crate is already compiled.

The only reason we need incremental builds, is because we need HIR hashes, and `rustc` only enables those if incremental builds are enabled: https://github.com/rust-lang/rust/blob/6dce9f8c2d8dde4c9ea20bab981cd70229c37fdc/compiler/rustc_ast_lowering/src/lib.rs#L454

## Uses unsafe/unsafe reasons

`RQ5.ipynb` from the paper "How Do Programmers Use Unsafe Rust?" is outdated.
The reason is that `rustc` moved the places where `uses_unsafe` and `unsafe_reasons` is computed. See `extractor/lib.rs`'s commented-out function `unsafety_check_result` for how this worked previously.

## Duplicate builds

Qrates builds every package in isolation.
When building a package, all its dependencies are built as well, and this adds up to quite a lot of data.
In order to get good results, the queries typically make use of derived queries `selected_...` that are composed of only a subset of the entire corresponding relation, namely those rows originating from builds defined in the `CrateList.json`.

Because the data still exists, though, this causes a large speed loss (due to merging relations across different crates) and increased disk usage.

It might be worth looking into whether we can determine if a build (and its rows) should be included in our merged database _before_ the `update-database` step completes.