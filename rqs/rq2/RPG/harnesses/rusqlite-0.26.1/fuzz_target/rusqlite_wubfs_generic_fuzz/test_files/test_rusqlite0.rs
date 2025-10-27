#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function0() {
    unsafe {
        rusqlite::bypass_sqlite_version_check();
        let _local1 = rusqlite::OpenFlags::all();
        let _local2 = rusqlite::OpenFlags::bits(&(_local1));
        let _local3 = rusqlite::OpenFlags::from_bits_unchecked(_local2);
        rusqlite::OpenFlags::symmetric_difference(_local1 ,_local3);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function0();
    });
}
