#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 260 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ctx = secp256k1::Secp256k1::new();
        let secp = &ctx;

        let mut keypair = match _to_u8(GLOBAL_DATA, 0) % 4 {
            0 => {
                let len = _to_u8(GLOBAL_DATA, 1) % 32 + 1;
                let s = _to_str(GLOBAL_DATA, 2, 2 + len as usize);
                let kp = secp256k1::KeyPair::from_seckey_str(secp, s);
                _unwrap_result(kp)
            }
            1 => {
                let sk_bytes = &GLOBAL_DATA[1..33];
                let sk = secp256k1::SecretKey::from_slice(sk_bytes).unwrap();
                secp256k1::KeyPair::from_secret_key(secp, sk)
            }
            2 => {
                let kp = secp256k1::KeyPair::from_seckey_slice(secp, &GLOBAL_DATA[1..33]);
                _unwrap_result(kp)
            }
            _ => {
                let s = _to_str(GLOBAL_DATA, 1, 33);
                let kp = secp256k1::KeyPair::from_str(s);
                _unwrap_result(kp)
            }
        };

        let pubkey = secp256k1::PublicKey::from_keypair(&keypair);
        let mut xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
        let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[33..65]).unwrap();

        let sig_ecdsa = secp.sign_ecdsa(&msg, &SecretKey::from_keypair(&keypair));
        let sig_schnorr = secp.sign_schnorr_no_aux_rand(&msg, &keypair);
        let verify_res = secp.verify_ecdsa(&msg, &sig_ecdsa, &pubkey);
        let serialized_sig = sig_ecdsa.serialize_der();

        let mut tweak_data = [0u8; 32];
        tweak_data.copy_from_slice(&GLOBAL_DATA[65..97]);
        let _ = keypair.tweak_add_assign(secp, &tweak_data);

        let shared_secret = secp256k1::ecdh::SharedSecret::new(&pubkey, &SecretKey::from_keypair(&keypair));
        println!("{:?} {:?}", xonly, shared_secret);

        let cmp_result = pubkey.cmp(&pubkey);
        let parity = keypair.secret_bytes()[31] % 2;
        let _parity_enum = secp256k1::Parity::from_u8(parity).unwrap();

        keypair.secret_bytes();

        let num_ops = _to_u8(GLOBAL_DATA, 97) % 5;
        for _ in 0..num_ops {
            match _to_u8(GLOBAL_DATA, 98) % 3 {
                0 => {
                    let sk = SecretKey::from_slice(&GLOBAL_DATA[98..130]).ok();
                    if let Some(sk) = sk {
                        let _ = secp.sign_ecdsa(&msg, &sk);
                    }
                }
                1 => {
                    let pk_bytes = &GLOBAL_DATA[98..130];
                    if let Ok(pk) = PublicKey::from_slice(pk_bytes) {
                        let _ = secp.verify_ecdsa(&msg, &sig_ecdsa, &pk);
                    }
                }
                _ => {
                    let tweak = &GLOBAL_DATA[98..130];
                    if let Ok(parity) = xonly.tweak_add_assign(secp, tweak) {
                        println!("Tweak result: {:?}", parity);
                    }
                }
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