#!/bin/bash


export SURF_HARNESS_GENERATOR_PATH=$HOME/deepSURF/code/harness_generator
export GLOBAL_DATA_PATH=$HOME/deepSURF/code/global_data
export SURF_ENABLE_VERBOSE=1
export SURF_ENABLE_LLMS=1
export SURF_DISABLE_REMOVE_LAST_TARGET=1
export SURF_SKIP_OPTION="condskip"
export OPENROUTER_API_KEY="<YOUR OPENROUTER API KEY>"
export SURF_ENABLE_OPTIMIZED_TREE_GEN=1
#export LLM_BACKEND="deepseek/deepseek-r1"

# The commands below reproduce the deepSURF harness-generation results of RQ2.
#
# Uncomment one or more sets of commands depending on what you want to reproduce.
# Some reproductions require toggling specific parameters; remember to
# uncomment those parameter lines as well.
#
# Hint: you can split the reproduction workload across multiple scripts
# and run them in parallel.


# ERASAN Crates (#27)

export SURF_ANALYZE_LIB=1
export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/algorithmica-0.1.8/
cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
unset SURF_ANALYZE_LIB
cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/algorithmica-0.1.8/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/toodee-0.2.4/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/toodee-0.2.4/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/http-0.1.19/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# unset SURF_DISABLE_LLM_DOCUMENTATION
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/http-0.1.19/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# export SURF_DISABLE_LLM_DOCUMENTATION=1

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/through-0.1.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/through-0.1.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/string-interner-0.7.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/string-interner-0.7.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/slice-deque-0.3.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/slice-deque-0.3.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/simple-slab-0.3.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/simple-slab-0.3.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/rdiff-0.1.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/rdiff-0.1.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/qwutils-0.3.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/qwutils-0.3.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/ordnung-0.0.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/ordnung-0.0.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/insert_many-0.1.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/insert_many-0.1.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/futures-0.3.5/futures-task/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/futures-0.3.5/futures-task/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/futures-0.3.3/futures-task/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/futures-0.3.3/futures-task/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/endian_trait-0.6.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/endian_trait-0.6.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/cbox-0.3.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/cbox-0.3.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/bumpalo-3.11.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/bumpalo-3.11.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/secp256k1-0.22.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/secp256k1-0.22.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/pnet_packet-0.26.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# unset SURF_DISABLE_LLM_DOCUMENTATION
# export SURF_DISABLE_TARGET_FLAG=1
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/pnet_packet-0.26.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_DISABLE_TARGET_FLAG
# export SURF_DISABLE_LLM_DOCUMENTATION=1

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/rusqlite-0.26.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# export SURF_ENABLE_FEATURES=bundled
# unset SURF_DISABLE_LLM_DOCUMENTATION
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/rusqlite-0.26.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_ENABLE_FEATURES
# export SURF_DISABLE_LLM_DOCUMENTATION=1

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/tokio-1.24.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# unset SURF_DISABLE_LLM_DOCUMENTATION
# export SURF_ENABLE_FEATURES=io-util,rt,macros
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/tokio-1.24.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_ENABLE_FEATURES
# export SURF_DISABLE_LLM_DOCUMENTATION=1

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/arc-swap-1.0.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/arc-swap-1.0.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/lru-0.6.6/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/lru-0.6.6/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/nano_arena-0.5.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/nano_arena-0.5.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/smallvec-0.6.6/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/smallvec-0.6.6/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/smallvec-1.6.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/smallvec-1.6.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/stack_dst-0.6.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/stack_dst-0.6.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/erasan_crates/stackvector-1.0.8/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/erasan_crates/stackvector-1.0.8/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"



# # RustSan Crates (#16)
# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/arenavec-0.1.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/arenavec-0.1.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/array-queue-0.3.3/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/array-queue-0.3.3/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/base64-0.5.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/base64-0.5.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/bumpalo-3.2.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build --all-features
# unset SURF_ANALYZE_LIB
# export SURF_ENABLE_FEATURES=collections
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/bumpalo-3.2.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_ENABLE_FEATURES

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/chttp-0.1.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/chttp-0.1.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/flatbuffers-0.8.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/flatbuffers-0.8.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/generic-array-0.13.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/generic-array-0.13.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/id-map-0.2.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# export SURF_EXTRA_DEPS='id-set = "=0.2.1";'
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/id-map-0.2.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_EXTRA_DEPS

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/safe-transmute-0.10.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/safe-transmute-0.10.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/scratchpad-1.3.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/scratchpad-1.3.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/sized-chunks-0.6.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# export SURF_EXTRA_DEPS='typenum = "*";'
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/sized-chunks-0.6.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_EXTRA_DEPS

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/slice-deque-0.1.15/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/slice-deque-0.1.15/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/smallvec-0.6.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/smallvec-0.6.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/smallvec-0.6.3/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/smallvec-0.6.3/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/stack-0.3.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/stack-0.3.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rustsan_crates/sys-info-0.7.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rustsan_crates/sys-info-0.7.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"



# # RUG Crates (#12)
# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/bincode-2.0.0-rc.3/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/bincode-2.0.0-rc.3/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/chrono-0.5.0-alpha.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/chrono-0.5.0-alpha.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/hashes/blake2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/hashes/blake2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/hashes/sha2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/hashes/sha2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/itoa-1.0.6/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/itoa-1.0.6/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/serde_json-1.0.96/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# export SURF_EXTRA_DEPS='serde = { version = "*", features = ["derive"] };'
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/serde_json-1.0.96/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# unset SURF_EXTRA_DEPS

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/nom-7.1.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/nom-7.1.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/num-traits-0.2.15/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/num-traits-0.2.15/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/crc32fast-1.3.2/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/crc32fast-1.3.2/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/ryu-1.0.13/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/ryu-1.0.13/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/semver-1.0.17/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/semver-1.0.17/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/time/time/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/time/time/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/rug_crates/uuid-1.4.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/rug_crates/uuid-1.4.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"


# # CrabTree Crates (#8)
# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/integer-encoding-3.0.4/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/integer-encoding-3.0.4/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/sparsey-0.7.0/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# unset SURF_DISABLE_LLM_DOCUMENTATION
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/sparsey-0.7.0/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"
# export SURF_DISABLE_LLM_DOCUMENTATION=1

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/oxidebpf-0.2.3/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/oxidebpf-0.2.3/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/leapfrog-0.2.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/leapfrog-0.2.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/fixedbitset-0.4.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/fixedbitset-0.4.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/roaring-0.10.1/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/roaring-0.10.1/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/triomphe-0.1.6/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/triomphe-0.1.6/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"

# export SURF_ANALYZE_LIB=1
# export SURF_WORKING_PATH=$HOME/deepSURF/dataset/crabtree_crates/sharded-slab-0.1.4/
# cd $SURF_WORKING_PATH && cargo clean && RUSTFLAGS="-Zub-checks=no -Awarnings" cargo +rustc_surf build
# unset SURF_ANALYZE_LIB
# cd $SURF_HARNESS_GENERATOR_PATH && RUSTFLAGS="-Awarnings" cargo run --release "$(ls "$HOME/deepSURF/dataset/crabtree_crates/sharded-slab-0.1.4/deepSURF/report"/*.urapi.json | head -n1 | sed -E 's/\.urapi\.json$//')"