#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::future::Future;
use std::ops::Deref;

struct MyWaker;

impl ArcWake for MyWaker {
    fn wake_by_ref(_arc_self: &std::sync::Arc<Self>) {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let index = _to_usize(GLOBAL_DATA, 0) % GLOBAL_DATA.len();
        let selector = GLOBAL_DATA[index] % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC in wake_by_ref");
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let num_ops = _to_u8(first_half, 0) % 65 + 1;
        let mut arcs = Vec::new();
        let mut wakers = Vec::new();

        for i in 0..num_ops {
            let op_byte = _to_u8(first_half, 1 + i as usize) % 8;
            match op_byte {
                0 => {
                    let arc = std::sync::Arc::new(MyWaker);
                    arcs.push(arc.clone());
                    let waker = waker(arc);
                    wakers.push(waker);
                }
                1 => {
                    let arc = std::sync::Arc::new(MyWaker);
                    arcs.push(arc.clone());
                    wakers.push(waker(arc));
                }
                2 => {
                    if !wakers.is_empty() {
                        let idx = _to_usize(first_half, 2 + i as usize) % wakers.len();
                        let waker = noop_waker();
                        let mut context = Context::from_waker(&waker);
                        let mut future = FutureObj::new(Box::new(async { () }));
                        let _ = std::pin::Pin::new(&mut future).poll(&mut context);
                        arcs.truncate(arcs.len().saturating_sub(1));
                    }
                }
                3 => {
                    if !arcs.is_empty() {
                        let idx = _to_usize(first_half, 3 + i as usize) % arcs.len();
                        println!("WakerRef: {:?}", waker_ref(&arcs[idx]).deref());
                        let arc = std::sync::Arc::new(MyWaker);
                        arcs.push(arc);
                    }
                }
                4 => {
                    let noop = noop_waker();
                    let mut context = Context::from_waker(&noop);
                    let mut future = LocalFutureObj::new(Box::new(async { () }));
                    let _ = std::pin::Pin::new(&mut future).poll(&mut context);
                }
                5 => {
                    let arc = std::sync::Arc::new(MyWaker);
                    let waker = waker(arc);
                    let mut context = Context::from_waker(&waker);
                    let mut future = FutureObj::new(Box::new(async { () }));
                    let _ = std::pin::Pin::new(&mut future).poll(&mut context);
                    wakers.push(waker);
                }
                6 => {
                    if !arcs.is_empty() {
                        let idx = _to_usize(first_half, 6 + i as usize) % arcs.len();
                        let waker = waker(arcs[idx].clone());
                        let mut context = Context::from_waker(&waker);
                        let mut future = LocalFutureObj::new(Box::new(async { () }));
                        let _ = std::pin::Pin::new(&mut future).poll(&mut context);
                    }
                }
                7 => {
                    let noop = noop_waker();
                    let mut context = Context::from_waker(&noop);
                    let future = FutureObj::from(Box::new(async { () }) as Box<_>);
                    let mut future = LocalFutureObj::from(future);
                    let _ = std::pin::Pin::new(&mut future).poll(&mut context);
                }
                _ => unreachable!(),
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