#![forbid(unsafe_code)]
use afl::fuzz_nohook;
use global_data::{get_global_data, set_global_data};
use toodee::{TooDee, TooDeeView, TooDeeOps, TooDeeOpsMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let cons_sel = _to_u8(GLOBAL_DATA, 0) % 6;
        let cap = _to_usize(GLOBAL_DATA, 8) % 129;
        let cols = _to_usize(GLOBAL_DATA, 16) % 129;
        let rows = _to_usize(GLOBAL_DATA, 24) % 129;

        let mut t1 = match cons_sel {
            0 => TooDee::new(cols, rows),
            1 => TooDee::with_capacity(cap),
            2 => {
                let v = vec![CustomType0(String::new()); cols * rows];
                TooDee::from_vec(cols, rows, v)
            },
            3 => {
                let view = TooDeeView::new(cols, rows, &[]);
                TooDee::from(view)
            },
            4 => TooDee::init(cols, rows, CustomType0(String::new())),
            5 => {
                let b = vec![CustomType0(String::new()); cols * rows].into_boxed_slice();
                TooDee::from_box(cols, rows, b)
            },
            _ => unreachable!()
        };

        let ops = _to_u8(GLOBAL_DATA, 32) % 128;
        for i in 0..ops {
            let op = _to_u8(GLOBAL_DATA, 40 + i as usize) % 10;
            let off = 168 + i as usize * 16;

            match op {
                0 => {
                    let view = t1.view((_to_usize(GLOBAL_DATA, off), _to_usize(GLOBAL_DATA, off+8)), (_to_usize(GLOBAL_DATA, off+16), _to_usize(GLOBAL_DATA, off+24)));
                    let last = view.rows().last();
                    println!("{:?}", last);
                },
                1 => {
                    let mut view_mut = t1.view_mut((0, 0), (cols, rows));
                    let rows_mut = view_mut.rows_mut();
                    let _ = rows_mut.last();
                },
                2 => {
                    let c = _to_usize(GLOBAL_DATA, off);
                    let _ = t1.col(c).next_back();
                },
                3 => {
                    let c = _to_usize(GLOBAL_DATA, off);
                    let mut col = t1.col_mut(c);
                    let _ = col.nth(_to_usize(GLOBAL_DATA, off+8));
                },
                4 => {
                    let r1 = _to_usize(GLOBAL_DATA, off);
                    let r2 = _to_usize(GLOBAL_DATA, off + 8);
                    t1.swap_rows(r1, r2);
                },
                5 => {
                    if let Some(drained) = t1.pop_col() {
                        for item in drained { println!("{:?}", item); }
                    }
                },
                6 => {
                    let c = _to_usize(GLOBAL_DATA, off);
                    let _drained = t1.remove_col(c);
                },
                7 => {
                    let c = _to_usize(GLOBAL_DATA, off);
                    let count = _to_usize(GLOBAL_DATA, off + 8) % 65;
                    let insert = vec![CustomType0(String::new()); count];
                    t1.insert_col(c, insert.into_iter());
                },
                8 => {
                    let r = _to_usize(GLOBAL_DATA, off);
                    let count = _to_usize(GLOBAL_DATA, off + 8) % 65;
                    let insert = vec![CustomType0(String::new()); count];
                    t1.insert_row(r, insert.into_iter());
                },
                9 => {
                    let coord = (_to_usize(GLOBAL_DATA, off), _to_usize(GLOBAL_DATA, off+8));
                    t1[coord] = CustomType0(String::from("fuzzed"));
                },
                _ => ()
            }
        }

        let t2 = &t1;
        let rs = t2.rows();
        let _ = rs.last();
        let t3 = &mut t1;
        let _ = t3.view_mut((0,0), (1,1)).swap_rows(0, 0);
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