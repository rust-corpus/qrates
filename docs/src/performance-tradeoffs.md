# Performance Tradeoffs

In Qrates there are different tradeoffs relating to ease of implementation and memory usage that every user can decide for themselves.
This chapter goes over the differences and implementation details.

Some concepts relating to the database are explained in [Add a New Query](./queries_add_new.md).

## Datalog

The original version of Qrates was built with Datalog in mind.
Datalog generally permits easy to write implementations at the cost of unpredictable performance and increased memory consumption, especially with the solver that we are using.
For an example of a Datalog query, see [Add a New Query](./queries_add_new.md).

The solver needs to load every relation used by the query into memory, which can get prohibitively expensive for large datasets like crates.io (hundreds of GB).

## Native Rust

There are a few options available in Qrates that allow for reducing memory usage.

### Streaming

By default, relations are loaded from disk in a streaming fashion.
This means that iteration over every row is cheap w.r.t. memory usage.

Unfortunately, some queries need to load more data than just a single row into memory.
For example, joining data from two relations requires one of them to be a lookup table for the streaming relation to received joined-in data.
Typically, this requires analyzing the relative sizes of the relations and picking the one that would result in a smaller lookup table, then `Iterator::collect`ing it into a `HashMap` or similar.

For this reason, Qrates provides the option of using a precomputed lookup table, the `RelationMap`. This is described in the next section.

### Relation Maps

`RelationMap`s are precomputed maps for relations and are backed by a key-value store on disk.
They are defined with the `keyed by <key column>` syntax in `schema.dl`, taking for example the `thir_blocks` relation:
```
relation thir_blocks(parent: ThirBlock, block: auto ThirBlock, safety: BlockSafety, check_mode: BlockCheckMode, span: Span) keyed by block;
```
The addition of `keyed by block` results in a new `Loader::load_thir_blocks_relation_map()` method being generated that loads a `RelationMap` mapping the column `block` to `(parent, safety, check_mode, span)` (the remaining columns).

Now, if we want to join data from `thir_blocks` and some other relation that has a foreign key `block` into `thir_blocks`, we can iterate over the other relation in a streaming fashion and join-in data from `thir_blocks` via the `RelationMap`, while not needing to load the entire `thir_blocks` relation into memory.

Note: At the moment there is a restriction of at most one `RelationMap` per relation.

**Warning**: The key for a `RelationMap` should be a primary key. If there are duplicate entries in the key column, only one row will be stored in the `RelationMap`. 

### `DiskMap`/`DiskVec`

Qrates provides alternative data structures to the `HashMap<K, V>` and `Vec<T>`: The `DiskMap<K, V>` and `DiskVec<T>`.
These are the backend for `RelationMap`s, but can be used on their own.
Because they offload data to the disk and as such easily outperformed by their memory-only counterparts, they should only be used sparingly and not for intermediate collections that are known to be small (e.g., the total set of all crates.io categories will never take up more than a few MB).

For example, the `counters` query uses a `DiskMap` for storing a denormalized version of a large relation:
```rust,no_run,noplayground
let mut thir_block_parent_to_children: DiskMap<_, Vec<_>> = DiskMap::create_temp();
for (parent, child, _safety, _check_mode, _span) in loader.load_iter_thir_blocks() {
    let mut children = thir_block_parent_to_children.get(parent).unwrap_or_default();
    children.push(child);
    thir_block_parent_to_children.insert(parent, children);
}
```


## Implementation Notes

### Datalog

We use [datapond](https://github.com/lqd/datapond) to write Datalog queries, which get solved by the [datafrog](https://github.com/rust-lang/datafrog) engine.

Currently everything loaded into datapond gets loaded into memory in its entirety.

### Database

The database exists in two distinct stages: During extraction, and during querying.

During extraction there is a per-crate database.
Because the extracted data is small on an individual crate basis, this version of the database is entirely in memory for speed.

After extraction all the databases from all the compiled crates get combined, as described in [Creating the Database](./creating_database.md).
This also turns the everything into disk-backed counterparts of the data structures.

In particular, during querying, everything accessible from the `Loader` struct (see [Add a New Query](./queries_add_new.md) for an introduction) is backed by a file on disk.
This includes relations, relation maps, and interning tables.
Our abstraction for this is the `DiskMap`, as described above.

To gain some speedup at the cost of very little memory, there is a static cache size setting for `DiskMap`s, configured by the associated constants `DiskMap::DISK_MAP_WRITE_CACHE_SIZE` and `DiskMap::DISK_MAP_REDB_CACHE_SIZE`.

Internally, the `DiskMap` uses the pure Rust key-value store [redb](https://github.com/cberner/redb).

Due to this, there are a few constraints on how `DiskMap` is used:
1. A `DiskMap` always has a backing file, even if it is for temporary computations. This is the redb database.
2. Because we are using redb internally, types must implement redb's traits for de-/serialization and lookup.

We provide trait implementations for the database types used by Qrates, so 2. is typically not an issue.

The important thing to note about 1. is that there are a few ways to create a `DiskMap` without specifying where the file should live. This is either explicitly via `DiskMap::create_temp()` or via trait implementations like `FromIterator`.
If you want to make use of `DiskMap`s without specifying a path for every map, you must set a root directory in which a new temporary file for every DiskMap created in this way will be stored.
Call `set_disk_map_temp_dir_root(...)` to set this path.
If you are working in the typical `manager/src/queries/` environment, this is already done for you.

These `DiskMap`s are called "temporary `DiskMap`"s, because their file gets deleted when the Rust value is dropped, so they should only be used for intermediate computations that would otherwise require too much memory.