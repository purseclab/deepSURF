#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use pnet_packet::*;
use global_data::*;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_u8(GLOBAL_DATA, 0) % 8 + 2;
        for i in 0..ops_count {
            let op_type = _to_u8(GLOBAL_DATA, 1 + i as usize) % 6;
            match op_type {
                0 => {
                    let mut vec1 = (0..32).map(|j| _to_u8(GLOBAL_DATA, 32 * i as usize + j)).collect::<Vec<_>>();
                    vec1.truncate(_to_u8(GLOBAL_DATA, 256) as usize % 32);
                    let t_0 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(vec1.clone());
                    let mut t_1 = _unwrap_option(t_0);
                    let t_2 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::new(&mut vec1[..]);
                    let t_3 = _unwrap_option(t_2);
                    let t_4 = t_3.consume_to_immutable();
                    println!("{:?}", &t_4);
                }
                1 => {
                    let mut vec2 = (0..64).map(|j| _to_u8(GLOBAL_DATA, 64 * i as usize + j)).collect::<Vec<_>>();
                    vec2.truncate(_to_u8(GLOBAL_DATA, 257) as usize % 64);
                    let t_5 = pnet_packet::icmpv6::ndp::MutableRouterSolicitPacket::new(&mut vec2[..]);
                    let t_6 = _unwrap_option(t_5);
                    let t_7 = t_6.consume_to_immutable();
                    let t_8 = pnet_packet::icmpv6::ndp::RouterSolicitPacket::from_packet(&t_7);
                    println!("{:?}", t_8);
                }
                2 => {
                    let mut vec3 = (0..48).map(|j| _to_u8(GLOBAL_DATA, 48 * i as usize + j)).collect::<Vec<_>>();
                    vec3.truncate(_to_u8(GLOBAL_DATA, 258) as usize % 48);
                    let t_9 = pnet_packet::icmpv6::ndp::MutableRedirectPacket::owned(vec3.clone());
                    let mut t_10 = _unwrap_option(t_9);
                    let t_11 = pnet_packet::icmpv6::ndp::MutableRedirectPacket::new(&mut vec3[..]);
                    let t_12 = _unwrap_option(t_11);
                    let t_12_imm = t_12.consume_to_immutable();
                    let t_13 = pnet_packet::icmpv6::ndp::RedirectPacket::from_packet(&t_12_imm);
                    println!("{:?}", t_13);
                }
                3 => {
                    let vec4 = (0..128).map(|j| _to_u8(GLOBAL_DATA, 128 * i as usize + j)).collect::<Vec<_>>();
                    let t_14 = pnet_packet::icmpv6::ndp::NeighborAdvertPacket::owned(vec4);
                    let t_15 = _unwrap_option(t_14);
                    let t_16 = &t_15;
                    let t_17 = pnet_packet::icmpv6::ndp::NeighborAdvertPacket::from_packet(t_16);
                    println!("{:?}", t_17);
                }
                4 => {
                    let mut vec5 = (0..96).map(|j| _to_u8(GLOBAL_DATA, 96 * i as usize + j)).collect::<Vec<_>>();
                    vec5.truncate(_to_u8(GLOBAL_DATA, 259) as usize % 96);
                    let t_18 = pnet_packet::icmpv6::ndp::MutableNdpOptionPacket::new(&mut vec5[..]);
                    let t_19 = _unwrap_option(t_18);
                    let t_19_imm = t_19.consume_to_immutable();
                    let t_20 = pnet_packet::icmpv6::ndp::NdpOptionPacket::from_packet(&t_19_imm);
                    println!("{:?}", t_20);
                }
                _ => {
                    let mut vec_na = (0..132).map(|j| _to_u8(GLOBAL_DATA, 132 * i as usize + j)).collect::<Vec<_>>();
                    let t_21 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(vec_na.clone());
                    let mut t_22 = _unwrap_option(t_21);
                    let t_23 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::new(&mut vec_na[..]);
                    let t_24 = _unwrap_option(t_23);
                    let t_25 = t_24.consume_to_immutable();
                    let t_26 = &t_25;
                    let t_27 = pnet_packet::icmpv6::ndp::NeighborAdvertPacket::from_packet(t_26);
                    println!("{:?}", t_27);
                    t_22.populate(&t_27);
                }
            }
        }

        let mut vec_final = (0..132).map(|j| _to_u8(GLOBAL_DATA, j)).collect::<Vec<_>>();
        let t_28 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::owned(vec_final.clone());
        let mut t_29 = _unwrap_option(t_28);
        let t_30 = pnet_packet::icmpv6::ndp::MutableNeighborAdvertPacket::new(&mut vec_final[..]);
        let t_31 = _unwrap_option(t_30);
        let t_32 = t_31.consume_to_immutable();
        let t_33 = &t_32;
        let t_34 = pnet_packet::icmpv6::ndp::NeighborAdvertPacket::from_packet(t_33);
        t_29.populate(&t_34);
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