#!/bin/sh

CRATE_PATH="`dirname \"$0\"`"

# variables for compilations
export RUSTC_WRAPPER=$CRATE_PATH/target/debug/extractor
export RUSTC_BACKTRACE=1

# target directory for extracted data
export EXTRACTOR_TARGET_DIR=$HOME/.rustql/crates



