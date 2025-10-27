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

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;

        for _ in 0..num_ops {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, offset) % 6;
            offset += 1;

            match op_selector {
                0 => {
                    if offset + 66 > GLOBAL_DATA.len() { continue; }
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut buffer = vec![0; vec_len as usize];
                    buffer.copy_from_slice(&GLOBAL_DATA[offset..offset + vec_len as usize]);
                    offset += vec_len as usize;

                    let t_option = pnet_packet::ipv4::MutableIpv4OptionPacket::owned(buffer);
                    let mut t_option = _unwrap_option(t_option);
                    println!("{:?}", pnet_packet::ipv4::MutableIpv4OptionPacket::from_packet(&t_option));
                }
                1 => {
                    let ipv4_payload = &GLOBAL_DATA[offset..offset + 20];
                    offset += 20;
                    let mut mutable_buffer = ipv4_payload.to_vec();
                    let ipv4_packet = pnet_packet::ipv4::MutableIpv4Packet::new(&mut mutable_buffer);
                    let ipv4_packet = _unwrap_option(ipv4_packet);
                    let ipv4 = pnet_packet::ipv4::MutableIpv4Packet::from_packet(&ipv4_packet);
                    println!("{:?}", ipv4);
                }
                2 => {
                    let buffer = &mut GLOBAL_DATA[offset..offset + 48].to_vec();
                    offset += 48;
                    let mut tcp_packet = pnet_packet::tcp::MutableTcpPacket::new(buffer.as_mut_slice());
                    let mut tcp_packet = _unwrap_option(tcp_packet);
                    let options = pnet_packet::tcp::TcpOption::nop();
                    tcp_packet.set_options(&[options]);
                }
                3 => {
                    let mut buffer = vec![0; 32];
                    buffer.copy_from_slice(&GLOBAL_DATA[offset..offset + 32]);
                    offset += 32;
                    let mut udp_packet = pnet_packet::udp::MutableUdpPacket::owned(buffer);
                    let udp_packet = _unwrap_option(udp_packet);
                    println!("{:?}", pnet_packet::udp::MutableUdpPacket::from_packet(&udp_packet));
                }
                4 => {
                    let mut opt_buffer = vec![0; 32];
                    opt_buffer.copy_from_slice(&GLOBAL_DATA[offset..offset + 32]);
                    offset += 32;
                    let mut mutable_opt = pnet_packet::ipv4::MutableIpv4OptionPacket::owned(opt_buffer);
                    let mut mutable_opt = _unwrap_option(mutable_opt);
                    
                    let source_data = &GLOBAL_DATA[offset..offset + 16];
                    offset += 16;
                    let src_packet = pnet_packet::ipv4::Ipv4OptionPacket::new(source_data);
                    let src_packet = _unwrap_option(src_packet);
                    let src_option = pnet_packet::ipv4::Ipv4OptionPacket::from_packet(&src_packet);
                    mutable_opt.populate(&src_option);
                }
                5 => {
                    let eth_data = &GLOBAL_DATA[offset..offset + 14];
                    offset += 14;
                    let eth_packet = pnet_packet::ethernet::EthernetPacket::new(eth_data);
                    let eth_packet = _unwrap_option(eth_packet);
                    let eth = pnet_packet::ethernet::EthernetPacket::from_packet(&eth_packet);
                    let mut eth_buffer = [0; 14];
                    let mut mutable_eth = pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_buffer).unwrap();
                    mutable_eth.populate(&eth);
                }
                _ => continue
            }
        }

        let mut t_0 = _to_u8(GLOBAL_DATA, offset) % 33;
        offset += 1;
        let mut t_1: Vec<u8> = GLOBAL_DATA[offset..offset + 32].to_vec();
        t_1.truncate(t_0 as usize);
        offset += 32;

        let t_34 = &mut t_1[..];
        let mut t_36 = _unwrap_option(pnet_packet::ipv4::MutableIpv4OptionPacket::new(t_34));

        let t_38 = _to_u8(GLOBAL_DATA, offset) % 33;
        offset += 1;
        let mut t_39: Vec<u8> = GLOBAL_DATA[offset..offset + 32].to_vec();
        t_39.truncate(t_38 as usize);
        offset += 32;

        let t_72 = pnet_packet::ipv4::MutableIpv4OptionPacket::owned(t_39);
        let t_73 = _unwrap_option(t_72);
        let t_74 = pnet_packet::ipv4::MutableIpv4OptionPacket::consume_to_immutable(t_73);
        let t_76 = pnet_packet::ipv4::Ipv4OptionPacket::from_packet(&t_74);
        t_36.populate(&t_76);
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