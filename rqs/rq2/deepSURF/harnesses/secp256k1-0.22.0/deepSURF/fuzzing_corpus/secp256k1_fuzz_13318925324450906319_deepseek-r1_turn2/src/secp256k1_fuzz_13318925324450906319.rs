#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use secp256k1_sys::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8 + 2;
        let mut public_keys = vec![];
        let mut key_pairs = vec![];

        let mut offset = 1;
        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            let aligned_size1 = _to_u8(GLOBAL_DATA, offset) % 65;
            offset += 1;
            let mut buf1 = vec![types::AlignedType::zeroed(); aligned_size1 as usize];
            let secp1 = Secp256k1::preallocated_new(&mut buf1[..]).ok();

            let aligned_size2 = _to_u8(GLOBAL_DATA, offset) % 65;
            offset += 1;
            let mut buf2 = vec![types::AlignedType::zeroed(); aligned_size2 as usize];
            let secp2 = Secp256k1::preallocated_signing_only(&mut buf2[..]).ok();

            match op_selector {
                0 => {
                    let sk_str_len = _to_u8(GLOBAL_DATA, offset) % 16;
                    offset += 1;
                    let sk_str = _to_str(GLOBAL_DATA, offset as usize, offset as usize + sk_str_len as usize);
                    offset += sk_str_len as usize;
                    
                    if let Ok(sk) = SecretKey::from_str(sk_str) {
                        let pk = secp256k1::PublicKey::from_secret_key(secp1.as_ref().unwrap(), &sk);
                        public_keys.push(pk);
                    }
                },
                1 => {
                    let keypair_sk_len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    let sk_slice = &GLOBAL_DATA[offset..offset + keypair_sk_len as usize];
                    offset += keypair_sk_len as usize;
                    
                    if let Ok(kp) = secp256k1::KeyPair::from_seckey_slice(secp1.as_ref().unwrap(), sk_slice) {
                        key_pairs.push(kp);
                        public_keys.push(secp256k1::PublicKey::from_keypair(&kp));
                    }
                },
                2 => {
                    let pk_str_len = _to_u8(GLOBAL_DATA, offset) % 16;
                    offset += 1;
                    let pk_str = _to_str(GLOBAL_DATA, offset as usize, offset as usize + pk_str_len as usize);
                    offset += pk_str_len as usize;
                    
                    if let Ok(pk) = secp256k1::PublicKey::from_str(pk_str) {
                        public_keys.push(pk);
                    }
                },
                3 => {
                    let pk_slice_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    if let Ok(pk) = secp256k1::PublicKey::from_slice(&GLOBAL_DATA[offset..offset + pk_slice_len as usize]) {
                        public_keys.push(pk);
                    }
                    offset += pk_slice_len as usize;
                },
                4 => {
                    if let (Some(pk1), Some(pk2)) = (public_keys.first(), public_keys.get(1)) {
                        let _ = pk1.combine(pk2);
                    }
                },
                5 => {
                    if let (Some(secp), Some(kp)) = (secp2.as_ref(), key_pairs.first()) {
                        let msg_slice = &GLOBAL_DATA[offset..offset + 32];
                        offset += 32;
                        if let Ok(msg) = Message::from_slice(msg_slice) {
                            let _ = secp.sign_ecdsa(&msg, &SecretKey::from(kp));
                        }
                    }
                },
                _ => {}
            }
        }

        for i in 1..public_keys.len() {
            let ord = public_keys[i-1].cmp(&public_keys[i]);
            println!("{:?}", ord);
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