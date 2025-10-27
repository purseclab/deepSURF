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

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        for i in 0..=op_count {
            let selector = _to_u8(GLOBAL_DATA, i as usize * 4) % 4;
            match selector {
                0 => {
                    let mut t_0 = _to_u8(GLOBAL_DATA, 1) % 65;
                    let mut vec_buf = Vec::with_capacity(t_0 as usize);
                    for j in 0..64 {
                        vec_buf.push(_to_u8(GLOBAL_DATA, j));
                    }
                    vec_buf.truncate(t_0 as usize);
                    
                    let mut pkt = pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::owned(vec_buf);
                    let mut t_35 = _unwrap_option(pkt);
                    let imm_pkt = t_35.to_immutable();
                    let ns_pkt = pnet_packet::icmpv6::ndp::NeighborSolicitPacket::from_packet(&imm_pkt);
                    pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::populate(&mut t_35, &ns_pkt);
                    
                    let mut opt_data = [GLOBAL_DATA[100]];
                    let mut opt_pkt = pnet_packet::icmpv6::ndp::MutableNdpOptionPacket::new(&mut opt_data).unwrap();
                    pnet_packet::icmpv6::ndp::MutableNdpOptionPacket::set_data(&mut opt_pkt, &[GLOBAL_DATA[101]]);
                    
                    let mut opts = vec![];
                    let opt_pkt_immutable = opt_pkt.to_immutable();
                    opts.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt_pkt_immutable));
                    pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::set_options(&mut t_35, &opts);
                    
                    let payload_len = _to_u8(GLOBAL_DATA, 200) % 65;
                    let payload_data = &GLOBAL_DATA[200..200 + payload_len as usize];
                    t_35.set_payload(payload_data);
                    
                    println!("{:?}", t_35.get_options_iter().next().unwrap());
                },
                1 => {
                    let mut rs_data = vec![GLOBAL_DATA[150]];
                    let rtr_sol = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::owned(rs_data).unwrap();
                    let converted = rtr_sol.consume_to_immutable();
                    println!("{:?}", converted.get_options_iter().next());
                },
                2 => {
                    let mut tcp_data = [GLOBAL_DATA[300]];
                    let mut tcp_opt = pnet_packet::tcp::MutableTcpOptionPacket::new(&mut tcp_data).unwrap();
                    pnet_packet::tcp::MutableTcpOptionPacket::set_data(&mut tcp_opt, &[GLOBAL_DATA[301]]);
                    let _ = tcp_opt.consume_to_immutable();
                },
                3 => {
                    let mut udp_buffer = [GLOBAL_DATA[400]; 8];
                    let mut udp_pkt = pnet_packet::udp::MutableUdpPacket::new(&mut udp_buffer).unwrap();
                    udp_pkt.set_payload(&[GLOBAL_DATA[401]]);
                    let _immut = udp_pkt.consume_to_immutable();
                },
                _ => {
                    let mut frag_data = [GLOBAL_DATA[500]; 16];
                    let mut frag_pkt = pnet_packet::ipv6::MutableFragmentPacket::new(&mut frag_data).unwrap();
                    frag_pkt.set_payload(&[GLOBAL_DATA[501]]);
                    let _frag = frag_pkt.to_immutable();
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