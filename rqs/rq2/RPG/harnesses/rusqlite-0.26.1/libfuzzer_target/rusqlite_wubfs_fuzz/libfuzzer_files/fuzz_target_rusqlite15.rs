#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function15() {
    let _local0 = rusqlite::OpenFlags::empty();
    let _local1 = rusqlite::OpenFlags::bits(&(_local0));
    rusqlite::OpenFlags::from_bits(_local1);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function15();
});
