#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::{
    ecdh::SharedSecret, ecdsa::Signature, ffi::types::AlignedType, 
    KeyPair, Message, PublicKey, Secp256k1, SecretKey, XOnlyPublicKey, ecdsa, schnorr,
};
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut t_1 = std::vec::Vec::with_capacity(t_0 as usize);
        for i in 0..t_0 {
            t_1.push(AlignedType::zeroed());
        }
        let t_34 = &mut t_1[..];
        let t_35 = Secp256k1::preallocated_new(t_34);
        let t_36 = _unwrap_result(t_35);
        let secp = &t_36;

        let mut data_idx = 1;
        let ops = _to_u8(GLOBAL_DATA, data_idx) % 5;
        data_idx += 1;

        for _ in 0..ops {
            let key_type = _to_u8(GLOBAL_DATA, data_idx) % 3;
            data_idx += 1;

            let keypair = match key_type {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let s = _to_str(GLOBAL_DATA, data_idx, data_idx + len as usize);
                    data_idx += len as usize;
                    _unwrap_result(KeyPair::from_seckey_str(secp, s))
                },
                1 => {
                    let len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let sl = &GLOBAL_DATA[data_idx..data_idx + len as usize];
                    data_idx += len as usize;
                    _unwrap_result(KeyPair::from_seckey_slice(secp, sl))
                },
                _ => _unwrap_result(KeyPair::from_seckey_str(secp, "0000000000000000000000000000000000000000000000000000000000000000"))
            };

            let mut secret = SecretKey::from(keypair);
            let msg_slice = &GLOBAL_DATA[data_idx..data_idx + 32];
            data_idx += 32;
            let msg = _unwrap_result(Message::from_slice(msg_slice));
            let sig = secp.sign_ecdsa(&msg, &secret);
            _unwrap_result(secp.verify_ecdsa(&msg, &sig, &PublicKey::from_secret_key(secp, &secret)));

            let tweak_slice = &GLOBAL_DATA[data_idx..data_idx + 32];
            data_idx += 32;
            let mut tweak = [0u8; 32];
            tweak.copy_from_slice(tweak_slice);
            secret.mul_assign(&tweak);

            let mut xonly = XOnlyPublicKey::from_keypair(&keypair);
            let parity_val = _to_u8(GLOBAL_DATA, data_idx) % 2;
            data_idx += 1;
            let original_xonly = xonly;
            let tweaked_parity = _unwrap_result(xonly.tweak_add_assign(secp, &tweak));

            let mut public = PublicKey::from_secret_key(secp, &secret);
            public.mul_assign(secp, &tweak).unwrap();

            let combine_slice = &GLOBAL_DATA[data_idx..data_idx + 33];
            data_idx += 33;
            let other_pub = _unwrap_result(PublicKey::from_slice(combine_slice));
            _unwrap_result(public.combine(&other_pub));

            let shared = SharedSecret::new(&other_pub, &secret);
            println!("{:?}", shared);

            let mut sersig = sig.serialize_der();
            println!("{:?}", sersig);

            if _to_bool(GLOBAL_DATA, data_idx) {
                let schnorr_sig = secp.sign_schnorr_no_aux_rand(&msg, &keypair);
                _unwrap_result(secp.verify_schnorr(&schnorr_sig, &msg, &xonly));
            }

            data_idx += 1;
            let combine_len = _to_usize(GLOBAL_DATA, data_idx) % 32;
            data_idx += 8;
            let combine_xonly = _unwrap_result(XOnlyPublicKey::from_slice(&GLOBAL_DATA[data_idx..data_idx + combine_len]));
            data_idx += combine_len;

            let verify_tweak_check = original_xonly.tweak_add_check(secp, &xonly, tweaked_parity, tweak);
            assert!(verify_tweak_check);
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