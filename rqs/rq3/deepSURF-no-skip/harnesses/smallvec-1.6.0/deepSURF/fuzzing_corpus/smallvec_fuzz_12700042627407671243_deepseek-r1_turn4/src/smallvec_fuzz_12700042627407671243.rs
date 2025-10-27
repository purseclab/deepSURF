#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::RangeBounds;

struct CustomType4(usize, usize);

impl RangeBounds<usize> for CustomType4 {
    fn start_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 588 + self.0) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        core::ops::Bound::Included(&self.0)
    }

    fn end_bound(&self) -> core::ops::Bound<&usize> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let selector = _to_u8(GLOBAL_DATA, 613 + self.1) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        core::ops::Bound::Excluded(&self.1)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 2048 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut sv = match _to_u8(GLOBAL_DATA, 0) % 7 {
            0 => SmallVec::<[u64; 32]>::new(),
            1 => SmallVec::with_capacity(_to_usize(GLOBAL_DATA, 1)),
            2 => SmallVec::from_slice(&[_to_u64(GLOBAL_DATA, 9)]),
            3 => SmallVec::from_vec(vec![_to_u64(GLOBAL_DATA, 17)]),
            4 => SmallVec::from_buf([_to_u64(GLOBAL_DATA, 256); 32]),
            5 => SmallVec::from_elem(_to_u64(GLOBAL_DATA, 264), _to_usize(GLOBAL_DATA, 272)),
            _ => {
                let slice: &[u64] = &[_to_u64(GLOBAL_DATA, 280), _to_u64(GLOBAL_DATA, 288)];
                slice.iter().cloned().collect()
            }
        };

        let num_ops = _to_usize(GLOBAL_DATA, 296) % 65;
        for idx in 0..num_ops {
            match _to_u8(GLOBAL_DATA, idx * 8) % 12 {
                0 => sv.push(_to_u64(GLOBAL_DATA, 300 + idx * 8)),
                1 => { sv.pop(); },
                2 => sv.insert(idx % (sv.len() + 1), _to_u64(GLOBAL_DATA, 400 + idx * 8)),
                3 => sv.truncate(_to_usize(GLOBAL_DATA, 500)),
                4 => sv.shrink_to_fit(),
                5 => sv.resize(_to_usize(GLOBAL_DATA, 600), _to_u64(GLOBAL_DATA, 608)),
                6 => sv.reserve(_to_usize(GLOBAL_DATA, 616)),
                7 => sv.extend_from_slice(&[_to_u64(GLOBAL_DATA, 624), _to_u64(GLOBAL_DATA, 632)]),
                8 => sv.dedup_by(|a, b| _to_u8(GLOBAL_DATA, 640 + idx) % 2 == 0),
                9 => sv.retain(|x| _to_u8(GLOBAL_DATA, 648 + idx) % 2 != 0),
                10 => sv.rotate_left(_to_usize(GLOBAL_DATA, 656)),
                _ => sv.append(&mut SmallVec::<[u64; 32]>::from_slice(&[_to_u64(GLOBAL_DATA, 664)])),
            };
        }

        let bounds = CustomType4(_to_usize(GLOBAL_DATA, 1500), _to_usize(GLOBAL_DATA, 1508));
        let elem = _to_u64(GLOBAL_DATA, 1600);
        sv.extend_from_slice(&[elem, elem.wrapping_add(1)]);
        
        let cap = sv.capacity();
        let slice = sv.as_mut_slice();
        println!("{:?} {:?}", cap, slice);
        
        sv.dedup();
        sv.insert_from_slice(0, &[_to_u64(GLOBAL_DATA, 1700)]);

        {
            let mut drain = sv.drain(bounds);
            drain.next_back();
            while let Some(item) = drain.next() {}
        }

        let comparison = _to_u8(GLOBAL_DATA, 1800);
        let _ = sv.cmp(&SmallVec::from_slice(&[_to_u64(GLOBAL_DATA, 1808)]));
        let _ = sv.partial_cmp(&SmallVec::from_slice(&[_to_u64(GLOBAL_DATA, 1816)]));

        if let Some(elem) = sv.as_slice().first() {
            println!("First element: {:?}", elem);
        }

        if sv.spilled() {
            let _ = sv.into_boxed_slice();
        } else {
            let _ = sv.into_vec();
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