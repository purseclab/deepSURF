#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate bumpalo;

fn test_function14() {
    let mut _local0 = bumpalo::Bump::new();
    bumpalo::Bump::reset(&mut (_local0));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function14();
});
