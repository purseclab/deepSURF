#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use secp256k1::ecdh::SharedSecret;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ctx = Secp256k1::new();
        let mut offset = 0;

        let ops = _to_u8(GLOBAL_DATA, offset) % 8;
        offset += 1;

        for i in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, offset + i as usize) % 7;
            match op_type {
                0 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset + 1) % 65;
                    let sk_str = _to_str(GLOBAL_DATA, offset + 2, (offset + 2 + key_len as usize) % GLOBAL_DATA.len());
                    let keypair = _unwrap_result(KeyPair::from_seckey_str(&ctx, sk_str));
                    let secret = SecretKey::from_keypair(&keypair);
                    let public = PublicKey::from_secret_key(&ctx, &secret);
                    println!("{:?}", public);
                }
                1 => {
                    let sk_start = (offset + 1) % GLOBAL_DATA.len();
                    let secret = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[sk_start..(sk_start+32)]));
                    let keypair = KeyPair::from_secret_key(&ctx, secret);
                    let xonly = XOnlyPublicKey::from_keypair(&keypair);
                    let parity = keypair.public_key().serialize()[0];
                    println!("{:?}", xonly);
                }
                2 => {
                    let sk_start = (offset + 1) % GLOBAL_DATA.len();
                    let secret = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[sk_start..(sk_start+32)]));
                    let msg_slice = &GLOBAL_DATA[(offset + 33) % GLOBAL_DATA.len()..(offset + 65) % GLOBAL_DATA.len()];
                    let msg = _unwrap_result(Message::from_slice(msg_slice));
                    let sig = ctx.sign_ecdsa(&msg, &secret);
                    let pubkey = PublicKey::from_secret_key(&ctx, &secret);
                    _unwrap_result(ctx.verify_ecdsa(&msg, &sig, &pubkey));
                }
                3 => {
                    let keypair = _unwrap_result(KeyPair::from_seckey_str(&ctx, _to_str(GLOBAL_DATA, offset+1, (offset+33) % GLOBAL_DATA.len())));
                    let xonly = XOnlyPublicKey::from_keypair(&keypair);
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[(offset+34) % GLOBAL_DATA.len()..(offset+66) % GLOBAL_DATA.len()]));
                    let aux_rand = &GLOBAL_DATA[(offset+67) % GLOBAL_DATA.len()..(offset+99) % GLOBAL_DATA.len()];
                    let sig = ctx.schnorrsig_sign_with_aux_rand(&msg, &keypair, <&[u8; 32]>::try_from(aux_rand).unwrap_or(&[0; 32]));
                    _unwrap_result(ctx.schnorrsig_verify(&sig, &msg, &xonly));
                }
                4 => {
                    let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[(offset+1) % GLOBAL_DATA.len()..(offset+33) % GLOBAL_DATA.len()]));
                    let pubkey = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[(offset+34) % GLOBAL_DATA.len()..(offset+67) % GLOBAL_DATA.len()]));
                    let shared = SharedSecret::new(&pubkey, &sk);
                    println!("{:?}", shared.as_ref());
                }
                5 => {
                    let keypair = _unwrap_result(KeyPair::from_seckey_str(&ctx, _to_str(GLOBAL_DATA, offset+1, (offset+33) % GLOBAL_DATA.len())));
                    let public = PublicKey::from(keypair);
                    let xonly = XOnlyPublicKey::from(public);
                    let mut tweaked = xonly;
                    let tweak = &GLOBAL_DATA[(offset+34) % GLOBAL_DATA.len()..(offset+66) % GLOBAL_DATA.len()];
                    _unwrap_result(tweaked.tweak_add_assign(&ctx, tweak));
                }
                _ => {
                    let keypair = _unwrap_result(KeyPair::from_seckey_str(&ctx, _to_str(GLOBAL_DATA, offset+1, (offset+33) % GLOBAL_DATA.len())));
                    let public = PublicKey::from(keypair);
                    let serialized = public.serialize();
                    let parsed = _unwrap_result(PublicKey::from_slice(&serialized));
                    println!("{:?}", parsed);
                }
            }
            offset += 99 % GLOBAL_DATA.len();
        }

        let key_len = _to_u8(GLOBAL_DATA, 0) % 65;
        let sk_str = _to_str(GLOBAL_DATA, 1, (1 + key_len as usize) % GLOBAL_DATA.len());
        let keypair = _unwrap_result(KeyPair::from_seckey_str(&ctx, sk_str));
        let public = PublicKey::from(keypair);
        let xonly = XOnlyPublicKey::from(public);
        println!("{:?}", xonly);
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