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
        if data.len() < 258 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let constructor_type = _to_u8(GLOBAL_DATA, 0) % 2;
        let ctx = match constructor_type {
            0 => secp256k1::Secp256k1::new(),
            1 => secp256k1::Secp256k1::gen_new(),
            _ => unreachable!()
        };

        let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[1..33]));
        let msg = _unwrap_result(secp256k1::Message::from_slice(&GLOBAL_DATA[33..65]));
        let keypair = _unwrap_result(secp256k1::KeyPair::from_seckey_slice(&ctx, &GLOBAL_DATA[1..33]));
        let pubkey = secp256k1::PublicKey::from_keypair(&keypair);
        let mut xpubkey = secp256k1::XOnlyPublicKey::from_keypair(&keypair);

        let sig_ecdsa = ctx.sign_ecdsa_low_r(&msg, &sk);
        let sig_schnorr = ctx.schnorrsig_sign_no_aux_rand(&msg, &keypair);
        let cloned_ctx = ctx.clone();
        
        let shared_secret = secp256k1::ecdh::SharedSecret::new(&pubkey, &sk);
        let combined_pubkey = _unwrap_result(secp256k1::PublicKey::combine_keys(&[&pubkey, &pubkey]));
        
        let _ = cloned_ctx.verify_ecdsa(&msg, &sig_ecdsa, &pubkey);
        let _ = cloned_ctx.schnorrsig_verify(&sig_schnorr, &msg, &xpubkey);
        let _ = _unwrap_result(secp256k1::ecdsa::Signature::from_der(&sig_ecdsa.serialize_der()));
        
        println!("{:?}", cloned_ctx);
        let parity = xpubkey.tweak_add_assign(&cloned_ctx, &GLOBAL_DATA[65..97]).unwrap();
        println!("{:?}", parity);

        let mut sig = sig_ecdsa.serialize_der();
        let _serialized = sig.to_signature();
        let aux_rand = _unwrap_result(<[u8;32]>::try_from(&GLOBAL_DATA[97..129]));
        let _sig_schnorr_aux = ctx.schnorrsig_sign_with_aux_rand(&msg, &keypair, &aux_rand);
        let _ = _unwrap_result(secp256k1::schnorr::Signature::from_slice(&GLOBAL_DATA[34..98]));
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