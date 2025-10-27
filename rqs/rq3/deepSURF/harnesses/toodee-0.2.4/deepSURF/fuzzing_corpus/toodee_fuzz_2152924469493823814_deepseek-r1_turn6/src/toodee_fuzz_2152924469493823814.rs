#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5070 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let ops_count = _to_u8(GLOBAL_DATA, 0) % 10;

        let mut toodee_objects = vec![];
        let t0_num_cols = _to_usize(GLOBAL_DATA, 1);
        let t0_num_rows = _to_usize(GLOBAL_DATA, 9);
        let t0 = TooDee::<CustomType0>::new(t0_num_cols, t0_num_rows);
        toodee_objects.push(t0);

        let t1_cap = _to_usize(GLOBAL_DATA, 17);
        let t1 = TooDee::<CustomType0>::with_capacity(t1_cap);
        toodee_objects.push(t1);

        let t2_num_cols = _to_usize(GLOBAL_DATA, 25);
        let t2_num_rows = _to_usize(GLOBAL_DATA, 33);
        let t2 = TooDee::<CustomType0>::init(t2_num_cols, t2_num_rows, CustomType0("init".to_string()));
        toodee_objects.push(t2);

        for op_idx in 0..=ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 41 + op_idx as usize * 8) % 10;
            match op_selector {
                0 => {
                    let src_idx = _to_usize(GLOBAL_DATA, 49 + op_idx as usize * 8) % toodee_objects.len();
                    if src_idx == 0 {
                        continue;
                    }
                    if let Some((t_dest, rest)) = toodee_objects.split_first_mut() {
                        let adjusted_idx = src_idx - 1;
                        if adjusted_idx >= rest.len() {
                            continue;
                        }
                        t_dest.clone_from_toodee(&rest[adjusted_idx]);
                        println!("Clone from toodee: {:?}", t_dest.data().len());
                    }
                },
                1 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 57 + op_idx as usize * 8);
                    if let Some(t) = toodee_objects.get_mut(0) {
                        let _ = t.remove_col(col_idx);
                        println!("Removed column: {}", col_idx);
                    }
                },
                2 => {
                    let col_idx = _to_usize(GLOBAL_DATA, 65 + op_idx as usize * 8);
                    if toodee_objects.len() >= 2 {
                        let (dest, src) = toodee_objects.split_at_mut(1);
                        if let (Some(t_src), Some(t_dest)) = (src.get(0), dest.get_mut(0)) {
                            let col_src = t_src.col(col_idx);
                            let col_dest = t_dest.col_mut(col_idx);
                            println!("Accessed columns: {} {}", col_src.len(), col_dest.len());
                        }
                    }
                },
                3 => {
                    let start = (_to_usize(GLOBAL_DATA, 73 + op_idx as usize * 8), _to_usize(GLOBAL_DATA, 81 + op_idx as usize * 8));
                    let end = (_to_usize(GLOBAL_DATA, 89 + op_idx as usize * 8), _to_usize(GLOBAL_DATA, 97 + op_idx as usize * 8));
                    if let Some(t) = toodee_objects.get_mut(0) {
                        let view_mut = t.view_mut(start, end);
                        let view = view_mut.view((0,0), (1,1));
                        println!("Created view of size: {:?}", view.size());
                    }
                },
                4 => {
                    let row_idx = _to_usize(GLOBAL_DATA, 105 + op_idx as usize * 8);
                    if let Some(t) = toodee_objects.get_mut(0) {
                        t.insert_row(row_idx, vec![CustomType0("insert".to_string()); t.num_cols()]);
                        println!("Inserted row at: {}", row_idx);
                    }
                },
                5 => {
                    let view_source = toodee_objects.get(0).unwrap().view((0,0), (1,1));
                    let mut merged = TooDee::from(view_source);
                    let src = toodee_objects.get(1);
                    if let Some(src) = src {
                        merged.clone_from_toodee(src);
                        println!("Merged view cloned");
                    }
                },
                6 => {
                    if let Some(t) = toodee_objects.get_mut(0) {
                        let col = _to_usize(GLOBAL_DATA, 113 + op_idx as usize * 8);
                        let num = _to_usize(GLOBAL_DATA, 121 + op_idx as usize * 8);
                        if let Some(mut drain) = t.pop_col() {
                            drain.nth(num);
                        }
                    }
                },
                7 => {
                    if let Some(t) = toodee_objects.get_mut(0) {
                        let row = _to_usize(GLOBAL_DATA, 129 + op_idx as usize * 8);
                        let other = _to_usize(GLOBAL_DATA, 137 + op_idx as usize * 8) % t.num_rows();
                        t.swap_rows(row, other);
                    }
                },
                8 => {
                    let data_len = _to_usize(GLOBAL_DATA, 145 + op_idx as usize * 8);
                    let capacity = _to_usize(GLOBAL_DATA, 153 + op_idx as usize * 8);
                    let mut new_toodee = TooDee::<CustomType0>::with_capacity(capacity);
                    new_toodee.reserve(data_len);
                },
                9 => {
                    if let Some(t) = toodee_objects.get_mut(0) {
                        let start = (_to_usize(GLOBAL_DATA, 161 + op_idx as usize * 8), _to_usize(GLOBAL_DATA, 169 + op_idx as usize * 8));
                        let end = (_to_usize(GLOBAL_DATA, 177 + op_idx as usize * 8), _to_usize(GLOBAL_DATA, 185 + op_idx as usize * 8));
                        let view = t.view(start, end);
                        let view2 = view.view((0,0), (1,1));
                        println!("Created nested view: {:?}", view2.size());
                    }
                },
                _ => unreachable!()
            }
        }

        let mut final_toodee = toodee_objects.remove(0);
        if let Some(view_source) = toodee_objects.get(0) {
            let view = view_source.view((0,0), (1,1));
            final_toodee.clone_from_toodee(&view);
        }
    });
}

// Type converter functions remain unchanged (not included here as per instruction)

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