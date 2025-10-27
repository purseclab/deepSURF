#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let FH = global_data.first_half;
        let SH = global_data.second_half;

        let mut vec_data: Vec<CustomType0> = Vec::with_capacity(65);
        let mut base = if FH.len() > 0 { _to_u8(FH, 0) as usize } else { 0 };
        let mut vlen = if FH.len() > 0 { (_to_u8(FH, FH.len()/2) % 65) as usize } else { 0 };
        if vlen == 0 { vlen = 1; }
        for i in 0..vlen {
            let lidx = if FH.len() > 0 { (base + i * 7) % FH.len() } else { 0 };
            let ridx = if FH.len() > 0 { (lidx + 1 + (_to_u8(FH, lidx) as usize % 32)) % FH.len() } else { 0 };
            let (start, end) = if lidx < ridx { (lidx, ridx) } else { (ridx, lidx) };
            let end = if end <= start { FH.len() } else { end };
            let end = if end == start { FH.len() } else { end };
            let end = if end > FH.len() { FH.len() } else { end };
            let s = if end > start { _to_str(FH, start, end) } else { _to_str(FH, 0, FH.len()) };
            vec_data.push(CustomType0(String::from(s)));
        }
        let slice_ref = &vec_data[..];

        let idx_c0 = if FH.len() > 8 { (_to_u8(FH, 1) as usize) % (FH.len() - 8) } else { 0 };
        let idx_r0 = if FH.len() > 8 { (_to_u8(FH, 2) as usize) % (FH.len() - 8) } else { 0 };
        let c0 = _to_usize(FH, idx_c0);
        let r0 = _to_usize(FH, idx_r0);
        let view_new = TooDeeView::new(c0, r0, slice_ref);

        let s1_idx = if FH.len() > 8 { (_to_u8(FH, 3) as usize) % (FH.len() - 8) } else { 0 };
        let s2_idx = if FH.len() > 8 { (_to_u8(FH, 4) as usize) % (FH.len() - 8) } else { 0 };
        let e1_idx = if FH.len() > 8 { (_to_u8(FH, 5) as usize) % (FH.len() - 8) } else { 0 };
        let e2_idx = if FH.len() > 8 { (_to_u8(FH, 6) as usize) % (FH.len() - 8) } else { 0 };
        let mut current_view = view_new.view((_to_usize(FH, s1_idx), _to_usize(FH, s2_idx)), (_to_usize(FH, e1_idx), _to_usize(FH, e2_idx)));

        let td_cols_idx = if SH.len() > 8 { (_to_u8(SH, 0) as usize) % (SH.len() - 8) } else { 0 };
        let td_rows_idx = if SH.len() > 8 { (_to_u8(SH, 1) as usize) % (SH.len() - 8) } else { 0 };
        let cols_b = _to_usize(SH, td_cols_idx);
        let rows_b = _to_usize(SH, td_rows_idx);
        let selector = if SH.len() > 0 { _to_u8(SH, 2) } else { 0 };
        let mut todee: TooDee<CustomType0> = if selector % 2 == 0 {
            TooDee::from_vec(cols_b, rows_b, vec_data.clone())
        } else {
            TooDee::from_box(cols_b, rows_b, vec_data.clone().into_boxed_slice())
        };

        let todee_ro = todee.clone();
        let vtd_s1_idx = if SH.len() > 8 { (_to_u8(SH, 3) as usize) % (SH.len() - 8) } else { 0 };
        let vtd_s2_idx = if SH.len() > 8 { (_to_u8(SH, 4) as usize) % (SH.len() - 8) } else { 0 };
        let vtd_e1_idx = if SH.len() > 8 { (_to_u8(SH, 5) as usize) % (SH.len() - 8) } else { 0 };
        let vtd_e2_idx = if SH.len() > 8 { (_to_u8(SH, 6) as usize) % (SH.len() - 8) } else { 0 };
        let mut view_from_td = todee_ro.view((_to_usize(SH, vtd_s1_idx), _to_usize(SH, vtd_s2_idx)), (_to_usize(SH, vtd_e1_idx), _to_usize(SH, vtd_e2_idx)));

        {
            let mut r_iter = current_view.rows();
            let nth_idx = if SH.len() > 8 { (_to_u8(SH, 7) as usize) % (SH.len() - 8) } else { 0 };
            let _ = r_iter.nth(_to_usize(SH, nth_idx));
            if let Some(row) = r_iter.next() {
                let first = row.get(0);
                println!("{:?}", first.map(|x| &x.0));
            }
        }
        {
            let mut c_iter = current_view.col(if SH.len() > 8 { _to_usize(SH, (_to_u8(SH, 8) as usize) % (SH.len() - 8)) } else { 0 });
            let _ = c_iter.next();
            let back = c_iter.next_back();
            println!("{:?}", back.map(|x| x));
        }

        let op_count = if SH.len() > 0 { (1 + (_to_u8(SH, 9) % 12)) as usize } else { 1 };
        for i in 0..op_count {
            let dispatch = if SH.len() > 0 { _to_u8(SH, (10 + i) % SH.len()) % 10 } else { 0 };
            match dispatch {
                0 => {
                    let si1 = if FH.len() > 8 { (_to_u8(FH, (11 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let si2 = if FH.len() > 8 { (_to_u8(FH, (12 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let ei1 = if FH.len() > 8 { (_to_u8(FH, (13 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let ei2 = if FH.len() > 8 { (_to_u8(FH, (14 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    current_view = view_new.view((_to_usize(FH, si1), _to_usize(FH, si2)), (_to_usize(FH, ei1), _to_usize(FH, ei2)));
                }
                1 => {
                    let ri = if SH.len() > 8 { (_to_u8(SH, (15 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let mut r = current_view.rows();
                    let _ = r.nth(_to_usize(SH, ri));
                    if let Some(row) = r.last() {
                        println!("{:?}", row.get(0).map(|x| &x.0));
                    }
                }
                2 => {
                    let ci_idx = if SH.len() > 8 { (_to_u8(SH, (16 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let mut c = current_view.col(_to_usize(SH, ci_idx));
                    let _ = c.nth(if SH.len() > 8 { _to_usize(SH, ci_idx) } else { 0 });
                    let _ = c.nth_back(if SH.len() > 8 { _to_usize(SH, ci_idx) } else { 0 });
                }
                3 => {
                    let idx_row = if FH.len() > 8 { (_to_u8(FH, (17 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let row_ref = &current_view[_to_usize(FH, idx_row)];
                    println!("{:?}", row_ref.get(0).map(|x| &x.0));
                }
                4 => {
                    let ic_idx = if FH.len() > 8 { (_to_u8(FH, (18 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let ir_idx = if FH.len() > 8 { (_to_u8(FH, (19 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let coord = (_to_usize(FH, ic_idx), _to_usize(FH, ir_idx));
                    let cell_ref = &current_view[coord];
                    println!("{:?}", &cell_ref.0);
                }
                5 => {
                    let vsi1 = if SH.len() > 8 { (_to_u8(SH, (20 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let vsi2 = if SH.len() > 8 { (_to_u8(SH, (21 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let vei1 = if SH.len() > 8 { (_to_u8(SH, (22 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let vei2 = if SH.len() > 8 { (_to_u8(SH, (23 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let mut vmut = todee.view_mut((_to_usize(SH, vsi1), _to_usize(SH, vsi2)), (_to_usize(SH, vei1), _to_usize(SH, vei2)));
                    let mut rm = vmut.rows_mut();
                    let nthm_idx = if FH.len() > 8 { (_to_u8(FH, (24 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let _ = rm.nth(_to_usize(FH, nthm_idx));
                    let mut cm = vmut.col_mut(if FH.len() > 8 { _to_usize(FH, nthm_idx) } else { 0 });
                    let _ = cm.next();
                    let _ = cm.nth_back(if FH.len() > 8 { _to_usize(FH, nthm_idx) } else { 0 });
                }
                6 => {
                    let s1 = if SH.len() > 8 { (_to_u8(SH, (25 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let s2 = if SH.len() > 8 { (_to_u8(SH, (26 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    view_from_td = todee_ro.view((_to_usize(SH, s1), _to_usize(SH, s2)), (_to_usize(SH, s2), _to_usize(SH, s1)));
                }
                7 => {
                    let ci = if FH.len() > 8 { (_to_u8(FH, (27 + i) % FH.len()) as usize) % (FH.len() - 8) } else { 0 };
                    let mut c = todee.col(_to_usize(FH, ci));
                    let _ = c.next_back();
                    let _ = c.nth(if FH.len() > 8 { _to_usize(FH, ci) } else { 0 });
                }
                8 => {
                    let pr = if SH.len() > 8 { (_to_u8(SH, (28 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let pc = if SH.len() > 8 { (_to_u8(SH, (29 + i) % SH.len()) as usize) % (SH.len() - 8) } else { 0 };
                    let _ = todee.pop_col();
                    let _ = todee.remove_col(_to_usize(SH, pc));
                    let _ = todee.rows();
                    let _ = todee.col(_to_usize(SH, pr));
                }
                9 => {
                    let tdv: TooDee<CustomType0> = TooDee::from(current_view.clone());
                    let mut it = tdv.rows();
                    let _ = it.next_back();
                }
                _ => {}
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