#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use algorithmica::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct CustomType0(String);

impl std::fmt::Debug for CustomType0 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        Ok(())
    }
}

fn _custom_fn0(str0: &CustomType0, str1: &CustomType0) -> std::cmp::Ordering {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.first_half;
    let custom_impl_num = _to_usize(GLOBAL_DATA, 553);
    let custom_impl_inst_num = str0.0.len();
    let selector = (custom_impl_num + custom_impl_inst_num) % 3;
    if selector == 0 {
        panic!("INTENTIONAL PANIC!");
    }
    let GLOBAL_DATA = match selector {
        1 => global_data.first_half,
        _ => global_data.second_half,
    };
    match (_to_usize(GLOBAL_DATA, 561) % 3usize) {
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
        _ => std::cmp::Ordering::Less,
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let custom_vec_len = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut custom_vec = Vec::with_capacity(custom_vec_len);
        for i in 0..custom_vec_len {
            let start = 1 + i * 17;
            let len = _to_u8(GLOBAL_DATA, start) % 17;
            let s = _to_str(GLOBAL_DATA, start + 1, start + 1 + len as usize);
            custom_vec.push(CustomType0(s.to_string()));
        }

        let mut primitive_vec = Vec::new();
        let primitive_count = _to_u8(GLOBAL_DATA, 1000) % 65;
        for i in 0..primitive_count as usize {
            let val = _to_i32(GLOBAL_DATA, 1001 + i * 4);
            primitive_vec.push(val);
        }

        let mut bst = if _to_u8(GLOBAL_DATA, 2000) % 2 == 0 {
            algorithmica::tree::bst::BST::new()
        } else {
            let val = _to_i32(GLOBAL_DATA, 2001);
            algorithmica::tree::bst::BST::create(val)
        };

        let operation_count = _to_u8(GLOBAL_DATA, 2005) % 20;
        for op_idx in 0..operation_count {
            let op_selector = _to_u8(GLOBAL_DATA, 2010 + op_idx as usize) % 7;
            match op_selector {
                0 => {
                    algorithmica::sort::merge_sort::sort_by(&mut custom_vec, &_custom_fn0);
                    let _ = algorithmica::sort::is_sorted::is_sorted_by(&custom_vec, |a, b| _custom_fn0(a, b).is_le());
                }
                1 => {
                    let val = _to_i32(GLOBAL_DATA, 2100 + op_idx as usize * 4);
                    bst.insert(val);
                    println!("BST: {:?}", bst);
                }
                2 => {
                    algorithmica::sort::bubble::sort(&mut primitive_vec);
                    let _ = algorithmica::search::binary::search(&primitive_vec, &_to_i32(GLOBAL_DATA, 3000));
                }
                3 => {
                    let subsets = algorithmica::subset::find_all_subset(&primitive_vec);
                    println!("Subset count: {}", subsets.len());
                }
                4 => {
                    algorithmica::sort::merge_sort::sort_by(&mut custom_vec, &_custom_fn0);
                }
                5 => {
                    let _ = algorithmica::sort::is_sorted::is_sorted_desc(&primitive_vec);
                }
                6 => {
                    let mut temp_vec = primitive_vec.clone();
                    algorithmica::sort::insertion::sort(&mut temp_vec);
                }
                _ => unreachable!(),
            }
        }

        let slice = &mut custom_vec[..];
        algorithmica::sort::merge_sort::sort_by(slice, &_custom_fn0);
        println!("Final state: {:?}", slice);
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