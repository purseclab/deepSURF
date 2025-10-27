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
        if data.len() < 1202 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let SECOND_DATA = global_data.second_half;

        let count = (_to_u8(GLOBAL_DATA, 0) % 65) as usize;
        let mut v = std::vec::Vec::with_capacity(65);
        for i in 0..count {
            let l = (_to_u8(GLOBAL_DATA, 1 + i) % 31) as usize;
            let start = 50 + i * 7;
            let end = start + l as usize;
            let s = _to_str(GLOBAL_DATA, start, end);
            let s_owned = String::from(s);
            let c = CustomType0(s_owned);
            v.push(c);
        }

        let cols = _to_usize(GLOBAL_DATA, 64);
        let rows = _to_usize(GLOBAL_DATA, 72);
        let mut td = toodee::TooDee::from_vec(cols, rows, v);

        let s0 = _to_usize(GLOBAL_DATA, 80);
        let s1 = _to_usize(GLOBAL_DATA, 88);
        let e0 = _to_usize(GLOBAL_DATA, 96);
        let e1 = _to_usize(GLOBAL_DATA, 104);
        let mut viewm = toodee::TooDee::view_mut(&mut td, (s0, s1), (e0, e1));

        let uvals = [
            _to_usize(GLOBAL_DATA, 112),
            _to_usize(GLOBAL_DATA, 120),
            _to_usize(GLOBAL_DATA, 128),
            _to_usize(GLOBAL_DATA, 136),
            _to_usize(GLOBAL_DATA, 144),
            _to_usize(GLOBAL_DATA, 152),
            _to_usize(GLOBAL_DATA, 160),
            _to_usize(GLOBAL_DATA, 168),
            _to_usize(GLOBAL_DATA, 176),
            _to_usize(GLOBAL_DATA, 184),
            _to_usize(GLOBAL_DATA, 192),
            _to_usize(GLOBAL_DATA, 200),
            _to_usize(GLOBAL_DATA, 208),
            _to_usize(GLOBAL_DATA, 216),
            _to_usize(GLOBAL_DATA, 224),
            _to_usize(GLOBAL_DATA, 232),
        ];

        let ops = (_to_u8(GLOBAL_DATA, 240) % 16) as usize;
        for i in 0..ops {
            let sel = _to_u8(SECOND_DATA, i % SECOND_DATA.len());
            match sel % 6 {
                0 => {
                    let r = (&viewm).index(uvals[i % 16]);
                    println!("{}", r.len());
                    if let Some(first) = r.get(0) {
                        println!("{:?}", first);
                    }
                }
                1 => {
                    let mut it = viewm.rows_mut();
                    let _ = it.nth(uvals[(i + 1) % 16]);
                    let _ = it.nth_back(uvals[(i + 2) % 16]);
                    let _ = it.last();
                }
                2 => {
                    let mut c = viewm.col_mut(uvals[(i + 3) % 16]);
                    let _ = c.nth(uvals[(i + 4) % 16]);
                    let _ = c.nth_back(uvals[(i + 5) % 16]);
                    let _ = c.last();
                }
                3 => {
                    let vimm = (&viewm).view(
                        (uvals[(i + 6) % 16], uvals[(i + 7) % 16]),
                        (uvals[(i + 8) % 16], uvals[(i + 9) % 16]),
                    );
                    let mut r = vimm.rows();
                    let _ = r.next();
                    let _ = r.next_back();
                    let _ = r.nth(uvals[(i + 10) % 16]);
                    let _ = r.nth_back(uvals[(i + 11) % 16]);
                    let _ = r.last();
                    let c = vimm.col(uvals[(i + 12) % 16]);
                    let _ = c.last();
                }
                4 => {
                    viewm.swap_rows(uvals[(i + 13) % 16], uvals[(i + 14) % 16]);
                }
                _ => {
                    let r2 = (&viewm).index((uvals[(i + 15) % 16], uvals[i % 16]));
                    println!("{:?}", r2);
                }
            }
        }

        let mut c = (&td).col(uvals[0]);
        let _ = c.next();
        let _ = c.nth(uvals[1]);
        let _ = c.nth_back(uvals[2]);
        let _ = c.last();

        let mut r = (&td).rows();
        let _ = r.next();
        let _ = r.nth(uvals[3]);
        let _ = r.nth_back(uvals[4]);
        let _ = r.last();

        let mut rm = (&mut td).rows_mut();
        let _ = rm.next();
        let _ = rm.nth(uvals[5]);
        let _ = rm.next_back();
        let _ = rm.last();

        let mut cells = (&td).cells();
        let _ = cells.next();
        let _ = cells.nth(uvals[6]);
        let _ = cells.next_back();
        let _ = cells.last();

        let mut v2 = std::vec::Vec::with_capacity(65);
        let count2 = (_to_u8(SECOND_DATA, 0) % 65) as usize;
        for i in 0..count2 {
            let l = (_to_u8(SECOND_DATA, 1 + i) % 23) as usize;
            let start = 30 + i * 3;
            let end = start + l as usize;
            let s = _to_str(SECOND_DATA, start, end);
            let s_owned = String::from(s);
            v2.push(CustomType0(s_owned));
        }
        let mut slice2 = v2.as_mut_slice();
        let c2 = _to_usize(SECOND_DATA, 8);
        let r2 = _to_usize(SECOND_DATA, 16);
        let mut viewm2 = toodee::TooDeeViewMut::new(c2, r2, &mut slice2);

        let rr = (&viewm2).index(_to_usize(SECOND_DATA, 24));
        println!("{}", rr.len());
        if let Some(first) = rr.get(0) {
            println!("{:?}", first);
        }

        let rrm = viewm2.index_mut(_to_usize(SECOND_DATA, 32));
        println!("{}", rrm.len());

        let vimm2 = (&viewm2).view(
            (_to_usize(SECOND_DATA, 40), _to_usize(SECOND_DATA, 48)),
            (_to_usize(SECOND_DATA, 56), _to_usize(SECOND_DATA, 64)),
        );
        let mut r3 = vimm2.rows();
        let _ = r3.next();
        let _ = r3.nth(_to_usize(SECOND_DATA, 72));
        let _ = r3.last();

        let v_from_mut: toodee::TooDeeView<CustomType0> = toodee::TooDeeViewMut::into(viewm2);
        let mut td_from_view = toodee::TooDee::from(v_from_mut);
        td.clone_from_toodee(&(&td_from_view).view((uvals[7], uvals[8]), (uvals[9], uvals[10])));

        let mut dc = td.pop_col();
        if let Some(mut d) = dc {
            let _ = d.next();
            let _ = d.next_back();
        }

        {
            let mut dr = toodee::TooDee::remove_col(&mut td_from_view, uvals[11]);
            let _ = dr.next();
            let _ = dr.next_back();
        }

        let v3cols = _to_usize(GLOBAL_DATA, 248);
        let v3rows = _to_usize(GLOBAL_DATA, 256);
        let mut v3 = std::vec::Vec::with_capacity(65);
        let num3 = (_to_u8(GLOBAL_DATA, 264) % 65) as usize;
        for i in 0..num3 {
            let l = (_to_u8(GLOBAL_DATA, 265 + i) % 19) as usize;
            let start = 300 + i * 5;
            let end = start + l as usize;
            let s = _to_str(GLOBAL_DATA, start, end);
            v3.push(CustomType0(String::from(s)));
        }
        let td3 = toodee::TooDee::from_vec(v3cols, v3rows, v3);
        let mut viewm3 = toodee::TooDee::view_mut(&mut td_from_view, (uvals[12], uvals[13]), (uvals[14], uvals[15]));
        let r4 = (&viewm3).index(_to_usize(GLOBAL_DATA, 272));
        println!("{}", r4.len());
        if let Some(first) = r4.get(0) {
            println!("{:?}", first);
        }
        let vimm3 = (&viewm3).view((uvals[0], uvals[1]), (uvals[2], uvals[3]));
        let mut c3 = vimm3.col(_to_usize(GLOBAL_DATA, 280));
        let _ = c3.next();
        let _ = c3.nth_back(_to_usize(GLOBAL_DATA, 288));
        let _ = c3.last();

        let _ = td3;
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