#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 540 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;

        for _ in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset = (offset + 1) % 128;

            match op_selector {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let mut vec_data = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_data.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset = (offset + vec_len as usize) % 128;
                    
                    let t = pnet_packet::icmp::echo_request::MutableEchoRequestPacket::owned(vec_data);
                    if let Some(mut packet) = t {
                        let src_len = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let mut src_data = Vec::with_capacity(src_len as usize);
                        for i in 0..src_len {
                            src_data.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                        }
                        offset = (offset + src_len as usize) % 128;
                        
                        let src_packet = pnet_packet::icmp::echo_request::EchoRequestPacket::new(&src_data[..]).unwrap();
                        packet.populate(&src_packet.from_packet());
                        println!("{:?}", packet.to_immutable());
                    }
                },
                1 => {
                    let buf_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let mut buf = vec![0; buf_size as usize];
                    let slice = &mut buf[..];
                    
                    let t = pnet_packet::icmpv6::MutableIcmpv6Packet::new(slice);
                    if let Some(mut packet) = t {
                        let payload_size = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let payload_data = &GLOBAL_DATA[offset..offset + payload_size as usize];
                        packet.set_payload(payload_data);
                        let icmp_struct = pnet_packet::icmpv6::Icmpv6Packet::new(payload_data).unwrap();
                        packet.populate(&icmp_struct.from_packet());
                    }
                    offset = (offset + buf_size as usize) % 128;
                },
                2 => {
                    let mut vec_data = Vec::with_capacity(64);
                    for i in 0..64 {
                        vec_data.push(_to_u8(GLOBAL_DATA, offset + i));
                    }
                    offset = (offset + 64) % 128;
                    
                    let t = pnet_packet::gre::MutableGrePacket::owned(vec_data);
                    if let Some(packet) = t {
                        let immut = packet.consume_to_immutable();
                        println!("{:?}", immut);
                    }
                },
                3 => {
                    let buf_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let mut buf = vec![0; buf_size as usize];
                    let slice = &mut buf[..];
                    
                    let t = pnet_packet::ipv4::MutableIpv4Packet::new(slice);
                    if let Some(mut packet) = t {
                        let options_data = &GLOBAL_DATA[offset..offset + 16];
                        let ipv4_packet = pnet_packet::ipv4::Ipv4Packet::new(options_data).unwrap();
                        let ipv4 = ipv4_packet.from_packet();
                        packet.populate(&ipv4);
                    }
                    offset = (offset + buf_size as usize) % 128;
                },
                4 => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let mut vec_data = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_data.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset = (offset + vec_len as usize) % 128;
                    
                    let t = pnet_packet::udp::MutableUdpPacket::owned(vec_data);
                    if let Some(mut packet) = t {
                        let payload_data = &GLOBAL_DATA[offset..offset + 32];
                        packet.set_payload(payload_data);
                        println!("{:?}", packet.to_immutable());
                    }
                    offset = (offset + 32) % 128;
                },
                5 => {
                    let buf_size = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset = (offset + 1) % 128;
                    let mut buf = vec![0; buf_size as usize];
                    let slice = &mut buf[..];
                    
                    let t = pnet_packet::icmp::MutableIcmpPacket::new(slice);
                    if let Some(mut packet) = t {
                        let payload_size = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset = (offset + 1) % 128;
                        let payload_data = &GLOBAL_DATA[offset..offset + payload_size as usize];
                        offset = (offset + payload_size as usize) % 128;
                        let icmp_packet = pnet_packet::icmp::IcmpPacket::new(payload_data).unwrap();
                        packet.populate(&icmp_packet.from_packet());
                        println!("{:?}", packet.consume_to_immutable());
                    }
                    offset = (offset + buf_size as usize) % 128;
                },
                _ => {}
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