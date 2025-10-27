#[macro_use]
extern crate afl;
extern crate secp256k1;

fn test_function43() {
    secp256k1::Secp256k1::preallocate_verification_size();
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function43();
    });
}
