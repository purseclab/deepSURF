#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;

#[derive(Debug, Copy)]
struct CustomType1(usize);

impl Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        CustomType1(_to_usize(GLOBAL_DATA, 18))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();

        let mut vec = Vec::with_capacity(32);
        for i in (0..256).step_by(8) {
            vec.push(CustomType1(_to_usize(global_data.first_half, i)));
        }
        vec.truncate((_to_u8(global_data.first_half, 9) % 33) as usize);

        let mut sv1 = SmallVec::<[CustomType1; 32]>::from_slice(&vec[..]);
        let mut sv2 = SmallVec::<[CustomType1; 32]>::from_vec(vec.clone());
        let mut sv3 = SmallVec::<[CustomType1; 32]>::with_capacity(_to_usize(global_data.second_half, 0));
        let mut sv4 = smallvec::ToSmallVec::<[CustomType1; 32]>::to_smallvec(vec.as_slice());

        for _ in 0.._to_u8(global_data.second_half, 8) % 16 {
            match _to_u8(global_data.second_half, 16) % 6 {
                0 => sv1.push(CustomType1(_to_u64(global_data.first_half, 64) as usize)),
                1 => {
                    sv2.insert(
                        _to_usize(global_data.second_half, 24),
                        CustomType1(_to_usize(global_data.second_half, 32))
                    );
                }
                2 => {
                    sv3.extend_from_slice(sv4.as_slice());
                }
                3 => {
                    sv4 = SmallVec::from_elem(
                        CustomType1(_to_usize(global_data.second_half, 40)),
                        _to_u16(global_data.second_half, 48) as usize % 65
                    );
                }
                4 => {
                    let _ = sv1.pop();
                    let _ = sv2.remove(_to_usize(global_data.second_half, 56) % sv2.len());
                }
                _ => sv3.truncate(_to_u16(global_data.second_half, 64) as usize % 65),
            };

            let borrowed1 = sv1.as_slice();
            let borrowed2 = sv2.as_slice();
            println!("{:?} {:?}", borrowed1, borrowed2);
        }

        let mut combined = SmallVec::<[CustomType1; 32]>::new();
        combined.append(&mut sv3);
        combined.append(&mut sv4);

        combined.shrink_to_fit();
        println!("{:?}", combined.as_ptr());
        sv1.as_mut_slice();
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