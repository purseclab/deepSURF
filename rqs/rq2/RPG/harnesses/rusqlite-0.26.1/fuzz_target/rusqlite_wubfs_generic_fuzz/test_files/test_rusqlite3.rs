#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function3() {
    rusqlite::Connection::open_in_memory();
    let _local1 = rusqlite::OpenFlags::all();
    rusqlite::OpenFlags::complement(_local1);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function3();
    });
}
