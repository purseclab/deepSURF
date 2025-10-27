#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 10;
        let mut data_offset = 1;

        for _ in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, data_offset);
            data_offset = data_offset.wrapping_add(1);

            match selector % 6 {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset = data_offset.wrapping_add(1);
                    let mut t_vec = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        t_vec.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let eth = pnet_packet::ethernet::MutableEthernetPacket::owned(t_vec);
                    if let Some(mut eth_pkt) = eth {
                        println!("{:?}", eth_pkt);
                        eth_pkt.set_ethertype(pnet_packet::ethernet::EtherType(_to_u16(GLOBAL_DATA, data_offset)));
                        data_offset = data_offset.wrapping_add(2);
                    }
                }
                1 => {
                    let ip_vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset = data_offset.wrapping_add(1);
                    let mut ip_data = Vec::with_capacity(ip_vec_len as usize);
                    for _ in 0..ip_vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        ip_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let mut ip_pkt = pnet_packet::ipv4::MutableIpv4Packet::owned(ip_data);
                    if let Some(ref mut pkt) = ip_pkt {
                        pkt.set_header_length(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                        println!("{:?}", pkt);
                    }
                }
                2 => {
                    let mut gre_data = Vec::with_capacity(32);
                    for _ in 0..32 {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        gre_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let gre = pnet_packet::gre::MutableGrePacket::owned(gre_data);
                    if let Some(mut gre_pkt) = gre {
                        let key_val = _to_u32(GLOBAL_DATA, data_offset);
                        data_offset = data_offset.wrapping_add(4);
                        let key_bytes = vec![
                            (key_val >> 24) as u8,
                            (key_val >> 16) as u8,
                            (key_val >> 8) as u8,
                            key_val as u8
                        ];
                        if let Some(u32be_pkt) = pnet_packet::gre::MutableU32BEPacket::owned(key_bytes) {
                            let u32be = pnet_packet::gre::MutableU32BEPacket::from_packet(&u32be_pkt);
                            gre_pkt.set_key(&[u32be]);
                        }
                        println!("{:?}", gre_pkt);
                    }
                }
                3 => {
                    let mut ndp_data = Vec::with_capacity(48);
                    for _ in 0..48 {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        ndp_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let ndp = pnet_packet::icmpv6::ndp::MutableNdpOptionPacket::new(&mut ndp_data);
                    if let Some(mut ndp_pkt) = ndp {
                        ndp_pkt.set_data(&[_to_u8(GLOBAL_DATA, data_offset)]);
                        data_offset = data_offset.wrapping_add(1);
                    }
                }
                4 => {
                    let tcp_opt_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset = data_offset.wrapping_add(1);
                    let mut tcp_data = Vec::with_capacity(tcp_opt_len as usize);
                    for _ in 0..tcp_opt_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        tcp_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let tcp_opt = pnet_packet::tcp::MutableTcpOptionPacket::new(&mut tcp_data);
                    if let Some(mut opt) = tcp_opt {
                        opt.set_length(&[_to_u8(GLOBAL_DATA, data_offset)]);
                        data_offset = data_offset.wrapping_add(1);
                        println!("{:?}", opt);
                    }
                }
                _ => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset) % 65;
                    data_offset = data_offset.wrapping_add(1);
                    let mut vlan_data = Vec::with_capacity(vec_len as usize);
                    for _ in 0..vec_len {
                        if data_offset >= GLOBAL_DATA.len() { break; }
                        vlan_data.push(_to_u8(GLOBAL_DATA, data_offset));
                        data_offset = data_offset.wrapping_add(1);
                    }
                    let vlan = pnet_packet::vlan::MutableVlanPacket::owned(vlan_data);
                    if let Some(vlan_pkt) = vlan {
                        println!("{:?}", vlan_pkt);
                    }
                }
            }
        }

        let mut gre_vec = Vec::with_capacity(2048);
        for i in 0..2048 {
            if data_offset + i >= GLOBAL_DATA.len() { break; }
            gre_vec.push(_to_u8(GLOBAL_DATA, data_offset + i));
        }
        let mut gre_pkt = _unwrap_option(pnet_packet::gre::MutableGrePacket::owned(gre_vec));
        let key_data: Vec<_> = (0..1024).map(|i| _to_u8(GLOBAL_DATA, data_offset + i)).collect();
        let u32be_pkts: Vec<_> = key_data.chunks(4)
            .filter_map(|c| pnet_packet::gre::MutableU32BEPacket::owned(c.to_vec()))
            .map(|p| pnet_packet::gre::MutableU32BEPacket::from_packet(&p))
            .collect();
        gre_pkt.set_key(&u32be_pkts);
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