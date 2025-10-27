#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function26() {
    let _local0 = rusqlite::OpenFlags::empty();
    rusqlite::OpenFlags::complement(_local0);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function26();
});
