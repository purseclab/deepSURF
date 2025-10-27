#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_u8(GLOBAL_DATA, 0) % 8 + 1;
        for i in 0..ops_count as usize {
            match _to_u8(GLOBAL_DATA, i + 1) % 5 {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, i + 2) % 2;
                    let vec_len = _to_u8(GLOBAL_DATA, i + 3) % 65;
                    let mut buf = Vec::with_capacity(vec_len as usize);
                    for j in 0..vec_len {
                        buf.push(_to_u8(GLOBAL_DATA, i + 4 + j as usize));
                    }
                    
                    let mut opt_packet = if constructor_choice == 0 {
                        _unwrap_option(pnet_packet::ipv4::MutableIpv4OptionPacket::owned(buf))
                    } else {
                        _unwrap_option(pnet_packet::ipv4::MutableIpv4OptionPacket::new(&mut buf[..]))
                    };
                    
                    let set_len_data = &GLOBAL_DATA[i*16..(i+1)*16];
                    opt_packet.set_length(set_len_data);
                    let _ = println!("{:?}", opt_packet.get_length());

                    let opt_packet_immutable = opt_packet.to_immutable();
                    let _ipv4_opt = pnet_packet::ipv4::Ipv4OptionPacket::from_packet(&opt_packet_immutable);
                }
                1 => {
                    let mut ipv6_buf = (0..40).map(|j| _to_u8(GLOBAL_DATA, i*40 + j)).collect::<Vec<_>>();
                    let mut ipv6_pkt = _unwrap_option(pnet_packet::ipv6::MutableIpv6Packet::new(&mut ipv6_buf[..]));
                    let payload_data = &GLOBAL_DATA[i*64..(i+1)*64];
                    ipv6_pkt.set_payload(payload_data);
                    let _ = println!("{:?}", ipv6_pkt.payload().len());
                }
                2 => {
                    let mut tcp_buf = (0..24).map(|j| _to_u8(GLOBAL_DATA, i*24 + j)).collect::<Vec<_>>();
                    let mut tcp_pkt = _unwrap_option(pnet_packet::tcp::MutableTcpPacket::new(&mut tcp_buf[..]));
                    tcp_pkt.set_payload(&GLOBAL_DATA[i*48..(i+1)*48]);
                    let _ = println!("{:?}", tcp_pkt.get_options());
                }
                3 => {
                    let mut eth_buf = vec![0u8; 64];
                    let mut eth_pkt = _unwrap_option(pnet_packet::ethernet::MutableEthernetPacket::new(&mut eth_buf[..]));
                    eth_pkt.set_payload(&GLOBAL_DATA[i*32..(i+1)*32]);
                    let _ = pnet_packet::ethernet::EthernetPacket::new(eth_pkt.packet());
                }
                4 => {
                    let mut opt_buf = (0..128).map(|j| _to_u8(GLOBAL_DATA, i*128 + j)).collect::<Vec<_>>();
                    let mut tcp_opt = _unwrap_option(pnet_packet::tcp::MutableTcpOptionPacket::new(&mut opt_buf[..]));
                    tcp_opt.set_length(&GLOBAL_DATA[i*16..(i+1)*16]);
                    let _ = tcp_opt.populate(&pnet_packet::tcp::TcpOption::nop());
                }
                _ => unreachable!()
            }
        }

        let mut t_0 = _to_u8(GLOBAL_DATA, 900) % 33;
        let mut t_1 = (0..32).map(|i| _to_u8(GLOBAL_DATA, 901 + i)).collect::<Vec<_>>();
        t_1.truncate(t_0 as usize);
        let mut t_35 = _unwrap_option(pnet_packet::ipv4::MutableIpv4OptionPacket::owned(t_1));

        let t_37 = _to_u8(GLOBAL_DATA, 933) % 33;
        let t_38 = (0..32).map(|i| _to_u8(GLOBAL_DATA, 934 + i)).collect::<Vec<_>>();
        let t_71 = &t_38[..t_37 as usize];
        t_35.set_length(t_71);

        let mut ipv4_data = vec![0u8; 160];
        let mut ipv4_pkt = _unwrap_option(pnet_packet::ipv4::MutableIpv4Packet::new(&mut ipv4_data[..]));
        ipv4_pkt.set_options(&[pnet_packet::ipv4::Ipv4OptionPacket::from_packet(&t_35.to_immutable())]);
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