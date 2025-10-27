#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2180 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let buffer = &mut [0u8; 4096];
        let mut gre_packet = _unwrap_option(pnet_packet::gre::MutableGrePacket::new(buffer));

        let op_count = _to_u8(GLOBAL_DATA, 0) % 15 + 1;
        let mut data_offset = 1;

        for _ in 0..op_count {
            if data_offset >= GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, data_offset) % 8;
            data_offset += 1;

            match op_selector {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut check_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        check_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    let u16be = _unwrap_option(pnet_packet::gre::U16BEPacket::owned(check_data));
                    let checksum = pnet_packet::gre::U16BEPacket::from_packet(&u16be);
                    gre_packet.set_checksum(&[checksum][..]);
                },
                1 => {
                    let key_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut key_data = Vec::with_capacity(key_len as usize);
                    for _ in 0..key_len {
                        key_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    let u32be = _unwrap_option(pnet_packet::gre::U32BEPacket::owned(key_data));
                    let key = pnet_packet::gre::U32BEPacket::from_packet(&u32be);
                    gre_packet.set_key(&[key][..]);
                },
                2 => {
                    let seq_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut seq_data = Vec::with_capacity(seq_len as usize);
                    for _ in 0..seq_len {
                        seq_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    let u32be = _unwrap_option(pnet_packet::gre::U32BEPacket::owned(seq_data));
                    let sequence = pnet_packet::gre::U32BEPacket::from_packet(&u32be);
                    gre_packet.set_sequence(&[sequence][..]);
                },
                3 => {
                    let offset_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut offset_data = Vec::with_capacity(offset_len as usize);
                    for _ in 0..offset_len {
                        offset_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    let u16be = _unwrap_option(pnet_packet::gre::U16BEPacket::owned(offset_data));
                    let offset = pnet_packet::gre::U16BEPacket::from_packet(&u16be);
                    gre_packet.set_offset(&[offset][..]);
                },
                4 => {
                    let payload_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let payload = &GLOBAL_DATA[data_offset..data_offset + payload_len as usize];
                    data_offset += payload_len as usize;
                    gre_packet.set_payload(payload);
                },
                5 => {
                    let gre = pnet_packet::gre::GrePacket::from_packet(&gre_packet.to_immutable());
                    gre_packet.populate(&gre);
                },
                6 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut routing_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        routing_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    gre_packet.set_routing(&routing_data[..]);
                },
                _ => {
                    let mut eth_buffer = [0u8; 64];
                    if data_offset + 64 > GLOBAL_DATA.len() { return; }
                    eth_buffer.copy_from_slice(&GLOBAL_DATA[data_offset..data_offset + 64]);
                    data_offset += 64;
                    let eth_packet = _unwrap_option(pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_buffer));
                    println!("{:?}", eth_packet.to_immutable().get_source());
                }
            }
        }

        println!("{:?}", gre_packet.to_immutable());
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