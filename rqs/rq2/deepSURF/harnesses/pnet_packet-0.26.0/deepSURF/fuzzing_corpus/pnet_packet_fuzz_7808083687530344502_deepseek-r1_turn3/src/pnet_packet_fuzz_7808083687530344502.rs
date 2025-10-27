#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        
        for i in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 1 + i as usize) % 6;
            match op_selector {
                0 => {
                    let mut t_1 = Vec::with_capacity(32);
                    for j in 0..32 {
                        t_1.push(_to_u8(GLOBAL_DATA, 2 + j + (i as usize)*32));
                    }
                    let t_0 = _to_u8(GLOBAL_DATA, 1 + (i as usize)*32) % 33;
                    t_1.truncate(t_0 as usize);
                    let t_34 = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::owned(t_1);
                    let mut t_35 = _unwrap_option(t_34);
                    
                    let slice_len = _to_u8(GLOBAL_DATA, 34 + (i as usize)*32) % 33;
                    let start = 35 + (i as usize)*32;
                    let end = start + slice_len as usize;
                    let mut t_71_data = GLOBAL_DATA[start..end].to_vec();
                    let t_72 = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::new(&mut t_71_data);
                    let t_73 = _unwrap_option(t_72);
                    let t_74 = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::consume_to_immutable(t_73);
                    let t_75 = &t_74;
                    let t_76 = pnet_packet::icmpv6::ndp::RouterSolicitPacket::from_packet(t_75);
                    let t_77 = &t_76;
                    t_35.populate(t_77);
                    println!("{:?}", t_35.packet());
                }
                1 => {
                    let vec_len = _to_u8(GLOBAL_DATA, 100 + (i as usize)*16) % 33;
                    let mut vec_data = Vec::new();
                    for j in 0..vec_len {
                        vec_data.push(_to_u8(GLOBAL_DATA, 101 + (j as usize) + (i as usize)*16));
                    }
                    let mut packet = pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::owned(vec_data).unwrap();
                    let src_slice = &GLOBAL_DATA[150 + (i as usize)*16..150 + (i as usize)*16 + 32];
                    let src_packet = pnet_packet::icmpv6::ndp::NeighborSolicitPacket::from_packet(
                        &pnet_packet::icmpv6::ndp::NeighborSolicitPacket::new(src_slice).unwrap()
                    );
                    packet.populate(&src_packet);
                    let _ = packet.consume_to_immutable();
                }
                2 => {
                    let slice = &GLOBAL_DATA[200 + (i as usize)*32..200 + (i as usize)*32 + 64];
                    let data = slice.to_vec();
                    let mut ipv6_packet = pnet_packet::ipv6::MutableIpv6Packet::owned(data).unwrap();
                    ipv6_packet.set_payload(&[ _to_u8(GLOBAL_DATA, 300 + i as usize) ]);
                    let _ = ipv6_packet.consume_to_immutable();
                }
                3 => {
                    let vec_data: Vec<u8> = (0..32).map(|j| _to_u8(GLOBAL_DATA, 400 + j + (i as usize)*32)).collect();
                    let mut udp_packet = pnet_packet::udp::MutableUdpPacket::owned(vec_data).unwrap();
                    udp_packet.set_payload(&GLOBAL_DATA[500..512]);
                    println!("{:?}", udp_packet.get_source());
                    let _immut = udp_packet.consume_to_immutable();
                }
                4 => {
                    let data = &GLOBAL_DATA[600 + i as usize..];
                    if let Some(packet) = pnet_packet::icmpv6::ndp::RouterSolicitPacket::new(data) {
                        let mut opts_iter = packet.get_options_iter();
                        while let Some(opt) = opts_iter.next() {
                            println!("{:?}", opt);
                        }
                    }
                }
                5 => {
                    let tcp_data = GLOBAL_DATA[700..720].to_vec();
                    let mut tcp_packet = pnet_packet::tcp::MutableTcpPacket::owned(tcp_data).unwrap();
                    tcp_packet.set_options(&[
                        pnet_packet::tcp::TcpOption::nop(),
                        pnet_packet::tcp::TcpOption::mss(_to_u16(GLOBAL_DATA, 720))
                    ]);
                    let _ = tcp_packet.consume_to_immutable();
                }
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