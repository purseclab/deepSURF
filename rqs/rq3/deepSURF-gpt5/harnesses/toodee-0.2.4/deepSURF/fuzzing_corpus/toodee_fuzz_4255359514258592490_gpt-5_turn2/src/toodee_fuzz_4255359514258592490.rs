#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Default, Debug)]
struct CustomType0(String);

impl FromStr for CustomType0 {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomType0(s.to_string()))
    }
}

fn make_vec(len_hint: usize) -> Vec<CustomType0> {
    let gd = get_global_data();
    let src = gd.second_half;
    let n = len_hint % 65;
    let s = _to_str(src, 0, src.len());
    let mut v = Vec::new();
    for i in 0..n {
        let b = if src.is_empty() { 0 } else { src[i % src.len()] };
        let rep = (b as usize % 17) + 1;
        let item = CustomType0(s.chars().take(rep).collect());
        v.push(item);
    }
    v
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 160 { return; }
        set_global_data(data);
        let gd = get_global_data();
        let g1 = gd.first_half;
        let g2 = gd.second_half;

        let a = _to_usize(g1, 0);
        let b = _to_usize(g1, 8);
        let c = _to_usize(g1, 16);
        let d = _to_usize(g1, 24);
        let e = _to_usize(g1, 32);
        let f = _to_usize(g1, 40);
        let g = _to_usize(g1, 48);
        let h = _to_usize(g1, 56);
        let i = _to_usize(g1, 64);
        let j = _to_usize(g1, 72);

        let choice = _to_u8(g1, 1) % 7;
        let mut td: TooDee<CustomType0> = match choice {
            0 => TooDee::with_capacity(a),
            1 => TooDee::new(b, c),
            2 => {
                let s = _to_str(g2, 0, g2.len());
                let initv = CustomType0::from_str(s).unwrap_or_default();
                TooDee::init(c, d, initv)
            }
            3 => {
                let v = make_vec(e);
                TooDee::from_vec(d, e, v)
            }
            4 => {
                let v = make_vec(f);
                TooDee::from_box(e, f, v.into_boxed_slice())
            }
            5 => {
                let base = TooDee::new(e, f);
                let v = base.view((g, h), (i, j));
                TooDee::from(v)
            }
            _ => {
                let mut base = TooDee::new(b, c);
                let mut vm = base.view_mut((d, e), (f, g));
                let sv = vm.view((h, i), (j, a));
                TooDee::from(sv)
            }
        };

        let pre_coord = (b, c);
        let r0 = td.index_mut(pre_coord);
        println!("{:?}", r0);

        let ops = (_to_u8(g1, 3) as usize % 20) + 1;
        for k in 0..ops {
            let opb = _to_u8(g2, k % g2.len());
            match opb % 10 {
                0 => {
                    let coord = (d, e);
                    let r = td.index(coord);
                    println!("{:?}", r);
                    let r2 = td.index_mut(coord);
                    println!("{:?}", r2);
                }
                1 => {
                    let v = td.view((f, g), (h, i));
                    let ci = v.col(a);
                    let idx = b;
                    let rr = &ci[idx];
                    println!("{:?}", rr);
                    let mut rows = v.rows();
                    if let Some(rw) = rows.next() {
                        println!("{}", rw.len());
                    }
                }
                2 => {
                    let mut vm = td.view_mut((i, j), (a, b));
                    vm.swap_rows(c, d);
                    let mut cm = vm.col_mut(e);
                    let _ = cm.nth(f);
                    let _ = cm.nth_back(g);
                    let rr = vm.index_mut((h, i));
                    println!("{:?}", rr);
                }
                3 => {
                    let mut cm = td.col_mut(h);
                    let _ = cm.next();
                    if let Some(cell) = cm.nth(i) {
                        let s = _to_str(g2, 0, g2.len());
                        *cell = CustomType0::from_str(s).unwrap_or_default();
                    }
                }
                4 => {
                    let v1 = make_vec(j);
                    let v2 = make_vec(a);
                    if _to_bool(g1, 5) {
                        td.push_row(v1);
                    } else {
                        td.insert_row(b, v2);
                    }
                }
                5 => {
                    let v1 = make_vec(c);
                    if _to_bool(g1, 7) {
                        td.push_col(v1);
                    } else {
                        td.insert_col(d, v1);
                    }
                }
                6 => {
                    if _to_bool(g1, 9) {
                        let mut dc = td.remove_col(e);
                        let _ = dc.next();
                        let _ = dc.next_back();
                    } else {
                        if let Some(mut dc) = td.pop_col() {
                            let _ = dc.next();
                        }
                    }
                }
                7 => {
                    let mut rowsm = td.rows_mut();
                    let _ = rowsm.nth(f);
                    let _ = rowsm.nth_back(g);
                    let _ = rowsm.last();
                }
                8 => {
                    td.swap_rows(h, i);
                    let colit = td.col(a);
                    let _ = colit.last();
                    let mut r = td.rows();
                    let _ = r.next_back();
                }
                _ => {
                    let coord = (g, h);
                    let r = td.index_mut(coord);
                    println!("{:?}", r);
                }
            }
        }

        let post_coord = (i, j);
        let r1 = td.index_mut(post_coord);
        println!("{:?}", r1);
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