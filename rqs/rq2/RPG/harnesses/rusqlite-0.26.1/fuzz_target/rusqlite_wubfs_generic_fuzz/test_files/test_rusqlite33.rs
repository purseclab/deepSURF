#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function33() {
    let _local0 = rusqlite::OpenFlags::empty();
    let _local1 = rusqlite::OpenFlags::empty();
    rusqlite::OpenFlags::union(_local0 ,_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function33();
    });
}
