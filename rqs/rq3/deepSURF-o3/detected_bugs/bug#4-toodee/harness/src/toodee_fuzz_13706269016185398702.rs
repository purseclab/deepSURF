#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;

#[derive(Debug, Clone, Default)]
struct CustomType1(String);
#[derive(Debug)]
struct CustomType0(String);
#[derive(Debug)]
struct CustomType2(String);

impl core::iter::IntoIterator for CustomType0 {
    type Item = CustomType1;
    type IntoIter = CustomType2;
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 57) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA =
            if selector == 1 { global_data.first_half } else { global_data.second_half };
        let len = _to_u8(GLOBAL_DATA, 65) % 17;
        let s = _to_str(GLOBAL_DATA, 66, 66 + len as usize);
        CustomType2(String::from(s))
    }
}

impl core::iter::ExactSizeIterator for CustomType2 {
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 16) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA =
            if selector == 1 { global_data.first_half } else { global_data.second_half };
        _to_usize(GLOBAL_DATA, 24)
    }
}

impl core::iter::Iterator for CustomType2 {
    type Item = CustomType1;
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 32) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA =
            if selector == 1 { global_data.first_half } else { global_data.second_half };
        let len = _to_u8(GLOBAL_DATA, 40) % 17;
        let s = _to_str(GLOBAL_DATA, 41, 41 + len as usize);
        Some(CustomType1(String::from(s)))
    }
}

impl core::iter::DoubleEndedIterator for CustomType2 {
    fn next_back(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.second_half;
        let selector = (_to_usize(GLOBAL_DATA, 48) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA =
            if selector == 1 { global_data.first_half } else { global_data.second_half };
        let len = _to_u8(GLOBAL_DATA, 56) % 17;
        let s = _to_str(GLOBAL_DATA, 57, 57 + len as usize);
        Some(CustomType1(String::from(s)))
    }
}

fn build_custom_row(offset: usize) -> CustomType0 {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.second_half;
    let len = _to_u8(GLOBAL_DATA, offset % GLOBAL_DATA.len()) % 17;
    let start = (offset + 1) % GLOBAL_DATA.len();
    let end = (start + len as usize).min(GLOBAL_DATA.len());
    let s = _to_str(GLOBAL_DATA, start, end);
    CustomType0(String::from(s))
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let constructor_sel = _to_u8(GLOBAL_DATA, 0) % 4;
        let mut toodee_instance: toodee::TooDee<CustomType1> = match constructor_sel {
            0 => {
                let cap = _to_usize(GLOBAL_DATA, 1);
                toodee::TooDee::with_capacity(cap)
            }
            1 => {
                let cols = _to_usize(GLOBAL_DATA, 9);
                let rows = _to_usize(GLOBAL_DATA, 17);
                toodee::TooDee::new(cols, rows)
            }
            2 => {
                let cols = (_to_usize(GLOBAL_DATA, 25)).max(1);
                let vec_len = (_to_u8(GLOBAL_DATA, 33) % 65) as usize;
                let mut v = Vec::new();
                for i in 0..vec_len {
                    v.push(CustomType1(String::from(_to_str(
                        GLOBAL_DATA,
                        34 + i as usize,
                        35 + i as usize,
                    ))));
                }
                let rows = if cols == 0 { 0 } else { v.len() / cols };
                toodee::TooDee::from_vec(cols, rows, v)
            }
            _ => {
                let cols = (_to_usize(GLOBAL_DATA, 41)).max(1);
                let box_len = (_to_u8(GLOBAL_DATA, 49) % 65) as usize;
                let mut v = Vec::new();
                for i in 0..box_len {
                    v.push(CustomType1(String::from(_to_str(
                        GLOBAL_DATA,
                        50 + i as usize,
                        51 + i as usize,
                    ))));
                }
                let rows = if cols == 0 { 0 } else { v.len() / cols };
                toodee::TooDee::from_box(cols, rows, v.into_boxed_slice())
            }
        };
        let op_count = (_to_u8(GLOBAL_DATA, 90) % 20) as usize;
        for i in 0..op_count {
            let sel = _to_u8(GLOBAL_DATA, 91 + i) % 10;
            match sel {
                0 => {
                    let row_iter = build_custom_row(100 + i * 3);
                    toodee_instance.push_row(row_iter);
                }
                1 => {
                    let index = _to_usize(GLOBAL_DATA, 200 + i * 8);
                    let row_iter = build_custom_row(150 + i * 5);
                    toodee_instance.insert_row(index, row_iter);
                }
                2 => {
                    let col_iter = build_custom_row(250 + i * 7);
                    toodee_instance.push_col(col_iter);
                }
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, 300 + i * 9);
                    let col_iter = build_custom_row(275 + i * 4);
                    toodee_instance.insert_col(idx, col_iter);
                }
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, 350 + i * 11);
                    let mut drain = toodee_instance.remove_col(idx);
                    if let Some(v) = drain.next() {
                        println!("{:?}", v);
                    }
                }
                5 => {
                    if let Some(mut drain) = toodee_instance.pop_col() {
                        if let Some(v) = drain.next_back() {
                            println!("{:?}", v);
                        }
                    }
                }
                6 => {
                    let r1 = _to_usize(GLOBAL_DATA, 400 + i * 13);
                    let r2 = _to_usize(GLOBAL_DATA, 420 + i * 13);
                    toodee_instance.swap_rows(r1, r2);
                }
                7 => {
                    let c1 = _to_usize(GLOBAL_DATA, 440 + i * 15);
                    let r1 = _to_usize(GLOBAL_DATA, 460 + i * 15);
                    let c2 = _to_usize(GLOBAL_DATA, 480 + i * 15);
                    let r2 = _to_usize(GLOBAL_DATA, 500 + i * 15);
                    let view = toodee_instance.view((c1, r1), (c2, r2));
                    if let Some(row) = view.rows().next() {
                        println!("{:?}", row);
                    }
                    if let Some(val) = view.col(0).last() {
                        println!("{:?}", val);
                    }
                }
                8 => {
                    let c1 = _to_usize(GLOBAL_DATA, 120 + i * 6);
                    let r1 = _to_usize(GLOBAL_DATA, 140 + i * 6);
                    let c2 = _to_usize(GLOBAL_DATA, 160 + i * 6);
                    let r2 = _to_usize(GLOBAL_DATA, 180 + i * 6);
                    let mut view_mut = toodee_instance.view_mut((c1, r1), (c2, r2));
                    view_mut.swap_rows(0, 0);
                    if let Some(row) = view_mut.rows_mut().next() {
                        println!("{:?}", row.len());
                    }
                    if let Some(v) = view_mut.col_mut(0).next() {
                        println!("{:?}", v);
                    }
                }
                _ => {
                    let col_idx = _to_usize(GLOBAL_DATA, 60 + i * 2);
                    if let Some(v) = toodee_instance.col(col_idx).nth(0) {
                        println!("{:?}", v);
                    }
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