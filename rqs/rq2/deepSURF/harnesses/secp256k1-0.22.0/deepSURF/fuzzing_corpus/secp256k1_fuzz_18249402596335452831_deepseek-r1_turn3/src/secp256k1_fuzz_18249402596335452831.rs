#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 64 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_1 = std::vec::Vec::with_capacity(32);
        for _ in 0..32 {
            t_1.push(secp256k1_sys::types::AlignedType::zeroed());
        }
        let trunc_len = _to_u8(GLOBAL_DATA, 0) % 33;
        t_1.truncate(trunc_len as usize);
        let buffer = &mut t_1[..];

        let ctor_type = _to_u8(GLOBAL_DATA, 1) % 2;
        let ctx = match ctor_type {
            0 => secp256k1::Secp256k1::preallocated_new(buffer),
            1 => secp256k1::Secp256k1::preallocated_gen_new(buffer),
            _ => unreachable!(),
        };
        let mut ctx = _unwrap_result(ctx);

        let num_ops = _to_u8(GLOBAL_DATA, 2) % 65;
        let mut idx = 3;
        for _ in 0..num_ops {
            let op = _to_u8(GLOBAL_DATA, idx) % 4;
            idx += 1;

            match op {
                0 => {
                    let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let pk = _unwrap_result(secp256k1::PublicKey::from_slice(&GLOBAL_DATA[idx..idx+33]));
                    idx += 33;
                    let _ = ctx.verify_ecdsa(&_unwrap_result(Message::from_slice(&GLOBAL_DATA[idx..idx+32])), &_unwrap_result(ecdsa::Signature::from_der(&GLOBAL_DATA[idx..idx+72])), &pk);
                    idx += 72;
                }
                1 => {
                    let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let msg = _unwrap_result(secp256k1::Message::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let sig = ctx.sign_ecdsa(&msg, &sk);
                    let sersig = sig.serialize_der();
                    let _ = ctx.verify_ecdsa(&msg, &_unwrap_result(ecdsa::Signature::from_der(&sersig)), &_unwrap_result(secp256k1::PublicKey::from_slice(&GLOBAL_DATA[idx..idx+33])));
                    idx += 33;
                }
                2 => {
                    let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let msg = _unwrap_result(secp256k1::Message::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let sig = ctx.sign_ecdsa_low_r(&msg, &sk);
                    let pk = secp256k1::PublicKey::from_secret_key(&ctx, &sk);
                    let mut serialized = sig.serialize_compact();
                    serialized[32] = 0x01;
                    _unwrap_result(ctx.verify_ecdsa(&msg, &_unwrap_result(ecdsa::Signature::from_compact(&serialized)), &pk));
                }
                3 => {
                    let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let keypair = secp256k1::KeyPair::from_secret_key(&ctx, sk);
                    let xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
                    let msg = _unwrap_result(secp256k1::Message::from_slice(&GLOBAL_DATA[idx..idx+32]));
                    idx += 32;
                    let sig = ctx.sign_schnorr_no_aux_rand(&msg, &keypair);
                    _unwrap_result(ctx.verify_schnorr(&sig, &msg, &xonly));
                }
                _ => unreachable!()
            }
        }

        let _ = ctx;
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