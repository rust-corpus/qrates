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

## Rust Jupyter Kernel

Currently, there are two places to define a query and there is no clear separation of concerns: The `manager/src/queries` Rust module, and the Jupyter notebooks.

The reason we need the Rust module is for performance and interaction with the Qrates database format.
The Jupyter notebook is used for fast iteration on pre-computed query results, where pre-computation is often just joining in data from different relations and resolving interned values.
By switching to a Rust kernel for Jupyter, it might be possible to get rid of the in-tree Rust module and instead write every query directly in Jupyter, perhaps with a set of pre-defined helper queries that join common relations/resolve interned values.
Additionally, this would let us re-use the existing type safety of Qrates and avoid the intermediate step of loading query results into a CSV for passing data between Rust and Jupyter with a Python kernel.

## Streamified Datalog Queries

Qrates provides two query paradigms - native Rust and a Datalog-style macro.
The Datalog-style macro has to be avoided at the moment for memory intensive workloads, because all relations are loaded into memory.
Can we build a Datalog solver that can analyze simple queries for the best streaming order and/or use pre-computed, disk-backed `RelationMap`s?