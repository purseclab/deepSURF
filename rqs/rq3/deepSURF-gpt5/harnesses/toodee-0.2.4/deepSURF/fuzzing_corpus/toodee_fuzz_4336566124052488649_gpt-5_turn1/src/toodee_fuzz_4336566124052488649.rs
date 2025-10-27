#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
struct CustomType0(u64);

fn build_vals(src: &[u8], start: usize, count: usize) -> Vec<CustomType0> {
    let mut v = Vec::with_capacity(count);
    for i in 0..count {
        let idx = start + i * 8;
        if idx + 8 <= src.len() {
            v.push(CustomType0(_to_u64(src, idx)));
        } else {
            v.push(CustomType0(0));
        }
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3000 { return; }
        set_global_data(data);
        let g = get_global_data();
        let fh = g.first_half;
        let sh = g.second_half;

        let len0 = (_to_u8(fh, 0) as usize % 65) + 11;
        let vec0 = build_vals(sh, 16, len0);
        let ctor_sel = _to_u8(fh, 8);
        let mut dest = match ctor_sel % 3 {
            0 => toodee::TooDee::<CustomType0>::from_vec(_to_usize(fh, 24), _to_usize(fh, 32), vec0),
            1 => toodee::TooDee::<CustomType0>::from_box(_to_usize(fh, 24), _to_usize(fh, 32), vec0.into_boxed_slice()),
            _ => {
                let cap = _to_usize(fh, 40);
                let mut td = toodee::TooDee::<CustomType0>::with_capacity(cap);
                let rows = (_to_u8(fh, 48) as usize % 4) + 1;
                for k in 0..rows {
                    let row_len = (_to_u8(fh, 49 + k) as usize % 65) + 1;
                    let row = build_vals(fh, 64 + k * 16, row_len);
                    td.push_row(row);
                }
                td
            }
        };

        let len1 = (_to_u8(sh, 0) as usize % 65) + 11;
        let vec1 = build_vals(fh, 256, len1);
        let mut src1 = if _to_u8(sh, 8) % 2 == 0 {
            toodee::TooDee::<CustomType0>::from_vec(_to_usize(sh, 24), _to_usize(sh, 32), vec1)
        } else {
            toodee::TooDee::<CustomType0>::from_box(_to_usize(sh, 24), _to_usize(sh, 32), vec1.into_boxed_slice())
        };
        let src_view = src1.view((_to_usize(sh, 40), _to_usize(sh, 48)), (_to_usize(sh, 56), _to_usize(sh, 64)));

        let len2 = (_to_u8(fh, 300) as usize % 65) + 11;
        let vec2 = build_vals(sh, 312, len2);
        let view2 = toodee::TooDeeView::<CustomType0>::new(_to_usize(fh, 320), _to_usize(fh, 328), &vec2[..]);

        let mut c0 = dest.col(_to_usize(fh, 336));
        let _ = c0.last();

        let v = dest.view((_to_usize(fh, 344), _to_usize(fh, 352)), (_to_usize(fh, 360), _to_usize(fh, 368)));
        let mut rows_it = v.rows();
        let _ = rows_it.next();
        let _ = rows_it.nth(_to_usize(fh, 376));

        let rref = &dest[_to_usize(fh, 384)];
        let cell_ref = &rref[_to_usize(fh, 392)];
        println!("{:?}", *cell_ref);

        let nops = (_to_u8(sh, 100) as usize % 8) + 1;
        for i in 0..nops {
            let base = 408 + i * 24;
            match _to_u8(sh, base) % 8 {
                0 => {
                    let mut rm = dest.rows_mut();
                    let _ = rm.next();
                    let _ = rm.nth_back(_to_usize(fh, base + 8));
                }
                1 => {
                    let col_idx = _to_usize(fh, base + 8);
                    let mut cm = dest.col_mut(col_idx);
                    if let Some(x) = cm.next() {
                        *x = CustomType0(x.0 ^ _to_u64(fh, base + 16));
                        println!("{:?}", *x);
                    }
                    let _ = cm.nth(_to_usize(fh, base + 24));
                }
                2 => {
                    let r1 = _to_usize(fh, base + 8);
                    let r2 = _to_usize(fh, base + 16);
                    dest.swap_rows(r1, r2);
                }
                3 => {
                    let row_len = (_to_u8(sh, base + 8) as usize % 65) + 1;
                    let row = build_vals(sh, base + 16, row_len);
                    dest.push_row(row);
                }
                4 => {
                    let idx = _to_usize(sh, base + 8);
                    let row_len = (_to_u8(sh, base + 16) as usize % 65) + 1;
                    let row = build_vals(fh, base + 24, row_len);
                    dest.insert_row(idx, row);
                }
                5 => {
                    let col_data_len = (_to_u8(fh, base + 8) as usize % 65) + 1;
                    let col_data = build_vals(fh, base + 16, col_data_len);
                    dest.push_col(col_data);
                }
                6 => {
                    let idx = _to_usize(sh, base + 8);
                    let col_data_len = (_to_u8(sh, base + 16) as usize % 65) + 1;
                    let col_data = build_vals(sh, base + 24, col_data_len);
                    dest.insert_col(idx, col_data);
                }
                _ => {
                    let idx = _to_usize(fh, base + 8);
                    let mut drain = dest.remove_col(idx);
                    if let Some(v0) = drain.next() {
                        println!("{:?}", v0);
                    }
                    let _ = drain.next_back();
                }
            }
        }

        dest.copy_from_toodee(&src_view);

        dest.copy_from_toodee(&src1);

        dest.copy_from_toodee(&view2);

        let vm_len = (_to_u8(fh, 500) as usize % 65) + 11;
        let mut vm_vec = build_vals(fh, 508, vm_len);
        let tvm = toodee::TooDeeViewMut::<CustomType0>::new(_to_usize(fh, 516), _to_usize(fh, 524), &mut vm_vec[..]);
        let tm_toodee = toodee::TooDee::<CustomType0>::from(tvm);
        dest.copy_from_toodee(&tm_toodee);

        let mut vmut = dest.view_mut((_to_usize(sh, 200), _to_usize(sh, 208)), (_to_usize(sh, 216), _to_usize(sh, 224)));
        let mut cm2 = vmut.col_mut(_to_usize(sh, 232));
        if let Some(x) = cm2.next_back() {
            println!("{:?}", *x);
        }
        let r2 = &vmut[_to_usize(sh, 240)];
        let c2 = &r2[_to_usize(sh, 244)];
        println!("{:?}", *c2);
        let mut rm2 = vmut.rows_mut();
        if let Some(rr) = rm2.next() {
            if !rr.is_empty() {
                let p = &mut rr[0];
                *p = CustomType0(p.0 ^ _to_u64(sh, 248));
                println!("{:?}", *p);
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