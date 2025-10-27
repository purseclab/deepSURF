#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA: &[u8] = global_data.first_half;
        let mut idx = 0;
        
        let num_ops = _to_u8(GLOBAL_DATA, idx);
        idx += 1;
        
        for _ in 0..(num_ops % 16) {
            match _to_u8(GLOBAL_DATA, idx) % 7 {
                0 => {
                    idx += 1;
                    let size = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    let mut v = Vec::new();
                    for i in 0..size {
                        v.push(_to_u8(GLOBAL_DATA, idx + i));
                    }
                    idx += size;
                    let ipv6_pkt = pnet_packet::ipv6::MutableIpv6Packet::owned(v);
                    if let Some(mut pkt) = ipv6_pkt {
                        pkt.set_payload(&GLOBAL_DATA[idx..idx+128]);
                        idx += 128;
                        let imm_pkt = pkt.to_immutable();
                    }
                }
                1 => {
                    idx += 1;
                    let mut v = Vec::new();
                    let size = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    for i in 0..size {
                        v.push(_to_u8(GLOBAL_DATA, idx + i));
                    }
                    idx += size;
                    if let Some(mut frag) = pnet_packet::ipv6::MutableFragmentPacket::owned(v) {
                        frag.set_payload(&GLOBAL_DATA[idx..idx+64]);
                        idx += 64;
                        let imm_frag = frag.to_immutable();
                        let _ = pnet_packet::ipv6::FragmentPacket::from_packet(&imm_frag);
                    }
                }
                2 => {
                    idx += 1;
                    let mut buffer = [0u8; 64];
                    buffer.copy_from_slice(&GLOBAL_DATA[idx..idx+64]);
                    if let Some(pkt) = pnet_packet::icmp::MutableIcmpPacket::new(&mut buffer) {
                        idx += 64;
                        let _ = pkt.consume_to_immutable();
                    }
                }
                3 => {
                    idx += 1;
                    let iter = pnet_packet::ipv6::ExtensionIterable::new(&GLOBAL_DATA[idx..idx+128]);
                    idx += 128;
                    let mut iter_mut = iter;
                    while let Some(pkt) = iter_mut.next() {
                        let _ = pkt.to_immutable();
                    }
                }
                4 => {
                    idx += 1;
                    let mut buffer = [0u8; 64];
                    buffer.copy_from_slice(&GLOBAL_DATA[idx..idx+64]);
                    if let Some(pkt) = pnet_packet::ipv4::MutableIpv4Packet::new(&mut buffer) {
                        idx += 64;
                        let _ = pkt.to_immutable();
                    }
                }
                5 => {
                    idx += 1;
                    let mut v = Vec::new();
                    let size = _to_u8(GLOBAL_DATA, idx) as usize % 65;
                    idx += 1;
                    for i in 0..size {
                        v.push(_to_u8(GLOBAL_DATA, idx + i));
                    }
                    idx += size;
                    let udp_pkt = pnet_packet::udp::MutableUdpPacket::owned(v);
                    if let Some(pkt) = udp_pkt {
                        let _ = pkt.to_immutable();
                    }
                }
                _ => {
                    let mut t0 = _to_u8(GLOBAL_DATA, idx) % 33;
                    idx += 1;
                    let mut v1 = (0..32).map(|i| _to_u8(GLOBAL_DATA, idx + i)).collect::<Vec<_>>();
                    idx += 32;
                    v1.truncate(t0 as usize);
                    if let Some(mut ext) = pnet_packet::ipv6::MutableExtensionPacket::owned(v1) {
                        let mut t1 = _to_u8(GLOBAL_DATA, idx) % 33;
                        idx += 1;
                        let mut v2 = (0..32).map(|i| _to_u8(GLOBAL_DATA, idx + i)).collect::<Vec<_>>();
                        idx += 32;
                        v2.truncate(t1 as usize);
                        if let Some(src) = pnet_packet::ipv6::MutableExtensionPacket::new(&mut v2) {
                            let extension = pnet_packet::ipv6::MutableExtensionPacket::from_packet(&src);
                            ext.populate(&extension);
                        }
                    }
                }
            }
        }
        
        let mut t_0 = _to_u8(GLOBAL_DATA, idx) % 33;
        idx += 1;
        let mut t_1 = (0..32).map(|i| _to_u8(GLOBAL_DATA, idx + i)).collect::<Vec<_>>();
        idx += 32;
        t_1.truncate(t_0 as usize);
        let t_34 = pnet_packet::ipv6::MutableExtensionPacket::owned(t_1);
        let mut t_35 = _unwrap_option(t_34);
        let mut t_36 = &mut t_35;

        let mut t_37 = _to_u8(GLOBAL_DATA, idx) % 33;
        idx += 1;
        let mut t_38 = (0..32).map(|i| _to_u8(GLOBAL_DATA, idx + i)).collect::<Vec<_>>();
        idx += 32;
        t_38.truncate(t_37 as usize);
        let t_71 = &mut t_38[..];
        let t_72 = pnet_packet::ipv6::MutableExtensionPacket::new(t_71);
        let t_73 = _unwrap_option(t_72);
        let t_74 = &t_73;
        let t_75 = pnet_packet::ipv6::MutableExtensionPacket::from_packet(t_74);
        let t_76 = &t_75;
        t_36.populate(t_76);

        let mut buffer = [0u8; 64];
        buffer.copy_from_slice(&GLOBAL_DATA[idx..idx+64]);
        if let Some(pkt) = pnet_packet::ethernet::MutableEthernetPacket::new(&mut buffer) {
            let _ = pkt.to_immutable();
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