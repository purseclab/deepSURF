#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let SECOND = global_data.second_half;
        if SECOND.is_empty() { return; }

        let cols0 = _to_usize(GLOBAL_DATA, 0);
        let rows0 = _to_usize(GLOBAL_DATA, 8);
        let cols1 = _to_usize(GLOBAL_DATA, 16);
        let rows1 = _to_usize(GLOBAL_DATA, 24);
        let cols2 = _to_usize(GLOBAL_DATA, 32);
        let rows2 = _to_usize(GLOBAL_DATA, 40);
        let with_cap = _to_usize(GLOBAL_DATA, 48);
        let n_vec = (_to_u8(GLOBAL_DATA, 56) % 65) as usize;

        let mut base_vec: Vec<CustomType0> = Vec::with_capacity(65);
        for i in 0..n_vec {
            let start = (60 + i * 3) % GLOBAL_DATA.len();
            let mut slen = _to_u8(GLOBAL_DATA, (120 + i) % GLOBAL_DATA.len()) as usize;
            slen %= 20;
            let end = std::cmp::min(GLOBAL_DATA.len(), start + slen);
            let s = _to_str(GLOBAL_DATA, start, end);
            let s_owned = String::from_str(s).unwrap_or_default();
            base_vec.push(CustomType0(s_owned));
        }
        if base_vec.is_empty() {
            base_vec.push(CustomType0(String::new()));
        }
        base_vec.truncate((_to_u8(GLOBAL_DATA, 200) % 65) as usize);

        let ctor_mode = _to_u8(GLOBAL_DATA, 64) % 5;
        let mut td: TooDee<CustomType0> = match ctor_mode {
            0 => TooDee::new(cols0, rows0),
            1 => {
                let init_idx = (72 + (cols0 as usize % GLOBAL_DATA.len())) % GLOBAL_DATA.len();
                let s2 = _to_str(GLOBAL_DATA, init_idx, std::cmp::min(GLOBAL_DATA.len(), init_idx + ((_to_u8(GLOBAL_DATA, 73) % 20) as usize)));
                TooDee::init(cols1, rows1, CustomType0(String::from(s2)))
            }
            2 => TooDee::from_vec(cols2, rows2, base_vec.clone()),
            3 => TooDee::from_box(_to_usize(GLOBAL_DATA, 80), _to_usize(GLOBAL_DATA, 88), base_vec.clone().into_boxed_slice()),
            _ => TooDee::with_capacity(with_cap),
        };

        let v0 = td.view((_to_usize(GLOBAL_DATA, 96), _to_usize(GLOBAL_DATA, 104)), (_to_usize(GLOBAL_DATA, 112), _to_usize(GLOBAL_DATA, 120)));
        let _ = v0.num_cols();
        let _ = v0.num_rows();
        let idx_row = _to_usize(GLOBAL_DATA, 128);
        if v0.num_rows() > 0 {
            let rref = &v0[idx_row];
            println!("{:?}", rref.len());
        }

        let mut rm = td.rows_mut();
        let _ = rm.nth(_to_usize(GLOBAL_DATA, 136));
        let _ = rm.nth_back(_to_usize(GLOBAL_DATA, 144));

        let mut cm = td.col_mut(_to_usize(GLOBAL_DATA, 152));
        if let Some(cell) = cm.nth(_to_usize(GLOBAL_DATA, 160)) {
            println!("{:?}", &*cell);
        }
        let _ = cm.nth_back(_to_usize(GLOBAL_DATA, 168));

        td.swap_rows(_to_usize(GLOBAL_DATA, 176), _to_usize(GLOBAL_DATA, 184));

        let mk_view_from_td = _to_bool(GLOBAL_DATA, 192);
        let mut scratch = base_vec.clone();
        scratch.truncate((_to_u8(GLOBAL_DATA, 193) % 65) as usize);
        if scratch.is_empty() {
            scratch.push(CustomType0(String::new()));
        }

        let mut tvm = if mk_view_from_td {
            td.view_mut((_to_usize(GLOBAL_DATA, 194), _to_usize(GLOBAL_DATA, 202)), (_to_usize(GLOBAL_DATA, 210), _to_usize(GLOBAL_DATA, 218)))
        } else {
            TooDeeViewMut::new(_to_usize(GLOBAL_DATA, 226), _to_usize(GLOBAL_DATA, 234), &mut scratch[..])
        };

        {
            let rows = tvm.rows();
            let mut riter = rows;
            let _ = riter.count();
            let rview = tvm.view((_to_usize(GLOBAL_DATA, 242), _to_usize(GLOBAL_DATA, 250)), (_to_usize(GLOBAL_DATA, 258), _to_usize(GLOBAL_DATA, 266)));
            let _ = rview.num_cols();
            let _ = rview.num_rows();
            let c = tvm.col(_to_usize(GLOBAL_DATA, 274));
            let mut cit = c;
            let _ = cit.last();
            let _ = tvm.rows_mut().last();
            let _ = tvm.rows_mut().nth(_to_usize(GLOBAL_DATA, 282));
            { let _ = tvm.col_mut(_to_usize(GLOBAL_DATA, 290)).last(); }
            {
                let mut cmm2 = tvm.col_mut(_to_usize(GLOBAL_DATA, 290));
                if let Some(elt) = cmm2.next() {
                    println!("{:?}", &*elt);
                }
            }
            tvm.swap_rows(_to_usize(GLOBAL_DATA, 298), _to_usize(GLOBAL_DATA, 306));
        }

        let mut td2 = TooDee::<CustomType0>::from(tvm);

        let op_count = (_to_u8(GLOBAL_DATA, 314) % 16) as usize + 1;
        for i in 0..op_count {
            let sel = _to_u8(SECOND, i % SECOND.len());
            match sel % 12 {
                0 => {
                    let v = td2.view((_to_usize(GLOBAL_DATA, 322 + i), _to_usize(GLOBAL_DATA, 330 + i)), (_to_usize(GLOBAL_DATA, 338 + i), _to_usize(GLOBAL_DATA, 346 + i)));
                    let _ = v.num_cols();
                    let _ = v.num_rows();
                    let c = v.col(_to_usize(GLOBAL_DATA, 354 + i));
                    let mut it = c;
                    let _ = it.nth(_to_usize(GLOBAL_DATA, 362 + i));
                }
                1 => {
                    let mut r = td2.rows_mut();
                    if let Some(row) = r.next() {
                        if !row.is_empty() {
                            println!("{:?}", &row[0]);
                        }
                    }
                }
                2 => {
                    let mut c = td2.col_mut(_to_usize(GLOBAL_DATA, 370 + i));
                    let _ = c.nth_back(_to_usize(GLOBAL_DATA, 378 + i));
                }
                3 => {
                    td2.swap_rows(_to_usize(GLOBAL_DATA, 386 + i), _to_usize(GLOBAL_DATA, 394 + i));
                }
                4 => {
                    let mut vrow: Vec<CustomType0> = Vec::with_capacity(65);
                    let want = ((td2.num_cols()) % 65) as usize;
                    for j in 0..want {
                        let sidx = (402 + i + j) % GLOBAL_DATA.len();
                        let e = std::cmp::min(GLOBAL_DATA.len(), sidx + ((_to_u8(GLOBAL_DATA, (410 + i + j) % GLOBAL_DATA.len()) % 10) as usize));
                        let s = _to_str(GLOBAL_DATA, sidx, e);
                        vrow.push(CustomType0(String::from(s)));
                    }
                    td2.push_row(vrow);
                }
                5 => {
                    let mut vcol: Vec<CustomType0> = Vec::with_capacity(65);
                    let want = ((td2.num_rows()) % 65) as usize;
                    for j in 0..want {
                        let sidx = (418 + i + j) % GLOBAL_DATA.len();
                        let e = std::cmp::min(GLOBAL_DATA.len(), sidx + ((_to_u8(GLOBAL_DATA, (426 + i + j) % GLOBAL_DATA.len()) % 10) as usize));
                        let s = _to_str(GLOBAL_DATA, sidx, e);
                        vcol.push(CustomType0(String::from(s)));
                    }
                    td2.insert_col(_to_usize(GLOBAL_DATA, 434 + i), vcol);
                }
                6 => {
                    let _ = td2.pop_col();
                }
                7 => {
                    let dr = td2.remove_col(_to_usize(GLOBAL_DATA, 442 + i));
                    let mut it = dr;
                    let _ = it.next_back();
                }
                8 => {
                    let v = td2.view((_to_usize(GLOBAL_DATA, 450 + i), _to_usize(GLOBAL_DATA, 458 + i)), (_to_usize(GLOBAL_DATA, 466 + i), _to_usize(GLOBAL_DATA, 474 + i)));
                    let idx = (_to_usize(GLOBAL_DATA, 482 + i), _to_usize(GLOBAL_DATA, 490 + i));
                    let r = &v[idx];
                    println!("{:?}", r.0.len());
                }
                9 => {
                    let mut vm = td2.view_mut((_to_usize(GLOBAL_DATA, 498 + i), _to_usize(GLOBAL_DATA, 506 + i)), (_to_usize(GLOBAL_DATA, 514 + i), _to_usize(GLOBAL_DATA, 522 + i)));
                    let _ = vm.rows_mut().nth(_to_usize(GLOBAL_DATA, 530 + i));
                    let mut c = vm.col_mut(_to_usize(GLOBAL_DATA, 538 + i));
                    if let Some(x) = c.next_back() {
                        println!("{:?}", &*x);
                    }
                    let _td3 = TooDee::<CustomType0>::from(vm);
                }
                10 => {
                    let mut vcol: Vec<CustomType0> = Vec::with_capacity(65);
                    let want = ((td2.num_rows()) % 65) as usize;
                    for j in 0..want {
                        let sidx = (546 + i + j) % GLOBAL_DATA.len();
                        let e = std::cmp::min(GLOBAL_DATA.len(), sidx + ((_to_u8(GLOBAL_DATA, (554 + i + j) % GLOBAL_DATA.len()) % 10) as usize));
                        let s = _to_str(GLOBAL_DATA, sidx, e);
                        vcol.push(CustomType0(String::from(s)));
                    }
                    td2.push_col(vcol);
                }
                _ => {
                    let v = td2.rows();
                    let mut it = v;
                    let _ = it.nth_back(_to_usize(GLOBAL_DATA, 562 + i));
                }
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