#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Debug, Default)]
struct CustomType0(String);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 80 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let first = global_data.first_half;
        let second = global_data.second_half;

        let l1 = (_to_u8(first, 1) as usize % 65) + 1;
        let l2 = (_to_u8(first, 2) as usize % 65) + 1;
        let mut row1: Vec<CustomType0> = Vec::with_capacity(l1);
        for i in 0..l1 {
            let idx = i % second.len();
            let s = format!("A{}_{}", second[idx], i);
            row1.push(CustomType0(s));
        }
        let mut row2: Vec<CustomType0> = Vec::with_capacity(l2);
        for i in 0..l2 {
            let idx = (i * 2) % second.len();
            let s = format!("B{}_{}", second[idx], i);
            row2.push(CustomType0(s));
        }

        let mut td = TooDee::<CustomType0>::default();
        td.push_row(row1.clone());
        td.push_row(row2.clone());

        let s0 = _to_usize(first, 8);
        let s1 = _to_usize(first, 16);
        let e0 = _to_usize(first, 24);
        let e1 = _to_usize(first, 32);
        let view = td.view((s0, s1), (e0, e1));

        let mut r = view.rows();
        if let Some(slice) = r.next() {
            println!("{}", slice.len());
        }
        let n1 = _to_usize(second, 0);
        r.nth_back(n1);

        {
            let cidx = _to_usize(first, 0);
            let col = td.col(cidx);
            let idx_in_col = _to_usize(second, 24);
            let cref = &col[idx_in_col];
            println!("{:?}", cref);
        }

        {
            let c0 = _to_usize(first, 32);
            let r0 = _to_usize(second, 32);
            let cell = &td[(c0, r0)];
            println!("{:?}", cell);
        }

        {
            let ridx = _to_usize(first, 24);
            let row_slice = &view[ridx];
            println!("{}", row_slice.len());
        }

        {
            let mut td2 = TooDee::from(view.clone());
            let mut rm = td2.rows_mut();
            let n2 = _to_usize(second, 8);
            rm.nth_back(n2);
            let mut cm = td2.col_mut(_to_usize(first, 8));
            cm.nth_back(_to_usize(second, 16));
            let mut vm = td2.view_mut((s0, s1), (e0, e1));
            let mut rm2 = vm.rows_mut();
            rm2.nth_back(_to_usize(first, 24));
            let v_from_mut: TooDeeView<CustomType0> = vm.into();
            let mut rows_from_view = v_from_mut.rows();
            rows_from_view.nth_back(_to_usize(second, 24));
            let c2 = v_from_mut.col(_to_usize(first, 16));
            let c2r = &c2[_to_usize(second, 24)];
            println!("{:?}", c2r);
        }

        {
            let mut cells = td.cells();
            cells.nth_back(_to_usize(second, 24));
        }

        let ops = (_to_u8(first, 3) as usize % 7) + 1;
        for i in 0..ops {
            let n = _to_usize(second, (i % 5) * 8);
            match i % 5 {
                0 => {
                    let mut it = view.rows();
                    it.nth_back(n);
                }
                1 => {
                    let mut it = td.rows();
                    it.nth_back(n);
                }
                2 => {
                    let mut td3 = TooDee::from(view.clone());
                    let mut it = td3.rows_mut();
                    it.nth_back(n);
                }
                3 => {
                    let mut c = td.col(_to_usize(first, 8));
                    let mut c_rev = c.rev();
                    let _ = c_rev.next();
                }
                _ => {
                    let mut cells2 = td.cells();
                    cells2.nth_back(n);
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