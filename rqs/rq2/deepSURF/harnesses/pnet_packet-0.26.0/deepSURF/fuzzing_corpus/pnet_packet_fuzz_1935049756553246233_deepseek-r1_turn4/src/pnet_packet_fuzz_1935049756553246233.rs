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
        
        let mut ptr = 0;
        let num_ops = _to_u8(GLOBAL_DATA, ptr) % 10;
        ptr += 1;

        for _ in 0..num_ops {
            let op_type = _to_u8(GLOBAL_DATA, ptr) % 6;
            ptr += 1;

            match op_type {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, ptr) % 65;
                    ptr += 1;
                    let mut vec_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        vec_data.push(_to_u8(GLOBAL_DATA, ptr));
                        ptr += 1;
                    }
                    let t = pnet_packet::ipv6::MutableFragmentPacket::owned(vec_data);
                    if let Some(mut frag) = t {
                        frag.set_payload(&GLOBAL_DATA[ptr..ptr+10]);
                        ptr += 10;
                        let imm_frag = frag.consume_to_immutable();
                        println!("{:?}", imm_frag.payload());
                    }
                }
                1 => {
                    let slice_start = _to_usize(GLOBAL_DATA, ptr) % GLOBAL_DATA.len();
                    ptr += 4;
                    let slice_end = _to_usize(GLOBAL_DATA, ptr) % GLOBAL_DATA.len();
                    ptr += 4;
                    let data_slice = &GLOBAL_DATA[slice_start..slice_end];
                    let mut buffer = data_slice.to_vec();
                    let t = pnet_packet::ipv6::MutableIpv6Packet::new(&mut buffer);
                    if let Some(mut ipv6) = t {
                        ipv6.set_payload(&GLOBAL_DATA[ptr..ptr+20]);
                        ptr += 20;
                        let imm_ipv6 = ipv6.consume_to_immutable();
                        let mut route_buffer = imm_ipv6.payload().to_vec();
                        let t2 = pnet_packet::ipv6::MutableRoutingPacket::new(&mut route_buffer);
                        if let Some(mut route) = t2 {
                            route.set_data(&GLOBAL_DATA[ptr..ptr+30]);
                            ptr += 30;
                        }
                    }
                }
                2 => {
                    let mut buffer = vec![0u8; 128];
                    let t = pnet_packet::ipv6::MutableRoutingPacket::owned(buffer);
                    if let Some(mut route) = t {
                        let mut buffer = &mut GLOBAL_DATA[ptr..ptr+64].to_vec();
                        let t2 = pnet_packet::ipv6::MutableRoutingPacket::new(buffer);
                        if let Some(src_mut) = t2 {
                            let routing = src_mut.from_packet();
                            route.populate(&routing);
                            println!("{:?}", route.payload());
                        }
                        ptr += 64;
                    }
                }
                3 => {
                    let vec_len = _to_u8(GLOBAL_DATA, ptr) % 65;
                    ptr += 1;
                    let mut vec_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        vec_data.push(_to_u8(GLOBAL_DATA, ptr));
                        ptr += 1;
                    }
                    let t = pnet_packet::ipv6::Ipv6Packet::owned(vec_data);
                    if let Some(ipv6) = t {
                        let mut buffer = vec![0u8; 128];
                        let mut fragment = pnet_packet::ipv6::MutableFragmentPacket::new(&mut buffer).unwrap();
                        fragment.set_payload(ipv6.payload());
                    }
                }
                4 => {
                    let mut buffer = vec![0u8; 64];
                    let t = pnet_packet::ipv6::MutableRoutingPacket::new(&mut buffer);
                    if let Some(mut route) = t {
                        let vec_len = _to_u8(GLOBAL_DATA, ptr) % 65;
                        ptr += 1;
                        let mut vec_data = Vec::with_capacity(vec_len as usize);
                        for _ in 0..vec_len {
                            vec_data.push(_to_u8(GLOBAL_DATA, ptr));
                            ptr += 1;
                        }
                        let src_mut = pnet_packet::ipv6::MutableRoutingPacket::owned(vec_data).unwrap();
                        let routing = src_mut.from_packet();
                        route.populate(&routing);
                    }
                }
                _ => {
                    let mut vec1 = Vec::new();
                    for _ in 0..32 {
                        vec1.push(_to_u8(GLOBAL_DATA, ptr));
                        ptr += 1;
                    }
                    let mut vec2 = Vec::new();
                    for _ in 0..32 {
                        vec2.push(_to_u8(GLOBAL_DATA, ptr));
                        ptr += 1;
                    }
                    let mut route1 = pnet_packet::ipv6::MutableRoutingPacket::owned(vec1).unwrap();
                    let mut route2 = pnet_packet::ipv6::MutableRoutingPacket::new(&mut vec2).unwrap();
                    let imm1 = route1.consume_to_immutable();
                    let mut buffer = imm1.payload().to_vec();
                    let src_mut = pnet_packet::ipv6::MutableRoutingPacket::new(&mut buffer).unwrap();
                    let routing = src_mut.from_packet();
                    route2.populate(&routing);
                }
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