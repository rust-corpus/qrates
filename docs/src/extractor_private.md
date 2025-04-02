# Extracting Data from a Private Rust Project

This section shows how to extract data from a private Rust project.

## Building Qrates

The first step is to check out and compile Qrates:

```bash
git clone https://github.com/rust-corpus/qrates.git
cd qrates
git submodule update --init
cargo build --all --release
```

After the successful build, in the `target/release` directory there should be an executable file called `rustc`. We will extract the information by using this special `rustc` to compile the project. To do so, we need to set environment variable `RUSTC` to contain its path:

```bash
export RUSTC="$(pwd)/target/release/rustc"
```

We also need to set the environment variable `SYSROOT` to contain the sysroot of the Rust version we used to compile Qrates and `LD_LIBRARY_PATH` to contain the `lib` directory in `SYSROOT`:

```bash
export SYSROOT="$(rustc --print sysroot)"
export LD_LIBRARY_PATH="$SYSROOT/lib"
```

We also need to create a directory to store the extracted data and set the environment variable `CORPUS_RESULTS_DIR` to point to it:

```bash
mkdir -p ../workspace/rust-corpus/crate_name-0.0.0/
export CORPUS_RESULTS_DIR="$(pwd)/../workspace/rust-corpus/crate_name-0.0.0/"
```

## Compiling a Project

As an example, let's try to extract information from the `master` branch of the [rand](https://github.com/rust-random/rand) crate.

Clone the `rand` crate repository:

```bash
cd /tmp
git clone https://github.com/rust-random/rand.git
cd rand
```

Check that the environment variables `RUSTC`, `SYSROOT`, `LD_LIBRARY_PATH`, and `CORPUS_RESULTS_DIR` are set correctly.

Compile the project:

```bash
cargo build
```

If the compilation was successful, `CORPUS_RESULTS_DIR` directory should contain many `bincode` files:

```console
$ ls $CORPUS_RESULTS_DIR
build_script_build_641a6913d88f2b1b.bincode  ppv_lite86_89695c0a0a962fc8.bincode
build_script_build_679051cf1df6d8f8.bincode  rand_0330e33c1ee64866.bincode
cfg_if_f903336a35b88a26.bincode              rand_chacha_23c71b977e463cb8.bincode
getrandom_9a46159fdf341523.bincode           rand_core_272caddfb637ce01.bincode
```

If you want to use the query functionality, `CrateList.json` needs to contain the crate, and there needs to be a `success` file in the extracted directory:

```bash
touch $CORPUS_RESULTS_DIR/success
```
