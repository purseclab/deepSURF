#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate secp256k1;

fn test_function43() {
    secp256k1::Secp256k1::preallocate_verification_size();
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function43();
});
