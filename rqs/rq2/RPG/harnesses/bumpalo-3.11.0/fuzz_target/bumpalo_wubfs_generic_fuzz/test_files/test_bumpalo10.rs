#[macro_use]
extern crate afl;
extern crate bumpalo;

fn test_function10() {
    unsafe {
        let _local0 = bumpalo::Bump::new();
        bumpalo::Bump::iter_allocated_chunks_raw(&(_local0));
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function10();
    });
}
