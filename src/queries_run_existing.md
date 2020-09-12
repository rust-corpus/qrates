# Running Existing Queries

To run all existing queries, execute:

```bash
cargo run --release -- query all
```

This will invoke the query `all` that is a meta-query that runs all other queries. The queries are defined in `manager/src/queries`. You can find the documentation of what exactly each of them does in their doc-comments.

Most queries store results in CSV files that can be found in the `../workspace/reports` directory.
