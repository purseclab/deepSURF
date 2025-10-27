#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut offset = 0;
        let num_ops = _to_u8(GLOBAL_DATA, offset) % 5;
        offset += 1;

        for _ in 0..num_ops {
            if offset >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op {
                0 => {
                    let secp = Secp256k1::signing_only();
                    if offset + 64 > GLOBAL_DATA.len() { continue; }
                    let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[offset..offset+32]));
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[offset+32..offset+64]));
                    offset += 64;
                    let sig = secp.sign_ecdsa(&msg, &sk);
                    let pubkey = PublicKey::from_secret_key(&secp, &sk);
                    let verify_secp = Secp256k1::verification_only();
                    _unwrap_result(verify_secp.verify_ecdsa(&msg, &sig, &pubkey));
                    println!("{:?}", sig);
                }
                1 => {
                    let mut buf = [secp256k1_sys::types::AlignedType::zeroed(); 1024];
                    let secp = _unwrap_result(Secp256k1::preallocated_new(&mut buf));
                    let neg_one = SecretKey::from_slice(&[0xff; 32]).unwrap();
                    let keypair = KeyPair::from_secret_key(&secp, neg_one);
                    let xonly = XOnlyPublicKey::from_keypair(&keypair);
                    println!("{:?}", xonly.serialize());
                }
                2 => {
                    let secp = Secp256k1::verification_only();
                    if offset + 33 > GLOBAL_DATA.len() { continue; }
                    let pk = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[offset..offset+33]));
                    offset += 33;
                    let compressed = pk.serialize();
                    let uncompressed = pk.serialize_uncompressed();
                    println!("{:?} {:?}", compressed, uncompressed);
                    let combined = PublicKey::combine(&pk, &pk).unwrap();
                    let mut tweaked_pk = pk;
                    _unwrap_result(tweaked_pk.add_exp_assign(&secp, &GLOBAL_DATA[offset..offset+32]));
                }
                3 => {
                    let secp = Secp256k1::<All>::gen_new();
                    let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[offset..offset+32]));
                    offset += 32;
                    let tweak = &GLOBAL_DATA[offset..offset+32];
                    offset += 32;
                    let mut keypair = _unwrap_result(KeyPair::from_seckey_slice(&secp, &sk.secret_bytes()));
                    _unwrap_result(keypair.tweak_add_assign(&secp, tweak));
                    let new_pk = PublicKey::from_keypair(&keypair);
                    let xonly = XOnlyPublicKey::from_keypair(&keypair);
                    let schnor_sig = secp.sign_schnorr_no_aux_rand(&Message::from_slice(tweak).unwrap(), &keypair);
                    let verify_secp = Secp256k1::verification_only();
                    _unwrap_result(verify_secp.verify_schnorr(&schnor_sig, &Message::from_slice(tweak).unwrap(), &xonly));
                    println!("{:?}", new_pk);
                }
                4 => {
                    let secp = Secp256k1::signing_only();
                    let keypair = _unwrap_result(KeyPair::from_seckey_str(&secp, _to_str(GLOBAL_DATA, offset, offset + 32)));
                    offset += 32;
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[offset..offset+32]));
                    offset += 32;
                    let aux_rand = [GLOBAL_DATA[offset % GLOBAL_DATA.len()]; 32];
                    offset += 1;
                    let sig = secp.sign_schnorr_with_aux_rand(&msg, &keypair, &aux_rand);
                    let xonly = XOnlyPublicKey::from_keypair(&keypair);
                    let verify_secp = Secp256k1::verification_only();
                    _unwrap_result(verify_secp.verify_schnorr(&sig, &msg, &xonly));
                }
                5 => {
                    let secp = Secp256k1::signing_only();
                    let ctx_size = secp256k1::Secp256k1::preallocate_size();
                    let mut buf = vec![secp256k1_sys::types::AlignedType::zeroed(); ctx_size];
                    let mut secp_prealloc = _unwrap_result(Secp256k1::preallocated_new(&mut buf));
                    secp_prealloc.seeded_randomize(&GLOBAL_DATA[offset..offset+32].try_into().unwrap());
                    offset += 32;
                    let keypair = _unwrap_result(KeyPair::from_seckey_slice(&secp_prealloc, &GLOBAL_DATA[offset..offset+32]));
                    let pubkey = PublicKey::from_keypair(&keypair);
                    let sk = SecretKey::from_keypair(&keypair);
                    let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[offset+32..offset+64]));
                    let sig = secp_prealloc.sign_ecdsa_low_r(&msg, &sk);
                    offset += 64;
                }
                _ => (),
            }
        }

        let ctx = secp256k1::Secp256k1::signing_only();
        let parity = _unwrap_result(Parity::from_u8(GLOBAL_DATA[offset % GLOBAL_DATA.len()]));
        println!("{:?}", parity);
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