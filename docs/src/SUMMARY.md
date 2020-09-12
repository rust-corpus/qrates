# Qrates: Analysing Rust Code Corpus

**Qrates** is a tool for running large scale analysis of Rust code. To be scalable, the process is split into four phases:

1. *Data extraction.* `extractor` is a modified version of the Rust compiler that saves the information about the compiled crate to a file so that it can be easily accessed later.
2. *Database creation.* To be able to run queries that span multiple crates, the information from multiple files need to be merged into a single database.
3. *Queries.* The content of the database can be queried by using Rust programs. The procedural macros from [Datapond](https://github.com/lqd/datapond) can be used to write fix-point computations in Datalog.
4. *Query results analysis.* Typically, the query results are saved as CSV files so that they can be easily visualized by using data analysis tools such as [Pandas](https://pandas.pydata.org/).

Chapters:

1. Building:

    - [Compiling Qrates](./building.md)

2. Extractor:

    - [Extracting Data from Crates Published on crates.io](./extractor_crates_io.md)
    - [Extracting Data from a Private Rust Project](./extractor_private.md)

3. Database:

    - [Creating the Database](./creating_database.md)
    - [Database Structure](./database_structure.md)

4. Queries:

    - [Running Existing Queries](./queries_run_existing.md)
    - [Add a New Query](./queries_add_new.md)

5. Analysis:

    - [Analysing Query Results with Jupyter](./analysis_jupyter.md)
