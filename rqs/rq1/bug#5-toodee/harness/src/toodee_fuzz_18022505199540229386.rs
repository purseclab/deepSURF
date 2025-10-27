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
        let custom_impl_num = _to_usize(GLOBAL_DATA, 652);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
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

impl core::iter::ExactSizeIterator for CustomType2 {
    
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 636);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_149 = _to_usize(GLOBAL_DATA, 644);
        return t_149;
    }
}

impl core::iter::DoubleEndedIterator for CustomType2 {
    
    fn next_back(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 561);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_134 = _to_u8(GLOBAL_DATA, 569) % 17;
        let t_135 = _to_str(GLOBAL_DATA, 570, 570 + t_134 as usize);
        let t_136 = String::from(t_135);
        let t_137 = CustomType0(t_136);
        let t_138 = Some(t_137);
        return t_138;
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
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
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

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1388 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_0 = _to_usize(GLOBAL_DATA, 0);
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 33;
        
        let mut t_3 = std::vec::Vec::with_capacity(32);
        for chunk in (17..=544).step_by(17) {
            let len = _to_u8(GLOBAL_DATA, chunk) % 17;
            let s = _to_str(GLOBAL_DATA, chunk + 1, chunk + 1 + len as usize);
            t_3.push(CustomType0(String::from(s)));
        }
        t_3.truncate(t_2 as usize);

        let constructor_selector = _to_u8(GLOBAL_DATA, 677) % 3;
        let mut t_132 = match constructor_selector {
            0 => TooDee::from_vec(t_0, t_1, t_3),
            1 => TooDee::new(t_0, t_1),
            _ => TooDee::with_capacity(_to_usize(GLOBAL_DATA, 680)),
        };
        let mut t_133 = &mut t_132;

        let num_ops = _to_usize(GLOBAL_DATA, 700) % 6;
        for i in 0..num_ops {
            let op_byte = GLOBAL_DATA[710 + i];
            match op_byte % 6 {
                0 => {
                    let mut view = t_133.view_mut((0,0), (t_133.num_cols(), t_133.num_rows()));
                    let col_idx = _to_usize(global_data.first_half, 750 + i*8);
                    let _ = view.col_mut(col_idx);
                }
                1 => {
                    let row_idx = _to_usize(global_data.second_half, 800 + i*8);
                    println!("Row: {:?}", &t_133[row_idx]);
                }
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 850 + i*8);
                    let _drain = t_133.remove_col(idx);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 900 + i*8);
                    let _ = t_133.pop_col();
                }
                4 => {
                    let r1 = _to_usize(GLOBAL_DATA, 950 + i*8);
                    let r2 = _to_usize(GLOBAL_DATA, 958 + i*8);
                    t_133.swap_rows(r1, r2);
                }
                _ => {
                    let slice = t_133.data_mut();
                    println!("Data slice: {:?}", &*slice);
                }
            }
        }

        let iter_type = _to_u8(GLOBAL_DATA, 1000) % 2;
        match iter_type {
            0 => {
                let mut t_154 = _to_u8(GLOBAL_DATA, 1001) % 17;
                let t_155 = _to_str(GLOBAL_DATA, 1002, 1002 + t_154 as usize);
                t_133.push_col(CustomType1(String::from(t_155)));
            }
            _ => {
                let idx = _to_usize(GLOBAL_DATA, 1050);
                let items = t_133.col(idx).cloned().collect::<Vec<_>>();
                t_133.push_col(items);
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