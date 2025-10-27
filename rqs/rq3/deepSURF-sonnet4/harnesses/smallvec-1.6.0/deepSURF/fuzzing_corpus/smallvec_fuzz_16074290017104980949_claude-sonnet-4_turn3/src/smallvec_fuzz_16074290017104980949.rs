#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);

#[derive(Debug)]
struct ArrayType([i32; 16]);

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2000 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let num_operations = _to_u8(GLOBAL_DATA, 0) % 20 + 1;
        
        for op_idx in 0..num_operations {
            let base_offset = (op_idx * 80) as usize;
            if base_offset + 80 > GLOBAL_DATA.len() {
                break;
            }
            
            let operation_type = _to_u8(GLOBAL_DATA, base_offset) % 8;
            
            match operation_type {
                0 => {
                    let constructor_choice = _to_u8(GLOBAL_DATA, base_offset + 1) % 6;
                    let mut target_smallvec = match constructor_choice {
                        0 => SmallVec::<[i32; 16]>::new(),
                        1 => {
                            let capacity = _to_usize(GLOBAL_DATA, base_offset + 2);
                            SmallVec::<[i32; 16]>::with_capacity(capacity)
                        },
                        2 => {
                            let element = _to_i32(GLOBAL_DATA, base_offset + 2);
                            let count = _to_usize(GLOBAL_DATA, base_offset + 10);
                            SmallVec::<[i32; 16]>::from_elem(element, count)
                        },
                        3 => {
                            let mut vec = Vec::new();
                            let vec_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 64;
                            for i in 0..vec_size {
                                let elem = _to_i32(GLOBAL_DATA, base_offset + 3 + (i as usize * 4));
                                vec.push(elem);
                            }
                            SmallVec::<[i32; 16]>::from(vec)
                        },
                        4 => {
                            let slice_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 32;
                            let mut slice_data = Vec::new();
                            for i in 0..slice_size {
                                let elem = _to_i32(GLOBAL_DATA, base_offset + 3 + (i as usize * 4));
                                slice_data.push(elem);
                            }
                            SmallVec::<[i32; 16]>::from_slice(&slice_data)
                        },
                        _ => {
                            let iter_size = _to_u8(GLOBAL_DATA, base_offset + 2) % 32;
                            let mut iter_data = Vec::new();
                            for i in 0..iter_size {
                                let elem = _to_i32(GLOBAL_DATA, base_offset + 3 + (i as usize * 4));
                                iter_data.push(elem);
                            }
                            SmallVec::<[i32; 16]>::from_iter(iter_data.into_iter())
                        }
                    };
                    
                    let sv_len = target_smallvec.len();
                    println!("{:?}", sv_len);
                    
                    let slice_ref = target_smallvec.as_slice();
                    if !slice_ref.is_empty() {
                        let first_elem = &slice_ref[0];
                        println!("{:?}", *first_elem);
                    }
                    
                    let mut_slice = target_smallvec.as_mut_slice();
                    if !mut_slice.is_empty() {
                        let first_mut = &mut mut_slice[0];
                        println!("{:?}", *first_mut);
                    }
                    
                    let capacity = target_smallvec.capacity();
                    println!("{:?}", capacity);
                    
                    target_smallvec.shrink_to_fit();
                    let shrunk_capacity = target_smallvec.capacity();
                    println!("{:?}", shrunk_capacity);
                },
                1 => {
                    let mut vec = Vec::new();
                    let vec_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 64;
                    for i in 0..vec_size {
                        let elem = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        vec.push(elem);
                    }
                    
                    let target_smallvec = SmallVec::<[i32; 16]>::from(vec);
                    
                    let capacity = target_smallvec.capacity();
                    println!("{:?}", capacity);
                    
                    let into_vec = target_smallvec.into_vec();
                    println!("{:?}", into_vec.len());
                },
                2 => {
                    let mut sv1 = SmallVec::<[i32; 16]>::new();
                    let mut sv2 = SmallVec::<[i32; 16]>::new();
                    
                    let elem1 = _to_i32(GLOBAL_DATA, base_offset + 1);
                    let elem2 = _to_i32(GLOBAL_DATA, base_offset + 5);
                    sv1.push(elem1);
                    sv2.push(elem2);
                    
                    let comparison = sv1.cmp(&sv2);
                    println!("{:?}", comparison);
                    
                    let partial_cmp = sv1.partial_cmp(&sv2);
                    if let Some(ord) = partial_cmp {
                        println!("{:?}", ord);
                    }
                    
                    let equality = sv1.eq(&sv2);
                    println!("{:?}", equality);
                    
                    sv1.append(&mut sv2);
                    println!("{:?}", sv1.len());
                },
                3 => {
                    let mut target_sv = SmallVec::<[i32; 16]>::new();
                    let num_pushes = _to_u8(GLOBAL_DATA, base_offset + 1) % 32;
                    
                    for i in 0..num_pushes {
                        let elem = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        target_sv.push(elem);
                    }
                    
                    target_sv.reserve(_to_usize(GLOBAL_DATA, base_offset + 66));
                    
                    if !target_sv.is_empty() {
                        let drain_start = _to_usize(GLOBAL_DATA, base_offset + 66);
                        let drain_end = _to_usize(GLOBAL_DATA, base_offset + 70);
                        
                        let actual_end = if drain_end > target_sv.len() { target_sv.len() } else { drain_end };
                        let actual_start = if drain_start > actual_end { 0 } else { drain_start };
                        
                        let mut drain_iter = target_sv.drain(actual_start..actual_end);
                        if let Some(item) = drain_iter.next() {
                            println!("{:?}", item);
                        }
                    }
                },
                4 => {
                    let mut target_sv = SmallVec::<[i32; 16]>::new();
                    let operations = _to_u8(GLOBAL_DATA, base_offset + 1) % 16;
                    
                    for i in 0..operations {
                        let sub_op = _to_u8(GLOBAL_DATA, base_offset + 2 + i as usize) % 6;
                        match sub_op {
                            0 => {
                                let elem = _to_i32(GLOBAL_DATA, base_offset + 18 + (i as usize * 4));
                                target_sv.push(elem);
                            },
                            1 => {
                                if let Some(popped) = target_sv.pop() {
                                    println!("{:?}", popped);
                                }
                            },
                            2 => {
                                let index = _to_usize(GLOBAL_DATA, base_offset + 50 + (i as usize * 2));
                                if index < target_sv.len() {
                                    let removed = target_sv.remove(index);
                                    println!("{:?}", removed);
                                }
                            },
                            3 => {
                                let len = _to_usize(GLOBAL_DATA, base_offset + 66 + (i as usize * 2));
                                target_sv.truncate(len);
                            },
                            4 => {
                                let index = _to_usize(GLOBAL_DATA, base_offset + 50 + (i as usize * 2));
                                let elem = _to_i32(GLOBAL_DATA, base_offset + 18 + (i as usize * 4));
                                if index <= target_sv.len() {
                                    target_sv.insert(index, elem);
                                }
                            },
                            _ => {
                                let index = _to_usize(GLOBAL_DATA, base_offset + 50 + (i as usize * 2));
                                if index < target_sv.len() {
                                    let swapped = target_sv.swap_remove(index);
                                    println!("{:?}", swapped);
                                }
                            }
                        }
                    }
                    
                    if !target_sv.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, base_offset + 74) % target_sv.len();
                        let indexed_ref = &target_sv[index];
                        println!("{:?}", *indexed_ref);
                    }
                },
                5 => {
                    let slice_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 32;
                    let mut slice_data = Vec::new();
                    for i in 0..slice_size {
                        let elem = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        slice_data.push(elem);
                    }
                    
                    let target_sv = SmallVec::<[i32; 16]>::from_slice(&slice_data);
                    
                    let to_smallvec_result: SmallVec<[i32; 16]> = slice_data.as_slice().to_smallvec();
                    println!("{:?}", to_smallvec_result.len());
                    
                    let deref_slice = target_sv.deref();
                    if !deref_slice.is_empty() {
                        let first_elem = &deref_slice[0];
                        println!("{:?}", *first_elem);
                    }
                    
                    let cloned_sv = target_sv.clone();
                    println!("{:?}", cloned_sv.len());
                    
                    let slice_ref = target_sv.as_slice();
                    println!("{:?}", slice_ref.len());
                },
                6 => {
                    let iter_size = _to_u8(GLOBAL_DATA, base_offset + 1) % 32;
                    let mut iter_data = Vec::new();
                    for i in 0..iter_size {
                        let elem = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        iter_data.push(elem);
                    }
                    
                    let target_sv = SmallVec::<[i32; 16]>::from_iter(iter_data.into_iter());
                    
                    let mut into_iter = target_sv.into_iter();
                    if let Some(first) = into_iter.next() {
                        println!("{:?}", first);
                    }
                    
                    let remaining_slice = into_iter.as_slice();
                    if !remaining_slice.is_empty() {
                        let elem_ref = &remaining_slice[0];
                        println!("{:?}", *elem_ref);
                    }
                    
                    let mut_remaining = into_iter.as_mut_slice();
                    if !mut_remaining.is_empty() {
                        let elem_mut_ref = &mut_remaining[0];
                        println!("{:?}", *elem_mut_ref);
                    }
                },
                _ => {
                    let mut target_sv = SmallVec::<[i32; 16]>::new();
                    let num_elements = _to_u8(GLOBAL_DATA, base_offset + 1) % 16;
                    
                    for i in 0..num_elements {
                        let elem = _to_i32(GLOBAL_DATA, base_offset + 2 + (i as usize * 4));
                        target_sv.push(elem);
                    }
                    
                    let reserve_amount = _to_usize(GLOBAL_DATA, base_offset + 66);
                    target_sv.reserve(reserve_amount);
                    
                    let new_capacity = target_sv.capacity();
                    println!("{:?}", new_capacity);
                    
                    target_sv.shrink_to_fit();
                    let shrunk_capacity = target_sv.capacity();
                    println!("{:?}", shrunk_capacity);
                    
                    if !target_sv.is_empty() {
                        let index = _to_usize(GLOBAL_DATA, base_offset + 74) % target_sv.len();
                        let indexed_ref = &target_sv[index];
                        println!("{:?}", *indexed_ref);
                    }
                    
                    target_sv.clear();
                    println!("{:?}", target_sv.len());
                    
                    let resize_len = _to_usize(GLOBAL_DATA, base_offset + 70);
                    let resize_value = _to_i32(GLOBAL_DATA, base_offset + 74);
                    target_sv.resize(resize_len, resize_value);
                    println!("{:?}", target_sv.len());
                    
                    if !target_sv.is_empty() {
                        let slice_for_extend = &[_to_i32(GLOBAL_DATA, base_offset + 72)];
                        target_sv.extend_from_slice(slice_for_extend);
                        println!("{:?}", target_sv.len());
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