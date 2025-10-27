#!/bin/bash

export AFL_EXIT_WHEN_DONE=1
export AFL_NO_AFFINITY=1
INIT_DELAY=0.2
#for i in {1..4}; do cd "/home/cdc/afl_fast_work/secp256k1-afl-work/test_secp256k1${i}" && cargo afl build --offline; done
#for i in {16..16}; do cd "/home/cdc/afl_fast_work/secp256k1-afl-work/test_secp256k1${i}" && cargo afl build --offline; done
#for i in {18..47}; do cd "/home/cdc/afl_fast_work/secp256k1-afl-work/test_secp256k1${i}" && cargo afl build --offline; done


for i in {1..4}; do 
    tmux new-session -d -s "test_secp256k1$i" \
        env AFL_EXIT_WHEN_DONE=1 \
        env AFL_NO_AFFINITY=1 \
        cargo afl fuzz \
        -i "/home/cdc/afl_fast_work/secp256k1-afl-work/afl_init/test_secp256k1$i" \
        -o "out/test_secp256k1$i" \
        "/home/cdc/afl_fast_work/secp256k1-afl-work/target/debug/test_secp256k1$i"
    sleep $INIT_DELAY
done

for i in {16..16}; do 
    tmux new-session -d -s "test_secp256k1$i" \
        env AFL_EXIT_WHEN_DONE=1 \
        env AFL_NO_AFFINITY=1 \
        cargo afl fuzz \
        -i "/home/cdc/afl_fast_work/secp256k1-afl-work/afl_init/test_secp256k1$i" \
        -o "out/test_secp256k1$i" \
        "/home/cdc/afl_fast_work/secp256k1-afl-work/target/debug/test_secp256k1$i"
    sleep $INIT_DELAY
done

for i in {18..47}; do 
    tmux new-session -d -s "test_secp256k1$i" \
        env AFL_EXIT_WHEN_DONE=1 \
        env AFL_NO_AFFINITY=1 \
        cargo afl fuzz \
        -i "/home/cdc/afl_fast_work/secp256k1-afl-work/afl_init/test_secp256k1$i" \
        -o "out/test_secp256k1$i" \
        "/home/cdc/afl_fast_work/secp256k1-afl-work/target/debug/test_secp256k1$i"
    sleep $INIT_DELAY
done