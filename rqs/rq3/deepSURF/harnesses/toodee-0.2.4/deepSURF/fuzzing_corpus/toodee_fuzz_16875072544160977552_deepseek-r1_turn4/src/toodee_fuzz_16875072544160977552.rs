#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8000 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let rows = _to_usize(GLOBAL_DATA, 0);
        let cols = _to_usize(GLOBAL_DATA, 8);
        let vec_size = _to_u8(GLOBAL_DATA, 16) % 65;
        let mut vec_data = Vec::with_capacity(vec_size as usize);
        let mut offset = 17;

        for _ in 0..vec_size {
            if offset + 1 >= GLOBAL_DATA.len() { break; }
            let str_len = _to_u8(GLOBAL_DATA, offset) % 17;
            offset += 1;
            let s = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
            vec_data.push(CustomType0(String::from(s)));
            offset += str_len as usize;
        }

        let vec_data2 = vec_data.clone();
        let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
        offset += 1;

        let mut dee = match constructor_choice {
            0 => TooDee::from_vec(rows, cols, vec_data),
            1 => TooDee::from_box(rows, cols, vec_data2.into_boxed_slice()),
            2 => TooDee::new(cols, rows),
            _ => unreachable!()
        };

        let ops_count = _to_u8(GLOBAL_DATA, offset) % 10;
        offset += 1;

        for _ in 0..ops_count {
            let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
            let end = (_to_usize(GLOBAL_DATA, offset+16), _to_usize(GLOBAL_DATA, offset+24));
            offset += 32;

            let view_mut = TooDee::view_mut(&mut dee, start, end);
            let view: TooDeeView<_> = view_mut.into();
            let mut new_dee = TooDee::from(view);

            let op_select = _to_u8(GLOBAL_DATA, offset) % 4;
            offset += 1;

            match op_select {
                0 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset);
                    let row_idx = _to_usize(GLOBAL_DATA, offset+8);
                    offset += 16;
                    let val = new_dee[(col_idx, row_idx)].clone();
                    new_dee.insert_row(row_idx, vec![val]);
                },
                1 => {
                    let col_op = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut col = new_dee.col_mut(col_op);
                    for item in &mut col {
                        *item = CustomType0(String::new());
                    }
                },
                2 => {
                    let swap_row1 = _to_usize(GLOBAL_DATA, offset);
                    let swap_row2 = _to_usize(GLOBAL_DATA, offset+8);
                    offset += 16;
                    new_dee.swap_rows(swap_row1, swap_row2);
                    let popped = new_dee.pop_row();
                    println!("{:?}", popped);
                },
                3 => {
                    let remove_idx = _to_usize(GLOBAL_DATA, offset) % new_dee.num_cols();
                    offset += 8;
                    let _ = new_dee.remove_col(remove_idx);
                    let col_data = vec![CustomType0(String::new()); new_dee.num_rows()];
                    new_dee.push_col(col_data.into_iter());
                },
                _ => unreachable!()
            }

            let rev = new_dee.rows_mut().rev();
            for row in rev {
                if !row.is_empty() {
                    println!("{:?}", row[0]);
                }
            }
        }

        let final_start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset+8));
        let final_end = (_to_usize(GLOBAL_DATA, offset+16), _to_usize(GLOBAL_DATA, offset+24));
        offset += 32;

        let final_view_mut = TooDee::view_mut(&mut dee, final_start, final_end);
        let final_view: TooDeeView<_> = final_view_mut.into();
        let _final_dee = TooDee::from(final_view);

        let capacity_choice = _to_u8(GLOBAL_DATA, offset) % 2;
        offset += 1;
        match capacity_choice {
            0 => dee.reserve(_to_usize(GLOBAL_DATA, offset)),
            1 => dee.reserve_exact(_to_usize(GLOBAL_DATA, offset)),
            _ => unreachable!()
        };
        offset += 8;

        let _cloned = dee.clone();
        dee.shrink_to_fit();
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