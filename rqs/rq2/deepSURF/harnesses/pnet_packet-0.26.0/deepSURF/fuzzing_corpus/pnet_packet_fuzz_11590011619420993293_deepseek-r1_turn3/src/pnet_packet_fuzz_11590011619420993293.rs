#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut data_idx = 1;
        
        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, data_idx) % 5;
            data_idx += 1;

            match op_type {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut eth_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        eth_data.push(_to_u8(GLOBAL_DATA, data_idx));
                        data_idx += 1;
                    }
                    let eth_pkt = pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_data);
                    let _ = _unwrap_option(eth_pkt);
                }
                1 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut ip_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        ip_data.push(_to_u8(GLOBAL_DATA, data_idx));
                        data_idx += 1;
                    }
                    let ip_pkt = pnet_packet::ipv4::MutableIpv4Packet::new(&mut ip_data);
                    let mut ip_pkt = _unwrap_option(ip_pkt);
                    let immut_ip = pnet_packet::ipv4::Ipv4Packet::from_packet(&ip_pkt.to_immutable());
                    ip_pkt.populate(&immut_ip);
                }
                2 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut tcp_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        tcp_data.push(_to_u8(GLOBAL_DATA, data_idx));
                        data_idx += 1;
                    }
                    let tcp_pkt = pnet_packet::tcp::MutableTcpPacket::new(&mut tcp_data);
                    let mut tcp_pkt = _unwrap_option(tcp_pkt);
                    tcp_pkt.set_payload(&[]);
                }
                3 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut udp_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        udp_data.push(_to_u8(GLOBAL_DATA, data_idx));
                        data_idx += 1;
                    }
                    let udp_pkt = pnet_packet::udp::MutableUdpPacket::new(&mut udp_data);
                    let udp_pkt = _unwrap_option(udp_pkt);
                    let immut_udp = pnet_packet::udp::UdpPacket::from_packet(&udp_pkt.to_immutable());
                    println!("{:?}", immut_udp);
                }
                _ => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_idx) % 65;
                    data_idx += 1;
                    let mut icmp_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        icmp_data.push(_to_u8(GLOBAL_DATA, data_idx));
                        data_idx += 1;
                    }
                    let icmp_pkt = pnet_packet::icmp::MutableIcmpPacket::new(&mut icmp_data);
                    let mut icmp_pkt = _unwrap_option(icmp_pkt);
                    icmp_pkt.populate(&pnet_packet::icmp::IcmpPacket::from_packet(&icmp_pkt.to_immutable()));
                }
            }
        }

        let mut t_0 = _to_u8(GLOBAL_DATA, data_idx) % 33;
        data_idx += 1;
        let mut t_1 = Vec::with_capacity(32);
        for _ in 0..32 {
            t_1.push(_to_u8(GLOBAL_DATA, data_idx));
            data_idx += 1;
        }
        t_1.truncate(t_0 as usize);
        let t_34 = pnet_packet::vlan::MutableVlanPacket::owned(t_1);
        let mut t_35 = _unwrap_option(t_34);
        let mut t_36 = &mut t_35;

        let mut t_37 = _to_u8(GLOBAL_DATA, data_idx) % 33;
        data_idx += 1;
        let mut t_38 = Vec::with_capacity(32);
        for _ in 0..32 {
            t_38.push(_to_u8(GLOBAL_DATA, data_idx));
            data_idx += 1;
        }
        t_38.truncate(t_37 as usize);
        let t_71 = &mut t_38[..];
        let t_72 = pnet_packet::vlan::MutableVlanPacket::new(t_71);
        let t_73 = _unwrap_option(t_72);
        let t_74 = pnet_packet::vlan::MutableVlanPacket::consume_to_immutable(t_73);
        let t_75 = &t_74;
        let t_76 = pnet_packet::vlan::VlanPacket::from_packet(t_75);
        let t_77 = &t_76;
        t_36.populate(t_77);
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