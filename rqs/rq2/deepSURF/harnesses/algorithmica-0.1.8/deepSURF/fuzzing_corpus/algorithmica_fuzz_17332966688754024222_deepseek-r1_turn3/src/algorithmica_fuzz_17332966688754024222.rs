#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use algorithmica::*;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Clone)]
struct CustomType0(String);

impl std::cmp::Ord for CustomType0 {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 34);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_8 = _to_usize(GLOBAL_DATA, 42);
        match (t_8 % 3) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Less,
        }
    }
}

impl std::cmp::Eq for CustomType0 {}

impl std::cmp::PartialEq for CustomType0 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 9)
    }
}

impl std::fmt::Debug for CustomType0 {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 26);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        Ok(())
    }
}

impl std::cmp::PartialOrd for CustomType0 {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 10);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_3 = _to_usize(GLOBAL_DATA, 18);
        Some(match (t_3 % 3) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            _ => std::cmp::Ordering::Less,
        })
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut custom_vec = Vec::with_capacity(32);
        for _ in 0..(_to_usize(GLOBAL_DATA, 1) % 65) {
            let len = _to_u8(GLOBAL_DATA, 2) % 17;
            let s = _to_str(GLOBAL_DATA, 3, 3 + len as usize);
            custom_vec.push(CustomType0(s.to_string()));
        }

        let mut bst = tree::bst::BST::new();
        let mut matrix = vec![];
        let mut node_opt = None;

        for op_index in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 4 + op_index) % 6;
            match op_selector {
                0 => {
                    algorithmica::sort::merge_sort::sort(&mut custom_vec[..]);
                    let _ = algorithmica::sort::is_sorted::is_sorted(&custom_vec);
                }
                1 => {
                    algorithmica::sort::bubble::sort(&mut custom_vec[..]);
                    let _ = algorithmica::sort::is_sorted::is_sorted_desc(&custom_vec);
                }
                2 => {
                    let val = _to_u32(GLOBAL_DATA, 20 + op_index) as i32;
                    let new_node = algorithmica::tree::Node::create(val);
                    node_opt = Some(new_node);
                    println!("{:?}", node_opt);
                }
                3 => {
                    let mat1 = vec![
                        vec![_to_f32(GLOBAL_DATA, 50), _to_f32(GLOBAL_DATA, 54)],
                        vec![_to_f32(GLOBAL_DATA, 58), _to_f32(GLOBAL_DATA, 62)]
                    ];
                    let mat2 = vec![
                        vec![_to_f32(GLOBAL_DATA, 66), _to_f32(GLOBAL_DATA, 70)],
                        vec![_to_f32(GLOBAL_DATA, 74), _to_f32(GLOBAL_DATA, 78)]
                    ];
                    matrix = algorithmica::math::matrix::add(&mat1, &mat2);
                    let mat3 = algorithmica::math::matrix::multiply(&mat1, &mat2);
                    let mat4 = algorithmica::math::matrix::add(&matrix, &mat3);
                    matrix = mat4;
                }
                4 => {
                    if !custom_vec.is_empty() {
                        let _subsets = algorithmica::subset::find_all_subset(&custom_vec);
                    }
                }
                5 => {
                    let val_idx = 100 + op_index * 4;
                    let val_str = _to_str(GLOBAL_DATA, val_idx, val_idx + 10);
                    let custom_val = CustomType0(val_str.to_string());
                    bst.insert(custom_val);
                    let search_val = CustomType0(_to_str(GLOBAL_DATA, val_idx, val_idx + 10).to_string());
                    let _ = bst.find(search_val);
                    let _ = format!("{:?}", bst);
                }
                _ => {
                    algorithmica::sort::selection::sort(&mut custom_vec[..]);
                    let mut bst_clone = tree::bst::BST::create(CustomType0(String::new()));
                    std::mem::swap(&mut bst, &mut bst_clone);
                    let _ = algorithmica::search::binary::search(&custom_vec, &custom_vec[0]);
                }
            }
        }

        if !custom_vec.is_empty() {
            algorithmica::sort::merge_sort::sort(&mut custom_vec[..]);
            let _ = algorithmica::sort::is_sorted::is_sorted_by(&custom_vec, |a, b| a < b);
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