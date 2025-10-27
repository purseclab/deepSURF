#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::Borrow;

struct CustomType0(String);
struct CustomType1(String);

#[derive(Debug)]
struct AlternativeArray<T> {
    data: [T; 15],
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 450 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let operations_count = _to_u8(GLOBAL_DATA, 0) % 5 + 1;
        
        for op_idx in 0..operations_count {
            let base_offset = op_idx as usize * 80;
            let operation_type = _to_u8(GLOBAL_DATA, base_offset) % 8;
            
            match operation_type {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 5;
                    let mut smallvec = match constructor_choice {
                        0 => smallvec::SmallVec::<[u32; 16]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[u32; 16]>::with_capacity(cap)
                        },
                        2 => {
                            let elem = _to_u32(GLOBAL_DATA, base_offset + 10);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 14) % 65;
                            smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count)
                        },
                        3 => {
                            let vec_size = _to_usize(GLOBAL_DATA, base_offset + 22) % 65;
                            let mut vec = Vec::new();
                            for i in 0..vec_size {
                                vec.push(_to_u32(GLOBAL_DATA, base_offset + 30 + (i * 4) % 40));
                            }
                            smallvec::SmallVec::<[u32; 16]>::from_vec(vec)
                        },
                        _ => {
                            let array: [u32; 16] = [
                                _to_u32(GLOBAL_DATA, base_offset + 2),
                                _to_u32(GLOBAL_DATA, base_offset + 6),
                                _to_u32(GLOBAL_DATA, base_offset + 10),
                                _to_u32(GLOBAL_DATA, base_offset + 14),
                                _to_u32(GLOBAL_DATA, base_offset + 18),
                                _to_u32(GLOBAL_DATA, base_offset + 22),
                                _to_u32(GLOBAL_DATA, base_offset + 26),
                                _to_u32(GLOBAL_DATA, base_offset + 30),
                                _to_u32(GLOBAL_DATA, base_offset + 34),
                                _to_u32(GLOBAL_DATA, base_offset + 38),
                                _to_u32(GLOBAL_DATA, base_offset + 42),
                                _to_u32(GLOBAL_DATA, base_offset + 46),
                                _to_u32(GLOBAL_DATA, base_offset + 50),
                                _to_u32(GLOBAL_DATA, base_offset + 54),
                                _to_u32(GLOBAL_DATA, base_offset + 58),
                                _to_u32(GLOBAL_DATA, base_offset + 62),
                            ];
                            smallvec::SmallVec::from_buf(array)
                        }
                    };
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 66);
                    let result = smallvec.try_reserve_exact(reserve_amount);
                    println!("{:?}", result);
                    
                    let slice_ref = smallvec.as_slice();
                    println!("{:?}", slice_ref);
                    
                    let len = smallvec.len();
                    println!("{}", len);
                    
                    let capacity = smallvec.capacity();
                    println!("{}", capacity);
                }
                1 => {
                    let alt_constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 3;
                    let mut alt_smallvec = match alt_constructor_choice {
                        0 => smallvec::SmallVec::<[i32; 15]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[i32; 15]>::with_capacity(cap)
                        },
                        _ => {
                            let slice_data = [
                                _to_i32(GLOBAL_DATA, base_offset + 10),
                                _to_i32(GLOBAL_DATA, base_offset + 14),
                                _to_i32(GLOBAL_DATA, base_offset + 18),
                                _to_i32(GLOBAL_DATA, base_offset + 22),
                                _to_i32(GLOBAL_DATA, base_offset + 26),
                            ];
                            smallvec::SmallVec::<[i32; 15]>::from_slice(&slice_data)
                        }
                    };
                    
                    let try_reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 30);
                    let result = alt_smallvec.try_reserve_exact(try_reserve_amount);
                    println!("{:?}", result);
                    
                    let mutable_slice = alt_smallvec.as_mut_slice();
                    println!("{:?}", mutable_slice);
                    
                    let is_empty = alt_smallvec.is_empty();
                    println!("{}", is_empty);
                }
                2 => {
                    let string_constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 4;
                    let mut string_smallvec = match string_constructor_choice {
                        0 => smallvec::SmallVec::<[String; 12]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[String; 12]>::with_capacity(capacity)
                        },
                        2 => {
                            let str_len = _to_u8(GLOBAL_DATA, base_offset + 10) % 20;
                            let str_data = _to_str(GLOBAL_DATA, base_offset + 11, base_offset + 11 + str_len as usize);
                            let elem = String::from(str_data);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 31) % 65;
                            smallvec::SmallVec::<[String; 12]>::from_elem(elem, count)
                        },
                        _ => {
                            let vec_len = _to_u8(GLOBAL_DATA, base_offset + 39) % 10;
                            let mut string_vec = Vec::new();
                            for i in 0..vec_len {
                                let substr_len = _to_u8(GLOBAL_DATA, base_offset + 40 + i as usize) % 5;
                                let substr = _to_str(GLOBAL_DATA, base_offset + 50 + (i as usize * 5), base_offset + 50 + (i as usize * 5) + substr_len as usize);
                                string_vec.push(String::from(substr));
                            }
                            smallvec::SmallVec::<[String; 12]>::from_vec(string_vec)
                        }
                    };
                    
                    let exact_reserve = _to_usize(GLOBAL_DATA, base_offset + 75);
                    let reserve_result = string_smallvec.try_reserve_exact(exact_reserve);
                    println!("{:?}", reserve_result);
                    
                    let borrowed: &[String] = string_smallvec.borrow();
                    println!("{:?}", borrowed);
                }
                3 => {
                    let byte_vec_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 3;
                    let mut byte_smallvec = match byte_vec_choice {
                        0 => smallvec::SmallVec::<[u8; 32]>::new(),
                        1 => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[u8; 32]>::with_capacity(cap)
                        },
                        _ => {
                            let from_iter_size = _to_u8(GLOBAL_DATA, base_offset + 10) % 65;
                            let iter_data: Vec<u8> = (0..from_iter_size).map(|i| _to_u8(GLOBAL_DATA, base_offset + 11 + i as usize)).collect();
                            smallvec::SmallVec::<[u8; 32]>::from_iter(iter_data)
                        }
                    };
                    
                    byte_smallvec.push(_to_u8(GLOBAL_DATA, base_offset + 76));
                    byte_smallvec.push(_to_u8(GLOBAL_DATA, base_offset + 77));
                    
                    let reserve_exact_val = _to_usize(GLOBAL_DATA, base_offset + 78);
                    let result = byte_smallvec.try_reserve_exact(reserve_exact_val);
                    println!("{:?}", result);
                    
                    let popped = byte_smallvec.pop();
                    println!("{:?}", popped);
                    
                    let deref_slice = byte_smallvec.deref();
                    println!("{:?}", deref_slice);
                }
                4 => {
                    let float_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 2;
                    let mut float_smallvec = match float_choice {
                        0 => smallvec::SmallVec::<[f64; 20]>::new(),
                        _ => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[f64; 20]>::with_capacity(cap)
                        }
                    };
                    
                    for i in 0..3 {
                        let float_val = _to_f64(GLOBAL_DATA, base_offset + 10 + (i * 8));
                        float_smallvec.push(float_val);
                    }
                    
                    let exact_reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 34);
                    let result = float_smallvec.try_reserve_exact(exact_reserve_amount);
                    println!("{:?}", result);
                    
                    let as_ptr = float_smallvec.as_ptr();
                    println!("{:?}", as_ptr);
                    
                    float_smallvec.clear();
                    
                    let is_empty_after_clear = float_smallvec.is_empty();
                    println!("{}", is_empty_after_clear);
                }
                5 => {
                    let choice_a = _to_u8(GLOBAL_DATA, base_offset + 1) % 2;
                    let mut sv_a = match choice_a {
                        0 => smallvec::SmallVec::<[i64; 24]>::new(),
                        _ => {
                            let cap = _to_usize(GLOBAL_DATA, base_offset + 2);
                            smallvec::SmallVec::<[i64; 24]>::with_capacity(cap)
                        }
                    };
                    
                    let choice_b = _to_u8(GLOBAL_DATA, base_offset + 10) % 2;
                    let mut sv_b = match choice_b {
                        0 => smallvec::SmallVec::<[i64; 24]>::new(),
                        _ => {
                            let elem = _to_i64(GLOBAL_DATA, base_offset + 11);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 19) % 65;
                            smallvec::SmallVec::<[i64; 24]>::from_elem(elem, count)
                        }
                    };
                    
                    sv_a.push(_to_i64(GLOBAL_DATA, base_offset + 27));
                    sv_a.push(_to_i64(GLOBAL_DATA, base_offset + 35));
                    sv_b.push(_to_i64(GLOBAL_DATA, base_offset + 43));
                    
                    let result_a = sv_a.try_reserve_exact(_to_usize(GLOBAL_DATA, base_offset + 51));
                    println!("{:?}", result_a);
                    
                    let result_b = sv_b.try_reserve_exact(_to_usize(GLOBAL_DATA, base_offset + 59));
                    println!("{:?}", result_b);
                    
                    let cmp_result = sv_a.cmp(&sv_b);
                    println!("{:?}", cmp_result);
                    
                    let partial_cmp_result = sv_a.partial_cmp(&sv_b);
                    println!("{:?}", partial_cmp_result);
                    
                    sv_a.append(&mut sv_b);
                    println!("{}", sv_a.len());
                    println!("{}", sv_b.len());
                }
                6 => {
                    let mut bool_smallvec = smallvec::SmallVec::<[bool; 64]>::new();
                    
                    for i in 0..5 {
                        let bool_val = _to_bool(GLOBAL_DATA, base_offset + 1 + i);
                        bool_smallvec.push(bool_val);
                    }
                    
                    let exact_reserve_size = _to_usize(GLOBAL_DATA, base_offset + 6);
                    let result = bool_smallvec.try_reserve_exact(exact_reserve_size);
                    println!("{:?}", result);
                    
                    let drain_start = _to_usize(GLOBAL_DATA, base_offset + 14);
                    let drain_end = _to_usize(GLOBAL_DATA, base_offset + 22);
                    let drain_iter = bool_smallvec.drain(drain_start..drain_end);
                    
                    for drained_item in drain_iter {
                        println!("{}", drained_item);
                    }
                    
                    let mut_slice = bool_smallvec.as_mut_slice();
                    println!("{:?}", mut_slice);
                }
                _ => {
                    let char_array: [char; 18] = [
                        _to_char(GLOBAL_DATA, base_offset + 1),
                        _to_char(GLOBAL_DATA, base_offset + 5),
                        _to_char(GLOBAL_DATA, base_offset + 9),
                        _to_char(GLOBAL_DATA, base_offset + 13),
                        _to_char(GLOBAL_DATA, base_offset + 17),
                        _to_char(GLOBAL_DATA, base_offset + 21),
                        _to_char(GLOBAL_DATA, base_offset + 25),
                        _to_char(GLOBAL_DATA, base_offset + 29),
                        _to_char(GLOBAL_DATA, base_offset + 33),
                        _to_char(GLOBAL_DATA, base_offset + 37),
                        _to_char(GLOBAL_DATA, base_offset + 41),
                        _to_char(GLOBAL_DATA, base_offset + 45),
                        _to_char(GLOBAL_DATA, base_offset + 49),
                        _to_char(GLOBAL_DATA, base_offset + 53),
                        _to_char(GLOBAL_DATA, base_offset + 57),
                        _to_char(GLOBAL_DATA, base_offset + 61),
                        _to_char(GLOBAL_DATA, base_offset + 65),
                        _to_char(GLOBAL_DATA, base_offset + 69),
                    ];
                    let mut char_smallvec = smallvec::SmallVec::<[char; 18]>::from_buf(char_array);
                    
                    let exact_reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 73);
                    let result = char_smallvec.try_reserve_exact(exact_reserve_amount);
                    println!("{:?}", result);
                    
                    let clone_result = char_smallvec.clone();
                    println!("{:?}", clone_result);
                    
                    let into_vec_result = char_smallvec.into_vec();
                    println!("{:?}", into_vec_result);
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