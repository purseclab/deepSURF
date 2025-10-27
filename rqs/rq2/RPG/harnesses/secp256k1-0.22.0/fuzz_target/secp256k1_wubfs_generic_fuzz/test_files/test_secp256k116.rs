#[macro_use]
extern crate afl;
extern crate secp256k1;

fn test_function16() {
    unsafe {
        let _local0: secp256k1::Secp256k1<secp256k1::InvalidParityValue> = secp256k1::Secp256k1::gen_new();
        let mut _local1 = secp256k1::Secp256k1::ctx(&(_local0));
        secp256k1::Secp256k1::from_raw_all(*(*(_local1)) as *mut Currently not supported);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function16();
    });
}
