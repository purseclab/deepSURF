#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::task::{Context, Poll};
use std::ops::Deref;
use std::future::Future;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.is_empty() { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let iter_count = (GLOBAL_DATA[0] % 8) as usize;

        for i in 0..iter_count {
            match GLOBAL_DATA[i % GLOBAL_DATA.len()] % 6 {
                0 => {
                    let waker = futures_task::noop_waker();
                    let waker_ref = futures_task::WakerRef::new(&waker);
                    println!("{:?}", *waker_ref.deref());
                    let mut ctx = Context::from_waker(&waker);
                    let future = futures_task::LocalFutureObj::new(Box::new(async {}));
                    let _ = futures_task::FutureObj::new(Box::new(async {}));
                }
                1 => {
                    let waker = futures_task::noop_waker();
                    let mut ctx = Context::from_waker(&waker);
                    let mut future = futures_task::LocalFutureObj::from(Box::new(std::future::ready(())));
                    let pinned = core::pin::Pin::new(&mut future);
                    let _ = pinned.poll(&mut ctx);
                    let _ = futures_task::FutureObj::from(Box::new(async {}));
                }
                2 => {
                    let waker = futures_task::noop_waker();
                    let manual_waker = core::mem::ManuallyDrop::new(waker);
                    let _waker_ref = futures_task::WakerRef::new_unowned(manual_waker);
                    let _ = std::alloc::Layout::new::<u8>();
                    let _ = std::ptr::NonNull::<u8>::dangling();
                }
                3 => {
                    let size = GLOBAL_DATA.get(i % GLOBAL_DATA.len()).copied().unwrap_or(0) as usize;
                    let align = GLOBAL_DATA.get((i + 1) % GLOBAL_DATA.len()).copied().unwrap_or(1) as usize;
                    let _ = std::alloc::Layout::from_size_align(size, align);
                    let _ = std::alloc::Layout::array::<u8>(size);
                    let _ = std::ptr::NonNull::new(core::ptr::NonNull::<u8>::dangling().as_ptr());
                }
                4 => {
                    let mut future = futures_task::LocalFutureObj::from(Box::new(async {}));
                    let waker = futures_task::noop_waker();
                    let mut ctx = Context::from_waker(&waker);
                    let _ = core::pin::Pin::new(&mut future).poll(&mut ctx);
                    let _future_obj = futures_task::FutureObj::new(Box::new(async {}));
                }
                5 => {
                    let value = GLOBAL_DATA.get(i % GLOBAL_DATA.len()).copied().unwrap_or(0);
                    let arc = std::sync::Arc::new(value);
                    let _ = std::sync::Arc::new(value);
                    println!("{:?}", *arc);
                }
                _ => (),
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