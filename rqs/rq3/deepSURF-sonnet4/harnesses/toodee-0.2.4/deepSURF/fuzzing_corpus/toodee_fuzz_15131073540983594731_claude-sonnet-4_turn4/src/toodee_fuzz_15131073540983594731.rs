#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use toodee::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, Default)]
struct CustomType0(String);

fn main (){
	fuzz_nohook!(|data: &[u8]| {
		if data.len() < 512 {return;}
		set_global_data(data);
		let global_data = get_global_data();
		let GLOBAL_DATA = global_data.first_half;
		
		let num_operations = _to_usize(GLOBAL_DATA, 0) % 65;
		
		for op_idx in 0..num_operations {
			let base_offset = 8 + op_idx * 64;
			if base_offset + 64 > GLOBAL_DATA.len() { break; }
			
			let op_type = _to_u8(GLOBAL_DATA, base_offset) % 8;
			
			match op_type {
				0 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut t_2 = match _to_u8(GLOBAL_DATA, base_offset + 1) % 4 {
						0 => toodee::TooDee::<CustomType0>::new(t_0, t_1),
						1 => toodee::TooDee::<CustomType0>::with_capacity(t_0),
						2 => {
							let init_val = CustomType0(String::from("init"));
							toodee::TooDee::<CustomType0>::init(t_0, t_1, init_val)
						},
						_ => toodee::TooDee::<CustomType0>::with_capacity(t_0),
					};
					
					let t_4 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let mut t_5 = toodee::TooDee::<CustomType0>::col(&t_2, t_4);
					println!("{:?}", t_5.next_back());
					
					let mut t_6 = toodee::TooDee::<CustomType0>::col_mut(&mut t_2, t_4);
					println!("{:?}", t_6.next_back());
					
					let t_7 = toodee::TooDee::<CustomType0>::rows(&t_2);
					let mut t_8 = t_7.collect::<Vec<_>>();
					
					let mut t_9 = toodee::TooDee::<CustomType0>::rows_mut(&mut t_2);
					let mut t_10 = t_9.collect::<Vec<_>>();
				},
				1 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let mut t_4 = toodee::TooDee::<CustomType0>::rows(&t_2);
					println!("{:?}", t_4.next());
					
					let mut t_5 = toodee::TooDee::<CustomType0>::rows_mut(&mut t_2);
					println!("{:?}", t_5.next_back());
					
					let t_6 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let mut t_7 = toodee::TooDee::<CustomType0>::col(&t_2, t_6);
					println!("{:?}", t_7.next_back());
					
					let t_8 = toodee::TooDee::<CustomType0>::data(&t_2);
					println!("{:?}", t_8.len());
					
					let t_9 = toodee::TooDee::<CustomType0>::data_mut(&mut t_2);
					println!("{:?}", t_9.len());
				},
				2 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let t_3 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let t_4 = _to_usize(GLOBAL_DATA, base_offset + 32);
					let t_5 = _to_usize(GLOBAL_DATA, base_offset + 40);
					let t_6 = _to_usize(GLOBAL_DATA, base_offset + 48);
					
					let view_start = (t_3, t_4);
					let view_end = (t_5, t_6);
					
					let t_7 = toodee::TooDee::<CustomType0>::view(&t_2, view_start, view_end);
					let mut t_8 = toodee::TooDeeView::<CustomType0>::col(&t_7, t_3);
					println!("{:?}", t_8.next_back());
					
					let mut t_9 = toodee::TooDee::<CustomType0>::view_mut(&mut t_2, view_start, view_end);
					let mut t_10 = toodee::TooDeeViewMut::<CustomType0>::col_mut(&mut t_9, t_3);
					println!("{:?}", t_10.next_back());
					
					let t_11 = toodee::TooDeeViewMut::<CustomType0>::rows(&t_9);
					let mut t_12 = t_11.collect::<Vec<_>>();
				},
				3 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let slice_data = vec![CustomType0(String::from("test")); t_0 % 65];
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let t_2 = _to_usize(GLOBAL_DATA, base_offset + 24);
					
					let t_3 = toodee::TooDeeView::<CustomType0>::new(t_1, t_2, &slice_data);
					let mut t_4 = toodee::TooDeeView::<CustomType0>::col(&t_3, t_1);
					println!("{:?}", t_4.next_back());
					
					let mut slice_data_mut = vec![CustomType0(String::from("test_mut")); t_0 % 65];
					let t_5 = toodee::TooDeeViewMut::<CustomType0>::new(t_1, t_2, &mut slice_data_mut);
					let mut t_6 = toodee::TooDeeViewMut::<CustomType0>::col(&t_5, t_1);
					println!("{:?}", t_6.next_back());
					
					let t_7 = toodee::TooDeeView::<CustomType0>::rows(&t_3);
					let mut t_8 = t_7.collect::<Vec<_>>();
				},
				4 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let t_3 = _to_usize(GLOBAL_DATA, base_offset + 24);
					
					let t_4 = toodee::TooDee::<CustomType0>::cells(&t_2);
					let mut t_5 = t_4.collect::<Vec<_>>();
					
					let t_6 = toodee::TooDee::<CustomType0>::cells_mut(&mut t_2);
					let mut t_7 = t_6.collect::<Vec<_>>();
					
					if let Some(mut drain_result) = toodee::TooDee::<CustomType0>::pop_col(&mut t_2) {
						println!("{:?}", drain_result.next_back());
					}
					
					let mut drain_result2 = toodee::TooDee::<CustomType0>::remove_col(&mut t_2, t_3);
					println!("{:?}", drain_result2.next_back());
				},
				5 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let mut t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let t_3 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let t_4 = _to_usize(GLOBAL_DATA, base_offset + 32);
					
					let mut t_5 = toodee::TooDee::<CustomType0>::col(&t_2, t_3);
					println!("{:?}", t_5.nth_back(t_4));
					
					let mut t_6 = toodee::TooDee::<CustomType0>::col_mut(&mut t_2, t_3);
					println!("{:?}", t_6.nth_back(t_4));
					
					let t_7 = toodee::TooDee::<CustomType0>::size(&t_2);
					println!("{:?}", t_7);
					
					let t_8 = toodee::TooDee::<CustomType0>::is_empty(&t_2);
					println!("{:?}", t_8);
				},
				6 => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let t_3 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let mut t_4 = toodee::TooDee::<CustomType0>::col(&t_2, t_3);
					
					if t_4.size_hint().0 > 0 {
						let t_5 = _to_usize(GLOBAL_DATA, base_offset + 32);
						println!("{:?}", t_4.nth(t_5));
					}
					
					let t_6 = toodee::TooDee::<CustomType0>::bounds(&t_2);
					println!("{:?}", t_6);
					
					let t_7 = toodee::TooDee::<CustomType0>::capacity(&t_2);
					println!("{:?}", t_7);
				},
				_ => {
					let t_0 = _to_usize(GLOBAL_DATA, base_offset + 8);
					let t_1 = _to_usize(GLOBAL_DATA, base_offset + 16);
					let t_2 = toodee::TooDee::<CustomType0>::new(t_0, t_1);
					
					let t_3 = _to_usize(GLOBAL_DATA, base_offset + 24);
					let t_4 = _to_usize(GLOBAL_DATA, base_offset + 32);
					let t_5 = _to_usize(GLOBAL_DATA, base_offset + 40);
					let t_6 = _to_usize(GLOBAL_DATA, base_offset + 48);
					
					let view_start = (t_3, t_4);
					let view_end = (t_5, t_6);
					let t_7 = toodee::TooDee::<CustomType0>::view(&t_2, view_start, view_end);
					
					let t_8 = toodee::TooDee::<CustomType0>::from(t_7);
					let mut t_9 = toodee::TooDee::<CustomType0>::col(&t_8, t_3);
					println!("{:?}", t_9.next_back());
					
					let vec_data = vec![CustomType0(String::from("vec_test")); t_0 % 65];
					let t_10 = toodee::TooDee::<CustomType0>::from_vec(t_0, t_1, vec_data);
					
					let box_data = vec![CustomType0(String::from("box_test")); t_0 % 65].into_boxed_slice();
					let t_11 = toodee::TooDee::<CustomType0>::from_box(t_0, t_1, box_data);
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