#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::ops::Index;

#[derive(Clone, Debug, Default)]
struct CustomType0(String);

#[derive(Clone, Debug, Default)]
struct CustomType2(String);

#[derive(Clone, Debug, Default)]
struct CustomType1(Vec<CustomType2>);

impl Index<usize> for CustomType1 {
    type Output = CustomType2;
    fn index(&self, _: usize) -> &Self::Output {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 2493);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let GLOBAL_DATA = match selector { 1 => global_data.first_half, _ => global_data.second_half };
        let t_585 = _to_u8(GLOBAL_DATA, 2501) % 17;
        let t_586 = _to_str(GLOBAL_DATA, 2502, 2502 + t_585 as usize);
        let t_587 = String::from(t_586);
        let t_588 = CustomType2(t_587);
        Box::leak(Box::new(t_588))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 5070 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let cols = (_to_u8(GLOBAL_DATA, 0) % 16 + 1) as usize;
        let rows = (_to_u8(GLOBAL_DATA, 1) % 16 + 1) as usize;
        let capacity = _to_usize(GLOBAL_DATA, 2) % 128 + 1;

        let selector = _to_u8(GLOBAL_DATA, 10) % 5;
        let mut target: TooDee<CustomType0> = match selector {
            0 => TooDee::new(cols, rows),
            1 => {
                let init_val = CustomType0("init".into());
                TooDee::init(cols, rows, init_val)
            }
            2 => TooDee::with_capacity(capacity),
            3 => {
                let mut v = Vec::new();
                for i in 0..(cols * rows) {
                    let idx = 20 + (i % (data.len() - 18));
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    let s = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + len as usize);
                    v.push(CustomType0(String::from(s)));
                }
                TooDee::from_vec(cols, rows, v)
            }
            _ => {
                let mut v = Vec::new();
                for i in 0..(cols * rows) {
                    let idx = 100 + (i % (data.len() - 18));
                    let len = _to_u8(GLOBAL_DATA, idx) % 17;
                    let s = _to_str(GLOBAL_DATA, idx + 1, idx + 1 + len as usize);
                    v.push(CustomType0(String::from(s)));
                }
                let boxed: Box<[CustomType0]> = v.into_boxed_slice();
                TooDee::from_box(cols, rows, boxed)
            }
        };

        let mut source = target.clone();
        if let Some(col_data) = source.pop_col() {
            let _: Option<CustomType0> = col_data.last();
        }
        let src_view = source.view((0, 0), (cols.saturating_sub(1), rows.saturating_sub(1)));

        let copy_choice = _to_bool(GLOBAL_DATA, 2000);
        if copy_choice {
            target.clone_from_toodee(&src_view);
        } else {
            target.clone_from_toodee(&source);
        }

        let ops = (_to_u8(GLOBAL_DATA, 2010) % 10) as usize;
        for i in 0..ops {
            let op_sel = _to_u8(GLOBAL_DATA, 2011 + i) % 8;
            match op_sel {
                0 => {
                    let r1 = _to_usize(GLOBAL_DATA, 2100 + i * 8) % rows;
                    let r2 = _to_usize(GLOBAL_DATA, 2104 + i * 8) % rows;
                    target.swap_rows(r1, r2);
                }
                1 => {
                    let view = target.view((0, 0), (cols.min(3), rows.min(3)));
                    let b = view.bounds();
                    println!("{:?}", b);
                }
                2 => {
                    let c = _to_usize(GLOBAL_DATA, 2200 + i * 4) % cols;
                    let col_iter = target.col(c);
                    let _ = col_iter.size_hint();
                }
                3 => {
                    let mut rows_iter = target.rows();
                    let _ = rows_iter.next_back();
                }
                4 => {
                    let mut rows_mut_iter = target.rows_mut();
                    let _ = rows_mut_iter.next();
                }
                5 => {
                    let new_row = vec![CustomType0("row".into()); cols];
                    target.push_row(new_row);
                }
                6 => {
                    if let Some(mut drain) = target.pop_col() {
                        let _ = drain.next();
                    }
                }
                _ => {
                    let _ = target.capacity();
                }
            }
        }

        let ref_val = &target[(0, 0)];
        println!("{:?}", ref_val);
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