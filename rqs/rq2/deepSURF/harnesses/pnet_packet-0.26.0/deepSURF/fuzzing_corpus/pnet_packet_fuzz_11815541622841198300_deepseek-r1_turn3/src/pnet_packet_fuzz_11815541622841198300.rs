#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut options: Vec<pnet_packet::icmpv6::ndp::NdpOption> = Vec::new();
        let mut offset = 34;
        let num_ops = _to_u8(GLOBAL_DATA, 33) % 10;

        for _ in 0..num_ops {
            if offset + 33 >= GLOBAL_DATA.len() { break; }

            let mut vec_data = Vec::with_capacity(32);
            for i in 0..32 {
                vec_data.push(_to_u8(GLOBAL_DATA, offset + i));
            }
            let vec_size = _to_u8(GLOBAL_DATA, offset + 32) % 33;
            vec_data.truncate(vec_size as usize);
            offset += 33;

            match _to_u8(GLOBAL_DATA, offset - 33) % 4 {
                0 => {
                    let buffer = &mut vec_data[..];
                    if let Some(mut pkt) = pnet_packet::icmpv6::ndp::MutableRedirectPacket::new(buffer) {
                        if let Some(opt) = pkt.get_options_iter().next() {
                            options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt));
                        }
                    }
                }
                1 => {
                    if let Some(pkt) = pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::owned(vec_data) {
                        let mut iter = pkt.get_options_iter();
                        if let Some(opt) = pnet_packet::icmpv6::ndp::NdpOptionIterable::next(&mut iter) {
                            options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt));
                        }
                        let _ = pkt.consume_to_immutable();
                    }
                }
                2 => {
                    if let Some(pkt) = pnet_packet::icmpv6::ndp::MutableNeighborSolicitPacket::owned(vec_data) {
                        let mut iter = pkt.get_options_iter();
                        if let Some(opt) = pnet_packet::icmpv6::ndp::NdpOptionIterable::next(&mut iter) {
                            options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt));
                        }
                    }
                }
                3 => {
                    if let Some(pkt) = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(vec_data) {
                        let mut iter = pkt.get_options_iter();
                        if let Some(opt) = iter.next() {
                            options.push(pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&opt));
                        }
                    }
                }
                _ => ()
            }
        }

        let mut packet_buffer = Vec::with_capacity(32);
        for i in 0..32 {
            packet_buffer.push(_to_u8(GLOBAL_DATA, i));
        }
        let t_size = _to_u8(GLOBAL_DATA, 0) % 33;
        packet_buffer.truncate(t_size as usize);

        if let Some(mut redirect_pkt) = pnet_packet::icmpv6::ndp::MutableRedirectPacket::new(&mut packet_buffer[..]) {
            redirect_pkt.set_options(&options);
            let _ = redirect_pkt.consume_to_immutable();
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