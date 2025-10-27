#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;

#[derive(Debug)]
struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
    fn into_32(self) -> [u8; 32] {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut buffer = [0u8; 32];
        buffer.copy_from_slice(&GLOBAL_DATA[40..72]);
        buffer
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let secp = Secp256k1::new();
        let mut pos = 0;

        let ops = _to_u8(GLOBAL_DATA, pos) % 5;
        pos += 1;

        for _ in 0..ops {
            let op_type = _to_u8(GLOBAL_DATA, pos) % 4;
            pos += 1;

            match op_type {
                0 => {
                    let msg = Message::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let sk = SecretKey::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let sig = secp.sign_ecdsa(&msg, &sk);
                    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);
                    println!("Verifying: {:?}", pk);
                    secp.verify_ecdsa(&msg, &sig, &pk).unwrap();
                }
                1 => {
                    let sk_slice = &GLOBAL_DATA[pos..pos+32];
                    pos += 32;
                    let keypair = secp256k1::KeyPair::from_seckey_slice(&secp, sk_slice).unwrap();
                    let sk = secp256k1::SecretKey::from_keypair(&keypair);
                    let msg = Message::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let sig = secp.sign_grind_r(&msg, &sk, pos % 1000);
                    let xonly_pk = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
                    let pk = secp256k1::PublicKey::from_keypair(&keypair);
                    secp.verify_ecdsa(&msg, &sig, &pk).unwrap();
                }
                2 => {
                    let str_len = _to_u8(GLOBAL_DATA, pos) as usize;
                    pos += 1;
                    let sk_str = _to_str(GLOBAL_DATA, pos, pos + str_len);
                    pos += str_len;
                    let sk = secp256k1::SecretKey::from_str(sk_str).unwrap();
                    let pk = secp256k1::PublicKey::from_secret_key(&secp, &sk);
                    let msg = Message::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let mut ctx_buf = vec![secp256k1_sys::types::AlignedType::zeroed(); Secp256k1::preallocate_size()];
                    let pre_secp = Secp256k1::preallocated_signing_only(&mut ctx_buf).unwrap();
                    let sig = pre_secp.sign_low_r(&msg, &sk);
                    secp.verify_ecdsa(&msg, &sig, &pk).unwrap();
                }
                3 => {
                    let sk = secp256k1::SecretKey::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let tweak = &GLOBAL_DATA[pos..pos+32];
                    pos += 32;
                    let mut keypair = secp256k1::KeyPair::from_secret_key(&secp, sk);
                    keypair.tweak_add_assign(&secp, tweak).unwrap();
                    let msg = Message::from_slice(&GLOBAL_DATA[pos..pos+32]).unwrap();
                    pos += 32;
                    let sig = secp.sign_ecdsa_low_r(&msg, &secp256k1::SecretKey::from_keypair(&keypair));
                    let pk = secp256k1::PublicKey::from_keypair(&keypair);
                    println!("Verify tweaked: {:?}", sig);
                    secp.verify_ecdsa(&msg, &sig, &pk).unwrap();
                }
                _ => unreachable!(),
            }
        }

        let custom_str = _to_str(GLOBAL_DATA, pos, pos + 32);
        let custom_msg = CustomType0(custom_str.to_string());
        let msg = Message::from(custom_msg);
        let sk_bytes = &GLOBAL_DATA[pos+32..pos+64];
        let keypair = secp256k1::KeyPair::from_seckey_slice(&secp, sk_bytes).unwrap();
        let pk = secp256k1::PublicKey::from_keypair(&keypair);
        let sig = secp.sign_grind_r(&msg, &secp256k1::SecretKey::from_keypair(&keypair), pos % 100);
        secp.verify(&msg, &sig, &pk).unwrap();
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