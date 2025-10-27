#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use secp256k1_sys::*;
use secp256k1_sys::types::AlignedType;

struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
    
    fn into_32(self) -> [u8; 32] {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                    1 => global_data.first_half,
                    _ => global_data.second_half,
        };
        let mut t_38 = std::vec::Vec::with_capacity(32);
        for i in 9..41 {
            t_38.push(_to_u8(GLOBAL_DATA, i));
        }
        t_38.truncate(32);
        let t_71: [_; 32] = t_38.try_into().unwrap();
        return t_71;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 384 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut offset = 0;
        let iterations = _to_usize(GLOBAL_DATA, offset) % 5;
        offset += 1;

        for _ in 0..iterations {
            let mut ctx_buf = vec![AlignedType::zeroed(); 1024];
            let mut verify_ctx_buf = vec![AlignedType::zeroed(); 1024];
            let ctx = _unwrap_result(Secp256k1::preallocated_new(&mut ctx_buf));
            let verify_ctx = _unwrap_result(Secp256k1::preallocated_verification_only(&mut verify_ctx_buf));

            let keypair = {
                let constructor_selector = _to_u8(GLOBAL_DATA, offset) % 3;
                offset +=1;
                match constructor_selector {
                    0 => {
                        let mut t_150 = vec![0; 32];
                        t_150.iter_mut().enumerate().for_each(|(i, v)| *v = _to_u8(GLOBAL_DATA, offset + i));
                        offset += 32;
                        _unwrap_result(secp256k1::KeyPair::from_seckey_slice(&ctx, &t_150))
                    },
                    1 => {
                        let len = _to_u8(GLOBAL_DATA, offset);
                        offset +=1;
                        let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                        offset += len as usize;
                        _unwrap_result(secp256k1::KeyPair::from_seckey_str(&ctx, s))
                    },
                    _ => {
                        let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[offset..offset+32]));
                        offset +=32;
                        secp256k1::KeyPair::from_secret_key(&ctx, sk)
                    }
                }
            };

            let msg = {
                let len = _to_u8(GLOBAL_DATA, offset) % 32;
                offset +=1;
                let s = _to_str(GLOBAL_DATA, offset, offset + len as usize);
                offset += len as usize;
                Message::from(CustomType0(s.to_string()))
            };

            let sig = ctx.sign_ecdsa(&msg, &SecretKey::from(&keypair));
            let _ = verify_ctx.verify_ecdsa(&msg, &sig, &secp256k1::PublicKey::combine_keys(&[&secp256k1::PublicKey::from(&keypair)]).unwrap());

            let mut xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
            println!("XOnly: {:?}", xonly.serialize());

            let tweak_selector = _to_u8(GLOBAL_DATA, offset) % 3;
            offset +=1;
            let tweak = &GLOBAL_DATA[offset..offset + 32];
            offset +=32;
            let _ = xonly.tweak_add_assign(&ctx, tweak).unwrap();

            let mut sig_grind = ctx.sign_ecdsa_grind_r(&msg, &SecretKey::from(&keypair), _to_usize(GLOBAL_DATA, offset) % 100);
            offset +=1;
            sig_grind.normalize_s();

            let mut public_keys = vec![];
            for _ in 0..(_to_usize(GLOBAL_DATA, offset) % 3) {
                offset +=1;
                let mut pk_data = [0u8; 33];
                for i in 0..33 {
                    pk_data[i] = _to_u8(GLOBAL_DATA, offset + i);
                }
                offset +=33;
                public_keys.push(_unwrap_result(secp256k1::PublicKey::from_slice(&pk_data)));
            }
            let combined_pubkey = _unwrap_result(secp256k1::PublicKey::combine_keys(public_keys.iter().collect::<Vec<_>>().as_slice()));
            println!("Combined pubkey: {:?}", combined_pubkey.serialize());

            let schnorr_sig = ctx.sign_schnorr_no_aux_rand(&msg, &keypair);
            let _ = ctx.verify_schnorr(&schnorr_sig, &msg, &xonly).unwrap();

            let mut secret = [0u8;32];
            for i in 0..32 {
                secret[i] = _to_u8(GLOBAL_DATA, offset + i);
            }
            offset +=32;
            let mut gen_ctx = Secp256k1::<All>::gen_new();
            gen_ctx.seeded_randomize(&secret);
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