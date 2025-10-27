#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::iter::IntoIterator for CustomType1 {
    type Item = CustomType0;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 652);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_150 = _to_u8(GLOBAL_DATA, 660) % 17;
        let t_151 = _to_str(GLOBAL_DATA, 661, 661 + t_150 as usize);
        let t_152 = String::from(t_151);
        let t_153 = CustomType2(t_152);
        return t_153;
    }
}

impl core::iter::DoubleEndedIterator for CustomType2 {
    
    fn next_back(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 611);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_144 = _to_u8(GLOBAL_DATA, 619) % 17;
        let t_145 = _to_str(GLOBAL_DATA, 620, 620 + t_144 as usize);
        let t_146 = String::from(t_145);
        let t_147 = CustomType0(t_146);
        let t_148 = Some(t_147);
        return t_148;
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType0;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 586);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_139 = _to_u8(GLOBAL_DATA, 594) % 17;
        let t_140 = _to_str(GLOBAL_DATA, 595, 595 + t_139 as usize);
        let t_141 = String::from(t_140);
        let t_142 = CustomType0(t_141);
        let t_143 = Some(t_142);
        return t_143;
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 636);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let t_149 = _to_usize(GLOBAL_DATA, 644);
        return t_149;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_0 = _to_usize(GLOBAL_DATA, 0);
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 33;
        let mut t_3 = std::vec::Vec::with_capacity(32);
        for i in (0..32).map(|x| x * 17 + 17) {
            let mut t = _to_u8(GLOBAL_DATA, i) % 17;
            let s = _to_str(GLOBAL_DATA, i+1, i+1 + t as usize);
            t_3.push(CustomType0(String::from(s)));
        }
        t_3.truncate(t_2 as usize);
        
        let constructor_type = _to_u8(GLOBAL_DATA, 160) % 3;
        let mut t_132 = match constructor_type {
            0 => TooDee::from_vec(t_0, t_1, t_3),
            1 => {
                let mut vec = Vec::new();
                vec.extend(t_3.iter().cloned());
                TooDee::from_vec(t_0, t_1, vec)
            },
            _ => TooDee::with_capacity(_to_usize(GLOBAL_DATA, 168))
        };
        
        let ops = _to_usize(GLOBAL_DATA, 176) % 8;
        for op_idx in 0..ops {
            let op_selector = _to_u8(GLOBAL_DATA, 184 + op_idx*16) % 8;
            let param1 = _to_usize(GLOBAL_DATA, 192 + op_idx*16);
            let param2 = _to_usize(GLOBAL_DATA, 200 + op_idx*16);
            
            match op_selector {
                0 => {
                    let view_mut = t_132.view_mut((param1, param2), (param1+_to_usize(GLOBAL_DATA, 208), param2+_to_usize(GLOBAL_DATA, 216)));
                    let cell = &view_mut[(0, 0)];
                    println!("ViewMut cell: {:?}", cell);
                },
                1 => {
                    let mut rows = t_132.rows_mut();
                    if let Some(row) = rows.nth(_to_usize(GLOBAL_DATA, 224) % rows.len()) {
                        let cell = &row[0];
                        println!("Row cell: {:?}", cell);
                    }
                },
                2 => {
                    let mut col = t_132.col_mut(param1);
                    if let Some(cell) = col.next() {
                        println!("Col cell: {:?}", *cell);
                    }
                },
                3 => {
                    t_132.remove_col(param1);
                },
                4 => {
                    t_132.swap_rows(param1, param2);
                },
                5 => {
                    let t_154 = _to_u8(GLOBAL_DATA, 232) % 17;
                    let t_155 = _to_str(GLOBAL_DATA, 233, 233 + t_154 as usize);
                    let t_156 = String::from(t_155);
                    let t_157 = CustomType1(t_156);
                    t_132.push_col(t_157);
                },
                6 => {
                    let drain = t_132.pop_col();
                    if let Some(d) = drain {
                        for item in d {
                            println!("Drained: {:?}", item);
                        }
                    }
                },
                _ => {
                    let row_idx = param1 % t_132.num_rows();
                    let col_idx = param2 % t_132.num_cols();
                    let cell = &t_132[(col_idx, row_idx)];
                    println!("Cell accessed: {:?}", cell);
                }
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