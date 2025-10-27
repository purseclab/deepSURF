#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);
struct CustomType1(String);
struct CustomType2(String);

impl core::iter::IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 660);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_151 = _to_u8(GLOBAL_DATA, 668) % 17;
        let t_152 = _to_str(GLOBAL_DATA, 669, 669 + t_151 as usize);
        let t_153 = String::from(t_152);
        let t_154 = CustomType2(t_153);
        return t_154;
    }
}

impl core::iter::DoubleEndedIterator for CustomType2 {
    
    fn next_back(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 635);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_146 = _to_u8(GLOBAL_DATA, 643) % 17;
        let t_147 = _to_str(GLOBAL_DATA, 644, 644 + t_146 as usize);
        let t_148 = String::from(t_147);
        let t_149 = CustomType0(t_148);
        let t_150 = Some(t_149);
        return t_150;
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType0;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 569);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_135 = _to_u8(GLOBAL_DATA, 577) % 17;
        let t_136 = _to_str(GLOBAL_DATA, 578, 578 + t_135 as usize);
        let t_137 = String::from(t_136);
        let t_138 = CustomType0(t_137);
        let t_139 = Some(t_138);
        return t_139;
    }
    
    fn rev(self) -> core::iter::Rev<Self> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 594);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_140 = _to_u8(GLOBAL_DATA, 602) % 17;
        let t_141 = _to_str(GLOBAL_DATA, 603, 603 + t_140 as usize);
        let t_142 = String::from(t_141);
        let t_143 = CustomType2(t_142);
        t_143.rev()
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 619);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_145 = _to_usize(GLOBAL_DATA, 627);
        t_145
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2800 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_type = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut t_132 = match constructor_type {
            0 => {
                let t_0 = _to_usize(GLOBAL_DATA, 0);
                let t_1 = _to_usize(GLOBAL_DATA, 8);
                toodee::TooDee::new(t_0, t_1)
            },
            1 => {
                let capacity = _to_usize(GLOBAL_DATA, 16);
                toodee::TooDee::with_capacity(capacity)
            },
            2 => {
                let t_0 = _to_usize(GLOBAL_DATA, 24);
                let t_1 = _to_usize(GLOBAL_DATA, 32);
                let mut vec = Vec::new();
                let vec_size = _to_u8(GLOBAL_DATA, 40) % 65;
                for i in 0..vec_size {
                    let len = _to_u8(GLOBAL_DATA, 41 + (i as usize)) % 17;
                    let start = 42 + (i * 17) as usize;
                    let end = start + len as usize;
                    let s = _to_str(GLOBAL_DATA, start, end);
                    vec.push(CustomType0(s.to_string()));
                }
                toodee::TooDee::from_vec(t_0, t_1, vec)
            },
            _ => unreachable!()
        };

        let op_count = _to_u8(GLOBAL_DATA, 1000) % 8;
        let mut data_offset = 1001;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, data_offset) % 6;
            data_offset += 1;

            match op_type {
                0 => {
                    let index = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    data_offset += 1;
                    let s = _to_str(GLOBAL_DATA, data_offset, data_offset + len as usize);
                    data_offset += len as usize;
                    let iter = CustomType1(s.to_string());
                    t_132.insert_col(index, iter);
                },
                1 => {
                    let len = _to_u8(GLOBAL_DATA, data_offset) % 17;
                    data_offset += 1;
                    let s = _to_str(GLOBAL_DATA, data_offset, data_offset + len as usize);
                    data_offset += len as usize;
                    let iter = CustomType1(s.to_string());
                    t_132.push_col(iter);
                },
                2 => {
                    let index = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    t_132.remove_col(index);
                },
                3 => {
                    let start_col = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let start_row = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let end_col = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let end_row = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let mut view = t_132.view_mut((start_col, start_row), (end_col, end_row));
                    let r1 = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let r2 = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    view.swap_rows(r1, r2);
                    println!("{:?}", view[(0,0)]);
                },
                4 => {
                    let src = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let dst = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    t_132.swap_rows(src, dst);
                },
                5 => {
                    let index = _to_usize(GLOBAL_DATA, data_offset);
                    data_offset += 8;
                    let col = t_132.col(index);
                    println!("{:?}", col.len());
                },
                _ => {}
            }
        }

        let final_insert_idx = _to_usize(GLOBAL_DATA, 2000);
        let final_len = _to_u8(GLOBAL_DATA, 2008) % 17;
        let final_str = _to_str(GLOBAL_DATA, 2009, 2009 + final_len as usize);
        let final_iter = CustomType1(final_str.to_string());
        t_132.insert_col(final_insert_idx, final_iter);

        let remove_idx = _to_usize(GLOBAL_DATA, 2026);
        t_132.remove_col(remove_idx);

        let mut view = t_132.view((0,0), (t_132.num_cols(), t_132.num_rows()));
        println!("{:?}", view.rows().next());
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