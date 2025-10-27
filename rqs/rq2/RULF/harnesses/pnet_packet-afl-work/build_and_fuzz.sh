#!/bin/bash

export AFL_EXIT_WHEN_DONE=1
export AFL_NO_AFFINITY=1
INIT_DELAY=0.2
#for i in {0..2}; do cd "/home/cdc/afl_fast_work/pnet_packet-afl-work/test_pnet_packet${i}" && cargo afl build --offline; done
#for i in {4..21}; do cd "/home/cdc/afl_fast_work/pnet_packet-afl-work/test_pnet_packet${i}" && cargo afl build --offline; done


for i in {1..2}; do 
    tmux new-session -d -s "test_pnet_packet$i" \
        env AFL_EXIT_WHEN_DONE=1 \
        env AFL_NO_AFFINITY=1 \
        cargo afl fuzz \
        -i "/home/cdc/afl_fast_work/pnet_packet-afl-work/afl_init/test_pnet_packet$i" \
        -o "out/test_pnet_packet$i" \
        "/home/cdc/afl_fast_work/pnet_packet-afl-work/target/debug/test_pnet_packet$i"
    sleep $INIT_DELAY
done

for i in {4..21}; do 
    tmux new-session -d -s "test_pnet_packet$i" \
        env AFL_EXIT_WHEN_DONE=1 \
        env AFL_NO_AFFINITY=1 \
        cargo afl fuzz \
        -i "/home/cdc/afl_fast_work/pnet_packet-afl-work/afl_init/test_pnet_packet$i" \
        -o "out/test_pnet_packet$i" \
        "/home/cdc/afl_fast_work/pnet_packet-afl-work/target/debug/test_pnet_packet$i"
    sleep $INIT_DELAY
done
