#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 264 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut t_0 = _to_u8(GLOBAL_DATA, 0) % 33;
        let mut t_1 = Vec::with_capacity(32);
        for i in 1..33 {
            t_1.push(_to_u8(GLOBAL_DATA, i));
        }
        t_1.truncate(t_0 as usize);

        let constructor_choice = _to_u8(GLOBAL_DATA, 66) % 2;
        let t_34 = if constructor_choice == 0 {
            pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::owned(t_1)
        } else {
            let slice = &mut t_1[..];
            pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::new(slice)
        };
        let mut t_35 = _unwrap_option(t_34);
        let mut t_36 = &mut t_35;

        let mut t_37 = _to_u8(GLOBAL_DATA, 33) % 33;
        let mut t_38 = Vec::with_capacity(32);
        for i in 34..66 {
            t_38.push(_to_u8(GLOBAL_DATA, i));
        }
        t_38.truncate(t_37 as usize);
        let t_71 = &t_38[..];

        let ndp_len = _to_u8(GLOBAL_DATA, 67) % 33;
        let mut ndp_data = Vec::with_capacity(32);
        for i in 68..100 {
            ndp_data.push(_to_u8(GLOBAL_DATA, i));
        }
        ndp_data.truncate(ndp_len as usize);
        let mut ndp_pkt = _unwrap_option(pnet_packet::icmpv6::ndp::MutableNdpOptionPacket::owned(ndp_data));
        let ndp_imm = ndp_pkt.to_immutable();
        let ndp_option = pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&ndp_imm);
        t_36.set_options(&[ndp_option]);

        let ra_len = _to_u8(GLOBAL_DATA, 101) % 33;
        let mut ra_data = Vec::with_capacity(32);
        for i in 102..134 {
            ra_data.push(_to_u8(GLOBAL_DATA, i));
        }
        ra_data.truncate(ra_len as usize);
        let ra_pkt = _unwrap_option(pnet_packet::icmpv6::ndp::RouterAdvertPacket::owned(ra_data));
        let ra_struct = pnet_packet::icmpv6::ndp::MutableRouterAdvertPacket::from_packet(t_36);
        t_36.populate(&ra_struct);

        let mut icmpv6_data = Vec::with_capacity(32);
        for i in 135..167 {
            icmpv6_data.push(_to_u8(GLOBAL_DATA, i));
        }
        let mut icmpv6_pkt = _unwrap_option(pnet_packet::icmpv6::MutableIcmpv6Packet::owned(icmpv6_data));
        icmpv6_pkt.set_payload(t_71);

        t_36.set_payload(t_71);

        println!("{:?}", t_36);
        println!("{:?}", ndp_pkt);
        println!("{:?}", icmpv6_pkt);
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