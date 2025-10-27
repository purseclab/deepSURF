#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut index = 0;

        let ctx_type_signing = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;
        let ctx_type_verify = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;

        let mut buf_len = _to_u8(GLOBAL_DATA, index) as usize % 33;
        index += 1;
        let mut buf1: Vec<secp256k1_sys::types::AlignedType> = (0..32)
            .map(|_| secp256k1_sys::types::AlignedType::zeroed())
            .collect();
        buf1.truncate(buf_len);
        let signing_ctx = match ctx_type_signing {
            0 => Secp256k1::preallocated_new(&mut buf1),
            1 => Secp256k1::preallocated_new(&mut buf1),
            _ => Secp256k1::preallocated_new(&mut buf1)
        };
        let signing_ctx = _unwrap_result(signing_ctx);

        let mut buf_len2 = _to_u8(GLOBAL_DATA, index) as usize % 33;
        index += 1;
        let mut buf2: Vec<secp256k1_sys::types::AlignedType> = (0..32)
            .map(|_| secp256k1_sys::types::AlignedType::zeroed())
            .collect();
        buf2.truncate(buf_len2);
        let verify_ctx = match ctx_type_verify {
            0 => Secp256k1::preallocated_new(&mut buf2),
            1 => Secp256k1::preallocated_new(&mut buf2),
            _ => Secp256k1::preallocated_new(&mut buf2)
        };
        let verify_ctx = _unwrap_result(verify_ctx);

        let sk_bytes = &GLOBAL_DATA[index..index + 32];
        index += 32;
        let secret_key = secp256k1::SecretKey::from_slice(sk_bytes).unwrap();

        let keypair = match _to_u8(GLOBAL_DATA, index) % 3 {
            0 => secp256k1::KeyPair::from_seckey_slice(&signing_ctx, sk_bytes).map_err(|_| "Failed to create KeyPair from slice"),
            1 => secp256k1::KeyPair::from_seckey_str(&signing_ctx, _to_str(GLOBAL_DATA, index, index + 32)).map_err(|_| "Failed to create KeyPair from str"),
            _ => Ok(secp256k1::KeyPair::from_secret_key(&signing_ctx, secret_key))
        };
        let keypair = _unwrap_result(keypair);
        let public_key = secp256k1::PublicKey::from_secret_key(&signing_ctx, &secret_key);

        let msg_slice = &GLOBAL_DATA[index..index + 32];
        index += 32;
        let message = secp256k1::Message::from_slice(msg_slice).unwrap();

        let ops_count = _to_u8(GLOBAL_DATA, index) % 5 + 1;
        index += 1;

        for _ in 0..ops_count {
            let op = _to_u8(GLOBAL_DATA, index) % 4;
            index += 1;

            match op {
                0 => {
                    let sig = signing_ctx.sign_ecdsa(&message, &secret_key);
                    let serialized = ecdsa::SerializedSignature::from_signature(&sig);
                    let sig2 = serialized.to_signature().unwrap();
                    verify_ctx.verify_ecdsa(&message, &sig2, &public_key).unwrap();
                    println!("{:?}", serialized);
                }
                1 => {
                    let sig = signing_ctx.sign_ecdsa_low_r(&message, &secret_key);
                    let serialized = ecdsa::SerializedSignature::from_signature(&sig);
                    println!("{:?}", sig);
                }
                2 => {
                    let mut sig = signing_ctx.sign_ecdsa(&message, &secret_key);
                    sig.normalize_s();
                    let serialized = ecdsa::SerializedSignature::from_signature(&sig);
                    println!("{:?}", public_key);
                }
                3 => {
                    let sig = signing_ctx.sign_ecdsa_grind_r(&message, &secret_key, 1000);
                    let tweaked = keypair.public_key().tweak_add_assign(&verify_ctx, &[0x42; 32]).unwrap();
                    let serialized = ecdsa::SerializedSignature::from_signature(&sig);
                    println!("{:?}", tweaked);
                }
                _ => (),
            }
        }

        let xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
        let combined = secp256k1::PublicKey::combine_keys(&[&public_key, &public_key]).unwrap();
        println!("{:?}", combined);
    });
}

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