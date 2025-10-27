#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let op_count = _to_u8(GLOBAL_DATA, 0) % 16;
        let mut current_idx = 1;

        for _ in 0..op_count {
            if current_idx >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, current_idx) % 6;
            current_idx += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    let mut buf = vec![0; len as usize];
                    for i in 0..len as usize {
                        if current_idx + i >= GLOBAL_DATA.len() { break; }
                        buf[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                    }
                    current_idx += len as usize;

                    if let Some(mut gre) = pnet_packet::gre::MutableGrePacket::new(&mut buf) {
                        let src_len = _to_u8(GLOBAL_DATA, current_idx) % 65;
                        current_idx += 1;
                        let mut src_buf = vec![0; src_len as usize];
                        for i in 0..src_len as usize {
                            if current_idx + i >= GLOBAL_DATA.len() { break; }
                            src_buf[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                        }
                        current_idx += src_len as usize;

                        if let Some(src) = pnet_packet::gre::GrePacket::new(&src_buf) {
                            gre.populate(&pnet_packet::gre::GrePacket::from_packet(&src));
                            println!("{:?}", gre);
                        }
                    }
                },
                1 => {
                    let buf_size = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    let mut buffer = vec![0; buf_size as usize];
                    for i in 0..buf_size as usize {
                        if current_idx + i >= GLOBAL_DATA.len() { break; }
                        buffer[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                    }
                    current_idx += buf_size as usize;

                    if let Some(mut eth) = pnet_packet::ethernet::MutableEthernetPacket::new(&mut buffer) {
                        eth.set_payload(&GLOBAL_DATA[current_idx..current_idx+8]);
                        current_idx += 8;
                        println!("{:?}", eth);
                    }
                },
                2 => {
                    let mut tcp_buf = vec![0; 20];
                    if let Some(mut tcp) = pnet_packet::tcp::MutableTcpPacket::new(&mut tcp_buf) {
                        let mss_value = _to_u16(GLOBAL_DATA, current_idx);
                        current_idx += 2;
                        tcp.set_options(&[pnet_packet::tcp::TcpOption::mss(mss_value)]);
                        current_idx += 2;
                        println!("{:?}", tcp);
                    }
                },
                3 => {
                    let icmp_len = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    let mut icmp_buf = vec![0; icmp_len as usize];
                    for i in 0..icmp_len as usize {
                        if current_idx + i >= GLOBAL_DATA.len() { break; }
                        icmp_buf[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                    }
                    current_idx += icmp_len as usize;

                    if let Some(mut icmp) = pnet_packet::icmp::MutableIcmpPacket::new(&mut icmp_buf) {
                        icmp.populate(&pnet_packet::icmp::IcmpPacket::from_packet(
                            &pnet_packet::icmp::IcmpPacket::new(&[GLOBAL_DATA[current_idx]]).unwrap()
                        ));
                        current_idx += 1;
                        println!("{:?}", icmp);
                    }
                },
                4 => {
                    let header_size = _to_u8(GLOBAL_DATA, current_idx) % 65;
                    current_idx += 1;
                    let mut hdr_buf = vec![0; header_size as usize];
                    for i in 0..header_size as usize {
                        hdr_buf[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                    }
                    current_idx += header_size as usize;

                    if let Some(mut u16be) = pnet_packet::gre::MutableU16BEPacket::new(&mut hdr_buf) {
                        let payload_size = _to_u8(GLOBAL_DATA, current_idx) % 65;
                        current_idx += 1;
                        let payload = &GLOBAL_DATA[current_idx..current_idx+payload_size as usize];
                        current_idx += payload_size as usize;
                        if let Some(src) = pnet_packet::gre::U16BEPacket::new(payload) {
                            u16be.populate(&pnet_packet::gre::U16BEPacket::from_packet(&src));
                            println!("{:?}", u16be);
                        }
                    }
                },
                _ => {
                    let arp_size = _to_u8(GLOBAL_DATA, current_idx) % 32;
                    current_idx += 1;
                    let mut arp_buf = vec![0; arp_size as usize];
                    for i in 0..arp_size as usize {
                        arp_buf[i] = _to_u8(GLOBAL_DATA, current_idx + i);
                    }
                    current_idx += arp_size as usize;

                    if let Some(mut arp) = pnet_packet::arp::MutableArpPacket::new(&mut arp_buf) {
                        arp.populate(&pnet_packet::arp::ArpPacket::from_packet(
                            &pnet_packet::arp::ArpPacket::new(&[GLOBAL_DATA[current_idx]]).unwrap()
                        ));
                        current_idx += 1;
                        println!("{:?}", arp);
                    }
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