# Creating the Database

To be able to run queries, the extracted information must be merged into a single database. Assuming you followed one of the previous sections for extracting files, you can create the database by running the following command from the directory in which you cloned Qrates:

```bash
cargo run --release -- update-database
```

This command expects to find the extracted files in directory `../workspace/rust-corpus/`. If you stored them somewhere else, you can specify the path to the workspace by using the `--workspace` argument.
