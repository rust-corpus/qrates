#!/bin/sh

# Licensed under the MIT license <LICENSE or
# http://opensource.org/licenses/MIT>. This file may not be copied,
# modified, or distributed except according to those terms.

CRATE_PATH=$(realpath "$(dirname \"$0\")")

# variables for compilations
export RUSTC_WRAPPER="$CRATE_PATH/target/release/rustql-extractor"
export RUSTC_BACKTRACE=1

# target directory for extracted data
export EXTRACTOR_TARGET_DIR=$HOME/.rustql/crates
