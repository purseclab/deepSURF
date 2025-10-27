#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function20() {
    let _local0 = rusqlite::OpenFlags::all();
    let _local1 = rusqlite::OpenFlags::all();
    rusqlite::OpenFlags::intersection(_local0 ,_local1);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function20();
});
