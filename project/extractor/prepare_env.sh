#!/bin/sh

CRATE_PATH="`dirname \"$0\"`"

export RUSTC_WRAPPER=$CRATE_PATH/target/debug/extractor
export RUSTC_BACKTRACE=1


