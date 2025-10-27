#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;    
use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 105 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let t_0 = _to_u8(GLOBAL_DATA, 0);
        let t_1 = _to_u16(GLOBAL_DATA, 1);
        let t_2 = _to_u32(GLOBAL_DATA, 3);
        let t_3 = _to_u64(GLOBAL_DATA, 7);
        let t_4 = _to_u128(GLOBAL_DATA, 15);
        let t_5 = _to_usize(GLOBAL_DATA, 31);
        let t_6 = _to_i8(GLOBAL_DATA, 39);
        let t_7 = _to_i16(GLOBAL_DATA, 40);
        let t_8 = _to_i32(GLOBAL_DATA, 42);
        let t_9 = _to_i64(GLOBAL_DATA, 46);
        let t_10 = _to_i128(GLOBAL_DATA, 54);
        let t_11 = _to_isize(GLOBAL_DATA, 70);
        let t_12 = _to_f32(GLOBAL_DATA, 78);
        let t_13 = _to_f64(GLOBAL_DATA, 82);
        let t_14 = _to_char(GLOBAL_DATA, 90);
        let t_15 = _to_bool(GLOBAL_DATA, 94);
        let t_16 = _to_str(GLOBAL_DATA, 95, 105);
        let t_17 = String::from(t_16);

        let num_cols = (t_5 % 65) as usize;
        let num_rows = (t_11.unsigned_abs() % 65) as usize;
        let data_size = num_cols * num_rows;
        let init_data = global_data.second_half.get(..data_size).unwrap_or(&[]);
        let mut toodee = TooDee::from_vec(num_cols, num_rows, init_data.to_vec());

        if num_cols > 0 && num_rows > 0 {
            let push_len = num_cols;
            let push_data = global_data.second_half.get(data_size..data_size + push_len).unwrap_or(&[]);
            toodee.push_row(push_data.iter().cloned());

            let insert_pos = (t_6.unsigned_abs() as usize) % (toodee.num_rows() + 1);
            let insert_data = global_data.second_half.get(data_size + push_len..data_size + push_len + num_cols).unwrap_or(&[]);
            toodee.insert_row(insert_pos, insert_data.iter().cloned());

            let r1 = (t_7.unsigned_abs() as usize) % toodee.num_rows();
            let r2 = (t_8.unsigned_abs() as usize) % toodee.num_rows();
            toodee.swap_rows(r1, r2);

            let view_start = (
                (t_9.unsigned_abs() as usize) % num_cols,
                (t_10.unsigned_abs() as usize) % num_rows,
            );
            let view_end = (
                (view_start.0 + (t_1 as usize % (num_cols - view_start.0 + 1))).min(num_cols),
                (view_start.1 + (t_2 as usize % (num_rows - view_start.1 + 1))).min(num_rows),
            );
            let mut view = toodee.view_mut(view_start, view_end);

            let clone_src_len = view.num_cols() * view.num_rows();
            let clone_src = global_data.second_half.get(data_size + push_len + num_cols..data_size + push_len + num_cols + clone_src_len).unwrap_or(&[]);
            view.clone_from_slice(clone_src);

            view.flip_rows();
            view.flip_cols();

            let _ = &view[0];
            let _ = &mut view[0];
        }

        let total_clone_len = toodee.num_cols() * toodee.num_rows();
        let total_clone_src = global_data.second_half.get(..total_clone_len).unwrap_or(&[]);
        toodee.clone_from_slice(total_clone_src);

        println!("{:?}", toodee);
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