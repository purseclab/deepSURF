#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use scratchpad::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType0(Vec<u8>);
struct CustomType1(String);

impl scratchpad::Tracking for CustomType1 {
    fn set(&mut self, _: usize, _: usize) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 82);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
    }
    
    fn capacity(&self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 90);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 98)
    }
    
    fn get(&self, _: usize) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 106);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 114)
    }
}

impl scratchpad::Buffer for CustomType0 {
    fn as_bytes_mut(&mut self) -> &mut [u8] {
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
        
        let t_0 = _to_u8(GLOBAL_DATA, 8) % 65;
        self.0.clear();
        for i in 0..32 {
            self.0.push(_to_u8(GLOBAL_DATA, 9 + i));
        }
        self.0.truncate(t_0 as usize);
        &mut self.0[..]
    }
    
    fn as_bytes(&self) -> &[u8] {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 41);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        
        let t_35 = _to_u8(GLOBAL_DATA, 49) % 65;
        &self.0[..t_35 as usize]
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let ops_count = _to_usize(GLOBAL_DATA, 0) % 16;
        let mut t_72 = _to_u8(GLOBAL_DATA, 122) % 17;
        let t_73 = _to_str(GLOBAL_DATA, 123, 123 + t_72 as usize);
        let t_74 = t_73.as_bytes().to_vec();
        let t_75 = CustomType0(t_74);
        
        let mut t_76 = _to_u8(GLOBAL_DATA, 139) % 17;
        let t_77 = _to_str(GLOBAL_DATA, 140, 140 + t_76 as usize);
        let t_78 = String::from(t_77);
        let t_79 = CustomType1(t_78);
        
        let t_80 = scratchpad::Scratchpad::new(t_75, t_79);
        
        for i in 0..ops_count {
            let op_selector = _to_usize(GLOBAL_DATA, 200 + i * 4) % 4;
            match op_selector {
                0 => {
                    let marker = t_80.mark_front();
                    if let Ok(m) = marker {
                        let alloc = m.allocate_array_with(
                            _to_usize(GLOBAL_DATA, 300 + i * 8) % 32,
                            |idx| (idx * 2) as u32
                        );
                        if let Ok(mut a) = alloc {
                            a.reverse();
                        }
                    }
                }
                1 => {
                    let marker = t_80.mark_back();
                    if let Ok(m) = marker {
                        let src_data: Vec<u8> = (0..16)
                            .map(|x| _to_u8(GLOBAL_DATA, 400 + i * 16 + x))
                            .collect();
                        let src_data: [u8; 16] = src_data.try_into().unwrap();
                        let alloc = m.allocate_slice_copy::<[u8], _>(&src_data[..]);
                        if let Ok(a) = alloc {
                            let _ = a.get(0);
                        }
                    }
                }
                2 => {
                    if let Ok(marker) = t_80.mark_front() {
                        let mut buffer = marker.allocate_default::<[u16; 8]>().unwrap();
                        buffer.fill(0);
                        t_80.mark_back();
                    }
                }
                _ => {
                    let t_81 = &t_80;
                    t_81.mark_back();
                    if let Ok(marker) = t_80.mark_back() {
                        let alloc = marker.allocate(_to_u64(GLOBAL_DATA, 500 + i * 8));
                        if let Ok(a) = alloc {
                            let value = *a;
                            let _ = format!("{:?}", value);
                        }
                    }
                }
            }
        }
        
        let t_81 = &t_80;
        t_81.mark_back();
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