#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use sized_chunks::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use typenum::U64;

struct CustomType1(String);
#[derive(Debug)]
struct CustomType2(String);
struct CustomType0(String);
struct CustomType4(String);
struct CustomType5(String);

impl std::iter::Iterator for CustomType0 {
    type Item = CustomType2;
    
    fn next(&mut self) -> Option<Self::Item> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_0 = _to_u8(GLOBAL_DATA, 8) % 17;
        let t_1 = _to_str(GLOBAL_DATA, 9, 9 + t_0 as usize);
        let t_2 = String::from(t_1);
        Some(CustomType2(t_2))
    }
}

impl std::iter::ExactSizeIterator for CustomType0 {    
    fn len(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 25);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 33)
    }
}

impl std::iter::IntoIterator for CustomType5 {
    type Item = CustomType2;
    type IntoIter = CustomType0;
    
    fn into_iter(self) -> Self::IntoIter {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 155);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_40 = _to_u8(GLOBAL_DATA, 163) % 17;
        let t_41 = _to_str(GLOBAL_DATA, 164, 164 + t_40 as usize);
        CustomType0(String::from(t_41))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 5 + 3;
        let mut chunks = Vec::new();

        for i in 0..op_count {
            let selector = _to_u8(GLOBAL_DATA, 10 + i as usize) % 4;
            match selector {
                0 => {
                    let c = Chunk::<CustomType2, U64>::new();
                    chunks.push(c);
                }
                1 => {
                    let a = CustomType2(format!("elem{}a", i));
                    let b = CustomType2(format!("elem{}b", i));
                    chunks.push(Chunk::pair(a, b));
                }
                2 => {
                    let mut iter = CustomType0(format!("iter{}", i).into()).into_iter();
                    let count = _to_usize(GLOBAL_DATA, 30 + i as usize) % 64;
                    let chunk = Chunk::collect_from(&mut iter, count);
                    chunks.push(chunk);
                }
                3 => {
                    let index = _to_usize(GLOBAL_DATA, 50 + i as usize);
                    let val = CustomType2(format!("unit{}", i));
                    chunks.push(Chunk::unit(val));
                }
                _ => (),
            }
        }

        let mut main_chunk = if !chunks.is_empty() { chunks.remove(0) } else { Chunk::new() };

        let ops = _to_u8(GLOBAL_DATA, 100) % 4;
        for _ in 0..ops {
            let val = CustomType2(format!("push{}", ops).into());
            main_chunk.push_back(val);
        }

        if let Some(ref mut chunk) = chunks.get_mut(0) {
            let idx = _to_usize(GLOBAL_DATA, 200) % chunk.len();
            let val = CustomType2("inserted".into());
            chunk.insert(idx, val);
        }

        let src_index = _to_usize(GLOBAL_DATA, 300) % main_chunk.len();
        if let Some(target) = chunks.get_mut(1) {
            main_chunk.drain_from_front(target, _to_usize(GLOBAL_DATA, 301) % 5);
        }

        let insert_index = _to_usize(GLOBAL_DATA, 350) % main_chunk.len().max(1);
        let iter = CustomType5(format!("insert_iter").into()).into_iter();
        main_chunk.insert_from(insert_index, iter);

        println!("Full chunk: {:?}", main_chunk.as_slice());
        
        let mut sparse = SparseChunk::<CustomType2, U64>::new();
        let sparse_idx = _to_usize(GLOBAL_DATA, 400) % 64;
        sparse.insert(sparse_idx, CustomType2("sparse_val".into()));
        println!("Sparse get: {:?}", sparse.get(sparse_idx));

        if let Some(drain_chunk) = chunks.get_mut(2) {
            let drained = drain_chunk.drain();
            println!("Drained items: {}", drained.count());
        }

        main_chunk.clear();
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