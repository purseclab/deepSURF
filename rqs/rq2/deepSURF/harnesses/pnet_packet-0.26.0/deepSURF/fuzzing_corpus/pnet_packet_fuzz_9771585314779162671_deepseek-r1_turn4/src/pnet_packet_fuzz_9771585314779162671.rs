#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use pnet_packet::ipv4::MutableIpv4Packet;
use pnet_packet::ipv6::MutableIpv6Packet;
use pnet_packet::icmp::destination_unreachable::MutableDestinationUnreachablePacket;
use pnet_packet::udp::MutableUdpPacket;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let operations = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut packets = Vec::new();

        for i in 0..operations {
            match _to_u8(GLOBAL_DATA, i as usize + 1) % 6 {
                0 => {
                    let mut vec = (0..=_to_u8(GLOBAL_DATA, 10) % 65).map(|j| _to_u8(GLOBAL_DATA, j as usize + 20)).collect();
                    let pkt = MutableDestinationUnreachablePacket::owned(vec);
                    if let Some(mut p) = pkt {
                        p.set_payload(&GLOBAL_DATA[50..100]);
                        println!("{:?}", p.to_immutable());
                        packets.push(p);
                    }
                },
                1 => {
                    let buf = &mut [0u8; 150][..];
                    let mut pkt = MutableIpv4Packet::new(buf).unwrap();
                    pkt.set_payload(&GLOBAL_DATA[100..150]);
                    pkt.populate(&pnet_packet::ipv4::Ipv4Packet::from_packet(&pkt.to_immutable()));
                    println!("{:?}", pkt.to_immutable());
                },
                2 => {
                    let mut vec = (0..=_to_u8(GLOBAL_DATA, 20) % 65).map(|j| _to_u8(GLOBAL_DATA, j as usize + 30)).collect();
                    let pkt = MutableUdpPacket::owned(vec);
                    if let Some(mut p) = pkt {
                        p.set_payload(&GLOBAL_DATA[200..250]);
                        println!("{:?}", p.consume_to_immutable());
                    }
                },
                3 => {
                    let buf = &mut [0u8; 200][..];
                    let mut pkt = MutableIpv6Packet::new(buf).unwrap();
                    pkt.set_payload(&GLOBAL_DATA[300..350]);
                    let _ = pkt.populate(&pnet_packet::ipv6::Ipv6Packet::from_packet(&pkt.to_immutable()));
                },
                4 => {
                    let mut vec = (0..=_to_u8(GLOBAL_DATA, 40) % 65).map(|j| _to_u8(GLOBAL_DATA, j as usize + 50)).collect();
                    let mut pkt = MutableDestinationUnreachablePacket::owned(vec).unwrap();
                    pkt.set_payload(&GLOBAL_DATA[400..450]);
                    packets.push(pkt);
                },
                _ => {
                    let mut vec = (0..=_to_u8(GLOBAL_DATA, 60) % 65).map(|j| _to_u8(GLOBAL_DATA, j as usize + 70)).collect();
                    let pkt = MutableDestinationUnreachablePacket::owned(vec);
                    if let Some(p) = pkt {
                        packets.push(p);
                    }
                }
            }
        }

        if let Some(mut last_pkt) = packets.pop() {
            last_pkt.set_payload(&GLOBAL_DATA[500..]);
            println!("{:?}", last_pkt.to_immutable());
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