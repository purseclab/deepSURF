#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType2(String);
struct CustomType0(String);
#[derive(Debug, Default, Clone)]
struct CustomType1(String);

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 24);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 32) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 33, 33 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType1(t_6);
        Some(t_7)
    }
}

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_9 = _to_u8(GLOBAL_DATA, 57) % 17;
        let t_10 = _to_str(GLOBAL_DATA, 58, 58 + t_9 as usize);
        let t_11 = String::from(t_10);
        CustomType2(t_11)
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 8);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 16)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;
        
        let constructor_sel = _to_u8(g, 0) % 3;
        let mut toodee = match constructor_sel {
            0 => TooDee::with_capacity(_to_usize(g, 1)),
            1 => {
                let cols = _to_usize(g, 9);
                let rows = _to_usize(g, 17) % 65;
                TooDee::new(cols, rows)
            }
            _ => {
                let data_len = _to_usize(g, 25) % 65;
                let mut vec = Vec::with_capacity(data_len);
                for i in 0..data_len {
                    let s_len = _to_u8(g, 33 + i) % 17;
                    let s = _to_str(g, 34 + i, 34 + i + s_len as usize);
                    vec.push(CustomType1(s.to_string()));
                }
                TooDee::from_vec(2, 2, vec)
            }
        };
        
        let num_ops = _to_usize(g, 50) % 8;
        let mut data_ptr = 60;
        
        for _ in 0..num_ops {
            let op_sel = _to_u8(g, data_ptr) % 7;
            data_ptr += 1;
            
            match op_sel {
                0 => {
                    let row_data = _to_str(g, data_ptr, data_ptr + 10);
                    data_ptr += 10;
                    toodee.push_row(CustomType0(row_data.to_string()));
                }
                1 => {
                    let idx = _to_usize(g, data_ptr);
                    data_ptr += 2;
                    let len = _to_usize(g, data_ptr) % 65;
                    data_ptr += 2;
                    let items: Vec<CustomType1> = (0..len).map(|i| {
                        let s = _to_str(g, data_ptr + i, data_ptr + i + 1);
                        CustomType1(s.to_string())
                    }).collect();
                    toodee.insert_col(idx, items.into_iter());
                }
                2 => {
                    let idx = _to_usize(g, data_ptr);
                    data_ptr += 2;
                    let _ = toodee.remove_col(idx);
                }
                3 => {
                    if toodee.num_rows() > 0 {
                        let _ = toodee.pop_col();
                    }
                }
                4 => {
                    let r1 = _to_usize(g, data_ptr);
                    data_ptr += 4;
                    let r2 = _to_usize(g, data_ptr);
                    data_ptr += 4;
                    toodee.swap_rows(r1, r2);
                    println!("{:?}", toodee.data());
                }
                5 => {
                    let view = toodee.view((0, 0), (toodee.num_cols(), toodee.num_rows()));
                    for row in view.rows() {
                        println!("{:?}", row);
                    }
                }
                6 => {
                    let mut view_mut = toodee.view_mut((0,0), (toodee.num_cols(), toodee.num_rows()));
                    let col_idx = _to_usize(g, data_ptr);
                    data_ptr += 2;
                    let col = view_mut.col_mut(col_idx);
                    for (i, item) in col.enumerate() {
                        println!("{}: {:?}", i, item);
                    }
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