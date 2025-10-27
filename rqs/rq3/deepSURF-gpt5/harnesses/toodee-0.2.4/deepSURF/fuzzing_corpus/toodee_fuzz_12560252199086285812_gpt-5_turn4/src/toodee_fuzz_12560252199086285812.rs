#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default, Debug)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 96 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let mut fi: usize = 0;
        let capacity = _to_usize(first, fi); fi += 8;

        let mut t: TooDee<CustomType0>;
        let sel = first[fi % first.len()]; fi = (fi + 1) % first.len();
        match sel % 3 {
            0 => {
                t = toodee::TooDee::with_capacity(capacity);
            }
            1 => {
                if fi + 8 > first.len() { fi = 0; }
                let c = _to_usize(first, fi); fi += 8;
                if fi + 8 > first.len() { fi = 0; }
                let r = _to_usize(first, fi); fi += 8;
                t = toodee::TooDee::new(c, r);
            }
            _ => {
                if fi + 8 > first.len() { fi = 0; }
                let c = _to_usize(first, fi); fi += 8;
                if fi + 8 > first.len() { fi = 0; }
                let r = _to_usize(first, fi); fi += 8;
                let s = if second.len() >= 2 {
                    let a = second[0] as usize;
                    let b = second[1] as usize;
                    let start = a % second.len();
                    let mut end = start + 1 + (b % (second.len() - start));
                    if end > second.len() { end = second.len(); }
                    _to_str(second, start, end).to_string()
                } else {
                    String::new()
                };
                t = toodee::TooDee::init(c, r, CustomType0(s));
            }
        }

        let mut si: usize = 0;
        let row_len = (second[0] as usize % 65) + 1;
        let mut row: Vec<CustomType0> = Vec::new();
        for _ in 0..row_len {
            if second.len() >= 2 {
                let a = second[si % second.len()] as usize;
                let b = second[(si + 1) % second.len()] as usize;
                si = (si + 2) % second.len();
                let start = a % second.len();
                let mut end = start + 1 + (b % (second.len() - start));
                if end > second.len() { end = second.len(); }
                let s = _to_str(second, start, end).to_string();
                row.push(CustomType0(s));
            } else {
                row.push(CustomType0(String::new()));
            }
        }
        let rows_to_push = (first[fi % first.len()] as usize % 4);
        for _ in 0..rows_to_push {
            t.push_row(row.clone());
        }

        let rows_now = t.num_rows();
        if rows_now > 0 {
            let mut colv: Vec<CustomType0> = Vec::new();
            for _ in 0..rows_now {
                if second.len() >= 2 {
                    let a = second[si % second.len()] as usize;
                    let b = second[(si + 1) % second.len()] as usize;
                    si = (si + 2) % second.len();
                    let start = a % second.len();
                    let mut end = start + 1 + (b % (second.len() - start));
                    if end > second.len() { end = second.len(); }
                    let s = _to_str(second, start, end).to_string();
                    colv.push(CustomType0(s));
                } else {
                    colv.push(CustomType0(String::new()));
                }
            }
            t.push_col(colv);
        }

        if fi + 8 > first.len() { fi = 0; }
        let ins_row_idx = _to_usize(first, fi); fi += 8;
        let cols_now = t.num_cols();
        if cols_now > 0 {
            let mut r2: Vec<CustomType0> = Vec::new();
            for _ in 0..cols_now {
                if second.len() >= 2 {
                    let a = second[si % second.len()] as usize;
                    let b = second[(si + 1) % second.len()] as usize;
                    si = (si + 2) % second.len();
                    let start = a % second.len();
                    let mut end = start + 1 + (b % (second.len() - start));
                    if end > second.len() { end = second.len(); }
                    let s = _to_str(second, start, end).to_string();
                    r2.push(CustomType0(s));
                } else {
                    r2.push(CustomType0(String::new()));
                }
            }
            t.insert_row(ins_row_idx, r2);
        }

        if fi + 8 > first.len() { fi = 0; }
        let ins_col_idx = _to_usize(first, fi); fi += 8;
        let rows_now2 = t.num_rows();
        if rows_now2 > 0 {
            let mut c2: Vec<CustomType0> = Vec::new();
            for _ in 0..rows_now2 {
                if second.len() >= 2 {
                    let a = second[si % second.len()] as usize;
                    let b = second[(si + 1) % second.len()] as usize;
                    si = (si + 2) % second.len();
                    let start = a % second.len();
                    let mut end = start + 1 + (b % (second.len() - start));
                    if end > second.len() { end = second.len(); }
                    let s = _to_str(second, start, end).to_string();
                    c2.push(CustomType0(s));
                } else {
                    c2.push(CustomType0(String::new()));
                }
            }
            t.insert_col(ins_col_idx, c2);
        }

        if fi + 8 > first.len() { fi = 0; }
        let r1 = _to_usize(first, fi); fi += 8;
        if fi + 8 > first.len() { fi = 0; }
        let r2 = _to_usize(first, fi); fi += 8;
        t.swap_rows(r1, r2);

        let ops = (first[fi % first.len()] % 16) as usize;
        for _ in 0..ops {
            if fi + 8 > first.len() { fi = 0; }
            let col_index = _to_usize(first, fi); fi += 8;
            {
                let mut cm = t.col_mut(col_index);
                if fi + 8 > first.len() { fi = 0; }
                let n = _to_usize(first, fi); fi += 8;
                if let Some(v) = cm.nth(n) { println!("{:?}", v); }
                if fi + 8 > first.len() { fi = 0; }
                let nb = _to_usize(first, fi); fi += 8;
                if let Some(vb) = cm.nth_back(nb) { println!("{:?}", vb); }
                if let Some(lv) = cm.last() { println!("{:?}", lv); }
            }
            {
                let mut cm2 = t.col_mut(col_index);
                if fi + 8 > first.len() { fi = 0; }
                let p = _to_usize(first, fi); fi += 8;
                println!("{:?}", &cm2[p]);
                if fi + 8 > first.len() { fi = 0; }
                let p2 = _to_usize(first, fi); fi += 8;
                let e = &mut cm2[p2];
                println!("{:?}", e);
            }
        }

        if fi + 8 > first.len() { fi = 0; }
        let cidx = _to_usize(first, fi); fi += 8;
        {
            let mut c = t.col(cidx);
            let _ = c.next();
            if fi + 8 > first.len() { fi = 0; }
            let nthc = _to_usize(first, fi); fi += 8;
            let _ = c.nth(nthc);
            let _ = c.last();
        }

        if fi + 8 > first.len() { fi = 0; }
        let rem = _to_usize(first, fi); fi += 8;
        {
            let mut dc = t.remove_col(rem);
            let _ = dc.next();
            let _ = dc.next_back();
        }

        let v = t.view((0, 0), (t.num_cols(), t.num_rows()));
        let _ = v.rows();
        let _ = v.col(cidx);
        let _ = &v[(0, 0)];
        println!("{:?}", &v[(0, 0)]);

        let mut vm = t.view_mut((0, 0), (t.num_cols(), t.num_rows()));
        let _ = vm.rows_mut();
        if fi + 8 > first.len() { fi = 0; }
        let vmc = _to_usize(first, fi); fi += 8;
        {
            let mut cm = vm.col_mut(vmc);
            let _ = cm.next();
        }
        let v_from_vm: TooDeeView<'_, CustomType0> = vm.into();
        let _t2: TooDee<CustomType0> = TooDee::from(v_from_vm);
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