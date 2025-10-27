#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1200 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let first = gd.first_half;
        let second = gd.second_half;

        let mut vec_a = Vec::new();
        let len_a = (_to_u8(first, 0) as usize) % 65;
        for j in 0..len_a {
            let off = (2 * j + 2) % (first.len().saturating_sub(2).max(1));
            let sl = (_to_u8(first, off) % 16) as usize;
            let s = _to_str(first, off, (off + sl).min(first.len()));
            vec_a.push(String::from(s));
        }

        let mut vec_b = Vec::new();
        let len_b = (_to_u8(second, 1) as usize) % 65;
        for j in 0..len_b {
            let off = (3 * j + 3) % (second.len().saturating_sub(2).max(1));
            let sl = (_to_u8(second, off) % 16) as usize;
            let s = _to_str(second, off, (off + sl).min(second.len()));
            vec_b.push(String::from(s));
        }

        let cols0 = _to_usize(first, 8);
        let rows0 = _to_usize(first, 16);
        let cap0 = _to_usize(first, 24);
        let fill_s = {
            let l = (_to_u8(first, 32) % 16) as usize;
            let s = _to_str(first, 33, (33 + l).min(first.len()));
            String::from_str(s).unwrap_or_default()
        };
        let ctor_sel = _to_u8(first, 40) % 5;
        let mut dest = match ctor_sel {
            0 => TooDee::<String>::new(cols0, rows0),
            1 => TooDee::<String>::init(cols0, rows0, fill_s.clone()),
            2 => TooDee::<String>::with_capacity(cap0),
            3 => TooDee::<String>::from_vec(cols0, rows0, vec_a.clone()),
            _ => TooDee::<String>::from_box(cols0, rows0, vec_a.clone().into_boxed_slice()),
        };

        let ops = (_to_u8(second, 48) % 20) as usize;
        for i in 0..ops {
            let sel = _to_u8(second, 49 + i % 10) % 12;
            if sel == 0 {
                let mut r = dest.rows();
                let _ = r.next();
                let _ = r.nth(_to_usize(first, 56 + i));
                let _ = r.next_back();
            } else if sel == 1 {
                let cidx = _to_usize(first, 88 + i);
                let mut c = dest.col(cidx);
                let _ = c.next();
                let _ = c.nth(_to_usize(second, 64 + i));
                let _ = c.last();
            } else if sel == 2 {
                let cidx = _to_usize(second, 96 + i);
                let mut cm = dest.col_mut(cidx);
                let _ = cm.next();
                let _ = cm.nth(_to_usize(first, 104 + i));
                let _ = cm.nth_back(_to_usize(second, 112 + i));
            } else if sel == 3 {
                let r1 = _to_usize(first, 120 + i);
                let r2 = _to_usize(second, 128 + i);
                dest.swap_rows(r1, r2);
            } else if sel == 4 {
                dest.push_row(vec_b.clone());
            } else if sel == 5 {
                let idx = _to_usize(first, 136 + i);
                dest.insert_row(idx, vec_b.clone());
            } else if sel == 6 {
                dest.push_col(vec_b.clone());
            } else if sel == 7 {
                let idx = _to_usize(second, 144 + i);
                dest.insert_col(idx, vec_b.clone());
            } else if sel == 8 {
                if let Some(mut dc) = dest.pop_col() {
                    let _ = dc.next();
                    let _ = dc.next_back();
                }
            } else if sel == 9 {
                let mut rm = dest.rows_mut();
                let _ = rm.next();
                let _ = rm.nth(_to_usize(first, 152 + i));
                let _ = rm.nth_back(_to_usize(second, 160 + i));
            } else if sel == 10 {
                let v = dest.view((_to_usize(first, 168 + i), _to_usize(first, 176 + i)), (_to_usize(second, 184 + i), _to_usize(second, 192 + i)));
                let mut colit = v.col(_to_usize(first, 200 + i));
                let _ = colit.next();
                if let Some(row) = v.rows().last() {
                    if !row.is_empty() {
                        println!("{}", row[0].len());
                    }
                }
                let coord = (_to_usize(first, 208 + i), _to_usize(first, 216 + i));
                let cell = &v[coord];
                println!("{}", cell.len());
            } else {
                let coord_row = _to_usize(second, 224 + i);
                let coord = (_to_usize(first, 232 + i), _to_usize(second, 240 + i));
                let row_ref = &dest[coord_row];
                println!("{}", row_ref.len());
                let cell_ref = &dest[coord];
                println!("{}", cell_ref.len());
            }
        }

        let src_sel = _to_u8(first, 248) % 3;
        if src_sel == 0 {
            let v = dest.view((_to_usize(first, 256), _to_usize(first, 264)), (_to_usize(first, 272), _to_usize(first, 280)));
            let tv = TooDee::<String>::from(v);
            dest.clone_from_toodee(&tv);
        } else if src_sel == 1 {
            let sc = _to_usize(second, 288);
            let sr = _to_usize(second, 296);
            let src = TooDee::<String>::from_vec(sc, sr, vec_b.clone());
            dest.clone_from_toodee(&src);
        } else {
            let sc = _to_usize(first, 304);
            let sr = _to_usize(first, 312);
            let mut temp = vec_b.clone();
            let vm = {
                let slice = &mut temp[..];
                TooDeeViewMut::new(sc, sr, slice)
            };
            dest.clone_from_toodee(&vm);
        }

        let post_ops = (_to_u8(second, 320) % 10) as usize;
        for i in 0..post_ops {
            let sel = _to_u8(first, 321 + i % 7) % 8;
            if sel == 0 {
                let mut r = dest.rows();
                let _ = r.next();
            } else if sel == 1 {
                let cidx = _to_usize(first, 328 + i);
                let mut c = dest.col(cidx);
                let _ = c.next_back();
            } else if sel == 2 {
                let vm = dest.view_mut((_to_usize(second, 336 + i), _to_usize(second, 344 + i)), (_to_usize(second, 352 + i), _to_usize(second, 360 + i)));
                let v2 = vm.view((_to_usize(first, 368 + i), _to_usize(first, 376 + i)), (_to_usize(first, 384 + i), _to_usize(first, 392 + i)));
                let _t = TooDee::<String>::from(v2);
            } else if sel == 3 {
                let idx = _to_usize(first, 400 + i);
                let row_ref = &dest[idx];
                if !row_ref.is_empty() {
                    println!("{}", row_ref[0].len());
                }
            } else if sel == 4 {
                let coord = (_to_usize(second, 408 + i), _to_usize(second, 416 + i));
                let cell_ref = &dest[coord];
                println!("{}", cell_ref.len());
            } else if sel == 5 {
                let _ = dest.pop_col();
            } else if sel == 6 {
                dest.remove_col(_to_usize(first, 424 + i));
            } else {
                dest.insert_row(_to_usize(second, 432 + i), vec_a.clone());
            }
        }

        let data_slice_len = (_to_u8(first, 440) % 32) as usize;
        let ds_start = 444 % first.len();
        let ds_end = (ds_start + data_slice_len).min(first.len());
        let slice = &vec_a[..vec_a.len().min(32)];
        if !slice.is_empty() && ds_end > ds_start {
            let _ = &first[ds_start..ds_end];
            let clone_src = &slice[..slice.len()];
            let cols = dest.num_cols();
            let rows = dest.num_rows();
            let mut view_mut_dest = {
                let slice_mut: &mut [String] = dest.data_mut();
                TooDeeViewMut::new(cols, rows, slice_mut)
            };
            let _ = view_mut_dest.rows_mut().next();
            if !clone_src.is_empty() && clone_src.len() <= view_mut_dest.num_cols().saturating_mul(view_mut_dest.num_rows()).max(1) {
                view_mut_dest.clone_from_slice(clone_src);
            }
            let view_read: TooDeeView<'_, String> = view_mut_dest.into();
            let _d2 = TooDee::<String>::from(view_read);
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