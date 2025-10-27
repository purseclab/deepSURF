#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::{Secp256k1, SecretKey, Message, XOnlyPublicKey, PublicKey, KeyPair, ecdsa, schnorr};
use secp256k1::ffi::types::AlignedType;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_u8(GLOBAL_DATA, 0) % 8 + 2;
        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, i as usize + 1) % 7;
            
            match op_selector {
                0 => {
                    let ctx_type = _to_u8(GLOBAL_DATA, i as usize + 2) % 3;
                    let buf_size = 1024;
                    let mut buf = vec![AlignedType::zeroed(); buf_size];
                    match ctx_type {
                        0 => {
                            _unwrap_result(Secp256k1::preallocated_new(&mut buf));
                        },
                        1 => {
                            _unwrap_result(Secp256k1::preallocated_signing_only(&mut buf));
                        },
                        _ => {
                            _unwrap_result(Secp256k1::preallocated_verification_only(&mut buf));
                        }
                    };
                },
                1 => {
                    let sk_bytes = &GLOBAL_DATA[i as usize * 32..(i as usize + 1)*32];
                    let sk = _unwrap_result(SecretKey::from_slice(sk_bytes));
                    let keypair = KeyPair::from_secret_key(&Secp256k1::new(), sk);
                    let public_key = PublicKey::from_secret_key(&Secp256k1::new(), &sk);
                    let mut combined = _unwrap_result(PublicKey::combine_keys(&[&public_key]));
                    println!("{:?}", keypair.secret_bytes());
                },
                2 => {
                    let msg_slice = &GLOBAL_DATA[i as usize * 32..(i as usize + 1)*32];
                    let msg = _unwrap_result(Message::from_slice(msg_slice));
                    let secp = Secp256k1::new();
                    let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[64..96]));
                    let public_key = PublicKey::from_secret_key(&secp, &sk);
                    let sig = secp.sign_ecdsa(&msg, &sk);
                    let serialize = sig.serialize_der();
                    let _sig_der = _unwrap_result(ecdsa::Signature::from_der(&serialize));
                    let _ = secp.verify_ecdsa(&msg, &sig, &public_key);
                },
                3 => {
                    let xonly_bytes = &GLOBAL_DATA[i as usize * 32..(i as usize + 1)*32];
                    let mut xonly = _unwrap_result(XOnlyPublicKey::from_slice(xonly_bytes));
                    let mut tweak = [0u8; 32];
                    tweak.copy_from_slice(&GLOBAL_DATA[128..160]);
                    let _parity = xonly.tweak_add_assign(&Secp256k1::new(), &tweak);
                },
                4 => {
                    let sig_bytes = &GLOBAL_DATA[i as usize * 64..(i as usize + 1)*64];
                    let sig = _unwrap_result(schnorr::Signature::from_slice(sig_bytes));
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[192..224]));
                    let xonly = _unwrap_result(XOnlyPublicKey::from_slice(&GLOBAL_DATA[224..256]));
                    let secp = Secp256k1::new();
                    let _ = secp.verify_schnorr(&sig, &msg, &xonly);
                },
                5 => {
                    let pubkey1 = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[256..289]));
                    let pubkey2 = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[289..322]));
                    let pubkeys = vec![&pubkey1, &pubkey2];
                    let mut combined = _unwrap_result(PublicKey::combine_keys(&pubkeys));
                    let tweaked = combined.add_exp_assign(&Secp256k1::new(), &[0u8; 32]);
                    println!("{:?}", combined.serialize());
                },
                _ => {
                    let secp = Secp256k1::new();
                    let sk = _unwrap_result(SecretKey::from_str("0000000000000000000000000000000000000000000000000000000000000001"));
                    let keypair = KeyPair::from_secret_key(&secp, sk);
                    let aux_rand = [0u8; 32];
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[322..354]));
                    let sig = secp.sign_schnorr_with_aux_rand(&msg, &keypair, &aux_rand);
                    let xonly = keypair.public_key();
                    let _ver = secp.verify_schnorr(&sig, &msg, &xonly);
                    let _secret = keypair.secret_bytes();
                }
            }
        }

        let ctx_buf = &mut vec![AlignedType::zeroed(); 1024][..];
        let secp_verify = _unwrap_result(Secp256k1::preallocated_verification_only(ctx_buf));
        let sig = _unwrap_result(schnorr::Signature::from_slice(&GLOBAL_DATA[354..418]));
        let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[418..450]));
        let xonly = _unwrap_result(XOnlyPublicKey::from_slice(&GLOBAL_DATA[450..482]));
        _unwrap_result(secp_verify.verify_schnorr(&sig, &msg, &xonly));
    });
}

// The type converters (from _to_u8 to _unwrap_result) follow here but are omitted as per directions.

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