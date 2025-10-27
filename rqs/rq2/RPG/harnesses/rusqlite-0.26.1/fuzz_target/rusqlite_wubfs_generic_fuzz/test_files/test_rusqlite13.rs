#[macro_use]
extern crate afl;
extern crate rusqlite;

fn test_function13() {
    let _local0 = rusqlite::OpenFlags::empty();
    rusqlite::Connection::open_in_memory_with_flags(_local0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function13();
    });
}
