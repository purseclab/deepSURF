#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let operations = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut data_offset = 1;

        for _ in 0..operations {
            if data_offset + 4 >= GLOBAL_DATA.len() { break; }
            let op_type = _to_u8(GLOBAL_DATA, data_offset) % 5;
            data_offset += 1;

            match op_type {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    let t_redirect = pnet_packet::icmpv6::ndp::MutableRedirectPacket::owned(t_vec);
                    let mut t_redirect = _unwrap_option(t_redirect);
                    let payload_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut p_vec = Vec::with_capacity(payload_len as usize);
                    for _ in 0..payload_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        p_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    t_redirect.set_payload(&p_vec);
                    let _ = t_redirect.consume_to_immutable();
                },
                1 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    let t_router = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::owned(t_vec);
                    let mut t_router = _unwrap_option(t_router);
                    t_router.set_payload(&[_to_u8(GLOBAL_DATA, data_offset)]);
                    data_offset += 1;
                    let _ = t_router.consume_to_immutable();
                },
                2 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    let t_neighbor = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(t_vec);
                    let mut t_neighbor = _unwrap_option(t_neighbor);
                    let payload_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut p_vec = Vec::with_capacity(payload_len as usize);
                    for _ in 0..payload_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        p_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    t_neighbor.set_payload(&p_vec);
                    let _ = t_neighbor.consume_to_immutable();
                },
                3 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    let t_icmp = pnet_packet::icmpv6::MutableIcmpv6Packet::owned(t_vec);
                    let mut t_icmp = _unwrap_option(t_icmp);
                    t_icmp.set_payload(&[_to_u8(GLOBAL_DATA, data_offset)]);
                    data_offset += 1;
                    let _ = t_icmp.consume_to_immutable();
                },
                4 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset += 1;
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset += 1;
                    }
                    
                    let t_udp = pnet_packet::udp::MutableUdpPacket::owned(t_vec);
                    let mut t_udp = _unwrap_option(t_udp);
                    t_udp.set_payload(&[_to_u8(GLOBAL_DATA, data_offset)]);
                    data_offset += 1;
                    let _ = t_udp.consume_to_immutable();
                },
                _ => ()
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