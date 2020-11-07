# Compiling Qrates

**Note:** These instructions were tested on Ubuntu 18.04 and 20.04.

Install dependencies:

```bash
sudo apt install build-essential curl git libssl-dev
```

Install Rust:

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
```

Clone the repository and all its dependencies:

```bash
git clone https://github.com/rust-corpus/qrates.git
cd qrates
git submodule update --init
```

Add the missing components (we need them because the extractor uses the Rust compiler as a library):

```bash
rustup component add rustc-dev
rustup component add rust-src
rustup component add llvm-tools-preview
```

Build the project in release mode:

```bash
cargo build --all --release
```
