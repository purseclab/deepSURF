#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use string_interner::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};

struct CustomType1(String);
struct CustomType2(String);
struct CustomType3(String);
#[derive(Debug)]
struct CustomType0(usize);

impl std::marker::Copy for CustomType0 {}

impl std::clone::Clone for CustomType0 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 74);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_15 = _to_usize(GLOBAL_DATA, 82);
        CustomType0(t_15)
    }
}

impl std::hash::Hasher for CustomType2 {
    fn write(&mut self, _: &[u8]) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 98);
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

    fn finish(&self) -> u64 {
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
        _to_u64(GLOBAL_DATA, 114)
    }
}

impl std::convert::Into<String> for CustomType3 {
    fn into(self) -> String {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 189);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_34 = _to_u8(GLOBAL_DATA, 197) % 17;
        let t_35 = _to_str(GLOBAL_DATA, 198, 198 + t_34 as usize);
        String::from(t_35)
    }
}

impl std::cmp::Ord for CustomType0 {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 58);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_13 = _to_usize(GLOBAL_DATA, 66);
        match (t_13 % 3usize) {
            0 => std::cmp::Ordering::Equal,
            1 => std::cmp::Ordering::Greater,
            2 => std::cmp::Ordering::Less,
            _ => unreachable!(),
        }
    }
}

impl std::cmp::PartialEq for CustomType0 {
    fn eq(&self, _: &Self) -> bool {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 49);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_bool(GLOBAL_DATA, 57)
    }
}

impl std::cmp::PartialOrd for CustomType0 {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let t_0 = _to_usize(GLOBAL_DATA, 8);
        let t_1 = string_interner::Sym::from_usize(t_0);
        let t_2 = &t_1;
        let t_3 = _to_usize(GLOBAL_DATA, 16);
        let t_4 = string_interner::Sym::from_usize(t_3);
        let t_5 = &t_4;
        Some(string_interner::Sym::cmp(t_2, t_5))
    }
}

impl std::cmp::Eq for CustomType0 {}

impl std::hash::BuildHasher for CustomType1 {
    type Hasher = CustomType2;

    fn build_hasher(&self) -> Self::Hasher {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 122);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_19 = _to_u8(GLOBAL_DATA, 130) % 17;
        let t_20 = _to_str(GLOBAL_DATA, 131, 131 + t_19 as usize);
        CustomType2(String::from(t_20))
    }
}

impl string_interner::Symbol for CustomType0 {
    fn from_usize(_: usize) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let t_8 = _to_u8(GLOBAL_DATA, 24);
        if t_8 % 2 == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let t_9 = _to_usize(GLOBAL_DATA, 25);
        CustomType0(t_9)
    }

    fn to_usize(self) -> usize {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 33);
        let custom_impl_inst_num = self.0;
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        _to_usize(GLOBAL_DATA, 41)
    }
}

impl std::convert::AsRef<str> for CustomType3 {
    fn as_ref(&self) -> &str {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 164);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector {
            1 => global_data.first_half,
            _ => global_data.second_half,
        };
        let mut t_29 = _to_u8(GLOBAL_DATA, 172) % 17;
        let t_30 = _to_str(GLOBAL_DATA, 173, 173 + t_29 as usize);
        t_30
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 4092 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut data_ptr = 0;

        let constructor_sel = _to_u8(GLOBAL_DATA, data_ptr) % 3;
        data_ptr += 1;

        let mut interner: StringInterner<CustomType0> = match constructor_sel {
            0 => StringInterner::new(),
            1 => {
                let cap = _to_usize(GLOBAL_DATA, data_ptr);
                data_ptr += 8;
                StringInterner::with_capacity(cap)
            }
            _ => {
                let num_strs = _to_u8(GLOBAL_DATA, data_ptr) % 10;
                data_ptr += 1;
                let mut strings = Vec::new();
                for _ in 0..num_strs {
                    let len = _to_u8(GLOBAL_DATA, data_ptr) % 17;
                    data_ptr += 1;
                    let s = _to_str(GLOBAL_DATA, data_ptr, data_ptr + len as usize);
                    data_ptr += len as usize;
                    strings.push(CustomType3(s.to_string()));
                }
                strings.into_iter().collect()
            }
        };

        let num_ops = _to_u8(GLOBAL_DATA, data_ptr) % 100;
        data_ptr += 1;

        for _ in 0..num_ops {
            if data_ptr >= GLOBAL_DATA.len() { break; }
            let action = _to_u8(GLOBAL_DATA, data_ptr) % 8;
            data_ptr += 1;

            match action {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, data_ptr) % 17;
                    data_ptr += 1;
                    let s = _to_str(GLOBAL_DATA, data_ptr, data_ptr + len as usize);
                    data_ptr += len as usize;
                    let sym = interner.get_or_intern(CustomType3(s.to_string()));
                    println!("Interned: {:?}", sym);
                }
                1 => {
                    let val = _to_usize(GLOBAL_DATA, data_ptr);
                    data_ptr += 8;
                    if let Some(s) = interner.resolve(CustomType0(val)) {
                        println!("Resolved: {}", s);
                    }
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, data_ptr) % 17;
                    data_ptr += 1;
                    let s = _to_str(GLOBAL_DATA, data_ptr, data_ptr + len as usize);
                    data_ptr += len as usize;
                    if let Some(sym) = interner.get(s) {
                        println!("Exists: {:?}", sym);
                    }
                }
                3 => {
                    for (sym, s) in interner.iter() {
                        println!("Entry: {:?} => {}", sym, s);
                    }
                }
                4 => {
                    interner.shrink_to_fit();
                    println!("Shrunk");
                }
                5 => {
                    let new_cap = _to_usize(GLOBAL_DATA, data_ptr);
                    data_ptr += 8;
                    interner.reserve(new_cap);
                }
                6 => {
                    let idx = _to_usize(GLOBAL_DATA, data_ptr);
                    data_ptr += 8;
                    println!("Capacity: {}", interner.capacity());
                    println!("Is empty: {}", interner.is_empty());
                }
                _ => {
                    let sym1 = string_interner::Sym::from_usize(_to_usize(GLOBAL_DATA, data_ptr));
                    data_ptr += 8;
                    let sym2 = string_interner::Sym::from_usize(_to_usize(GLOBAL_DATA, data_ptr));
                    data_ptr += 8;
                    println!("Cmp: {:?}", string_interner::Sym::cmp(&sym1, &sym2));
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