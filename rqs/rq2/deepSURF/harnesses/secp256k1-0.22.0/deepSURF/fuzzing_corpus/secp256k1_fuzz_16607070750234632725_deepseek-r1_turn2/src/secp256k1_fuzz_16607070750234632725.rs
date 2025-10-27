#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let secp = Secp256k1::new();

        let mut offset = 0;
        let ctx_choice = _to_u8(GLOBAL_DATA, offset);
        offset += 1;

        let mut secret_key = if ctx_choice % 2 == 0 {
            let sk_slice = &GLOBAL_DATA[offset..offset + 32];
            _unwrap_result(SecretKey::from_slice(sk_slice))
        } else {
            let sk_str = _to_str(GLOBAL_DATA, offset, offset + 64);
            _unwrap_result(SecretKey::from_str(sk_str))
        };

        offset += 64;

        for _ in 0..(_to_u8(GLOBAL_DATA, offset) % 5) {
            let method = _to_u8(GLOBAL_DATA, offset + 1) % 2;
            offset += 1;
            if method == 0 {
                let tweak = &GLOBAL_DATA[offset..offset + 32];
                let _ = secret_key.add_assign(tweak);
            } else {
                let tweak = &GLOBAL_DATA[offset..offset + 32];
                let _ = secret_key.mul_assign(tweak);
            }
            offset += 32;
        }

        let keypair = KeyPair::from_secret_key(&secp, secret_key);
        let mut pubkey = PublicKey::from_keypair(&keypair);

        if ctx_choice % 3 == 0 {
            let combined = _unwrap_result(PublicKey::combine_keys(&[
                &pubkey,
                &PublicKey::from_secret_key(&secp, &SecretKey::from_slice(&GLOBAL_DATA[offset..offset + 32]).unwrap())
            ]));
            pubkey = combined;
            offset += 32;
        }

        pubkey.serialize_uncompressed();

        let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[offset..offset + 32]));
        offset += 32;

        let sig = secp.sign_ecdsa(&msg, &secret_key);
        let serialized = sig.serialize_der();
        let _ = serialized.to_signature();

        let mut xonly = XOnlyPublicKey::from_keypair(&keypair);
        let tweak = &GLOBAL_DATA[offset..offset + 32];
        let _ = xonly.tweak_add_assign(&secp, tweak);

        let random_data = &GLOBAL_DATA[offset..];
        for i in 0..(random_data.len() / 32) {
            let start = i * 32;
            let end = start + 32;
            if let Ok(sk) = SecretKey::from_slice(&random_data[start..end]) {
                let kp = KeyPair::from_secret_key(&secp, sk);
                let _pub = PublicKey::from_keypair(&kp);
                let _x = XOnlyPublicKey::from_keypair(&kp);
            }
        }
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