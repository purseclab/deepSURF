#!/bin/bash

# python3 fuzz_engine.py algorithmica-0.1.8/
# echo "algorithmica-0.1.8 -- DONE"
# sleep 60
python3 fuzz_engine.py arc-swap-1.0.0/
echo "arc-swap-1.0.0 -- DONE"
sleep 60
python3 fuzz_engine.py bumpalo-3.11.0/
echo "bumpalo-3.11.0 -- DONE"
sleep 60
python3 fuzz_engine.py cbox-0.3.0/
echo "cbox-0.3.0 -- DONE"
python3 fuzz_engine.py endian_trait-0.6.0/
echo "endian_trait-0.6.0 -- DONE"
sleep 60
python3 fuzz_engine.py http-0.1.19/
echo "http-0.1.19 -- DONE"
sleep 60
python3 fuzz_engine.py nano_arena-0.5.2/
echo "nano_arena-0.5.2 -- DONE"
sleep 60
python3 fuzz_engine.py ordnung-0.0.1/
echo "ordnung-0.0.1 -- DONE"
sleep 60
python3 fuzz_engine.py qwutils-0.3.0/
echo "qwutils-0.3.0 -- DONE"
sleep 60
python3 fuzz_engine.py rdiff-0.1.2/
echo "rdiff-0.1.2 -- DONE"
sleep 60
python3 fuzz_engine.py simple-slab-0.3.2/
echo "simple-slab-0.3.2 -- DONE"
sleep 60
python3 fuzz_engine.py slice-deque-0.3.0/
echo "slice-deque-0.3.0 -- DONE"
sleep 60
python3 fuzz_engine.py stack_dst-0.6.0/
echo "stack_dst-0.6.0 -- DONE"
sleep 60
python3 fuzz_engine.py string-interner-0.7.0/
echo "string-interner-0.7.0 -- DONE"
sleep 60
python3 fuzz_engine.py through/
echo "through -- DONE"