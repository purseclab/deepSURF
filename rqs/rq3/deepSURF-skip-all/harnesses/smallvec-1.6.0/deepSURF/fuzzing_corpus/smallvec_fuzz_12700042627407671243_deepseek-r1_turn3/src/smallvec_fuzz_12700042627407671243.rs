#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::RangeBounds;

#[derive(Debug)]
struct CustomType4(String);
#[derive(Debug)]
struct CustomType3(String);

impl Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 19) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(data_part, 27) % 17;
        let s = String::from(_to_str(data_part, 28, 28 + len as usize));
        CustomType3(s)
    }
}

impl RangeBounds<usize> for CustomType4 {
    fn end_bound(&self) -> std::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 588) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => GLOBAL_DATA,
            _ => global_data.second_half,
        };
        let len = _to_u8(data_part, 596) % 17;
        let value = _to_usize(data_part, 597 + len as usize);
        let leaked = Box::leak(Box::new(value));
        std::ops::Bound::Included(leaked)
    }

    fn start_bound(&self) -> std::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = (_to_usize(GLOBAL_DATA, 613) + self.0.len()) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let data_part = match selector {
            1 => GLOBAL_DATA,
            _ => global_data.second_half,
        };
        let len = _to_u8(data_part, 621) % 17;
        let value = _to_usize(data_part, 622 + len as usize);
        let leaked = Box::leak(Box::new(value));
        std::ops::Bound::Excluded(leaked)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        
        let mut smallvecs = vec![];
        for _ in 0..3 {
            let mut vec = Vec::new();
            let count = _to_usize(global_data.first_half, 0) % 65;
            for i in 0..count {
                let len = _to_u8(global_data.first_half, 1 + i*2) % 17;
                let s = String::from(_to_str(global_data.first_half, 2 + i*3, 2 + i*3 + len as usize));
                vec.push(CustomType3(s));
            }

            match _to_u8(global_data.first_half, 100) % 4 {
                0 => smallvecs.push(SmallVec::<[CustomType3; 32]>::from_vec(vec)),
                1 => {
                    let mut sv = SmallVec::<[CustomType3; 32]>::new();
                    sv.extend(vec);
                    smallvecs.push(sv);
                },
                2 => {
                    let mut sv = SmallVec::<[CustomType3; 32]>::from_iter(vec.into_iter());
                    sv.truncate(_to_usize(global_data.first_half, 200));
                    smallvecs.push(sv);
                },
                _ => smallvecs.push(SmallVec::<[CustomType3; 32]>::new()),
            }
        }

        for sv in &mut smallvecs {
            for _ in 0..5 {
                match _to_u8(global_data.second_half, 0) % 6 {
                    0 => {
                        let len = _to_u8(global_data.second_half, 1) % 17;
                        let s = String::from(_to_str(global_data.second_half, 2, 2 + len as usize));
                        sv.push(CustomType3(s));
                    },
                    1 => { sv.pop(); },
                    2 => {
                        let idx = _to_usize(global_data.second_half, 20);
                        let len = _to_u8(global_data.second_half, 28) % 17;
                        let s = String::from(_to_str(global_data.second_half, 30, 30 + len as usize));
                        sv.insert(idx, CustomType3(s));
                    },
                    3 => {
                        let idx = _to_usize(global_data.second_half, 50);
                        sv.remove(idx);
                    },
                    4 => {
                        let mut drain = sv.drain(..);
                        while let Some(item) = drain.next_back() {
                            println!("{:?}", item.0);
                        }
                    },
                    _ => {
                        let range_data = _to_str(global_data.second_half, 100, 116);
                        let range = CustomType4(range_data.into());
                        let mut drain = sv.drain(range);
                        drain.next_back();
                    },
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