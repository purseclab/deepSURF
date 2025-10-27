#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;

fn test_function9() {
    unsafe {
        let _local0 = rusqlite::OpenFlags::empty();
        let _local1 = rusqlite::OpenFlags::bits(&(_local0));
        let _local2 = rusqlite::OpenFlags::from_bits_unchecked(_local1);
        rusqlite::OpenFlags::intersects(&(_local0) ,_local2);
    }
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function9();
});
