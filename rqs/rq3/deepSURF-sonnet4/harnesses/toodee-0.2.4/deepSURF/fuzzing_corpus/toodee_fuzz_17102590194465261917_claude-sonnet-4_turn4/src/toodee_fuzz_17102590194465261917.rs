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
        if data.len() < 2000 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut offset = 1;
        
        for op_idx in 0..num_operations {
            let operation = _to_u8(GLOBAL_DATA, offset) % 12;
            offset += 1;
            
            match operation {
                0 => {
                    let t_0 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let t_1 = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut t_2 = _to_u8(GLOBAL_DATA, offset) % 33;
                    offset += 1;
                    
                    let mut t_3 = std::vec::Vec::with_capacity(32);
                    for i in 0..t_2 {
                        let mut str_len = _to_u8(GLOBAL_DATA, offset) % 17;
                        offset += 1;
                        if offset + str_len as usize > GLOBAL_DATA.len() { break; }
                        let t_str = _to_str(GLOBAL_DATA, offset, offset + str_len as usize);
                        offset += str_len as usize;
                        let t_string = String::from(t_str);
                        let t_custom = CustomType0(t_string);
                        t_3.push(t_custom);
                    }
                    
                    let constructor_choice = _to_u8(GLOBAL_DATA, offset % GLOBAL_DATA.len()) % 7;
                    offset += 1;
                    
                    match constructor_choice {
                        0 => {
                            if t_3.len() >= t_0 * t_1 {
                                let mut t_132 = toodee::TooDee::from_vec(t_0, t_1, t_3);
                                if t_132.num_cols() > 0 {
                                    let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                    offset += 8;
                                    let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                    let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                    offset += 8;
                                    t_135.nth_back(nth_idx);
                                    let next_result = t_135.next();
                                    if let Some(item) = next_result {
                                        println!("{:?}", *item);
                                    }
                                }
                            }
                        },
                        1 => {
                            let mut t_132 = toodee::TooDee::init(t_0, t_1, CustomType0("init".to_string()));
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        },
                        2 => {
                            let mut t_132: toodee::TooDee<CustomType0> = toodee::TooDee::new(t_0, t_1);
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        },
                        3 => {
                            let mut t_132: toodee::TooDee<CustomType0> = toodee::TooDee::with_capacity(t_0 * t_1);
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        },
                        4 => {
                            let mut data_vec = vec![CustomType0("test".to_string()); t_0 * t_1];
                            let mut t_132 = toodee::TooDee::from_vec(t_0, t_1, data_vec);
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        },
                        5 => {
                            let boxed_data = vec![CustomType0("boxed".to_string()); t_0 * t_1].into_boxed_slice();
                            let mut t_132 = toodee::TooDee::from_box(t_0, t_1, boxed_data);
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        },
                        _ => {
                            let view_data = vec![CustomType0("view".to_string()); t_0 * t_1];
                            let view = toodee::TooDeeView::new(t_0, t_1, &view_data);
                            let mut t_132 = toodee::TooDee::from(view);
                            if t_132.num_cols() > 0 {
                                let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                let mut t_135 = toodee::TooDee::col_mut(&mut t_132, col_idx);
                                let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                                offset += 8;
                                t_135.nth_back(nth_idx);
                            }
                        }
                    }
                },
                1 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("init".to_string()));
                    let start_coord = (_to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len()), _to_usize(GLOBAL_DATA, (offset + 8) % GLOBAL_DATA.len()));
                    offset += 16;
                    let end_coord = (_to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len()), _to_usize(GLOBAL_DATA, (offset + 8) % GLOBAL_DATA.len()));
                    offset += 16;
                    if toodee.num_cols() > 0 && toodee.num_rows() > 0 {
                        let mut view_mut = toodee.view_mut(start_coord, end_coord);
                        if view_mut.num_cols() > 0 {
                            let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            let mut col_iter = view_mut.col_mut(col_idx);
                            let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            col_iter.nth_back(nth_idx);
                            let result = col_iter.nth(_to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len()));
                            offset += 8;
                            if let Some(item) = result {
                                println!("{:?}", *item);
                            }
                        }
                    }
                },
                2 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut data_slice = vec![CustomType0("slice".to_string()); rows * cols];
                    let mut view_mut = toodee::TooDeeViewMut::new(cols, rows, &mut data_slice);
                    if view_mut.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut col_iter = view_mut.col_mut(col_idx);
                        let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        col_iter.nth_back(nth_idx);
                        let next_result = col_iter.next_back();
                        if let Some(item) = next_result {
                            println!("{:?}", *item);
                        }
                    }
                },
                3 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("test".to_string()));
                    if toodee.num_rows() > 0 {
                        let mut rows_iter = toodee.rows_mut();
                        let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        rows_iter.nth_back(nth_idx);
                        if let Some(row) = rows_iter.next() {
                            println!("{:?}", row.len());
                        }
                    }
                },
                4 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("test".to_string()));
                    if toodee.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let col_iter = toodee.col(col_idx);
                        let mut col_iter_copy = col_iter;
                        let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        col_iter_copy.nth_back(nth_idx);
                        if let Some(item) = col_iter_copy.last() {
                            println!("{:?}", *item);
                        }
                    }
                },
                5 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("test".to_string()));
                    if toodee.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut drain_col_result = toodee.pop_col();
                        if let Some(mut drain_col) = drain_col_result {
                            let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            if let Some(item) = drain_col.next() {
                                println!("{:?}", item);
                            }
                        }
                    }
                },
                6 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("remove".to_string()));
                    if toodee.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut drain_col = toodee.remove_col(col_idx);
                        let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        if let Some(item) = drain_col.next_back() {
                            println!("{:?}", item);
                        }
                    }
                },
                7 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let data_vec = vec![CustomType0("view".to_string()); rows * cols];
                    let view = toodee::TooDeeView::new(cols, rows, &data_vec);
                    if view.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut col_iter = view.col(col_idx);
                        let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        col_iter.nth_back(nth_idx);
                        let next_result = col_iter.next();
                        if let Some(item) = next_result {
                            println!("{:?}", *item);
                        }
                    }
                },
                8 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("swap".to_string()));
                    if toodee.num_rows() >= 2 {
                        let r1 = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let r2 = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        toodee.swap_rows(r1, r2);
                        
                        if toodee.num_cols() > 0 {
                            let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            let mut col_iter = toodee.col_mut(col_idx);
                            let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            col_iter.nth_back(nth_idx);
                        }
                    }
                },
                9 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("cells".to_string()));
                    let mut cells_iter = toodee.cells_mut();
                    let nth_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                    offset += 8;
                    if let Some(cell) = cells_iter.nth(nth_idx) {
                        println!("{:?}", *cell);
                    }
                    
                    if toodee.num_cols() > 0 {
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut col_mut = toodee.col_mut(col_idx);
                        let nth_back_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        col_mut.nth_back(nth_back_idx);
                    }
                },
                10 => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("index".to_string()));
                    if toodee.num_rows() > 0 && toodee.num_cols() > 0 {
                        let row_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let coord = (col_idx, row_idx);
                        let item_ref = &toodee[coord];
                        println!("{:?}", *item_ref);
                        
                        let col_idx2 = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        let mut col_mut = toodee.col_mut(col_idx2);
                        let nth_back_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                        offset += 8;
                        col_mut.nth_back(nth_back_idx);
                    }
                },
                _ => {
                    let rows = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let cols = _to_usize(GLOBAL_DATA, offset);
                    offset += 8;
                    let mut toodee = toodee::TooDee::init(cols, rows, CustomType0("multi".to_string()));
                    
                    for _ in 0..3 {
                        if toodee.num_cols() > 0 {
                            let col_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            let mut col_mut = toodee.col_mut(col_idx);
                            let nth_back_idx = _to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len());
                            offset += 8;
                            col_mut.nth_back(nth_back_idx);
                            col_mut.nth(_to_usize(GLOBAL_DATA, offset % GLOBAL_DATA.len()));
                            offset += 8;
                        }
                    }
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