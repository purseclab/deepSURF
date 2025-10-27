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
        let GLOBAL_DATA = global_data.first_half;
        let SECOND = global_data.second_half;

        let mode = _to_u8(GLOBAL_DATA, 0);
        let cols1 = (_to_usize(GLOBAL_DATA, 8) % 65) + 1;
        let rows1 = (_to_usize(GLOBAL_DATA, 16) % 65) + 1;
        let cols2 = (_to_usize(GLOBAL_DATA, 24) % 65) + 1;
        let rows2 = (_to_usize(GLOBAL_DATA, 32) % 65) + 1;
        let col_idx = _to_usize(GLOBAL_DATA, 40);

        match mode % 4 {
            0 => {
                let mut td: TooDee<CustomType0> = TooDee::new(cols1, rows1);
                let mut loops = (_to_u8(GLOBAL_DATA, 8) % 4) as usize + 1;
                let mut i = 0usize;
                while i < loops {
                    let mut row: Vec<CustomType0> = Vec::with_capacity(cols1);
                    let mut j = 0usize;
                    while j < cols1 {
                        let b = SECOND[(i + j) % SECOND.len()];
                        let s = b.to_string();
                        row.push(CustomType0(s));
                        j += 1;
                    }
                    td.push_row(row);
                    i += 1;
                }
                {
                    let v = td.view((_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24)), (_to_usize(GLOBAL_DATA, 32), _to_usize(GLOBAL_DATA, 40)));
                    let r = v.rows();
                    let _ = r.size_hint();
                    let c = v.col(col_idx);
                    let mut c2 = v.col(_to_usize(GLOBAL_DATA, 8));
                    c2.next_back();
                    let _ = c.last();
                    let s = &v[(_to_usize(GLOBAL_DATA, 0), _to_usize(GLOBAL_DATA, 8))];
                    println!("{:?}", s);
                }
                {
                    let mut c = td.col(col_idx);
                    c.next_back();
                    let _ = c.nth_back(_to_usize(GLOBAL_DATA, 24));
                }
                {
                    let mut vm = td.view_mut((_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24)), (_to_usize(GLOBAL_DATA, 32), _to_usize(GLOBAL_DATA, 40)));
                    {
                        let _ = vm.rows_mut().next_back();
                    }
                    {
                        let mut cm = vm.col_mut(_to_usize(GLOBAL_DATA, 32));
                        let _ = cm.next_back();
                    }
                    let vv = vm.view((_to_usize(GLOBAL_DATA, 8), _to_usize(GLOBAL_DATA, 16)), (_to_usize(GLOBAL_DATA, 24), _to_usize(GLOBAL_DATA, 32)));
                    let mut vc = vv.col(_to_usize(GLOBAL_DATA, 40));
                    vc.next_back();
                }
                {
                    let _ = &td[_to_usize(GLOBAL_DATA, 8)];
                    let s = &td[(_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24))];
                    println!("{:?}", s);
                }
                {
                    let _r = td.rows();
                    let mut c = td.col(_to_usize(GLOBAL_DATA, 32));
                    let _ = &c[_to_usize(GLOBAL_DATA, 24)];
                    c.next_back();
                }
                {
                    let mut cm = td.col_mut(_to_usize(GLOBAL_DATA, 16));
                    let _ = cm.next_back();
                }
                {
                    let _ = td.pop_col().map(|mut d| {
                        let _ = d.next_back();
                    });
                }
            }
            1 => {
                let init_val = CustomType0(SECOND[0].to_string());
                let mut td: TooDee<CustomType0> = TooDee::init(cols1, rows1, init_val.clone());
                {
                    let mut c = td.col(col_idx);
                    c.next_back();
                }
                {
                    let mut rc = td.remove_col(_to_usize(GLOBAL_DATA, 8));
                    let _ = rc.next_back();
                }
                {
                    let v = td.view((_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24)), (_to_usize(GLOBAL_DATA, 32), _to_usize(GLOBAL_DATA, 40)));
                    let mut c = v.col(_to_usize(GLOBAL_DATA, 32));
                    c.next_back();
                }
                {
                    let _ = &td[_to_usize(GLOBAL_DATA, 16)];
                    let s = &td[(_to_usize(GLOBAL_DATA, 24), _to_usize(GLOBAL_DATA, 32))];
                    println!("{:?}", s);
                }
            }
            2 => {
                let total = cols2 * rows2;
                let mut backing: Vec<u8> = Vec::with_capacity(total);
                let mut k = 0usize;
                while k < total {
                    backing.push(SECOND[k % SECOND.len()]);
                    k += 1;
                }
                let mut td: TooDee<u8> = TooDee::from_vec(cols2, rows2, backing);
                {
                    let mut c = td.col(col_idx);
                    c.next_back();
                }
                {
                    let mut rv = td.view_mut((_to_usize(GLOBAL_DATA, 0), _to_usize(GLOBAL_DATA, 8)), (_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24)));
                    let mut c = rv.col(_to_usize(GLOBAL_DATA, 32));
                    c.next_back();
                }
                {
                    let mut cm = td.col_mut(_to_usize(GLOBAL_DATA, 16));
                    let _ = cm.next_back();
                }
                {
                    let _ = td.pop_col().map(|mut d| {
                        let _ = d.next_back();
                    });
                }
            }
            _ => {
                let cap = (_to_usize(GLOBAL_DATA, 24) % (65 * 65)) + 1;
                let mut td: TooDee<CustomType0> = TooDee::with_capacity(cap);
                {
                    let mut i = 0usize;
                    let limit = (_to_u8(GLOBAL_DATA, 32) % 5) as usize + 1;
                    while i < limit {
                        let mut coldata: Vec<CustomType0> = Vec::with_capacity(rows1);
                        let mut j = 0usize;
                        while j < rows1 {
                            let b = SECOND[(i + j) % SECOND.len()];
                            coldata.push(CustomType0(b.to_string()));
                            j += 1;
                        }
                        td.push_col(coldata);
                        i += 1;
                    }
                }
                {
                    let mut c = td.col(col_idx);
                    c.next_back();
                    let _ = c.nth_back(_to_usize(GLOBAL_DATA, 8));
                }
                {
                    let v = td.view((_to_usize(GLOBAL_DATA, 0), _to_usize(GLOBAL_DATA, 8)), (_to_usize(GLOBAL_DATA, 16), _to_usize(GLOBAL_DATA, 24)));
                    let mut c = v.col(_to_usize(GLOBAL_DATA, 32));
                    c.next_back();
                    let _ = &v[(_to_usize(GLOBAL_DATA, 24), _to_usize(GLOBAL_DATA, 32))];
                }
            }
        }

        let view_cols = (_to_usize(GLOBAL_DATA, 8) % 16) + 1;
        let view_rows = (_to_usize(GLOBAL_DATA, 16) % 16) + 1;
        let _ = {
            let data_ref: &[u8] = SECOND;
            let v = TooDeeView::new(view_cols, view_rows, data_ref);
            let mut c = v.col(_to_usize(GLOBAL_DATA, 24));
            c.next_back();
            let s = &v[(_to_usize(GLOBAL_DATA, 32), _to_usize(GLOBAL_DATA, 40))];
            println!("{:?}", s);
        };

        {
            let mut backing2 = SECOND.to_vec();
            let vmut = TooDeeViewMut::new(view_cols, view_rows, &mut backing2[..]);
            let mut rows = vmut.rows();
            let _ = rows.next_back();
            let c = vmut.col(_to_usize(GLOBAL_DATA, 32));
            let mut c2 = vmut.col(_to_usize(GLOBAL_DATA, 8));
            let _ = &c[_to_usize(GLOBAL_DATA, 16)];
            c2.next_back();
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