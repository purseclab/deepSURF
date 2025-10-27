#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use secp256k1::ecdh::SharedSecret;
use secp256k1_sys::types::AlignedType;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let op_count = _to_u8(GLOBAL_DATA, offset) % 10 + 1;
        offset += 1;

        let secp_gen = Secp256k1::new();
        let mut sigs = Vec::new();
        let mut messages = Vec::new();
        let mut keypairs = Vec::new();

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, offset % GLOBAL_DATA.len()) % 7;
            offset += 1;

            match op_type {
                0 => {
                    let keypair = if _to_u8(GLOBAL_DATA, offset % GLOBAL_DATA.len()) % 2 == 0 {
                        let sk_slice = &GLOBAL_DATA[offset..(offset + 32) % GLOBAL_DATA.len()];
                        let sk = _unwrap_result(SecretKey::from_slice(sk_slice));
                        _unwrap_result(secp256k1::KeyPair::from_seckey_slice(&secp_gen, sk_slice))
                    } else {
                        let sk_str_offset = offset % (GLOBAL_DATA.len() - 32);
                        let sk_str = _to_str(GLOBAL_DATA, sk_str_offset, (sk_str_offset + 32) % GLOBAL_DATA.len());
                        _unwrap_result(secp256k1::KeyPair::from_seckey_str(&secp_gen, sk_str))
                    };
                    keypairs.push(keypair);
                    offset = (offset + 32) % GLOBAL_DATA.len();
                }
                1 => {
                    let msg_slice = &GLOBAL_DATA[offset..(offset + 32) % GLOBAL_DATA.len()];
                    let msg = _unwrap_result(Message::from_slice(msg_slice));
                    messages.push(msg);
                    offset = (offset + 32) % GLOBAL_DATA.len();
                }
                2 => {
                    let ctx_type = _to_u8(GLOBAL_DATA, offset % GLOBAL_DATA.len()) % 3;
                    offset += 1;
                    let ctx = match ctx_type {
                        0 => Secp256k1::new(),
                        1 => Secp256k1::new(),
                        _ => Secp256k1::new(),
                    };

                    if let (Some(kp), Some(msg)) = (keypairs.last(), messages.last()) {
                        let sk = SecretKey::from(kp);
                        let sig = ctx.sign_ecdsa_low_r(msg, &sk);
                        sigs.push(sig);
                        let pubkey = secp256k1::PublicKey::from(kp);
                        let _ = ctx.verify_ecdsa(msg, &sig, &pubkey);
                    }
                }
                3 => {
                    if let Some(sig) = sigs.last() {
                        let serialized = sig.serialize_der();
                        let _ = _unwrap_result(secp256k1::ecdsa::Signature::from_der(&serialized.as_ref()));
                        let compact = sig.serialize_compact();
                        let _ = _unwrap_result(secp256k1::ecdsa::Signature::from_compact(&compact));
                    }
                }
                4 => {
                    if let (Some(kp1), Some(kp2)) = (keypairs.get(0), keypairs.get(1)) {
                        let mut pubkey = secp256k1::PublicKey::from(kp1);
                        let ctx = Secp256k1::verification_only();
                        pubkey.combine(&secp256k1::PublicKey::from(kp2)).unwrap();
                        let tweak = &GLOBAL_DATA[offset..(offset + 32) % GLOBAL_DATA.len()];
                        pubkey.add_exp_assign(&ctx, tweak).unwrap();
                        offset = (offset + 32) % GLOBAL_DATA.len();
                    }
                }
                5 => {
                    if let (Some(pubkey), Some(sk)) = (keypairs.get(0).map(|kp| secp256k1::PublicKey::from(kp)), keypairs.get(1).map(|kp| SecretKey::from(kp))) {
                        let shared = SharedSecret::new(&pubkey, &sk);
                        println!("{:?}", shared);
                    }
                }
                6 => {
                    if let Some(mut kp) = keypairs.pop() {
                        let tweak = &GLOBAL_DATA[offset..(offset + 32) % GLOBAL_DATA.len()];
                        kp.tweak_add_assign(&secp_gen, tweak).unwrap();
                        let mut xonly = secp256k1::XOnlyPublicKey::from_keypair(&kp);
                        let new_xonly = _unwrap_result(xonly.tweak_add_assign(&secp_gen, tweak));
                        println!("{:?} {:?}", xonly, new_xonly);
                        offset = (offset + 32) % GLOBAL_DATA.len();
                    }
                }
                _ => unreachable!(),
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