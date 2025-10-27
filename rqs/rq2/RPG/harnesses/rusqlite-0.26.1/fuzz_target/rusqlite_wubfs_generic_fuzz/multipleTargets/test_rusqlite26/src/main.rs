#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function26() {
    let _local0 = rusqlite::OpenFlags::empty();
    rusqlite::OpenFlags::complement(_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function26();
    });
}
