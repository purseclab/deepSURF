#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::DerefMut;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let mut ops = _to_u8(g, 0) % 32;
        let mut sv_list = Vec::new();

        for _ in 0..5 {
            let constructor_choice = _to_u8(g, 1) % 6;
            match constructor_choice {
                0 => sv_list.push(SmallVec::<[String; 128]>::new()),
                1 => {
                    let cap = _to_usize(g, 2);
                    sv_list.push(SmallVec::with_capacity(cap));
                }
                2 => {
                    let elem = _to_str(g, 10, 12).to_string();
                    let count = _to_usize(g, 20);
                    sv_list.push(SmallVec::from_elem(elem, count));
                }
                3 => {
                    let vec = vec![_to_str(g, 28, 30).to_string(); 64];
                    sv_list.push(SmallVec::from_vec(vec));
                }
                4 => {
                    let vec = (0..64).map(|_| _to_str(g, 36, 40).to_string()).collect::<Vec<_>>();
                    sv_list.push(SmallVec::from_vec(vec));
                }
                5 => {
                    let arr = [(); 128].map(|_| _to_str(g, 100, 104).to_string());
                    sv_list.push(SmallVec::from(arr));
                }
                _ => unreachable!(),
            }
        }

        let op_bytes = _to_u8(g, 500) as usize;
        for i in 0..op_bytes {
            let sv_idx = _to_usize(g, 501 + i) % sv_list.len();
            let op_choice = _to_u8(g, 600 + i) % 9;

            match op_choice {
                0 | 1 | 2 | 5 | 6 | 7 | 8 => {
                    let sv = &mut sv_list[sv_idx];
                    match op_choice {
                        0 => {
                            let elem = _to_str(g, 700 + i * 8, 700 + i * 8 + 4).to_string();
                            sv.push(elem);
                        }
                        1 => {
                            sv.pop();
                        }
                        2 => {
                            let idx = _to_usize(g, 800 + i * 8);
                            sv.remove(idx);
                        }
                        5 => {
                            let len = _to_usize(g, 1000 + i * 8);
                            sv.truncate(len);
                        }
                        6 => {
                            let drain = sv.drain(..);
                            for item in drain {
                                println!("{}", item);
                            }
                        }
                        7 => {
                            let elements = sv_list.remove(sv_idx).into_vec();
                            let new_sv = SmallVec::from_vec(elements);
                            sv_list.push(new_sv);
                        }
                        8 => {
                            let mut iter = sv.into_iter();
                            while let Some(elem) = iter.next() {
                                println!("{}", elem);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                3 | 4 => {
                    let sv = &sv_list[sv_idx];
                    match op_choice {
                        3 => {
                            println!("{:?}", sv.as_slice());
                        }
                        4 => {
                            let other_idx = _to_usize(g, 900 + i * 8) % sv_list.len();
                            let other_sv = &sv_list[other_idx];
                            let ordering = sv.cmp(other_sv);
                            println!("{:?}", ordering);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        let target_idx = _to_usize(g, 1500) % sv_list.len();
        let target_sv = &mut sv_list[target_idx];
        let mut iter = target_sv.deref_mut().into_iter();
        println!("Iterator count: {}", iter.size_hint().0);

        let _drained: Vec<_> = iter.collect();
        let cap = _to_usize(g, 1600);
        target_sv.reserve(cap);
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