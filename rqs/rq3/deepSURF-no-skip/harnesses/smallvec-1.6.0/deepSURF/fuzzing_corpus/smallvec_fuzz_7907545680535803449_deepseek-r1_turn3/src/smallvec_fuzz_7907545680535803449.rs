#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::RangeBounds;

#[derive(Debug, PartialEq, PartialOrd)]
struct CustomType3(String);
#[derive(Debug)]
struct CustomType4 {
    start: usize,
    end: usize,
}

impl Clone for CustomType3 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let selector = (_to_usize(global_data.first_half, 19) + self.0.len()) % 3;
        if selector == 0 { panic!("INTENTIONAL PANIC!"); }
        let data_part = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let len = _to_u8(data_part, 27) % 17;
        CustomType3(String::from(_to_str(data_part, 28, 28 + len as usize)))
    }
}

impl RangeBounds<usize> for CustomType4 {
    fn start_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Included(&self.start)
    }

    fn end_bound(&self) -> std::ops::Bound<&usize> {
        std::ops::Bound::Excluded(&self.end)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4096 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let mut ops = _to_u8(g, 0) % 8;
        let mut sv = match _to_u8(g, 1) % 3 {
            0 => SmallVec::<[CustomType3; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(g, 2) % 65),
            _ => SmallVec::from_elem(
                CustomType3(String::from(_to_str(g, 3, 3 + _to_u8(g, 4) as usize))), 
                _to_usize(g,5) % 65
            ),
        };

        for i in 0.._to_usize(g, 6) % 65 {
            let elem = CustomType3(String::from(_to_str(g, 7 + i*3, 7 + i*3 + _to_u8(g, 8 + i*3) as usize)));
            sv.push(elem);
        }

        if let Some(v) = sv.last_mut() {
            *v = CustomType3(String::from("modified"));
        }

        for _ in 0.._to_usize(g, 100) % 5 {
            sv.insert(
                _to_usize(g, 101) % (sv.len() + 1),
                CustomType3(String::from(_to_str(g, 102, 102 + _to_u8(g, 103) as usize)))
            );
        }

        let start = _to_usize(g, 200);
        let end = _to_usize(g, 208);
        let range = CustomType4 { start, end };
        let drained = sv.drain(range);
        let _ = drained.count();

        let mut other = SmallVec::from(sv.as_slice());
        other.append(&mut sv);
        let _ = other.partial_cmp(&sv);

        other.truncate(_to_usize(g, 300) % 65);
        other.reserve(_to_usize(g, 301));
        other.shrink_to_fit();
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