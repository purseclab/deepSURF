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
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        
        for i in 0..=op_count {
            let mut vec_len = _to_u8(GLOBAL_DATA, i as usize * 4) % 65;
            let start = i as usize * 16;
            let end = start + vec_len as usize;
            
            if end >= GLOBAL_DATA.len() { break; }
            
            match i % 4 {
                0 => {
                    let buffer = GLOBAL_DATA[start..end].to_vec();
                    let frag_packet = pnet_packet::ipv6::MutableFragmentPacket::owned(buffer);
                    let mut frag = _unwrap_option(frag_packet);
                    let src = pnet_packet::ipv6::FragmentPacket::from_packet(&frag.to_immutable());
                    frag.populate(&src);
                }
                1 => {
                    let mut buffer = GLOBAL_DATA[start..end].to_vec();
                    let mut ipv6 = _unwrap_option(pnet_packet::ipv6::MutableIpv6Packet::new(&mut buffer));
                    let payload = &GLOBAL_DATA[end..end+8];
                    ipv6.set_payload(payload);
                    let immut_ipv6 = ipv6.to_immutable();
                    println!("{:?}", immut_ipv6);
                }
                2 => {
                    let buffer = GLOBAL_DATA[start..end].to_vec();
                    let tcp = pnet_packet::tcp::MutableTcpPacket::owned(buffer);
                    if let Some(mut tcp) = tcp {
                        let opt_buffer = &GLOBAL_DATA[end..end+4];
                        tcp.set_options(&[pnet_packet::tcp::TcpOption::nop(), pnet_packet::tcp::TcpOption::wscale(opt_buffer[0])]);
                    }
                }
                3 => {
                    let mut buffer = GLOBAL_DATA[start..end].to_vec();
                    let udp = pnet_packet::udp::MutableUdpPacket::new(&mut buffer);
                    if let Some(mut udp) = udp {
                        let checksum = _to_u16(GLOBAL_DATA, end);
                        udp.set_checksum(checksum);
                        let immut_udp = udp.consume_to_immutable();
                        println!("{:?}", immut_udp);
                    }
                }
                _ => {
                    let buffer = GLOBAL_DATA[start..end].to_vec();
                    let mut frag = _unwrap_option(pnet_packet::ipv6::MutableFragmentPacket::owned(buffer));
                    let payload = &GLOBAL_DATA[end..end+16];
                    frag.set_payload(payload);
                }
            }
        }
        
        let t_0 = _to_u8(GLOBAL_DATA, 128) % 65;
        let mut t_1 = vec![0u8; t_0 as usize];
        t_1.copy_from_slice(&GLOBAL_DATA[129..129 + t_0 as usize]);
        let mut t_35 = _unwrap_option(pnet_packet::ipv6::MutableFragmentPacket::owned(t_1));
        let t_74 = pnet_packet::ipv6::MutableFragmentPacket::consume_to_immutable(t_35);
        let t_76 = pnet_packet::ipv6::FragmentPacket::from_packet(&t_74);
        
        let mut eth_buffer = GLOBAL_DATA[200..300].to_vec();
        let mut eth = _unwrap_option(pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_buffer));
        eth.set_payload(&GLOBAL_DATA[300..400]);
        let immut_eth = eth.to_immutable();
        println!("{:?}", immut_eth);
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