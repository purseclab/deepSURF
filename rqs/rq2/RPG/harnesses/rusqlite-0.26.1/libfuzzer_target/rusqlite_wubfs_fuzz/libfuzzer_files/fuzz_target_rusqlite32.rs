#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function32() {
    let _local0 = rusqlite::OpenFlags::empty();
    let _local1 = rusqlite::OpenFlags::empty();
    rusqlite::OpenFlags::difference(_local0 ,_local1);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function32();
});
