#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function1() {
    unsafe {
        rusqlite::bypass_sqlite_initialization();
        let _local1 = rusqlite::OpenFlags::empty();
        let _local2 = rusqlite::OpenFlags::bits(&(_local1));
        rusqlite::OpenFlags::from_bits_truncate(_local2);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function1();
    });
}
