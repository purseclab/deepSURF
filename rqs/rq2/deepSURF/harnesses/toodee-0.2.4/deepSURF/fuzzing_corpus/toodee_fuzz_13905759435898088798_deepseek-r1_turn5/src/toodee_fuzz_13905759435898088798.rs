#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_usize(GLOBAL_DATA, 0);
        let t_1 = _to_usize(GLOBAL_DATA, 8);
        let mut t_2 = _to_u8(GLOBAL_DATA, 16) % 33;
        
        let mut t_3 = Vec::with_capacity(32);
        for i in 0..32 {
            let data_idx = 17 + i * 17;
            if data_idx + 17 > GLOBAL_DATA.len() { break; }
            let str_len = GLOBAL_DATA[data_idx] as usize;
            let str_start = data_idx + 1;
            let str_end = str_start + str_len;
            t_3.push(String::from_utf8_lossy(&GLOBAL_DATA[str_start..str_end.min(GLOBAL_DATA.len())]).into_owned());
        }
        t_3.truncate(t_2 as usize);
        let t_132 = &mut t_3[..];

        let second_half = global_data.second_half;
        let td_cols = _to_usize(second_half, 0) % 65;
        let td_rows = _to_usize(second_half, 8) % 65;

        if td_cols > 0 && td_rows > 0 {
            let mut vec_data = Vec::with_capacity(td_cols * td_rows);
            for i in 0..(td_cols * td_rows) {
                let offset = 16 + i * 17;
                if offset >= second_half.len() { break; }
                let len = second_half[offset] as usize % 17;
                let start = offset + 1;
                let end = start + len;
                vec_data.push(String::from_utf8_lossy(&second_half[start..end.min(second_half.len())]).into_owned());
            }
            
            let mut too_dee = TooDee::from_vec(td_cols, td_rows, vec_data);
            let row1 = _to_usize(second_half, 16 + td_cols*td_rows*17);
            let row2 = _to_usize(second_half, 24 + td_cols*td_rows*17);
            too_dee.swap_rows(row1, row2);

            let offset = td_cols * td_rows * 17;
            let mut view_mut = too_dee.view_mut(
                (_to_usize(second_half, 32 + offset), _to_usize(second_half, 40 + offset)),
                (_to_usize(second_half, 48 + offset), _to_usize(second_half, 56 + offset))
            );

            let swap_row1 = _to_usize(second_half, 80 + offset);
            let swap_row2 = _to_usize(second_half, 88 + offset);
            view_mut.swap_rows(swap_row1, swap_row2);

            let col_idx = _to_usize(second_half, 64 + offset);
            let mut col = view_mut.col_mut(col_idx);
            if let Some(cell) = col.nth(_to_usize(second_half, 72 + offset)) {
                println!("Mut cell: {:?}", *cell);
            }

            for (i, row) in view_mut.rows_mut().enumerate() {
                println!("View row {}: {:?}", i, row);
            }

            let nested_view = view_mut.view_mut(
                (_to_usize(second_half, 96 + offset), _to_usize(second_half, 104 + offset)),
                (_to_usize(second_half, 112 + offset), _to_usize(second_half, 120 + offset))
            );
            if nested_view.num_rows() > 0 {
                let nested_col = nested_view.col(_to_usize(second_half, 128 + offset));
                println!("Nested col first: {:?}", nested_col.last());
            }

            let mut new_toodee = TooDee::from(view_mut);
            let rows_mut = new_toodee.rows_mut();
            for (i, row) in rows_mut.enumerate() {
                println!("NewToDee row {}: {:?}", i, row);
            }
        }

        let _ = toodee::TooDeeViewMut::new(t_0, t_1, t_132);

        if !t_132.is_empty() {
            println!("First element: {:?}", &t_132[0]);
            let middle = t_132.len() / 2;
            t_132.swap(0, middle);
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