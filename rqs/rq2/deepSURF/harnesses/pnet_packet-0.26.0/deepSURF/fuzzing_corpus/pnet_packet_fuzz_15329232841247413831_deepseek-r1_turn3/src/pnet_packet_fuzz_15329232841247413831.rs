#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 10;
        let mut offset = 1;

        for _ in 0..num_ops {
            if offset + 2 > GLOBAL_DATA.len() { break; }
            let op_type = _to_u8(GLOBAL_DATA, offset) % 4;
            offset += 1;
            
            match op_type {
                0 => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut vec_buf = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_buf.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset += vec_len as usize;
                    let icmp_pkt = pnet_packet::icmp::destination_unreachable::MutableDestinationUnreachablePacket::owned(vec_buf);
                    if let Some(mut p) = icmp_pkt {
                        let src_len = _to_u8(GLOBAL_DATA, offset) % 65;
                        offset += 1;
                        let mut src_vec = Vec::with_capacity(src_len as usize);
                        for i in 0..src_len {
                            src_vec.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                        }
                        offset += src_len as usize;
                        let src_pkt = pnet_packet::icmp::destination_unreachable::MutableDestinationUnreachablePacket::owned(src_vec);
                        if let Some(mut s) = src_pkt {
                            let src_struct = s.from_packet();
                            p.populate(&src_struct);
                            println!("{:?}", p.packet());
                        }
                    }
                }
                1 => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut vec_buf = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_buf.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset += vec_len as usize;
                    let gre_pkt = pnet_packet::gre::MutableGrePacket::owned(vec_buf);
                    if let Some(mut p) = gre_pkt {
                        let src = {
                            let immut_p = p.to_immutable();
                            pnet_packet::gre::GrePacket::from_packet(&immut_p)
                        };
                        p.populate(&src);
                        println!("{:?}", p.to_immutable().get_checksum());
                    }
                }
                2 => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut vec_buf = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_buf.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset += vec_len as usize;
                    let mut icmpv6_pkt = pnet_packet::icmpv6::MutableIcmpv6Packet::owned(vec_buf);
                    if let Some(mut p) = icmpv6_pkt.as_mut() {
                        let src_pkt = pnet_packet::icmpv6::Icmpv6Packet::new(p.packet()).unwrap();
                        let src = pnet_packet::icmpv6::Icmpv6Packet::from_packet(&src_pkt);
                        p.populate(&src);
                        println!("{:?}", p.to_immutable());
                    }
                }
                _ => {
                    let vec_len = _to_u8(GLOBAL_DATA, offset) % 65;
                    offset += 1;
                    let mut vec_buf = Vec::with_capacity(vec_len as usize);
                    for i in 0..vec_len {
                        vec_buf.push(_to_u8(GLOBAL_DATA, offset + i as usize));
                    }
                    offset += vec_len as usize;
                    let ipv4_pkt = pnet_packet::ipv4::MutableIpv4Packet::owned(vec_buf);
                    if let Some(mut p) = ipv4_pkt {
                        p.set_payload(&[GLOBAL_DATA[offset % GLOBAL_DATA.len()]]);
                        println!("{:?}", p.get_options_iter().next());
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