#!/usr/bin/env bash

cargo clean
rm Cargo.lock
cargo afl build
cargo afl fuzz -i fuzz/in -o fuzz/out target/debug/fuzz
