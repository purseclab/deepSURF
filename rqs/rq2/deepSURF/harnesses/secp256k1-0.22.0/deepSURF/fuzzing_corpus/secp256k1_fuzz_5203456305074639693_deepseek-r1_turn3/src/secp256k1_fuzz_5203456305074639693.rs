#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;

#[derive(Debug)]
struct CustomType0(String);

impl secp256k1::ThirtyTwoByteHash for CustomType0 {
    fn into_32(self) -> [u8; 32] {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut data = [0u8; 32];
        data.iter_mut().enumerate().for_each(|(i, d)| *d = _to_u8(GLOBAL_DATA, i + 10));
        data
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut op_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut offset = 1;
        
        let mut ctx = None;
        let mut keypairs = vec![];
        let mut messages = vec![];
        let mut signatures = vec![];

        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 5;
            offset = offset.wrapping_add(1);

            match op_selector {
                0 => {
                    let buf_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = offset.wrapping_add(1);
                    let secp = Secp256k1::new();
                    ctx = Some(secp);
                }
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, offset) % 33;
                    offset = offset.wrapping_add(1);
                    let mut key_bytes = vec![0u8; key_len as usize];
                    key_bytes.iter_mut().enumerate().for_each(|(i, b)| *b = _to_u8(GLOBAL_DATA, offset + i));
                    offset = offset.wrapping_add(key_len as usize);
                    if let Ok(sk) = SecretKey::from_slice(&key_bytes) {
                        if let Some(ctx) = &ctx {
                            let kp = KeyPair::from_secret_key(ctx, sk);
                            keypairs.push(kp);
                            let xonly = XOnlyPublicKey::from_keypair(&kp);
                            println!("XOnlyKey: {:?}", &xonly.serialize());
                        }
                    }
                }
                2 => {
                    let msg_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = offset.wrapping_add(1);
                    let msg_str = _to_str(GLOBAL_DATA, offset, offset + msg_len as usize);
                    messages.push(Message::from(CustomType0(msg_str.to_string())));
                    offset = offset.wrapping_add(msg_len as usize);
                }
                3 => {
                    if let (Some(ctx), Some(msg), Some(kp)) = (&ctx, messages.last(), keypairs.last()) {
                        let sk = SecretKey::from(kp.clone());
                        let counter = _to_usize(GLOBAL_DATA, offset) % 1000;
                        offset = offset.wrapping_add(4);
                        let sig = ctx.sign_ecdsa_grind_r(msg, &sk, counter);
                        signatures.push(sig);
                        let serialized = sig.serialize_der();
                        if let Ok(parsed) = Signature::from_der(&serialized) {
                            let _ = ctx.verify_ecdsa(msg, &parsed, &PublicKey::from_keypair(kp));
                        }
                    }
                }
                4 => {
                    if let (Some(ctx), Some(kp), Some(sig)) = (&ctx, keypairs.last_mut(), signatures.last()) {
                        let tweak = (0..32).map(|i| _to_u8(GLOBAL_DATA, offset + i)).collect::<Vec<_>>();
                        offset = offset.wrapping_add(32);
                        let _ = kp.tweak_add_assign(ctx, &tweak);
                        let xonly = XOnlyPublicKey::from_keypair(kp);
                        let msg = Message::from(CustomType0("tweak_test".to_string()));
                        let newsig = ctx.sign_ecdsa_grind_r(&msg, &SecretKey::from(kp.clone()), 0);
                        println!("NewSig: {:?}", &newsig.serialize_compact());
                    }
                }
                _ => {}
            }
        }

        if let (Some(ctx), Some(msg), Some(kp)) = (&ctx, messages.last(), keypairs.last()) {
            let sk = SecretKey::from(kp.clone());
            let grind_count = _to_usize(GLOBAL_DATA, offset) % 1000;
            ctx.sign_ecdsa_grind_r(msg, &sk, grind_count);
            let xonly = XOnlyPublicKey::from_keypair(kp);
            let mut aux_rand = [0u8; 32];
            for (i, b) in aux_rand.iter_mut().enumerate() {
                *b = _to_u8(GLOBAL_DATA, offset + i);
            }
            offset = offset.wrapping_add(32);
            let schnorr_sig = ctx.sign_schnorr_with_aux_rand(msg, kp, &aux_rand);
            let _ = ctx.verify_schnorr(&schnorr_sig, msg, &xonly);
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