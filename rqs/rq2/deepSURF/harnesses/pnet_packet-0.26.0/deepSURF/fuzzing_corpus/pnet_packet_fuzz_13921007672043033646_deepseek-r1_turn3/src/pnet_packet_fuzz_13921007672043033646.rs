#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
        let mut t_1 = vec![0u8; 32];
        for i in 0..32 {
            t_1[i] = _to_u8(GLOBAL_DATA, i + 1);
        }
        t_1.truncate(t_0 as usize);
        let t_34 = &mut t_1[..];
        let t_35 = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::new(t_34);
        let mut t_36 = _unwrap_option(t_35);
        let mut t_37 = &mut t_36;

        let mut options = Vec::new();
        let num_ops = _to_u8(GLOBAL_DATA, 33) % 5;
        let mut global_offset = 34;

        for _ in 0..num_ops {
            if global_offset + 2 > GLOBAL_DATA.len() { break; }

            let pkt_type = _to_u8(GLOBAL_DATA, global_offset) % 5;
            global_offset += 1;
            let buf_len = _to_u8(GLOBAL_DATA, global_offset) % 33;
            global_offset += 1;

            let buf_start = global_offset;
            let buf_end = buf_start + buf_len as usize;
            if buf_end > GLOBAL_DATA.len() { break; }
            
            let buffer = GLOBAL_DATA[buf_start..buf_end].to_vec();
            global_offset += buf_len as usize;

            match pkt_type {
                0 => {
                    let pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::owned(buffer));
                    let mut iter = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::get_options_iter(&pkt);
                    if let Some(opt) = iter.next() { options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt)); }
                }
                1 => {
                    let pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::owned(buffer));
                    let mut iter = pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::get_options_iter(&pkt);
                    if let Some(opt) = iter.next() { options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt)); }
                }
                2 => {
                    let pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::owned(buffer));
                    let mut iter = pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::get_options_iter(&pkt);
                    if let Some(opt) = iter.next() { options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt)); }
                }
                3 => {
                    let pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(buffer));
                    let mut iter = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::get_options_iter(&pkt);
                    if let Some(opt) = iter.next() { options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt)); }
                }
                4 => {
                    let pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableRedirectPacket::owned(buffer));
                    let mut iter = pnet_packet::icmpv6::ndp::MutableRedirectPacket::get_options_iter(&pkt);
                    if let Some(opt) = iter.next() { options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt)); }
                }
                _ => unreachable!()
            }
        }

        let opt_len = _to_u8(GLOBAL_DATA, global_offset) % 33;
        options.truncate(opt_len as usize);
        let t_1416 = &options[..];
        t_37.set_options(t_1416);
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