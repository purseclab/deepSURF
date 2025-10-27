#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use secp256k1::ecdh::SharedSecret;
use std::str::FromStr;
use global_data::{set_global_data, get_global_data};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut index = 0;

        let secp = Secp256k1::new();
        let secp_verifying = Secp256k1::verification_only();
        let mut secp_prealloc = vec![secp256k1_sys::types::AlignedType::zeroed(); Secp256k1::preallocate_size()];
        let secp_prealloc_ctx = Secp256k1::preallocated_new(secp_prealloc.as_mut_slice()).unwrap();

        let sk_len = 32;
        let sk = _unwrap_result(SecretKey::from_slice(&GLOBAL_DATA[index..index+sk_len]));
        index += sk_len;

        let use_constructors = _to_u8(GLOBAL_DATA, index) % 3;
        index += 1;
        let keypair = match use_constructors {
            0 => _unwrap_result(KeyPair::from_seckey_str(&secp, _to_str(GLOBAL_DATA, index, index + 64))),
            1 => _unwrap_result(KeyPair::from_seckey_slice(&secp, &GLOBAL_DATA[index..index+32])),
            _ => KeyPair::from_secret_key(&secp, sk),
        };
        index += 64;

        let pk = PublicKey::from_secret_key(&secp, &sk);
        let pk2 = _unwrap_result(PublicKey::from_slice(&GLOBAL_DATA[index..index+33]));
        index += 33;
        let combined_pk = _unwrap_result(PublicKey::combine_keys(&[&pk, &pk2]));
        let mut xonly = XOnlyPublicKey::from_keypair(&keypair);

        let str_len = (_to_u8(GLOBAL_DATA, index) % 65) as usize;
        index += 1;
        let s = _to_str(GLOBAL_DATA, index, index + str_len);
        let xonly_str = _unwrap_result(XOnlyPublicKey::from_str(s));
        index += str_len;

        let msg = _unwrap_result(Message::from_slice(&GLOBAL_DATA[index..index+32]));
        index += 32;

        let sig = secp.sign_ecdsa(&msg, &sk);
        let mut normalized_sig = sig.clone();
        normalized_sig.normalize_s();
        _unwrap_result(secp.verify_ecdsa(&msg, &sig, &combined_pk));
        _unwrap_result(secp_verifying.verify_ecdsa(&msg, &normalized_sig, &pk));

        let sig_low_r = secp.sign_ecdsa_low_r(&msg, &sk);
        let sig_grind = secp.sign_ecdsa_grind_r(&msg, &sk, _to_usize(GLOBAL_DATA, index) % 100);
        index += 8;

        let tweak = &GLOBAL_DATA[index..index+32];
        index += 32;
        let parity = _unwrap_result(xonly.tweak_add_assign(&secp, tweak));

        let schnorr_sig = secp.schnorrsig_sign_no_aux_rand(&msg, &keypair);
        let aux_rand = &GLOBAL_DATA[index..index+32];
        index +=32;
        let aux_rand: &[u8;32] = aux_rand.try_into().unwrap();
        let schnorr_sig_aux = secp_prealloc_ctx.schnorrsig_sign_with_aux_rand(&msg, &keypair, aux_rand);
        _unwrap_result(secp.schnorrsig_verify(&schnorr_sig_aux, &msg, &xonly));
        _unwrap_result(secp.verify_ecdsa(&msg, &_unwrap_result(Signature::from_der(&sig.serialize_der().as_ref())), &pk));

        let shared_secret = SharedSecret::new(&pk, &sk);
        let secret_bytes = shared_secret.secret_bytes();
        let display_secret = shared_secret.display_secret();
        let serialized_pk = combined_pk.serialize();
        let serialized_xonly = xonly.serialize();
        
        println!("{:?} {:?} {:?} {:?}", serialized_xonly, parity, secret_bytes, display_secret);
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