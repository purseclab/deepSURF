#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        if offset + 1 > GLOBAL_DATA.len() {return;}
        let num_ops = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 1;

        let ctx_choice = if offset + 1 <= GLOBAL_DATA.len() { _to_u8(GLOBAL_DATA, offset) % 3 } else { 0 };
        offset += 1;

        let size = secp256k1::Secp256k1::preallocate_size();
        let mut buf = vec![secp256k1_sys::types::AlignedType::zeroed(); size];
        let secp = secp256k1::Secp256k1::preallocated_new(&mut buf).unwrap();

        for _ in 0..num_ops {
            if offset + 1 > GLOBAL_DATA.len() {break;}
            let op = _to_u8(GLOBAL_DATA, offset) % 4;
            offset += 1;

            match op {
                0 => {
                    secp256k1::Secp256k1::preallocate_signing_size();
                }
                1 => {
                    if offset + 64 > GLOBAL_DATA.len() {break;}
                    let sk_data = &GLOBAL_DATA[offset..offset+32];
                    offset +=32;
                    if let Ok(sk) = secp256k1::SecretKey::from_slice(sk_data) {
                        let keypair = secp256k1::KeyPair::from_secret_key(&secp, sk);
                        let xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
                        let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);
                        let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[offset..offset+32]).unwrap();
                        offset +=32;
                        let sig = secp.sign_ecdsa(&msg, &sk);
                        secp.verify_ecdsa(&msg, &sig, &pk).unwrap();
                        let mut sig_der = sig.serialize_der();
                        println!("{:?}", sig_der);
                        let sig_compact = sig.serialize_compact();
                        let _ = secp256k1::ecdsa::Signature::from_der(&sig_der);
                        let _ = secp256k1::ecdsa::Signature::from_compact(&sig_compact);
                        if GLOBAL_DATA.len() > offset + 64 {
                            let aux_rand = &GLOBAL_DATA[offset..offset+32];
                            offset +=32;
                            let schnorr_sig = secp.sign_schnorr_with_aux_rand(&msg, &keypair, aux_rand.try_into().unwrap());
                            secp.verify_schnorr(&schnorr_sig, &msg, &xonly).unwrap();
                        }
                    }
                }
                2 => {
                    if offset + 64 > GLOBAL_DATA.len() {break;}
                    let sk_data = &GLOBAL_DATA[offset..offset+32];
                    offset +=32;
                    if let Ok(sk) = secp256k1::SecretKey::from_slice(sk_data) {
                        let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[offset..offset+32]).unwrap();
                        offset +=32;
                        let sig = secp.sign_ecdsa_low_r(&msg, &sk);
                        secp.verify_ecdsa(&msg, &sig, &secp256k1::PublicKey::from_secret_key(&secp, &sk)).unwrap();
                        if let Ok(pk_str) = std::str::from_utf8(&GLOBAL_DATA[offset..offset+64]) {
                            if let Ok(pk) = secp256k1::PublicKey::from_str(pk_str) {
                                let _ = secp256k1::ecdh::SharedSecret::new(&pk, &sk);
                            }
                        }
                        offset +=64;
                    }
                }
                3 => {
                    if offset + 2 > GLOBAL_DATA.len() {break;}
                    let iterations = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset +=1;
                    for _ in 0..iterations {
                        let sk_data = &GLOBAL_DATA[offset..offset+32];
                        if let Ok(sk) = secp256k1::SecretKey::from_slice(sk_data) {
                            let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[offset..offset+32]).unwrap();
                            let _ = secp.sign_ecdsa_grind_r(&msg, &sk, 16);
                        }
                        offset = (offset + 32) % GLOBAL_DATA.len();
                    }
                }
                _ => {}
            }
        }
    });
}



// ... (Type conversion functions remain unchanged but are excluded as per directions)

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