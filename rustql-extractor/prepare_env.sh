#!/bin/sh

# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

CRATE_PATH="`dirname \"$0\"`"

# variables for compilations
export RUSTC_WRAPPER=$CRATE_PATH/target/debug/rustql-extractor
export RUSTC_BACKTRACE=1

# target directory for extracted data
export EXTRACTOR_TARGET_DIR=$HOME/.rustql/crates
