#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use secp256k1::*;
use global_data::*;
use std::str::FromStr;

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
        let mut arr = [0u8; 32];
        for i in 0..32 {
            arr[i] = _to_u8(GLOBAL_DATA, 9 + i);
        }
        arr
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        let mut offset = 1;

        for _ in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    let ctx_size = _to_usize(GLOBAL_DATA, offset) % 4096;
                    let mut ctx_buf = vec![secp256k1_sys::types::AlignedType::zeroed(); ctx_size];
                    let ctx = Secp256k1::preallocated_new(&mut ctx_buf).map_err(|_| ()).ok();
                    let _ = ctx.map(|c| println!("Context created: {:?}", c.ctx()));
                }
                1 => {
                    let mut buf = vec![secp256k1_sys::types::AlignedType::zeroed();1024];
                    let ctx = _unwrap_result(Secp256k1::preallocated_new(&mut buf));
                    let pk_data = &GLOBAL_DATA[offset..offset+32];
                    let secret = _unwrap_result(secp256k1::SecretKey::from_slice(pk_data));
                    let keypair = secp256k1::KeyPair::from_secret_key(&ctx, secret);
                    let pubkey = secp256k1::PublicKey::from_keypair(&keypair);
                    let xonly = secp256k1::XOnlyPublicKey::from_keypair(&keypair);
                    let shared_secret = secp256k1::ecdh::SharedSecret::new(&pubkey, &secret);
                    let sig = ctx.sign_ecdsa(&Message::from(CustomType0("random".to_string())), &secret);
                    ctx.verify_ecdsa(&Message::from(CustomType0("random".to_string())), &sig, &pubkey).unwrap();
                    println!("Processed operations: KeyPair, SharedSecret, Signature");
                }
                2 => {
                    let msg_start = offset + 42;
                    let msg_str = _to_str(GLOBAL_DATA, msg_start, msg_start + 32);
                    let t75 = CustomType0(msg_str.to_string());
                    let msg = secp256k1::Message::from(t75);
                    let secp = Secp256k1::new();
                    let sk = _unwrap_result(secp256k1::SecretKey::from_slice(&GLOBAL_DATA[offset..offset+32]));
                    let sig = secp.sign_ecdsa(&msg, &sk);
                    let ser_sig = sig.serialize_der();
                    println!("Signature: {:?}", ser_sig);
                }
                3 => {
                    let secp = Secp256k1::verification_only();
                    let sig = _unwrap_result(secp256k1::ecdsa::Signature::from_der(&GLOBAL_DATA[offset..offset+72]));
                    let pubkey = _unwrap_result(secp256k1::PublicKey::from_slice(&GLOBAL_DATA[offset+72..offset+105]));
                    let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[offset+105..offset+137]).unwrap();
                    let _ = secp.verify_ecdsa(&msg, &sig, &pubkey);
                }
                4 => {
                    let secp = Secp256k1::new();
                    let mut keypair = _unwrap_result(secp256k1::KeyPair::from_seckey_str(&secp, _to_str(GLOBAL_DATA, offset, offset+64)));
                    let tweak = &GLOBAL_DATA[offset..offset+32];
                    _unwrap_result(keypair.tweak_add_assign(&secp, tweak));
                    println!("Tweaked keypair: {:?}", keypair.public_key().serialize());
                }
                _ => {
                    let ctx = Secp256k1::<All>::gen_new();
                    let sk = _unwrap_result(secp256k1::SecretKey::from_str(_to_str(GLOBAL_DATA, offset, offset+64)));
                    let msg = secp256k1::Message::from_slice(&GLOBAL_DATA[offset..offset+32]).unwrap();
                    let sig = ctx.sign_ecdsa_low_r(&msg, &sk);
                    let serialized = sig.serialize_compact();
                    println!("Low-R sig: {:?}", serialized);
                }
            }
            offset += 64;
        }
    });
}

// ... type conversion functions remain unchanged ...

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