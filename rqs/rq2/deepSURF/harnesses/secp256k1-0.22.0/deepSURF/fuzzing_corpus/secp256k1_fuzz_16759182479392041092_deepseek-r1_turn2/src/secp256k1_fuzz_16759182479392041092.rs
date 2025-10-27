#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut idx = 1;

        for _ in 0..num_ops {
            let op = _to_usize(GLOBAL_DATA, idx) % 7;
            idx = idx.wrapping_add(1);

            match op {
                0 => {
                    let sk = SecretKey::from_slice(&GLOBAL_DATA[idx..idx + 32]);
                    let sk = _unwrap_result(sk);
                    let secp = Secp256k1::new();
                    let kp = KeyPair::from_secret_key(&secp, sk);
                    let _ = kp.public_key();
                    println!("{:?}", _to_usize(GLOBAL_DATA, idx));
                }
                1 => {
                    let msg = Message::from_slice(&GLOBAL_DATA[idx..idx + 32]);
                    let msg = _unwrap_result(msg);
                    let secp = Secp256k1::new();
                    let sk = SecretKey::from_slice(&GLOBAL_DATA[idx + 32..idx + 64]);
                    let sk = _unwrap_result(sk);
                    let sig = secp.sign_ecdsa(&msg, &sk);
                    println!("{:?}", sig);
                }
                2 => {
                    let secp = Secp256k1::verification_only();
                    let msg = Message::from_slice(&GLOBAL_DATA[idx..idx + 32]);
                    let msg = _unwrap_result(msg);
                    let sig = ecdsa::Signature::from_der(&GLOBAL_DATA[idx..idx + 72]);
                    let sig = _unwrap_result(sig);
                    let pk = PublicKey::from_slice(&GLOBAL_DATA[idx + 72..idx + 105]);
                    let pk = _unwrap_result(pk);
                    _unwrap_result(secp.verify_ecdsa(&msg, &sig, &pk));
                }
                3 => {
                    let secp = Secp256k1::new();
                    let sk = SecretKey::from_slice(&GLOBAL_DATA[idx..idx + 32]);
                    let sk = _unwrap_result(sk);
                    let msg = Message::from_slice(&GLOBAL_DATA[idx + 32..idx + 64]);
                    let msg = _unwrap_result(msg);
                    let sig = secp.sign_schnorr_with_aux_rand(&msg, &KeyPair::from_secret_key(&secp, sk), &[0; 32]);
                    let pk = XOnlyPublicKey::from_slice(&GLOBAL_DATA[idx + 64..idx + 96]).unwrap();
                    _unwrap_result(secp.verify_schnorr(&sig, &msg, &pk));
                }
                4 => {
                    let mut pk1 = PublicKey::from_slice(&GLOBAL_DATA[idx..idx + 33]);
                    let pk1 = _unwrap_result(pk1.as_mut());
                    let secp = Secp256k1::new();
                    _unwrap_result(pk1.combine(&PublicKey::from_slice(&GLOBAL_DATA[idx + 33..idx + 66]).unwrap()));
                    println!("{:?}", pk1);
                }
                5 => {
                    let mut xpk = XOnlyPublicKey::from_slice(&GLOBAL_DATA[idx..idx + 32]).unwrap();
                    let secp = Secp256k1::new();
                    _unwrap_result(xpk.tweak_add_assign(&secp, &GLOBAL_DATA[idx + 32..idx + 64]));
                    println!("{:?}", xpk);
                }
                6 => {
                    let sk = SecretKey::from_str(_to_str(GLOBAL_DATA, idx, idx + 64));
                    let sk = _unwrap_result(sk);
                    let secp = Secp256k1::new();
                    let pk = PublicKey::from_secret_key(&secp, &sk);
                    let ss = ecdh::SharedSecret::new(&pk, &sk);
                    println!("{:?}", ss);
                }
                _ => unreachable!(),
            }
            idx = idx.wrapping_add(128);
        }

        let secp = Secp256k1::new();
        let pk = PublicKey::from_slice(&GLOBAL_DATA[0..33]).unwrap();
        let sk = SecretKey::from_slice(&GLOBAL_DATA[33..65]).unwrap();
        ecdh::shared_secret_point(&pk, &sk);
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