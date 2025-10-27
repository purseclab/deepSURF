#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug)]
struct CustomType1(String);

#[derive(Debug)]
struct CustomType0([u32; 16]);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 128 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let op_count = _to_usize(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..op_count {
			let operation = _to_u8(GLOBAL_DATA, 8 + op_idx * 8) % 20;
			
			match operation {
				0 => {
					let capacity = _to_usize(GLOBAL_DATA, 16 + op_idx * 8);
					let sv1: SmallVec<[u32; 32]> = SmallVec::with_capacity(capacity);
					let sv_ref = &sv1;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				1 => {
					let vec_size = _to_usize(GLOBAL_DATA, 24 + op_idx * 8) % 65;
					let mut vec_data = Vec::new();
					for i in 0..vec_size {
						vec_data.push(_to_u32(GLOBAL_DATA, 32 + op_idx * 8 + i * 4));
					}
					let sv2: SmallVec<[u32; 16]> = SmallVec::from_vec(vec_data);
					let sv_ref = &sv2;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				2 => {
					let elem = _to_u32(GLOBAL_DATA, 40 + op_idx * 8);
					let count = _to_usize(GLOBAL_DATA, 48 + op_idx * 8) % 65;
					let sv3: SmallVec<[u32; 64]> = SmallVec::from_elem(elem, count);
					let sv_ref = &sv3;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result[0]);
				},
				3 => {
					let arr = [_to_u32(GLOBAL_DATA, 56 + op_idx * 8); 24];
					let sv4: SmallVec<[u32; 24]> = SmallVec::from_buf(arr);
					let sv_ref = &sv4;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				4 => {
					let arr = [_to_u32(GLOBAL_DATA, 64 + op_idx * 8); 12];
					let len = _to_usize(GLOBAL_DATA, 72 + op_idx * 8);
					let sv5: SmallVec<[u32; 12]> = SmallVec::from_buf_and_len(arr, len);
					let sv_ref = &sv5;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				5 => {
					let slice_len = _to_usize(GLOBAL_DATA, 80 + op_idx * 8) % 65;
					let mut slice_data = Vec::new();
					for i in 0..slice_len {
						slice_data.push(_to_u32(GLOBAL_DATA, 88 + op_idx * 8 + i * 4));
					}
					let sv6: SmallVec<[u32; 128]> = SmallVec::from_slice(&slice_data);
					let sv_ref = &sv6;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				6 => {
					let iter_size = _to_usize(GLOBAL_DATA, 96 + op_idx * 8) % 65;
					let mut iter_data = Vec::new();
					for i in 0..iter_size {
						iter_data.push(_to_u32(GLOBAL_DATA, 104 + op_idx * 8 + i * 4));
					}
					let sv7: SmallVec<[u32; 256]> = SmallVec::from_iter(iter_data);
					let sv_ref = &sv7;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				7 => {
					let mut sv8: SmallVec<[u32; 64]> = SmallVec::new();
					let push_count = _to_usize(GLOBAL_DATA, 112 + op_idx * 8) % 65;
					for i in 0..push_count {
						sv8.push(_to_u32(GLOBAL_DATA, 120 + op_idx * 8 + i * 4));
					}
					let sv_ref = &sv8;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				8 => {
					let mut sv9: SmallVec<[u32; 32]> = SmallVec::new();
					let elem = _to_u32(GLOBAL_DATA, 128 + op_idx * 8);
					sv9.push(elem);
					let sv_ref = &sv9;
					let deref_result = sv_ref.deref();
					let index_result = &deref_result[0];
					println!("{:?}", index_result);
				},
				9 => {
					let mut sv10: SmallVec<[u32; 16]> = SmallVec::new();
					let extend_size = _to_usize(GLOBAL_DATA, 136 + op_idx * 8) % 65;
					let mut extend_data = Vec::new();
					for i in 0..extend_size {
						extend_data.push(_to_u32(GLOBAL_DATA, 144 + op_idx * 8 + i * 4));
					}
					sv10.extend(extend_data);
					let sv_ref = &sv10;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				10 => {
					let mut sv11: SmallVec<[u32; 24]> = SmallVec::new();
					let insert_idx = _to_usize(GLOBAL_DATA, 152 + op_idx * 8);
					let insert_val = _to_u32(GLOBAL_DATA, 160 + op_idx * 8);
					sv11.push(0);
					if insert_idx < sv11.len() {
						sv11.insert(insert_idx, insert_val);
					}
					let sv_ref = &sv11;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				11 => {
					let mut sv12: SmallVec<[u32; 8]> = SmallVec::new();
					let append_size = _to_usize(GLOBAL_DATA, 168 + op_idx * 8) % 65;
					for i in 0..append_size {
						sv12.push(_to_u32(GLOBAL_DATA, 176 + op_idx * 8 + i * 4));
					}
					let clone_sv = sv12.clone();
					let sv_ref = &clone_sv;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				12 => {
					let sv13: SmallVec<[u32; 96]> = SmallVec::new();
					let as_slice_result = sv13.as_slice();
					let sv_ref = &sv13;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				13 => {
					let mut sv14: SmallVec<[u32; 512]> = SmallVec::new();
					let reserve_amount = _to_usize(GLOBAL_DATA, 184 + op_idx * 8);
					sv14.reserve(reserve_amount);
					let sv_ref = &sv14;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				14 => {
					let mut sv15: SmallVec<[u32; 128]> = SmallVec::new();
					let capacity_val = _to_usize(GLOBAL_DATA, 192 + op_idx * 8);
					sv15.grow(capacity_val);
					let sv_ref = &sv15;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				15 => {
					let mut sv16: SmallVec<[u32; 256]> = SmallVec::new();
					let truncate_len = _to_usize(GLOBAL_DATA, 200 + op_idx * 8);
					for i in 0..10 {
						sv16.push(i);
					}
					sv16.truncate(truncate_len);
					let sv_ref = &sv16;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				16 => {
					let mut sv17: SmallVec<[u32; 64]> = SmallVec::new();
					let resize_len = _to_usize(GLOBAL_DATA, 208 + op_idx * 8) % 65;
					let resize_val = _to_u32(GLOBAL_DATA, 216 + op_idx * 8);
					sv17.resize(resize_len, resize_val);
					let sv_ref = &sv17;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				17 => {
					let mut sv18: SmallVec<[u32; 32]> = SmallVec::new();
					let extend_slice_size = _to_usize(GLOBAL_DATA, 224 + op_idx * 8) % 65;
					let mut slice_data = Vec::new();
					for i in 0..extend_slice_size {
						slice_data.push(_to_u32(GLOBAL_DATA, 232 + op_idx * 8 + i * 4));
					}
					sv18.extend_from_slice(&slice_data);
					let sv_ref = &sv18;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				18 => {
					let sv19: SmallVec<[u32; 16]> = SmallVec::new();
					let sv20: SmallVec<[u32; 16]> = SmallVec::new();
					let cmp_result = sv19.cmp(&sv20);
					let sv_ref = &sv19;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
				},
				_ => {
					let mut sv_default: SmallVec<[u32; 8]> = SmallVec::new();
					sv_default.push(_to_u32(GLOBAL_DATA, 240 + op_idx * 8));
					let sv_ref = &sv_default;
					let deref_result = sv_ref.deref();
					println!("{:?}", deref_result);
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