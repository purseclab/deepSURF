#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        for i in 0..=op_count {
            let op_selector = _to_u8(GLOBAL_DATA, i as usize + 1) % 5;
            match op_selector {
                0 => {
                    let mut sign_vec = create_preallocated_context(GLOBAL_DATA, 2);
                    let ctx_sign = Secp256k1::preallocated_signing_only(&mut sign_vec).unwrap();
                    let sk1 = _to_str(GLOBAL_DATA, 32, 64);
                    let sk2 = _to_str(GLOBAL_DATA, 64, 96);
                    
                    let keypair1 = KeyPair::from_seckey_str(&ctx_sign, sk1).unwrap();
                    let keypair2 = KeyPair::from_seckey_str(&ctx_sign, sk2).unwrap();
                    
                    let pubkey1 = PublicKey::from_keypair(&keypair1);
                    let pubkey2 = PublicKey::from_keypair(&keypair2);
                    pubkey1.partial_cmp(&pubkey2);
                    
                    let mut sig = ctx_sign.sign_ecdsa_low_r(&Message::from_slice(&[0xab;32]).unwrap(), &SecretKey::from_slice(&[0xcd;32]).unwrap());
                    let mut verify_vec = create_preallocated_context(GLOBAL_DATA, 3);
                    let ctx_verify = Secp256k1::preallocated_verification_only(&mut verify_vec).unwrap();
                    ctx_verify.verify_ecdsa(&Message::from_slice(&[0xab;32]).unwrap(), &sig, &pubkey1).ok();
                },
                1 => {
                    let sk_bytes = &GLOBAL_DATA[96..128];
                    let sk = SecretKey::from_slice(sk_bytes).unwrap();
                    let ctx = Secp256k1::signing_only();
                    let pubkey = PublicKey::from_secret_key(&ctx, &sk);
                    let mut xonly = XOnlyPublicKey::from(pubkey);
                    let ctx_verify = Secp256k1::verification_only();
                    let parity = xonly.tweak_add_assign(&ctx_verify, &[0x42;32]).unwrap();
                    println!("{:?}", parity);
                },
                2 => {
                    let mut t_vec = vec![secp256k1_sys::types::AlignedType::zeroed(); 256];
                    let ctx_verify = Secp256k1::preallocated_verification_only(&mut t_vec).unwrap();
                    let sig_der = _to_str(GLOBAL_DATA, 128, 200);
                    let mut sig = ecdsa::Signature::from_der_lax(sig_der.as_bytes()).unwrap();
                    sig.normalize_s();
                },
                3 => {
                    let ctx = Secp256k1::new();
                    let pair = KeyPair::from_seckey_slice(&ctx, &GLOBAL_DATA[200..232]).unwrap();
                    let shared = ecdh::SharedSecret::new(&PublicKey::from_keypair(&pair), &SecretKey::from_keypair(&pair));
                    println!("{}", shared.display_secret());
                },
                4 => {
                    let mut vec_context = create_preallocated_context(GLOBAL_DATA, 232);
                    let ctx_all = Secp256k1::preallocated_new(vec_context.as_mut()).unwrap();
                    let pk_comb = PublicKey::combine_keys(&[
                        &PublicKey::from_slice(&GLOBAL_DATA[240..273]).unwrap(),
                        &PublicKey::from_slice(&GLOBAL_DATA[273..306]).unwrap()
                    ]).unwrap();
                    let pk_str = pk_comb.to_string();
                    let pk_from_str = PublicKey::from_str(&pk_str).unwrap();
                    pk_comb.partial_cmp(&pk_from_str);
                },
                _ => ()
            }
        }
    });
}

fn create_preallocated_context(data: &[u8], offset: usize) -> Vec<secp256k1_sys::types::AlignedType> {
    let len = _to_u8(data, offset) % 33;
    let mut vec = Vec::with_capacity(256);
    for _ in 0..256 {
        vec.push(secp256k1_sys::types::AlignedType::zeroed());
    }
    vec.truncate(len as usize);
    vec
}

// Type conversion functions (_to_u8, _to_str, etc.) remain unchanged but are excluded here as per directions

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_u128(data:&[u8], index:usize)->u128 {
    let data0 = _to_u64(data, index) as u128;
    let data1 = _to_u64(data, index+8) as u128;
    data0 << 64 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}

fn _to_i64(data:&[u8], index:usize)->i64 {
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}

fn _to_i128(data:&[u8], index:usize)->i128 {
    let data0 = _to_i64(data, index) as i128;
    let data1 = _to_i64(data, index+8) as i128;
    data0 << 64 | data1
}

fn _to_isize(data:&[u8], index:usize)->isize {
    _to_i64(data, index) as isize
}

fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn _to_char(data:&[u8], index: usize)->char {
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {
        Some(c)=>c,
        None=>{
            std::process::exit(0);
        }
    }
}

fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            std::process::exit(0);
        }
    }
}

fn _unwrap_option<T>(opt: Option<T>) -> T {
    match opt {
        Some(_t) => _t,
        None => {
            std::process::exit(0);
        }
    }
}

fn _unwrap_result<T, E>(_res: std::result::Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            std::process::exit(0);
        },
    }
}