#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use slice_deque::*;
use global_data::*;
use std::ops::DerefMut;

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
        let t_4 = String::from(t_3);
        CustomType0(t_4)
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 3200 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let op_count = _to_u8(GLOBAL_DATA, 0) % 8;
        for i in 0..op_count {
            let constructor_sel = _to_usize(GLOBAL_DATA, 1 + i as usize) % 4;
            let mut deq = match constructor_sel {
                0 => SliceDeque::new(),
                1 => {
                    let cap = _to_usize(GLOBAL_DATA, 10) % 65;
                    SliceDeque::with_capacity(cap)
                }
                2 => {
                    let elem_cnt = _to_usize(GLOBAL_DATA, 20) % 65;
                    let elem_str = _to_str(GLOBAL_DATA, 30, 47);
                    from_elem(CustomType0(elem_str.to_string()), elem_cnt)
                }
                _ => {
                    let vec_len = _to_usize(GLOBAL_DATA, 100) % 65;
                    let mut v = Vec::with_capacity(vec_len);
                    for j in 0..vec_len {
                        let offset = 200 + j * 17;
                        let s_len = _to_u8(GLOBAL_DATA, offset) % 17;
                        let s = _to_str(GLOBAL_DATA, offset + 1, offset + 1 + s_len as usize);
                        v.push(CustomType0(s.to_string()));
                    }
                    SliceDeque::from(v.as_slice())
                }
            };

            let push_ops = _to_usize(GLOBAL_DATA, 500) % 10;
            for _ in 0..push_ops {
                let s_off = _to_usize(GLOBAL_DATA, 600) % 1000;
                let s_len = _to_u8(GLOBAL_DATA, s_off) % 17;
                let s = _to_str(GLOBAL_DATA, s_off + 1, s_off + 1 + s_len as usize);
                deq.push_back(CustomType0(s.to_string()));
            }

            if let Some(front) = deq.front_mut() {
                front.0.push('!');
                println!("Mutated front: {:?}", *front);
            }

            let trunc_pos = _to_usize(GLOBAL_DATA, 1000) % (deq.len() + 1);
            deq.truncate(trunc_pos);

            let drain_range = _to_usize(GLOBAL_DATA, 1010) % deq.len();
            let _ = deq.drain_filter(|x| x.0.len() > drain_range);

            let splice_start = _to_usize(GLOBAL_DATA, 1020) % deq.len();
            let splice_end = splice_start + _to_usize(GLOBAL_DATA, 1021) % (deq.len() - splice_start);
            let replacements = vec![
                CustomType0("splice1".into()),
                CustomType0("splice2".into())
            ];
            let _ = deq.splice(splice_start..splice_end, replacements.into_iter());

            let (s1, s2) = deq.as_slices();
            println!("Slices: {} + {}", s1.len(), s2.len());

            let cap_before = deq.capacity();
            deq.shrink_to_fit();
            if cap_before != deq.capacity() {
                let _ = deq.back_mut().map(|x| x.0.push_str(" shrunk"));
            }

            if let Some(f) = deq.front_mut() {
                *f = CustomType0("final_front".into());
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