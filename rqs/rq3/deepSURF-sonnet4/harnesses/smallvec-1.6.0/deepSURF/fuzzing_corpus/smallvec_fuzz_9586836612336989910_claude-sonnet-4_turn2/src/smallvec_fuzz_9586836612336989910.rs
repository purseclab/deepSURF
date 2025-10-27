#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let op_count = _to_u8(GLOBAL_DATA, 0) % 65;
        let mut current_index = 1;
        
        let constructor_choice = _to_u8(GLOBAL_DATA, current_index) % 7;
        current_index += 1;
        
        let mut vec1 = match constructor_choice {
            0 => {
                let capacity = _to_usize(GLOBAL_DATA, current_index);
                current_index += 8;
                smallvec::SmallVec::<[u32; 32]>::with_capacity(capacity)
            },
            1 => {
                let elem_count = _to_u8(GLOBAL_DATA, current_index) % 65;
                current_index += 1;
                let elem = _to_u32(GLOBAL_DATA, current_index);
                current_index += 4;
                smallvec::SmallVec::<[u32; 32]>::from_elem(elem, elem_count as usize)
            },
            2 => {
                let slice_len = _to_u8(GLOBAL_DATA, current_index) % 65;
                current_index += 1;
                let mut items = Vec::new();
                for _ in 0..slice_len {
                    items.push(_to_u32(GLOBAL_DATA, current_index));
                    current_index += 4;
                }
                smallvec::SmallVec::<[u32; 32]>::from_slice(&items)
            },
            3 => {
                let vec_len = _to_u8(GLOBAL_DATA, current_index) % 65;
                current_index += 1;
                let mut items = Vec::new();
                for _ in 0..vec_len {
                    items.push(_to_u32(GLOBAL_DATA, current_index));
                    current_index += 4;
                }
                smallvec::SmallVec::<[u32; 32]>::from_vec(items)
            },
            4 => {
                let iter_len = _to_u8(GLOBAL_DATA, current_index) % 65;
                current_index += 1;
                let mut items = Vec::new();
                for _ in 0..iter_len {
                    items.push(_to_u32(GLOBAL_DATA, current_index));
                    current_index += 4;
                }
                smallvec::SmallVec::<[u32; 32]>::from_iter(items.into_iter())
            },
            5 => {
                let arr: [u32; 32] = [
                    _to_u32(GLOBAL_DATA, current_index), _to_u32(GLOBAL_DATA, current_index + 4),
                    _to_u32(GLOBAL_DATA, current_index + 8), _to_u32(GLOBAL_DATA, current_index + 12),
                    _to_u32(GLOBAL_DATA, current_index + 16), _to_u32(GLOBAL_DATA, current_index + 20),
                    _to_u32(GLOBAL_DATA, current_index + 24), _to_u32(GLOBAL_DATA, current_index + 28),
                    _to_u32(GLOBAL_DATA, current_index + 32), _to_u32(GLOBAL_DATA, current_index + 36),
                    _to_u32(GLOBAL_DATA, current_index + 40), _to_u32(GLOBAL_DATA, current_index + 44),
                    _to_u32(GLOBAL_DATA, current_index + 48), _to_u32(GLOBAL_DATA, current_index + 52),
                    _to_u32(GLOBAL_DATA, current_index + 56), _to_u32(GLOBAL_DATA, current_index + 60),
                    _to_u32(GLOBAL_DATA, current_index + 64), _to_u32(GLOBAL_DATA, current_index + 68),
                    _to_u32(GLOBAL_DATA, current_index + 72), _to_u32(GLOBAL_DATA, current_index + 76),
                    _to_u32(GLOBAL_DATA, current_index + 80), _to_u32(GLOBAL_DATA, current_index + 84),
                    _to_u32(GLOBAL_DATA, current_index + 88), _to_u32(GLOBAL_DATA, current_index + 92),
                    _to_u32(GLOBAL_DATA, current_index + 96), _to_u32(GLOBAL_DATA, current_index + 100),
                    _to_u32(GLOBAL_DATA, current_index + 104), _to_u32(GLOBAL_DATA, current_index + 108),
                    _to_u32(GLOBAL_DATA, current_index + 112), _to_u32(GLOBAL_DATA, current_index + 116),
                    _to_u32(GLOBAL_DATA, current_index + 120), _to_u32(GLOBAL_DATA, current_index + 124)
                ];
                current_index += 128;
                smallvec::SmallVec::from_buf(arr)
            },
            _ => smallvec::SmallVec::<[u32; 32]>::new()
        };
        
        let mut vec2 = {
            let constructor_choice2 = _to_u8(GLOBAL_DATA, current_index) % 3;
            current_index += 1;
            match constructor_choice2 {
                0 => smallvec::SmallVec::<[u32; 16]>::new(),
                1 => {
                    let capacity = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    smallvec::SmallVec::<[u32; 16]>::with_capacity(capacity)
                },
                _ => {
                    let elem = _to_u32(GLOBAL_DATA, current_index);
                    current_index += 4;
                    let count = _to_u8(GLOBAL_DATA, current_index) % 65;
                    current_index += 1;
                    smallvec::SmallVec::<[u32; 16]>::from_elem(elem, count as usize)
                }
            }
        };

        let mut vec3 = smallvec::SmallVec::<[u32; 32]>::new();
        let mut vec4 = smallvec::SmallVec::<[u32; 64]>::new();
        
        for i in 0..op_count {
            if current_index + 30 >= GLOBAL_DATA.len() { break; }
            
            let operation = _to_u8(GLOBAL_DATA, current_index) % 20;
            current_index += 1;
            
            match operation {
                0 => {
                    let value = _to_u32(GLOBAL_DATA, current_index);
                    current_index += 4;
                    vec1.push(value);
                    println!("{:?}", vec1.len());
                },
                1 => {
                    if let Some(val) = vec1.pop() {
                        println!("{:?}", val);
                    }
                },
                2 => {
                    let capacity = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    vec1.reserve(capacity);
                },
                3 => {
                    let idx = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    if idx < vec1.len() {
                        let removed = vec1.remove(idx);
                        println!("{:?}", removed);
                    }
                },
                4 => {
                    let idx = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    let value = _to_u32(GLOBAL_DATA, current_index);
                    current_index += 4;
                    if idx <= vec1.len() {
                        vec1.insert(idx, value);
                    }
                },
                5 => {
                    vec1.append(&mut vec2);
                },
                6 => {
                    let new_len = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    vec1.truncate(new_len);
                },
                7 => {
                    vec1.clear();
                },
                8 => {
                    let capacity = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    vec1.grow(capacity);
                },
                9 => {
                    let additional = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    vec1.reserve_exact(additional);
                },
                10 => {
                    vec1.shrink_to_fit();
                },
                11 => {
                    let start = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    let end = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    if start <= end && end <= vec1.len() {
                        let drained: Vec<_> = vec1.drain(start..end).collect();
                        for item in drained {
                            println!("{:?}", item);
                        }
                    }
                },
                12 => {
                    if !vec1.is_empty() {
                        let slice = vec1.as_slice();
                        println!("{:?}", slice.len());
                        for item in slice {
                            println!("{:?}", *item);
                        }
                    }
                },
                13 => {
                    let slice = vec1.as_mut_slice();
                    println!("{:?}", slice.len());
                    for item in slice {
                        println!("{:?}", *item);
                    }
                },
                14 => {
                    let new_vec = vec1.into_vec();
                    vec1 = smallvec::SmallVec::<[u32; 32]>::from_vec(new_vec);
                },
                15 => {
                    if !vec1.is_empty() {
                        let idx = _to_usize(GLOBAL_DATA, current_index);
                        current_index += 8;
                        if idx < vec1.len() {
                            let swapped = vec1.swap_remove(idx);
                            println!("{:?}", swapped);
                        }
                    }
                },
                16 => {
                    let count = _to_u8(GLOBAL_DATA, current_index) % 65;
                    current_index += 1;
                    for _ in 0..count {
                        vec3.push(_to_u32(GLOBAL_DATA, current_index));
                        current_index += 4;
                    }
                    vec1.extend_from_slice(vec3.as_slice());
                },
                17 => {
                    let try_result = vec1.try_reserve(_to_usize(GLOBAL_DATA, current_index));
                    current_index += 8;
                    if try_result.is_ok() {
                        println!("Reserve successful");
                    }
                },
                18 => {
                    let count = _to_u8(GLOBAL_DATA, current_index) % 65;
                    current_index += 1;
                    for _ in 0..count {
                        vec4.push(_to_u32(GLOBAL_DATA, current_index));
                        current_index += 4;
                    }
                    let iter = vec4.into_iter();
                    vec1.extend(iter);
                    vec4 = smallvec::SmallVec::<[u32; 64]>::new();
                },
                _ => {
                    let cloned = vec1.clone();
                    let ordering = vec1.cmp(&cloned);
                    println!("{:?}", ordering);
                    
                    vec1.retain(|x| *x % 2 == 0);
                    
                    if vec1.len() > 2 {
                        vec1.dedup();
                    }
                }
            }
        }
        
        let comparison_vec = {
            let constructor_choice3 = _to_u8(GLOBAL_DATA, current_index) % 3;
            current_index += 1;
            match constructor_choice3 {
                0 => vec1.clone(),
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, current_index);
                    current_index += 8;
                    smallvec::SmallVec::<[u32; 32]>::with_capacity(cap)
                },
                _ => {
                    let elem = _to_u32(GLOBAL_DATA, current_index);
                    current_index += 4;
                    smallvec::SmallVec::<[u32; 32]>::from_elem(elem, 1)
                }
            }
        };
        
        let eq_result = vec1.eq(&comparison_vec);
        println!("{:?}", eq_result);
        
        if let Some(partial_cmp_result) = vec1.partial_cmp(&comparison_vec) {
            println!("{:?}", partial_cmp_result);
        }
        
        let cmp_result = vec1.cmp(&comparison_vec);
        println!("{:?}", cmp_result);
        
        println!("{:?}", vec1.capacity());
        println!("{:?}", vec1.len());
        println!("{:?}", vec1.is_empty());
        println!("{:?}", vec1.spilled());
        
        let vec_deref = &*vec1;
        println!("{:?}", vec_deref.len());
        
        let vec_deref_mut = &mut *vec1;
        println!("{:?}", vec_deref_mut.len());

        let vec_as_ref = vec1.as_ref();
        println!("{:?}", vec_as_ref.len());
        
        let vec_as_mut = vec1.as_mut();
        println!("{:?}", vec_as_mut.len());
        
        if current_index + 8 < GLOBAL_DATA.len() {
            let grow_capacity = _to_usize(GLOBAL_DATA, current_index);
            vec1.grow(grow_capacity);
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