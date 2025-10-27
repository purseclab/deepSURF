#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rdiff;

fn test_function2() {
    let _local0 = rdiff::Diff::new();
    rdiff::Diff::is_empty(&(_local0));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function2();
});
