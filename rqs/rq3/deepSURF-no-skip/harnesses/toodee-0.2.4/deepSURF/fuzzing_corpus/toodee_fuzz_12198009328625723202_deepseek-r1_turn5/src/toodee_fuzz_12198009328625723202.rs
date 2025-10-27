#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
#[derive(Debug, Default, Clone)]
struct CustomType0(String);

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 16);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_4 = _to_u8(GLOBAL_DATA, 24) % 17;
        let t_5 = _to_str(GLOBAL_DATA, 25, 25 + t_4 as usize);
        let t_6 = String::from(t_5);
        let t_7 = CustomType1(t_6);
        let t_8 = Some(t_7);
        return t_8;
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_9 = _to_usize(GLOBAL_DATA, 49);
        return t_9;
    }
}

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 57);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_10 = _to_u8(GLOBAL_DATA, 65) % 17;
        let t_11 = _to_str(GLOBAL_DATA, 66, 66 + t_10 as usize);
        let t_12 = String::from(t_11);
        let t_13 = CustomType2(t_12);
        return t_13;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 400 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let constructor_selector = _to_u8(GLOBAL_DATA, 0) % 3;
        let mut t1: toodee::TooDee<CustomType0> = match constructor_selector {
            0 => toodee::TooDee::with_capacity(_to_usize(GLOBAL_DATA, 0)),
            1 => toodee::TooDee::new(_to_usize(GLOBAL_DATA, 64), _to_usize(GLOBAL_DATA, 72)),
            _ => toodee::TooDee::from_vec(_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16), vec![CustomType0(String::new()); _to_usize(GLOBAL_DATA, 80)])
        };

        let op_count = _to_usize(GLOBAL_DATA, 120) % 8;
        for _ in 0..op_count {
            let op_selector = _to_u8(GLOBAL_DATA, 130) % 9;
            match op_selector {
                0 => {
                    let (col1, col2) = (_to_usize(GLOBAL_DATA, 140), _to_usize(GLOBAL_DATA, 148));
                    t1.swap_rows(col1, col2);
                },
                1 => {
                    let idx = _to_usize(GLOBAL_DATA, 156);
                    if !t1.is_empty() {
                        let row = &t1[idx];
                        println!("Row {}: {:?}", idx, row);
                    }
                },
                2 => {
                    let idx = _to_usize(GLOBAL_DATA, 164);
                    if let Some(row) = t1.rows_mut().nth(idx) {
                        let cell_idx = _to_usize(GLOBAL_DATA, 172);
                        println!("Mut row cell: {:?}", &row[cell_idx]);
                    }
                },
                3 => {
                    let start = (_to_usize(GLOBAL_DATA, 180), _to_usize(GLOBAL_DATA, 188));
                    let end = (_to_usize(GLOBAL_DATA, 196), _to_usize(GLOBAL_DATA, 204));
                    let mut view = t1.view_mut(start, end);
                    view.swap_rows(0, view.num_rows().saturating_sub(1));
                    if !view.is_empty() {
                        let mut col_iter = view.col_mut(_to_usize(GLOBAL_DATA, 212));
                        if let Some(cell) = col_iter.next() {
                            *cell = CustomType0(String::from("modified"));
                        }
                    }
                },
                4 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 212);
                    let _ = t1.pop_col().map(|c| c.count());
                },
                5 => {
                    if !t1.is_empty() {
                        let row_view = t1.view((0, 0), (t1.num_cols(), 1));
                        if let Some(row) = row_view.rows().next() {
                            println!("First row view: {:?}", row);
                        }
                    }
                },
                6 => {
                    let col = t1.col(_to_usize(GLOBAL_DATA, 224));
                    println!("Col len: {}", col.len());
                },
                7 => {
                    let rev_cols = t1.rows().rev();
                    for row in rev_cols {
                        println!("Reversed row: {:?}", row);
                    }
                },
                _ => {
                    let inserted_col = _to_usize(GLOBAL_DATA, 220);
                    t1.push_col((0.._to_usize(GLOBAL_DATA, 228)).map(|_| CustomType0(String::new())));
                }
            }
        }

        {
            let view_start = (_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16));
            let view_end = (_to_usize(GLOBAL_DATA, 24), _to_usize(GLOBAL_DATA, 32));
            let swap_row1 = _to_usize(GLOBAL_DATA, 40);
            let swap_row2 = _to_usize(GLOBAL_DATA, 48);
            let col_idx = _to_usize(GLOBAL_DATA, 64);
            
            let mut view = t1.view_mut(view_start, view_end);
            view.swap_rows(swap_row1, swap_row2);
            if let Some(cell) = view.col_mut(col_idx).next() {
                println!("First column element: {:?}", cell);
            }
            let converted = toodee::TooDee::from(view);
            converted.rows().for_each(|r| println!("Row: {:?}", r));
            let _ = converted.col(_to_usize(GLOBAL_DATA, 114));
        }

        if !t1.is_empty() {
            println!("First row: {:?}", &t1[0]);
        }

        let insert_idx = _to_usize(GLOBAL_DATA, 72);
        let row_count = _to_usize(GLOBAL_DATA, 80) % 65;
        let elements = (0..row_count).map(|i| {
            let str_len = _to_u8(GLOBAL_DATA, 150 + i) % 17;
            let str_start = 170 + i * 20;
            let str_slice = _to_str(GLOBAL_DATA, str_start, str_start + str_len as usize);
            CustomType0(str_slice.to_string())
        });
        t1.insert_row(insert_idx, elements);

        let removed_col = t1.remove_col(_to_usize(GLOBAL_DATA, 98));
        for elem in removed_col {
            let _ = format!("{:?}", elem);
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