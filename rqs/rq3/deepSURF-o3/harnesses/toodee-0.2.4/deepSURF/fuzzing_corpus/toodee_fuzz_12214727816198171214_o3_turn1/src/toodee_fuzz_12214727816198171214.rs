#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomCell(u8);

fn build_vec(data: &[u8], start: &mut usize, len: usize) -> Vec<CustomCell> {
    let mut v = Vec::with_capacity(len);
    for i in 0..len {
        let idx = (*start + i) % (data.len() - 1);
        let val = _to_u8(data, idx);
        v.push(CustomCell(val));
    }
    *start += len;
    v
}

fn generate_toodee(data: &[u8], start: &mut usize) -> TooDee<CustomCell> {
    let selector = _to_u8(data, *start % (data.len() - 1)) % 4;
    *start += 1;
    let cols = (_to_u8(data, *start % (data.len() - 1)) % 8 + 1) as usize;
    *start += 1;
    let rows = (_to_u8(data, *start % (data.len() - 1)) % 8 + 1) as usize;
    *start += 1;
    let total = cols * rows;
    let vec_data = build_vec(data, start, total);
    match selector {
        0 => TooDee::<CustomCell>::new(cols, rows),
        1 => {
            let init_val = CustomCell(_to_u8(data, *start % (data.len() - 1)));
            *start += 1;
            TooDee::<CustomCell>::init(cols, rows, init_val)
        }
        2 => TooDee::<CustomCell>::from_vec(cols, rows, vec_data),
        _ => TooDee::<CustomCell>::with_capacity(total),
    }
}

fn deref_print<T: std::fmt::Debug>(r: &T) {
    println!("{:?}", *r);
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut offset = 0usize;
        let mut td_a = generate_toodee(GLOBAL_DATA, &mut offset);
        let mut td_b = generate_toodee(GLOBAL_DATA, &mut offset);
        let op_count = (_to_u8(GLOBAL_DATA, offset % (GLOBAL_DATA.len() - 1)) % 20) as usize;
        offset += 1;
        for i in 0..op_count {
            let sel = _to_u8(GLOBAL_DATA, (offset + i) % (GLOBAL_DATA.len() - 1)) % 7;
            match sel {
                0 => {
                    let c = td_a.num_cols();
                    if c > 0 {
                        let col_idx =
                            _to_usize(GLOBAL_DATA, (offset + 8) % (GLOBAL_DATA.len() - 8)) % c;
                        let col_it = td_a.col(col_idx);
                        println!("{:?}", col_it.size_hint());
                    }
                }
                1 => {
                    let r = td_a.num_rows();
                    if r > 1 {
                        let r1 =
                            _to_usize(GLOBAL_DATA, (offset + 16) % (GLOBAL_DATA.len() - 8)) % r;
                        let r2 =
                            _to_usize(GLOBAL_DATA, (offset + 24) % (GLOBAL_DATA.len() - 8)) % r;
                        td_a.swap_rows(r1, r2);
                    }
                }
                2 => {
                    td_b.clone_from_toodee(&td_a);
                }
                3 => {
                    let (c, r) = td_a.size();
                    if c > 0 && r > 0 {
                        let s1 =
                            _to_usize(GLOBAL_DATA, (offset + 32) % (GLOBAL_DATA.len() - 8)) % c;
                        let s2 =
                            _to_usize(GLOBAL_DATA, (offset + 40) % (GLOBAL_DATA.len() - 8)) % r;
                        let e1 =
                            _to_usize(GLOBAL_DATA, (offset + 48) % (GLOBAL_DATA.len() - 8)) % c;
                        let e2 =
                            _to_usize(GLOBAL_DATA, (offset + 56) % (GLOBAL_DATA.len() - 8)) % r;
                        let view = td_a.view((s1, s2), (e1, e2));
                        println!("{:?}", view.size());
                    }
                }
                4 => {
                    if td_a.num_cols() > 0 {
                        let col =
                            _to_usize(GLOBAL_DATA, (offset + 64) % (GLOBAL_DATA.len() - 8))
                                % td_a.num_cols();
                        let drain = td_a.remove_col(col);
                        println!("{:?}", drain.size_hint());
                    }
                }
                5 => {
                    if let Some(mut d) = td_a.pop_col() {
                        println!("{:?}", d.next_back());
                    }
                }
                _ => {
                    let mut rows_it = td_a.rows();
                    println!("{:?}", rows_it.nth(0));
                }
            }
        }
        td_a.clone_from_toodee(&td_b);
        if td_a.num_cols() > 0 && td_a.num_rows() > 0 {
            let cell_ref = &td_a[(0usize, 0usize)];
            deref_print(cell_ref);
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