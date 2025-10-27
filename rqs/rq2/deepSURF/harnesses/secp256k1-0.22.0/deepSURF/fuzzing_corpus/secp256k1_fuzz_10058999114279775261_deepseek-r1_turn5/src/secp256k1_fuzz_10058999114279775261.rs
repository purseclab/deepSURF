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
        let custom_impl_num = _to_usize(GLOBAL_DATA, 2);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut vec = Vec::with_capacity(32);
        for i in 0..32 {
            vec.push(_to_u8(GLOBAL_DATA, 10 + i));
        }
        vec.truncate(32);
        vec.try_into().unwrap()
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut ctor_data = vec![AlignedType::zeroed(); 128];
        let ctor_select = _to_u8(GLOBAL_DATA, 0) % 4;
        let secp = match ctor_select {
            0 => Secp256k1::preallocated_new(&mut ctor_data[0..(_to_u8(GLOBAL_DATA,1) % 33) as usize]).unwrap(),
            1 => Secp256k1::preallocated_new(&mut ctor_data[0..(_to_u8(GLOBAL_DATA,2) % 33) as usize]).unwrap(),
            2 => Secp256k1::preallocated_new(&mut ctor_data[0..(_to_u8(GLOBAL_DATA,3) % 33) as usize]).unwrap(),
            _ => Secp256k1::preallocated_new(&mut ctor_data[0..(_to_u8(GLOBAL_DATA,4) % 33) as usize]).unwrap(),
        };

        let mut sign_ctx_data = vec![AlignedType::zeroed(); 64];
        sign_ctx_data.truncate((_to_u8(GLOBAL_DATA,5) as usize) % 65);
        let sign_ctx = Secp256k1::preallocated_signing_only(&mut sign_ctx_data).unwrap();

        let sk1 = SecretKey::from_str(_to_str(GLOBAL_DATA,6,6+(_to_u8(GLOBAL_DATA,7) % 17) as usize)).unwrap();
        let kp1 = secp256k1::KeyPair::from_secret_key(&secp, sk1);
        let sk2 = SecretKey::from_slice(&GLOBAL_DATA[20..52]).unwrap();
        let kp2 = secp256k1::KeyPair::from_seckey_slice(&secp, &GLOBAL_DATA[52..84]).unwrap();

        let message_args = [
            (_to_u8(GLOBAL_DATA,84) % 17, 85),
            (_to_u8(GLOBAL_DATA,102) % 17, 103),
            (_to_u8(GLOBAL_DATA,120) % 17, 121)
        ];
        let mut messages = vec![];
        for (len, start) in message_args {
            let s = _to_str(GLOBAL_DATA, start, start + len as usize);
            messages.push(Message::from(CustomType0(s.to_string())));
        }

        let mut signatures = vec![];
        for (i, msg) in messages.iter().enumerate() {
            let aux = [_to_u8(GLOBAL_DATA,138 + i) % 255; 32];
            let sig = match i % 3 {
                0 => sign_ctx.schnorrsig_sign_no_aux_rand(msg, &kp1),
                1 => sign_ctx.schnorrsig_sign_with_aux_rand(msg, &kp2, &aux),
                _ => sign_ctx.schnorrsig_sign_with_aux_rand(msg, &kp1, &aux),
            };
            signatures.push(sig);
        }

        let xonly_keys = vec![
            secp256k1::XOnlyPublicKey::from_keypair(&kp1),
            secp256k1::XOnlyPublicKey::from_slice(&GLOBAL_DATA[160..192]).unwrap(),
            secp256k1::XOnlyPublicKey::from(secp256k1::PublicKey::from_keypair(&kp2)),
        ];

        let mut verify_ctx_data = vec![AlignedType::zeroed(); (_to_u8(GLOBAL_DATA,200) % 65) as usize];
        let binding = verify_ctx_data.as_mut_slice();
        let verify_ctx = Secp256k1::preallocated_verification_only(binding).unwrap();

        for (i, (sig, msg)) in signatures.iter().zip(messages.iter()).enumerate() {
            verify_ctx.verify_schnorr(sig, msg, &xonly_keys[i % xonly_keys.len()]).unwrap();
        }

        let mut buffer = [0u8; 64];
        buffer.copy_from_slice(&GLOBAL_DATA[220..284]);
        let tweaked_sig = secp.sign_ecdsa(&messages[0], &sk2);
        let serialized = tweaked_sig.serialize_der();
        let _deserialized = secp.verify_ecdsa(&messages[0], &serialized.to_signature().unwrap(), &secp256k1::PublicKey::from_keypair(&kp1));

        let mut tweak_kp = kp2.clone();
        tweak_kp.tweak_add_assign(&secp, &GLOBAL_DATA[284..316]).unwrap();
        let tweaked_xonly = secp256k1::XOnlyPublicKey::from_keypair(&tweak_kp);
        let tweaked_msg = Message::from_slice(&GLOBAL_DATA[316..348]).unwrap();
        let tweaked_sig = sign_ctx.schnorrsig_sign_no_aux_rand(&tweaked_msg, &tweak_kp);
        verify_ctx.verify_schnorr(&tweaked_sig, &tweaked_msg, &tweaked_xonly).unwrap();
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