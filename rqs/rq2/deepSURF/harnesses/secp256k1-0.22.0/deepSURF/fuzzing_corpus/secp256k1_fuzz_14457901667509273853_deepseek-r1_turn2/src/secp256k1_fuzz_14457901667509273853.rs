#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use secp256k1_sys::*;

struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
    fn into_32(self) -> [u8; 32] {
        let global_data = get_global_data();
        let selector = _to_usize(global_data.second_half, 0) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = if selector == 1 { global_data.first_half } else { global_data.second_half };
        let mut res = [0u8; 32];
        for i in 0..32 {
            res[i] = _to_u8(GLOBAL_DATA, i * 2);
        }
        res
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let secp = Secp256k1::new();
        let operations = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut idx = std::mem::size_of::<usize>();

        let mut messages = Vec::new();
        let mut signatures = Vec::new();
        let mut keypairs = Vec::new();

        for _ in 0..operations {
            let op_selector = _to_u8(GLOBAL_DATA, idx) % 6;
            idx = idx.wrapping_add(1);

            match op_selector {
                0 => {
                    let start = idx % GLOBAL_DATA.len();
                    let end = start + 32;
                    if end > GLOBAL_DATA.len() { break; }
                    let msg = Message::from_slice(&GLOBAL_DATA[start..end]).unwrap_or_else(|_| std::process::exit(0));
                    messages.push(msg);
                    idx = end;
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, idx) % 64;
                    idx = idx.wrapping_add(1);
                    let s = _to_str(GLOBAL_DATA, idx, idx + len as usize);
                    let msg = String::from(s);
                    messages.push(Message::from(CustomType0(msg)));
                }
                2 => {
                    let start = idx % GLOBAL_DATA.len();
                    let end = start + 32;
                    if end > GLOBAL_DATA.len() { break; }
                    let keypair = secp256k1::KeyPair::from_seckey_slice(&secp, &GLOBAL_DATA[start..end]).unwrap_or_else(|_| std::process::exit(0));
                    keypairs.push(keypair);
                    idx = end;
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, idx) % 64;
                    idx = idx.wrapping_add(1);
                    let s = _to_str(GLOBAL_DATA, idx, idx + len as usize);
                    let keypair = secp256k1::KeyPair::from_seckey_str(&secp, s).unwrap_or_else(|_| std::process::exit(0));
                    keypairs.push(keypair);
                    idx = idx.wrapping_add(len as usize);
                }
                4 => {
                    let msg_idx = _to_usize(GLOBAL_DATA, idx) % messages.len();
                    idx = idx.wrapping_add(std::mem::size_of::<usize>());
                    let key_idx = _to_usize(GLOBAL_DATA, idx) % keypairs.len();
                    idx = idx.wrapping_add(std::mem::size_of::<usize>());
                    
                    let sig = secp.sign_grind_r(&messages[msg_idx], &secp256k1::SecretKey::from_keypair(&keypairs[key_idx]), _to_usize(GLOBAL_DATA, idx));
                    signatures.push(sig);
                    idx = idx.wrapping_add(std::mem::size_of::<usize>());
                }
                5 => {
                    let msg_idx = _to_usize(GLOBAL_DATA, idx) % messages.len();
                    let sig_idx = _to_usize(GLOBAL_DATA, idx) % signatures.len();
                    let key_idx = _to_usize(GLOBAL_DATA, idx) % keypairs.len();
                    idx = idx.wrapping_add(std::mem::size_of::<usize>()*3);
                    
                    let verify_secp = Secp256k1::verification_only();
                    println!("Verifying sig: {:?}", &signatures[sig_idx]);
                    verify_secp.verify_ecdsa(&messages[msg_idx], &signatures[sig_idx], &secp256k1::PublicKey::from_keypair(&keypairs[key_idx])).unwrap_or_else(|_| std::process::exit(0));
                }
                _ => break,
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