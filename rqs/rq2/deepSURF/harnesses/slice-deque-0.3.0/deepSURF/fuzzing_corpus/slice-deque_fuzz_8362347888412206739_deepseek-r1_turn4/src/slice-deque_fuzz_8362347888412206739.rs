#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::str::FromStr;

#[derive(Debug)]
struct CustomType0(String);

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 1);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 9) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 10, 10 + t_2 as usize);
        CustomType0(String::from(t_3))
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4500 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        
        let mut base_idx = 0;
        let mut t_1 = Vec::with_capacity(32);
        for _ in 0..(_to_u8(GLOBAL_DATA, base_idx) % 32) {
            let len = _to_u8(GLOBAL_DATA, base_idx + 1) % 17;
            let s = _to_str(GLOBAL_DATA, base_idx + 2, base_idx + 2 + len as usize);
            t_1.push(CustomType0(s.to_string()));
            base_idx += 2 + len as usize;
        }

        let mut deques = vec![
            SliceDeque::from(&mut t_1[..]),
            slice_deque::from_elem(CustomType0(String::new()), (_to_usize(GLOBAL_DATA, 200) % 65) + 1),
            SliceDeque::with_capacity(_to_usize(GLOBAL_DATA, 300) % 65)
        ];

        let ops_count = _to_u8(GLOBAL_DATA, 400) % 15;
        for i in 0..ops_count {
            let op_selector = _to_u8(GLOBAL_DATA, 401 + i as usize) % 8;
            match op_selector {
                0 => {
                    let elem = CustomType0(String::from(_to_str(GLOBAL_DATA, 500 + i as usize * 20, 510 + i as usize * 20)));
                    deques[0].try_push_front(elem);
                    if let Some(front) = deques[0].front_mut() {
                        println!("Front after push: {:?}", front.0);
                    }
                },
                1 => {
                    if let Some(front) = deques[1].front_mut() {
                        *front = CustomType0(String::from("modified"));
                        println!("{:?}", front.0);
                    }
                    let len = deques[1].len();
                    let _ = deques[1].drain(0..(_to_usize(GLOBAL_DATA, 600) % len));
                },
                2 => {
                    if deques.len() >= 3 {
                        let (d0_rest, d2_rest) = deques.split_at_mut(3);
                        d0_rest[0].append(&mut d2_rest[0]);
                    }
                },
                3 => {
                    let len = deques[1].len();
                    let range = _to_usize(GLOBAL_DATA, 600) % len;
                    let _ = deques[1].drain(0..range);
                    deques[1].reserve(_to_usize(GLOBAL_DATA, 923) % 128);
                },
                4 => {
                    let mut new_deque = SliceDeque::from_iter(t_1.iter().cloned());
                    deques[2].append(&mut new_deque);
                    println!("Appended {} elements", deques[2].len());
                },
                5 => {
                    if let Some(val) = deques[0].pop_back() {
                        deques[1].push_front(val);
                        let len = deques[1].len();
                        deques[1].truncate_back(_to_usize(GLOBAL_DATA, 873) % len);
                    }
                },
                6 => {
                    let len = deques[0].len();
                    if len > 0 {
                        let idx = _to_usize(GLOBAL_DATA, 700) % len;
                        let _ = deques[0].remove(idx);
                        println!("Removed item at {}", idx);
                    }
                },
                _ => {
                    let cap = _to_usize(GLOBAL_DATA, 800) % 100;
                    deques[2].reserve(cap);
                    deques[2].extend_from_slice(&[CustomType0(String::from("extend1")), CustomType0(String::from("extend2"))]);
                }
            }
        }

        let mut final_deque = SliceDeque::new();
        for mut d in deques {
            final_deque.append(&mut d);
            if let Some(item) = final_deque.back() {
                println!("Back item: {}", item.0);
            }
        }

        {
            let mut drain_iter = final_deque.drain(1..4);
            while let Some(item) = drain_iter.next() {
                println!("Drained: {}", item.0);
            }
        }

        if !final_deque.is_empty() {
            let len = final_deque.len();
            final_deque.truncate(_to_usize(GLOBAL_DATA, 900) % len);
            final_deque.splice(0..2, [CustomType0(String::from("splice1")), CustomType0(String::from("splice2"))]);
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