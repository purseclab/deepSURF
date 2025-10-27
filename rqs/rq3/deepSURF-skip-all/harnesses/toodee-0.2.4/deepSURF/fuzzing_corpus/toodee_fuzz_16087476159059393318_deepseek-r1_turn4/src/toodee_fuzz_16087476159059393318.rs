#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Default, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_cols = _to_usize(GLOBAL_DATA, 0);
        let num_rows = _to_usize(GLOBAL_DATA, 8);
        let mut vec_size = _to_u8(GLOBAL_DATA, 16) % 65;
        let mut vec = Vec::with_capacity(vec_size as usize);

        let mut offset = 17;
        for _ in 0..vec_size {
            if offset + 1 > GLOBAL_DATA.len() { break; }
            let str_len = _to_u8(GLOBAL_DATA, offset) as usize;
            offset += 1;
            let end = offset + str_len;
            let s = _to_str(GLOBAL_DATA, offset, end.min(GLOBAL_DATA.len()));
            vec.push(CustomType0(s.to_string()));
            offset = end;
        }
        vec.truncate(vec_size as usize);

        let constructor_choice = _to_u8(GLOBAL_DATA, offset) % 3;
        let mut toodee = match constructor_choice {
            0 => TooDee::from_vec(num_cols, num_rows, vec),
            1 => {
                let mut t = TooDee::new(num_cols, num_rows);
                t.push_col(vec.into_iter());
                t
            }
            _ => {
                let init_val = CustomType0(String::new());
                TooDee::init(num_cols, num_rows, init_val)
            }
        };

        let op_count = _to_u8(GLOBAL_DATA, offset + 1) % 32;
        offset += 2;
        for _ in 0..op_count {
            let op_byte = _to_u8(GLOBAL_DATA, offset) % 10;
            offset += 1;
            match op_byte {
                0 => {
                    let v = toodee.view((0, 0), (num_cols, num_rows));
                    let c = v.col(_to_usize(GLOBAL_DATA, offset));
                    println!("Col len: {}", c.len());
                }
                1 => {
                    let row1 = _to_usize(GLOBAL_DATA, offset) % toodee.num_rows();
                    let row2 = _to_usize(GLOBAL_DATA, offset + 8) % toodee.num_rows();
                    toodee.swap_rows(row1, row2);
                }
                2 => {
                    let col_idx = _to_usize(GLOBAL_DATA, offset) % toodee.num_cols();
                    let mut col = toodee.col_mut(col_idx);
                    println!("Col mut len: {}", col.len());
                }
                3 => {
                    let start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    let mut vm = toodee.view_mut(start, end);
                    vm.swap_rows(0, vm.num_rows().saturating_sub(1));
                }
                4 => {
                    let target_start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let target_end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    let parent = toodee.view(target_start, target_end);
                    let child = parent.view((0, 0), (parent.num_cols()/2, parent.num_rows()/2));
                    println!("Nested view columns: {}", child.num_cols());
                }
                5 => {
                    if let Some(mut dc) = toodee.pop_col() {
                        let d: Vec<_> = dc.by_ref().collect();
                        println!("Drained {} elements", d.len());
                    }
                }
                6 => {
                    let r = _to_usize(GLOBAL_DATA, offset) % toodee.num_rows();
                    println!("Row contents: {:?}", &toodee[r]);
                }
                7 => {
                    let insert_col = _to_usize(GLOBAL_DATA, offset) % (toodee.num_cols() + 1);
                    let mut data = vec![CustomType0(String::from("fuzz"))];
                    toodee.insert_col(insert_col, data.into_iter());
                }
                8 => {
                    let insert_row = _to_usize(GLOBAL_DATA, offset) % (toodee.num_rows() + 1);
                    let data = vec![CustomType0(String::from("fuzz")); toodee.num_cols()];
                    toodee.insert_row(insert_row, data);
                }
                9 => {
                    let src_start = (_to_usize(GLOBAL_DATA, offset), _to_usize(GLOBAL_DATA, offset + 8));
                    let src_end = (_to_usize(GLOBAL_DATA, offset + 16), _to_usize(GLOBAL_DATA, offset + 24));
                    let view = toodee.view(src_start, src_end);
                    let new_toodee = TooDee::from(view);
                    toodee.clone_from_toodee(&new_toodee);
                }
                _ => {}
            }
            offset += 48;
        }

        let target_start = (_to_usize(GLOBAL_DATA, 561), _to_usize(GLOBAL_DATA, 569));
        let target_end = (_to_usize(GLOBAL_DATA, 577), _to_usize(GLOBAL_DATA, 585));
        let view = toodee.view(target_start, target_end);
        let subview = view.view(target_start, target_end);
        println!("Final subview rows: {}", subview.num_rows());
    });
}

// The type converter functions (_to_u8, _to_usize, etc.) are omitted as per directions.

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