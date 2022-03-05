#!/bin/sh
export RUSTFLAGS="-C instrument-coverage"
export LLVM_PROFILE_FILE="%p-%m.profraw"

rm -rf *.profraw coverage

cargo build

cargo test

grcov . --binary-path target/debug/ -s . -t html --branch --ignore-not-existing -o ./coverage/
