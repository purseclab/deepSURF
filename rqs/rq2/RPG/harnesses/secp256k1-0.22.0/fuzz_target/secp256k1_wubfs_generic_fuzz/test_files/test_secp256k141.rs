#[macro_use]
extern crate afl;
extern crate secp256k1;

fn test_function41() {
    secp256k1::Secp256k1::preallocate_size();
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function41();
    });
}
