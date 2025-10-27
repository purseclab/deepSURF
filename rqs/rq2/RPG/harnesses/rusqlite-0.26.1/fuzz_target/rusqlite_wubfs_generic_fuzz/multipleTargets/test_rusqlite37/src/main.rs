#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function37() {
    let _local0 = rusqlite::OpenFlags::empty();
    let _local1 = rusqlite::OpenFlags::bits(&(_local0));
    let _local2 = rusqlite::OpenFlags::from_bits_truncate(_local1);
    rusqlite::OpenFlags::union(_local0 ,_local2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function37();
    });
}
