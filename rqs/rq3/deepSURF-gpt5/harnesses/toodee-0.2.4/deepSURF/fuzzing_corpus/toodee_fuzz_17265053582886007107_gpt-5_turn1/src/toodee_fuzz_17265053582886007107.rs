#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1500 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let GLOBAL_DATA = gd.first_half;
        let SECOND = gd.second_half;

        let g_len = GLOBAL_DATA.len();
        let s_len = SECOND.len();
        if g_len < 20 || s_len < 20 { return; }

        let cols = _to_usize(GLOBAL_DATA, 0);
        let rows = _to_usize(GLOBAL_DATA, 8);

        let mut v: Vec<CustomType0> = Vec::with_capacity(64);
        let cnt = _to_u8(GLOBAL_DATA, 16) % 65;
        for j in 0..cnt {
            let idx_b = (2 + j as usize) % g_len;
            let sl = (_to_u8(GLOBAL_DATA, idx_b) % 16) as usize;
            let start = (4 * (j as usize)) % (g_len.saturating_sub(1));
            let end = std::cmp::min(start + sl, g_len);
            if end > start {
                let s = _to_str(GLOBAL_DATA, start, end);
                v.push(CustomType0(String::from(s)));
            } else {
                v.push(CustomType0(String::new()));
            }
        }
        let mut alt: Vec<CustomType0> = Vec::with_capacity(64);
        let alt_cnt = _to_u8(SECOND, 0) % 65;
        for j in 0..alt_cnt {
            let idx_b = (1 + j as usize) % s_len;
            let sl = (_to_u8(SECOND, idx_b) % 16) as usize;
            let start = (3 * (j as usize) + 5) % (s_len.saturating_sub(1));
            let end = std::cmp::min(start + sl, s_len);
            if end > start {
                let s = _to_str(SECOND, start, end);
                alt.push(CustomType0(String::from(s)));
            } else {
                alt.push(CustomType0(String::new()));
            }
        }

        let choose_ctor = _to_u8(GLOBAL_DATA, 17);
        let mut td = if choose_ctor % 2 == 0 {
            toodee::TooDee::from_vec(cols, rows, v)
        } else {
            let bx: Box<[CustomType0]> = alt.into_boxed_slice();
            toodee::TooDee::from_box(cols, rows, bx)
        };

        let c0 = _to_usize(GLOBAL_DATA, (18 % (g_len.saturating_sub(9))));
        let r0 = _to_usize(GLOBAL_DATA, (26 % (g_len.saturating_sub(9))));
        let c1 = _to_usize(SECOND, (2 % (s_len.saturating_sub(9))));
        let r1 = _to_usize(SECOND, (10 % (s_len.saturating_sub(9))));
        let start1 = (c0, r0);
        let end1 = (c1, r1);

        let c2 = _to_usize(GLOBAL_DATA, (34 % (g_len.saturating_sub(9))));
        let r2 = _to_usize(GLOBAL_DATA, (42 % (g_len.saturating_sub(9))));
        let c3 = _to_usize(SECOND, (18 % (s_len.saturating_sub(9))));
        let r3 = _to_usize(SECOND, (26 % (s_len.saturating_sub(9))));
        let start2 = (c2, r2);
        let end2 = (c3, r3);

        let view_ro = td.view(start1, end1);
        let rows_iter = view_ro.rows();
        let mut rows_iter2 = rows_iter;
        let _ = rows_iter2.next();
        let _ = rows_iter2.nth(_to_usize(GLOBAL_DATA, (50 % (g_len.saturating_sub(9)))));
        let _ = rows_iter2.last();
        let mut some_col = view_ro.col(_to_usize(GLOBAL_DATA, (58 % (g_len.saturating_sub(9)))));
        let _ = some_col.next();
        let _ = some_col.nth(_to_usize(SECOND, (34 % (s_len.saturating_sub(9)))));
        let _ = some_col.last();
        let idx_row = _to_usize(GLOBAL_DATA, (66 % (g_len.saturating_sub(9))));
        let idx_col = _to_usize(GLOBAL_DATA, (74 % (g_len.saturating_sub(9))));
        let _ = view_ro.index(idx_row);
        let _ = view_ro.index((idx_col, idx_row));

        {
            let mut vm = td.view_mut(start2, end2);
            let mut rm = vm.rows_mut();
            let _ = rm.next();
            let _ = rm.nth(_to_usize(GLOBAL_DATA, (82 % (g_len.saturating_sub(9)))));
            let _ = rm.nth_back(_to_usize(SECOND, (42 % (s_len.saturating_sub(9)))));
            let _ = rm.last();
            let mut cm = vm.col_mut(_to_usize(GLOBAL_DATA, (90 % (g_len.saturating_sub(9)))));
            let _ = cm.next();
            let _ = cm.nth(_to_usize(GLOBAL_DATA, (98 % (g_len.saturating_sub(9)))));
            let _ = cm.nth_back(_to_usize(SECOND, (50 % (s_len.saturating_sub(9)))));
            let _ = cm.last();
            let col_idx = _to_usize(GLOBAL_DATA, (106 % (g_len.saturating_sub(9))));
            let _ = vm.col(col_idx);
            let irow = _to_usize(GLOBAL_DATA, (114 % (g_len.saturating_sub(9))));
            let icol = _to_usize(SECOND, (58 % (s_len.saturating_sub(9))));
            let _ = vm.index(irow);
            let _ = vm.index((icol, irow));
            vm.swap_rows(_to_usize(GLOBAL_DATA, (122 % (g_len.saturating_sub(9)))), _to_usize(SECOND, (66 % (s_len.saturating_sub(9)))));
            let vstart_a = (_to_usize(GLOBAL_DATA, (130 % (g_len.saturating_sub(9)))), _to_usize(GLOBAL_DATA, (138 % (g_len.saturating_sub(9)))));
            let vend_a = (_to_usize(SECOND, (74 % (s_len.saturating_sub(9)))), _to_usize(SECOND, (82 % (s_len.saturating_sub(9)))));
            let mut nested1 = vm.view_mut(vstart_a, vend_a);
            let vstart_b = (_to_usize(GLOBAL_DATA, (146 % (g_len.saturating_sub(9)))), _to_usize(GLOBAL_DATA, (154 % (g_len.saturating_sub(9)))));
            let vend_b = (_to_usize(SECOND, (90 % (s_len.saturating_sub(9)))), _to_usize(SECOND, (98 % (s_len.saturating_sub(9)))));
            let _nested2 = nested1.view_mut(vstart_b, vend_b);
            let rv = vm.view((_to_usize(GLOBAL_DATA, (162 % (g_len.saturating_sub(9)))), _to_usize(GLOBAL_DATA, (170 % (g_len.saturating_sub(9))))), (_to_usize(SECOND, (106 % (s_len.saturating_sub(9)))), _to_usize(SECOND, (114 % (s_len.saturating_sub(9))))));
            let _ = rv.rows();
            let _ = rv.col(_to_usize(GLOBAL_DATA, (178 % (g_len.saturating_sub(9)))));
        }

        let mut ops = (_to_u8(GLOBAL_DATA, 186) % 12) as usize;
        let mut pc = 0usize;
        while ops > 0 && g_len > 9 && s_len > 9 {
            let tag = _to_u8(GLOBAL_DATA, (pc % g_len)) % 8;
            match tag {
                0 => {
                    let vs = (_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))), _to_usize(GLOBAL_DATA, ((pc + 16) % (g_len - 8))));
                    let ve = (_to_usize(SECOND, ((pc + 24) % (s_len - 8))), _to_usize(SECOND, ((pc + 32) % (s_len - 8))));
                    let mut vm = td.view_mut(vs, ve);
                    let nvs = (_to_usize(GLOBAL_DATA, ((pc + 40) % (g_len - 8))), _to_usize(GLOBAL_DATA, ((pc + 48) % (g_len - 8))));
                    let nve = (_to_usize(SECOND, ((pc + 56) % (s_len - 8))), _to_usize(SECOND, ((pc + 64) % (s_len - 8))));
                    let _ = vm.view_mut(nvs, nve);
                }
                1 => {
                    td.swap_rows(_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))), _to_usize(SECOND, ((pc + 16) % (s_len - 8))));
                }
                2 => {
                    let v = td.view((_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))), _to_usize(GLOBAL_DATA, ((pc + 16) % (g_len - 8)))) , (_to_usize(SECOND, ((pc + 24) % (s_len - 8))), _to_usize(SECOND, ((pc + 32) % (s_len - 8)))));
                    let mut rs = v.rows();
                    let _ = rs.next();
                    let _ = rs.nth(_to_usize(GLOBAL_DATA, ((pc + 40) % (g_len - 8))));
                }
                3 => {
                    let mut vm = td.view_mut((_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))), _to_usize(GLOBAL_DATA, ((pc + 16) % (g_len - 8)))) , (_to_usize(SECOND, ((pc + 24) % (s_len - 8))), _to_usize(SECOND, ((pc + 32) % (s_len - 8)))));
                    let mut cm = vm.col_mut(_to_usize(GLOBAL_DATA, ((pc + 40) % (g_len - 8))));
                    if let Some(_) = cm.next() {}
                }
                4 => {
                    let mut c = td.col(_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))));
                    let _ = c.next_back();
                }
                5 => {
                    let mut rm = td.rows_mut();
                    let _ = rm.next_back();
                }
                6 => {
                    let _ = td.pop_col();
                }
                7 => {
                    let dr = td.remove_col(_to_usize(GLOBAL_DATA, ((pc + 8) % (g_len - 8))));
                    let mut it = dr;
                    let _ = it.next();
                    let _ = it.next_back();
                }
                _ => {}
            }
            pc = pc.wrapping_add(73);
            ops -= 1;
        }

        let vcols = td.num_cols();
        let vrows = td.num_rows();
        let d_mut = td.data_mut();
        let mut tvm = toodee::TooDeeViewMut::new(vcols, vrows, d_mut);
        let vm_start = (_to_usize(GLOBAL_DATA, (194 % (g_len.saturating_sub(9)))), _to_usize(GLOBAL_DATA, (202 % (g_len.saturating_sub(9)))));
        let vm_end = (_to_usize(SECOND, (122 % (s_len.saturating_sub(9)))), _to_usize(SECOND, (130 % (s_len.saturating_sub(9)))));
        let mut tvm2 = tvm.view_mut(vm_start, vm_end);
        let vm_start2 = (_to_usize(GLOBAL_DATA, (210 % (g_len.saturating_sub(9)))), _to_usize(GLOBAL_DATA, (218 % (g_len.saturating_sub(9)))));
        let vm_end2 = (_to_usize(SECOND, (138 % (s_len.saturating_sub(9)))), _to_usize(SECOND, (146 % (s_len.saturating_sub(9)))));
        let _ = tvm2.view_mut(vm_start2, vm_end2);

        let cells = td.cells();
        let mut cells2 = cells;
        let _ = cells2.next();
        let _ = cells2.nth(_to_usize(GLOBAL_DATA, (226 % (g_len.saturating_sub(9)))));
        let _ = cells2.next_back();

        let pr = _to_usize(GLOBAL_DATA, (234 % (g_len.saturating_sub(9))));
        let pc2 = _to_usize(GLOBAL_DATA, (242 % (g_len.saturating_sub(9))));
        let _ = td.index((pc2, pr));
        let _ = td.index_mut((pc2, pr));
        let _ = td.index(pr);
        let _ = td.index_mut(pr);
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