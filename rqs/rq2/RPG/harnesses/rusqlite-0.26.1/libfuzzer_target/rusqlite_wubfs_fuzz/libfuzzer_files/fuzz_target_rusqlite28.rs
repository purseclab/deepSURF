#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function28() {
    let _local0 = rusqlite::OpenFlags::all();
    rusqlite::Connection::open_in_memory_with_flags(_local0);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function28();
});
