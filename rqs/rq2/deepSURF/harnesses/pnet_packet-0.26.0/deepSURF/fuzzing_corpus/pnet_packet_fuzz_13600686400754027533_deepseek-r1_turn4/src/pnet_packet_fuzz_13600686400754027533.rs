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

        let op_count = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut data_offset = 1;

        for _ in 0..=op_count {
            match _to_u8(GLOBAL_DATA, data_offset) % 6 {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset + 1) % 65;
                    let payload_start = data_offset + 2;
                    let payload_end = payload_start + vec_len as usize;
                    let mut payload = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len as usize {
                        payload.push(_to_u8(GLOBAL_DATA, payload_start + i));
                    }
                    data_offset = payload_end;

                    let t_ex = if _to_u8(GLOBAL_DATA, data_offset) % 2 == 0 {
                        pnet_packet::icmp::time_exceeded::MutableTimeExceededPacket::owned(payload)
                    } else {
                        pnet_packet::icmp::time_exceeded::MutableTimeExceededPacket::new(&mut payload[..])
                    };
                    let mut t_ex = _unwrap_option(t_ex);
                    t_ex.set_payload(&GLOBAL_DATA[payload_start..payload_end]);
                    let immut_t_ex = t_ex.consume_to_immutable();
                    println!("{:?}", immut_t_ex.payload());
                }
                1 => {
                    let buf_size = _to_u8(GLOBAL_DATA, data_offset + 1) % 65;
                    let mut buf = vec![0; buf_size as usize];
                    let icmp = pnet_packet::icmp::MutableIcmpPacket::new(&mut buf[..]);
                    let mut icmp = _unwrap_option(icmp);
                    icmp.set_payload(&GLOBAL_DATA[data_offset..data_offset + buf_size as usize]);
                    let immut = icmp.to_immutable();
                    println!("{:?}", immut.payload());
                    data_offset += buf_size as usize;
                }
                2 => {
                    let mut frag_buf = vec![0; 128];
                    let mut frag = pnet_packet::ipv6::MutableFragmentPacket::new(&mut frag_buf[..]);
                    let mut frag = _unwrap_option(frag);
                    frag.set_payload(&GLOBAL_DATA[data_offset..data_offset + 64]);
                    let immut_frag = frag.consume_to_immutable();
                    println!("{:?}", immut_frag.payload());
                    data_offset += 64;
                }
                3 => {
                    let tcp_opt_size = _to_u8(GLOBAL_DATA, data_offset + 1) % 65;
                    let mut tcp_opt_buf = vec![0; tcp_opt_size as usize];
                    let mut tcp_opt = pnet_packet::tcp::MutableTcpOptionPacket::new(&mut tcp_opt_buf[..]);
                    let mut tcp_opt = _unwrap_option(tcp_opt);
                    tcp_opt.set_data(&GLOBAL_DATA[data_offset..data_offset + tcp_opt_size as usize]);
                    let immut_tcp_opt = tcp_opt.consume_to_immutable();
                    println!("{:?}", immut_tcp_opt.payload());
                    data_offset += tcp_opt_size as usize;
                }
                4 => {
                    let vec_len = _to_u8(GLOBAL_DATA, data_offset + 1) % 65;
                    let mut arp_buf = vec![0; vec_len as usize];
                    let mut arp = pnet_packet::arp::MutableArpPacket::new(&mut arp_buf[..]);
                    let mut arp = _unwrap_option(arp);
                    arp.set_payload(&GLOBAL_DATA[data_offset..data_offset + vec_len as usize]);
                    let immut_arp = arp.consume_to_immutable();
                    println!("{:?}", immut_arp.payload());
                    data_offset += vec_len as usize;
                }
                _ => {
                    let mut eth_buf = vec![0; 64];
                    let mut eth = pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_buf[..]);
                    let mut eth = _unwrap_option(eth);
                    eth.set_payload(&GLOBAL_DATA[data_offset..data_offset + 32]);
                    let immut_eth = eth.consume_to_immutable();
                    println!("{:?}", immut_eth.payload());
                    data_offset += 32;
                }
            }
            data_offset += 2;
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