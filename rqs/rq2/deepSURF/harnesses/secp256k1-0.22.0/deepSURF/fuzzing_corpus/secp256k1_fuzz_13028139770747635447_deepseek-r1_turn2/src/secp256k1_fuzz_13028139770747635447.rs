#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let ctx_type = _to_u8(GLOBAL_DATA, offset) % 4;
        offset += 1;

        let mut buf = Vec::with_capacity(1024);
        for _ in 0..1024 {
            buf.push(secp256k1_sys::types::AlignedType::zeroed());
        }
        let buf_slice = &mut buf[..];

        let secp = match ctx_type {
            0 => Secp256k1::preallocated_new(buf_slice),
            1 => Secp256k1::preallocated_gen_new(buf_slice),
            2 => Secp256k1::preallocated_new(buf_slice),
            _ => Secp256k1::preallocated_gen_new(buf_slice),
        };
        let secp = _unwrap_result(secp);

        let ops_count = _to_u8(GLOBAL_DATA, offset) % 8 + 1;
        offset += 1;

        for _ in 0..ops_count {
            let op_select = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_select {
                0 => {
                    let sk_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let sk_slice = _to_str(GLOBAL_DATA, offset, offset + sk_len as usize);
                    offset += sk_len as usize;
                    let sk = SecretKey::from_str(sk_slice);
                    let _ = _unwrap_result(sk);
                }
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key_data = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let key_pair = KeyPair::from_seckey_str(&secp, key_data);
                    let key_pair = _unwrap_result(key_pair);
                    let mut sk = SecretKey::from(key_pair);
                    println!("{:?}", sk);
                    sk.negate_assign();
                }
                2 => {
                    let msg_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let msg_data = &GLOBAL_DATA[offset..offset + msg_len as usize];
                    offset += msg_len as usize;
                    let msg = Message::from_slice(msg_data);
                    let msg = _unwrap_result(msg);
                    let key = _to_str(GLOBAL_DATA, offset, offset + 32);
                    offset += 32;
                    let key_pair = KeyPair::from_seckey_str(&secp, key);
                    let key_pair = _unwrap_result(key_pair);
                    let sig = secp.sign_ecdsa(&msg, &SecretKey::from(key_pair));
                    println!("{:?}", sig);
                }
                3 => {
                    let pk_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let pk_data = &GLOBAL_DATA[offset..offset + pk_len as usize];
                    offset += pk_len as usize;
                    let pk = PublicKey::from_slice(pk_data);
                    let pk = _unwrap_result(pk);
                    let combine_with = PublicKey::from_slice(&GLOBAL_DATA[offset..offset + 33]);
                    let combine_with = _unwrap_result(combine_with);
                    let _ = pk.combine(&combine_with);
                }
                4 => {
                    let sk_data = &GLOBAL_DATA[offset..offset + 32];
                    offset += 32;
                    let sk = SecretKey::from_slice(sk_data);
                    let sk = _unwrap_result(sk);
                    let mut xonly = XOnlyPublicKey::from(PublicKey::from_secret_key(&secp, &sk));
                    let tweak = &GLOBAL_DATA[offset..offset + 32];
                    offset += 32;
                    let _ = xonly.tweak_add_assign(&secp, tweak);
                }
                _ => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let key_data = _to_str(GLOBAL_DATA, offset, offset + key_len as usize);
                    offset += key_len as usize;
                    let key_pair = KeyPair::from_seckey_slice(&secp, key_data.as_bytes());
                    let key_pair = _unwrap_result(key_pair);
                    let mut pubkey = PublicKey::from_keypair(&key_pair);
                    let msg = Message::from_slice(&GLOBAL_DATA[offset..offset + 32]);
                    let msg = _unwrap_result(msg);
                    let sig = secp.sign_ecdsa(&msg, &SecretKey::from(key_pair));
                    let _ = secp.verify_ecdsa(&msg, &sig, &pubkey);
                    pubkey.negate_assign(&secp);
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