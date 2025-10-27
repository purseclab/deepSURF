#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone)]
struct RowSrc(Vec<u32>);

#[derive(Clone, Debug)]
struct RowIter {
    data: Vec<u32>,
    idx_front: usize,
    idx_back: usize,
}

impl core::iter::IntoIterator for RowSrc {
    type Item = u32;
    type IntoIter = RowIter;
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 49) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        RowIter { data: self.0, idx_front: 0, idx_back: 0 }
    }
}

impl core::iter::Iterator for RowIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 24) + self.data.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        if self.idx_front >= self.data.len() { None } else {
            let v = self.data[self.idx_front];
            self.idx_front += 1;
            Some(v)
        }
    }
}

impl core::iter::DoubleEndedIterator for RowIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() { return None; }
        if self.idx_back >= self.data.len() { return None; }
        let idx = self.data.len() - 1 - self.idx_back;
        if idx < self.idx_front { return None; }
        self.idx_back += 1;
        Some(self.data[idx])
    }
}

impl core::iter::ExactSizeIterator for RowIter {
    fn len(&self) -> usize {
        self.data.len().saturating_sub(self.idx_front + self.idx_back)
    }
}

fn create_row_vec(start: usize) -> RowSrc {
    let global_data = get_global_data();
    let GLOBAL_DATA = global_data.second_half;
    let len = (_to_u8(GLOBAL_DATA, start) % 65) as usize + 1;
    let mut v = Vec::new();
    for i in 0..len {
        let idx = start + 1 + i * 4;
        if idx + 3 < GLOBAL_DATA.len() {
            v.push(_to_u32(GLOBAL_DATA, idx));
        }
    }
    RowSrc(v)
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 800 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let num_cols = _to_usize(GLOBAL_DATA, 0);
        let num_rows = _to_usize(GLOBAL_DATA, 8);
        let init_val = _to_u32(GLOBAL_DATA, 16);
        let capacity = _to_usize(GLOBAL_DATA, 24);
        let constructor_sel = _to_u8(GLOBAL_DATA, 32) % 4;
        let mut base_vec = Vec::new();
        let vec_len = (_to_u8(GLOBAL_DATA, 33) % 65) as usize + 1;
        for i in 0..vec_len {
            base_vec.push(_to_u32(GLOBAL_DATA, 34 + i * 4));
        }
        let boxed_slice = base_vec.clone().into_boxed_slice();
        let mut td = match constructor_sel {
            0 => TooDee::new(num_cols, num_rows),
            1 => TooDee::init(num_cols, num_rows, init_val),
            2 => TooDee::with_capacity(capacity),
            _ => TooDee::from_box(num_cols, num_rows, boxed_slice),
        };
        let ops = (_to_u8(GLOBAL_DATA, 100) % 16) as usize + 1;
        for i in 0..ops {
            let op_sel = _to_u8(GLOBAL_DATA, 101 + i) % 12;
            match op_sel {
                0 => {
                    let row = create_row_vec(120 + i * 10);
                    td.push_row(row);
                }
                1 => {
                    let index = _to_usize(GLOBAL_DATA, 200 + i * 8);
                    let row = create_row_vec(220 + i * 10);
                    td.insert_row(index, row);
                }
                2 => {
                    let col = create_row_vec(300 + i * 10);
                    td.push_col(col);
                }
                3 => {
                    let index = _to_usize(GLOBAL_DATA, 350 + i * 8);
                    let col = create_row_vec(370 + i * 10);
                    td.insert_col(index, col);
                }
                4 => {
                    let index = _to_usize(GLOBAL_DATA, 420 + i * 8);
                    td.remove_col(index);
                }
                5 => {
                    td.pop_col();
                }
                6 => {
                    let r1 = _to_usize(GLOBAL_DATA, 450 + i * 8);
                    let r2 = _to_usize(GLOBAL_DATA, 460 + i * 8);
                    td.swap_rows(r1, r2);
                }
                7 => {
                    let view = td.view(
                        (_to_usize(GLOBAL_DATA, 480), _to_usize(GLOBAL_DATA, 488)),
                        (_to_usize(GLOBAL_DATA, 496), _to_usize(GLOBAL_DATA, 504)),
                    );
                    println!("{:?}", view.num_cols());
                }
                8 => {
                    let mut view_mut = td.view_mut(
                        (_to_usize(GLOBAL_DATA, 480), _to_usize(GLOBAL_DATA, 488)),
                        (_to_usize(GLOBAL_DATA, 496), _to_usize(GLOBAL_DATA, 504)),
                    );
                    let rows_mut = view_mut.rows_mut();
                    println!("{:?}", rows_mut.size_hint());
                }
                9 => {
                    let rows_iter = td.rows();
                    println!("{:?}", rows_iter.last());
                }
                10 => {
                    let col_iter = td.col(_to_usize(GLOBAL_DATA, 510 + i * 8));
                    println!("{:?}", col_iter.size_hint());
                }
                _ => {
                    td.reserve_exact(_to_usize(GLOBAL_DATA, 600 + i * 8));
                }
            }
        }
        let cell_ref = &td[(_to_usize(GLOBAL_DATA, 700), _to_usize(GLOBAL_DATA, 708))];
        println!("{:?}", *cell_ref);
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