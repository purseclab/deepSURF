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
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 16;
        for i in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, 1 + i as usize) % 5;
            
            match op_type {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, 128 + i as usize) % 65;
                    let mut payload = Vec::with_capacity(vec_len as usize);
                    for j in 0..vec_len {
                        payload.push(_to_u8(GLOBAL_DATA, 256 + j as usize));
                    }
                    
                    let mut ipv6_pkt = pnet_packet::ipv6::MutableIpv6Packet::owned(payload).unwrap();
                    let payload_len = _to_u16(GLOBAL_DATA, 64) as usize;
                    ipv6_pkt.set_payload(&GLOBAL_DATA[128..128+payload_len]);
                }
                1 => {
                    let frag_len = _to_u8(GLOBAL_DATA, 384) % 65;
                    let mut frag_data = vec![0; frag_len as usize];
                    let mut frag_pkt = pnet_packet::ipv6::MutableFragmentPacket::owned(frag_data).unwrap();
                    frag_pkt.set_payload(&GLOBAL_DATA[448..512]);
                    let immut_frag = frag_pkt.to_immutable();
                    println!("Fragment {:?}", immut_frag);
                }
                2 => {
                    let mut routing_data = Vec::new();
                    for j in 0..64 {
                        routing_data.push(_to_u8(GLOBAL_DATA, 512 + j as usize));
                    }
                    let mut routing_pkt = pnet_packet::ipv6::MutableRoutingPacket::owned(routing_data).unwrap();
                    routing_pkt.set_data(&GLOBAL_DATA[576..640]);
                    let typed_routing = pnet_packet::ipv6::MutableRoutingPacket::from_packet(&mut routing_pkt);
                }
                3 => {
                    let ext_len = _to_u8(GLOBAL_DATA, 640) % 65;
                    let mut ext_pkt = pnet_packet::ipv6::MutableExtensionPacket::owned(vec![0; ext_len as usize]).unwrap();
                    ext_pkt.set_options(&GLOBAL_DATA[704..768]);
                    let _ = ext_pkt.consume_to_immutable();
                }
                4 => {
                    let mut udp_payload = Vec::new();
                    for j in 0..32 {
                        udp_payload.push(_to_u8(GLOBAL_DATA, 768 + j as usize));
                    }
                    let mut udp_pkt = pnet_packet::udp::MutableUdpPacket::owned(udp_payload).unwrap();
                    udp_pkt.set_payload(&GLOBAL_DATA[800..864]);
                }
                _ => unreachable!()
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