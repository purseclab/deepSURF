#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ctx = Secp256k1::new();
        let mut op_index: usize = 0;

        let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[0..32]));
        let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[32..64]));
        let keypair = _unwrap_result(KeyPair::from_seckey_slice(&ctx, &GLOBAL_DATA[64..96]));
        let pubkey = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[96..129]));
        let pubkey2 = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[129..162]));
        let mut xonly = XOnlyPublicKey::from_keypair(&keypair);

        for i in 0..(_to_u8(GLOBAL_DATA, 128) % 8) {
            match _to_u8(GLOBAL_DATA, 129 + i as usize) % 7 {
                0 => {
                    let sig = ctx.sign_ecdsa(&msg, &sk);
                    let ser = sig.serialize_der();
                    let _ = ctx.verify_ecdsa(&msg, &sig, &pubkey);
                    let _ = ser.to_signature();
                }
                1 => {
                    let sig = ctx.sign_ecdsa_low_r(&msg, &sk);
                    let _ = ctx.verify_ecdsa(&msg, &sig, &pubkey);
                }
                2 => {
                    let mut data = vec![];
                    for j in 0..(_to_u8(GLOBAL_DATA, 137 + i as usize) % 72) {
                        data.push(_to_u8(GLOBAL_DATA, 200 + j as usize));
                    }
                    let _ = ecdsa::Signature::from_der_lax(&data);
                }
                3 => {
                    let compact = &GLOBAL_DATA[300..364];
                    let _ = ecdsa::Signature::from_compact(compact);
                }
                4 => {
                    let combined = _unwrap_result(PublicKey::combine(&pubkey, &pubkey2));
                    let tweaked = _unwrap_result(xonly.tweak_add_assign(&ctx, &GLOBAL_DATA[400..432]));
                }
                5 => {
                    let der = &GLOBAL_DATA[500..572];
                    let _ = ecdsa::Signature::from_der(der);
                }
                6 => {
                    let schnorr_sig = ctx.sign_schnorr_no_aux_rand(&msg, &keypair);
                    let _ = ctx.verify_schnorr(&schnorr_sig, &msg, &xonly);
                }
                _ => (),
            }
            op_index = op_index.wrapping_add(64);
        }

        let der_lax_len = _to_u8(GLOBAL_DATA, 0) % 72;
        let der_lax_start = 1024 % GLOBAL_DATA.len();
        let der_lax_end = der_lax_start + der_lax_len as usize;
        let t_34 = &GLOBAL_DATA[der_lax_start..der_lax_end.min(GLOBAL_DATA.len())];
        let _ = ecdsa::Signature::from_der_lax(t_34);
    });
}

// ... (type converter functions remain unchanged but are excluded as per directions)

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