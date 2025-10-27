#!/bin/bash

cd ./harness
cargo clean
cargo update -q -p home --precise 0.5.5 2>/dev/null
RUSTFLAGS="-Zsanitizer=address" cargo +rustc-replay-crash afl build --target x86_64-unknown-linux-gnu
cat ../crash_input |  target/x86_64-unknown-linux-gnu/debug/arenavec_fuzz_10473694077003418768