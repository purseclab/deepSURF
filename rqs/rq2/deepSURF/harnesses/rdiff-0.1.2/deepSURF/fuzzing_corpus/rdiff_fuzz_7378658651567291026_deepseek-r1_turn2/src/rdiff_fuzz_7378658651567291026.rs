#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use rdiff::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::io::Cursor;

struct CustomType0(String);

impl std::io::Read for CustomType0 {
    fn read(&mut self, _: &mut [u8]) -> Result<usize, std::io::Error> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 8);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_3 = _to_usize(GLOBAL_DATA, 16);
        Ok(t_3)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1200 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let num_ops = _to_usize(GLOBAL_DATA, 0) % 8;
        let mut current_offset = 8;

        let mut blocks = vec![];
        let mut diffs = vec![];
        let mut strings = vec![];

        for _ in 0..num_ops {
            if current_offset + 9 >= GLOBAL_DATA.len() { break; }
            let op_selector = _to_u8(GLOBAL_DATA, current_offset) % 6;
            current_offset += 1;

            match op_selector {
                0 => {
                    let block_size = _to_usize(GLOBAL_DATA, current_offset);
                    current_offset += 8;
                    blocks.push(BlockHashes::empty(block_size));
                }
                1 => {
                    let block_size = _to_usize(GLOBAL_DATA, current_offset);
                    current_offset += 8;
                    let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                    current_offset += 1;
                    let data = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                    current_offset += len as usize;
                    let reader = CustomType0(data.to_string());
                    if let Ok(bh) = BlockHashes::new(reader, block_size) {
                        blocks.push(bh);
                    }
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                    current_offset += 1;
                    let data = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                    current_offset += len as usize;
                    let mut reader = Cursor::new(data.as_bytes());
                    if let Ok(bh) = BlockHashes::expand_from(&mut reader) {
                        blocks.push(bh);
                    }
                }
                3 => {
                    if let Some(bh) = blocks.last_mut() {
                        let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                        current_offset += 1;
                        let data = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                        current_offset += len as usize;
                        let reader = CustomType0(data.to_string());
                        if let Ok(diff) = bh.diff_and_update(reader) {
                            let _ = println!("{:?}", diff);
                            diffs.push(diff);
                        }
                    }
                }
                4 => {
                    let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                    current_offset += 1;
                    let old_str = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                    current_offset += len as usize;
                    let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                    current_offset += 1;
                    let new_str = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                    current_offset += len as usize;
                    strings.push(rdiff::string_diff::find_diff(old_str, new_str, &rdiff::string_diff::EditDistance));
                }
                5 => {
                    if let Some(diff) = diffs.last_mut() {
                        let len = _to_u8(GLOBAL_DATA, current_offset) % 65;
                        current_offset += 1;
                        let data = _to_str(GLOBAL_DATA, current_offset, current_offset + len as usize);
                        current_offset += len as usize;
                        let mut reader = Cursor::new(data.as_bytes());
                        if let Ok(new_diff) = Diff::expand_from(&mut reader) {
                            let _ = println!("{:?}", new_diff);
                            diffs.push(new_diff);
                        }
                    }
                }
                _ => {}
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