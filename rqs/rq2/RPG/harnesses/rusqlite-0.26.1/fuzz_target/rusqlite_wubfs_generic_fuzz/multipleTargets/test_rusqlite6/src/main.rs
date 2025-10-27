#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function6() {
    unsafe {
        let mut _local0 = rusqlite::OpenFlags::empty();
        let _local1 = rusqlite::OpenFlags::bits(&(_local0));
        let _local2 = rusqlite::OpenFlags::from_bits_unchecked(_local1);
        rusqlite::OpenFlags::remove(&mut (_local0) ,_local2);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function6();
    });
}
