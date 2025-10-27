#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut ops_iter = GLOBAL_DATA.iter().cycle();
        let op_count = _to_u8(GLOBAL_DATA, 0) % 32;
        
        for _ in 0..=op_count {
            match ops_iter.next().unwrap() % 9 {
                0 => {
                    let mut eth_vec = Vec::with_capacity(32);
                    for i in 0..32 {
                        eth_vec.push(_to_u8(GLOBAL_DATA, i % GLOBAL_DATA.len()));
                    }
                    let mut eth_pkt = pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_vec).unwrap();
                    eth_pkt.set_payload(&GLOBAL_DATA[32..64]);
                    let immut_eth = eth_pkt.to_immutable();
                    println!("{:?}", immut_eth.packet());
                }
                1 => {
                    let mut ipv4_buf = vec![0u8; _to_usize(GLOBAL_DATA, 64) % 128];
                    let mut ipv4_pkt = pnet_packet::ipv4::MutableIpv4Packet::new(&mut ipv4_buf).unwrap();
                    ipv4_pkt.set_payload(&GLOBAL_DATA[128..160]);
                    let immut_ipv4 = ipv4_pkt.consume_to_immutable();
                    println!("{:?}", immut_ipv4.packet());
                }
                2 => {
                    let udp_data = &GLOBAL_DATA[160..224];
                    let mut udp_pkt = pnet_packet::udp::MutableUdpPacket::owned(udp_data.to_vec()).unwrap();
                    udp_pkt.populate(&pnet_packet::udp::UdpPacket::from_packet(&udp_pkt.to_immutable()));
                    println!("{:?}", udp_pkt.packet());
                }
                3 => {
                    let mut gre_buf = vec![0u8; _to_u8(GLOBAL_DATA, 224) as usize];
                    let mut gre_pkt = pnet_packet::gre::MutableGrePacket::new(&mut gre_buf).unwrap();
                    gre_pkt.set_routing(&GLOBAL_DATA[256..288]);
                    let immut_gre = gre_pkt.consume_to_immutable();
                    println!("{:?}", immut_gre.packet());
                }
                4 => {
                    let mut tcp_opt_buf = vec![0u8; 40];
                    let mut tcp_opt = pnet_packet::tcp::MutableTcpOptionPacket::new(&mut tcp_opt_buf).unwrap();
                    tcp_opt.set_data(&GLOBAL_DATA[288..352]);
                    let opt_pkt = tcp_opt.consume_to_immutable();
                    println!("{:?}", opt_pkt.packet());
                }
                5 => {
                    let mut vlan_buf = vec![0u8; 48];
                    let mut vlan_pkt = pnet_packet::vlan::MutableVlanPacket::new(&mut vlan_buf).unwrap();
                    vlan_pkt.set_payload(&GLOBAL_DATA[352..416]);
                    let immut_vlan = vlan_pkt.to_immutable();
                    println!("{:?}", immut_vlan.packet());
                }
                6 => {
                    let mut icmp_buf = vec![0u8; 56];
                    let mut icmp_pkt = pnet_packet::icmp::MutableIcmpPacket::new(&mut icmp_buf).unwrap();
                    icmp_pkt.populate(&pnet_packet::icmp::IcmpPacket::from_packet(&icmp_pkt.to_immutable()));
                    println!("{:?}", icmp_pkt.packet());
                }
                7 => {
                    let mut fragment_buf = GLOBAL_DATA[416..480].to_vec();
                    let mut frag_pkt = pnet_packet::ipv6::MutableFragmentPacket::new(&mut fragment_buf).unwrap();
                    frag_pkt.set_payload(&GLOBAL_DATA[480..544]);
                    let immut_frag = frag_pkt.consume_to_immutable();
                    println!("{:?}", immut_frag.packet());
                }
                _ => {
                    let mut arp_data = GLOBAL_DATA[544..608].to_vec();
                    let mut arp_pkt = pnet_packet::arp::MutableArpPacket::new(&mut arp_data).unwrap();
                    arp_pkt.set_payload(&GLOBAL_DATA[608..672]);
                    let immut_arp = arp_pkt.to_immutable();
                    println!("{:?}", immut_arp.packet());
                }
            }
        }
        
        let mut t_1 = Vec::with_capacity(32);
        for i in 672..704 {
            t_1.push(_to_u8(GLOBAL_DATA, i % GLOBAL_DATA.len()));
        }
        let mut gre_pkt = pnet_packet::gre::MutableGrePacket::owned(t_1).unwrap();
        gre_pkt.set_routing(&GLOBAL_DATA[704..768]);
        let immut_gre = gre_pkt.consume_to_immutable();
        println!("{:?}", immut_gre.packet());
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