Command line used to find this crash:

/home/cdc/.local/share/afl.rs/rustc-1.81.0-nightly-8337ba9/afl.rs-0.15.7/afl/bin/afl-fuzz -c0 -i input -o out -g 1200 ./bins/toodee_fuzz_13905759435898088798_asan

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 0 B.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please post
to https://github.com/AFLplusplus/AFLplusplus/issues/286 once the issues
 are fixed :)

