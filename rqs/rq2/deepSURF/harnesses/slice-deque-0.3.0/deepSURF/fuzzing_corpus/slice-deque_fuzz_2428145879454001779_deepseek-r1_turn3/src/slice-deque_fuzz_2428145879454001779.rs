#![forbid(unsafe_code)]
use afl::fuzz_nohook;
use global_data::{get_global_data, set_global_data};
use slice_deque::SliceDeque;

#[derive(Debug)]
struct CustomType0(String);

impl std::cmp::PartialEq for CustomType0 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 570);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_137 = _to_bool(GLOBAL_DATA, 578);
        return t_137;
    }
}

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
        let t_4 = String::from(t_3);
        let t_5 = CustomType0(t_4);
        return t_5;
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 8192 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let mut base_idx = 0;
        let elem_count = _to_usize(GLOBAL_DATA, base_idx) % 65;
        base_idx += 8;

        let mut elements = Vec::with_capacity(elem_count);
        for i in 0..elem_count {
            let len = _to_u8(GLOBAL_DATA, base_idx) % 17;
            base_idx += 1;
            let s = _to_str(GLOBAL_DATA, base_idx, base_idx + len as usize);
            base_idx += len as usize;
            elements.push(CustomType0(s.to_string()));
        }

        let constr_sel = _to_u8(GLOBAL_DATA, base_idx) % 3;
        base_idx += 1;

        let mut deque = match constr_sel {
            0 => SliceDeque::from(elements.as_slice()),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, base_idx);
                base_idx += 8;
                let mut dq = SliceDeque::with_capacity(cap);
                dq.extend(elements);
                dq
            }
            _ => slice_deque::from_elem(elements.pop().unwrap_or(CustomType0(String::new())), elem_count),
        };

        let op_count = _to_usize(GLOBAL_DATA, base_idx) % 16;
        base_idx += 8;

        for _ in 0..op_count {
            let op_type = _to_u8(GLOBAL_DATA, base_idx) % 7;
            base_idx += 1;

            match op_type {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, base_idx) % 17;
                    base_idx += 1;
                    let s = _to_str(GLOBAL_DATA, base_idx, base_idx + len as usize);
                    base_idx += len as usize;
                    deque.push_back(CustomType0(s.to_string()));
                }
                1 => {
                    if let Some(front) = deque.pop_front() {
                        println!("{:?}", front);
                    }
                }
                2 => {
                    let target_idx = _to_usize(GLOBAL_DATA, base_idx);
                    base_idx += 8;
                    if let Some(e) = deque.get(target_idx) {
                        println!("{:?}", e);
                    }
                }
                3 => {
                    let new_len = _to_usize(GLOBAL_DATA, base_idx);
                    base_idx += 8;
                    deque.truncate(new_len);
                }
                4 => {
                    let target_idx = _to_usize(GLOBAL_DATA, base_idx);
                    base_idx += 8;
                    let _ = deque.remove(target_idx);
                }
                5 => {
                    let len = _to_u8(GLOBAL_DATA, base_idx) % 17;
                    base_idx += 1;
                    let s = _to_str(GLOBAL_DATA, base_idx, base_idx + len as usize);
                    base_idx += len as usize;
                    let item = CustomType0(s.to_string());
                    deque.remove_item(&item);
                }
                _ => {
                    let (s1, s2) = deque.as_slices();
                    println!("{:?} {:?}", s1.len(), s2.len());
                }
            }
        }

        let final_idx = _to_usize(GLOBAL_DATA, base_idx);
        base_idx += 8;
        if let Some(e) = deque.get(final_idx) {
            println!("{:?}", e);
        }

        let len = _to_u8(GLOBAL_DATA, base_idx) % 17;
        base_idx += 1;
        let s = _to_str(GLOBAL_DATA, base_idx, base_idx + len as usize);
        let item = CustomType0(s.to_string());
        deque.remove_item(&item);
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