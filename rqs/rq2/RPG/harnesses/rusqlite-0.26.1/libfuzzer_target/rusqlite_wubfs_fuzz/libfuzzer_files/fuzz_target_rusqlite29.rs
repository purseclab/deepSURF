#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function29() {
    let _local0 = rusqlite::OpenFlags::all();
    rusqlite::OpenFlags::is_all(&(_local0));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function29();
});
