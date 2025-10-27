#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::ops::{Deref, DerefMut, Index, IndexMut};

type SV = SmallVec<[u8; 32]>;

struct Cursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    fn next_u8(&mut self) -> u8 {
        let v = _to_u8(self.data, self.pos);
        self.pos += 1;
        v
    }

    fn next_usize(&mut self) -> usize {
        let v = if self.pos + 8 <= self.data.len() {
            _to_usize(self.data, self.pos)
        } else {
            0usize
        };
        self.pos += 8;
        v
    }

    fn has_remaining(&self) -> bool {
        self.pos < self.data.len()
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 200 {
            return;
        }
        set_global_data(data);
        let global = get_global_data();
        let g = global.first_half;
        let mut cur = Cursor::new(g);

        let constructor_choice = cur.next_u8();
        let mut sv: SV = match constructor_choice % 5 {
            0 => SV::new(),
            1 => {
                let cap = cur.next_usize() % 65;
                SV::with_capacity(cap)
            }
            2 => {
                let len = (cur.next_u8() % 65) as usize;
                let mut v = Vec::new();
                for _ in 0..len {
                    v.push(cur.next_u8());
                }
                SV::from_vec(v)
            }
            3 => {
                let slice_len = (cur.next_u8() % 65) as usize;
                if cur.pos + slice_len > g.len() {
                    return;
                }
                let slice = &g[cur.pos..cur.pos + slice_len];
                cur.pos += slice_len;
                SV::from_slice(slice)
            }
            _ => {
                let elem = cur.next_u8();
                let count = cur.next_usize() % 65;
                SV::from_elem(elem, count)
            }
        };

        let op_count = cur.next_u8() % 25;
        for _ in 0..op_count {
            if !cur.has_remaining() {
                break;
            }
            match cur.next_u8() % 15 {
                0 => sv.push(cur.next_u8()),
                1 => { sv.pop(); }
                2 => {
                    if !sv.is_empty() {
                        let pos = cur.next_usize() % (sv.len() + 1);
                        sv.insert(pos, cur.next_u8());
                    }
                }
                3 => {
                    if !sv.is_empty() {
                        let pos = cur.next_usize() % sv.len();
                        sv.remove(pos);
                    }
                }
                4 => {
                    let new_len = cur.next_usize() % 65;
                    sv.truncate(new_len);
                }
                5 => {
                    let additional = cur.next_usize() % 65;
                    sv.reserve(additional);
                }
                6 => { sv.clear(); }
                7 => {
                    let slice_len = (cur.next_u8() % 65) as usize;
                    if cur.pos + slice_len > g.len() {
                        break;
                    }
                    let slice = &g[cur.pos..cur.pos + slice_len];
                    cur.pos += slice_len;
                    sv.extend_from_slice(slice);
                }
                8 => {
                    if !sv.is_empty() {
                        let pos = cur.next_usize() % sv.len();
                        sv.swap_remove(pos);
                    }
                }
                9 => { sv.dedup(); }
                10 => {
                    let add_len = cur.next_usize() % 65;
                    if add_len > 0 {
                        sv.resize(add_len, cur.next_u8());
                    }
                }
                11 => {
                    let elem = cur.next_u8();
                    let count = cur.next_usize() % 65;
                    sv.extend(std::iter::repeat(elem).take(count));
                }
                12 => {
                    if !sv.is_empty() {
                        let idx = cur.next_usize() % sv.len();
                        let val_ref = &sv[idx];
                        println!("{:?}", *val_ref);
                    }
                }
                13 => {
                    let clone = sv.clone();
                    let _ = sv.partial_cmp(&clone);
                }
                _ => {
                    let cap = sv.capacity();
                    if cap < 65 {
                        sv.grow(65);
                    }
                }
            }
        }

        if !sv.is_empty() {
            let drain_end = sv.len() / 2;
            let mut dr = sv.drain(0..drain_end);
            if let Some(item) = dr.next() {
                println!("{:?}", item);
            }
            dr.next_back();
        }

        println!("{:?}", sv.as_slice());

        let mut other = SV::from_slice(sv.as_slice());
        let _ = sv.cmp(&other);
        sv.append(&mut other);

        sv.clear();
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