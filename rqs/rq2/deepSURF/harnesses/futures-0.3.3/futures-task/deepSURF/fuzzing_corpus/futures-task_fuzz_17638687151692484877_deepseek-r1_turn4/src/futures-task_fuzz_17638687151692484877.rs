#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::task::Context;
use std::future::Future;
use std::pin::Pin;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 500 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let mut index = 0;

        let num_ops = _to_u8(global_data.first_half, index) % 5 + 1;
        index += 1;

        let mut futures = vec![];
        let mut wakers = vec![];

        for _ in 0..num_ops {
            let op = _to_u8(global_data.first_half, index) % 4;
            index += 1;

            match op {
                0 => {
                    let constructor = _to_u8(global_data.first_half, index) % 3;
                    index += 1;
                    let len = _to_u8(global_data.first_half, index) % 17;
                    index += 1;
                    let s = _to_str(global_data.first_half, index, index + len as usize);
                    index += len as usize;

                    let fut = std::future::ready(());
                    let future_obj = match constructor {
                        0 => FutureObj::new(Box::new(fut)),
                        1 => FutureObj::from(Box::new(fut)),
                        _ => FutureObj::from(Box::pin(fut)),
                    };
                    futures.push(future_obj);
                }
                1 => {
                    let waker = noop_waker();
                    wakers.push(waker);
                }
                2 => {
                    if let Some(mut fut) = futures.pop() {
                        if let Some(waker) = wakers.last() {
                            let ctx = &mut Context::from_waker(waker);
                            let _ = Pin::new(&mut fut).poll(ctx);
                            println!("{:?}", FutureObj::new(Box::new(fut)));
                        }
                    }
                }
                3 => {
                    let noop_waker = noop_waker();
                    let ctx = &mut Context::from_waker(&noop_waker);
                    let len = _to_u8(global_data.first_half, index) % 17;
                    index += 1;
                    let s = _to_str(global_data.first_half, index, index + len as usize);
                    index += len as usize;
                    
                    let fut = std::future::pending::<()>();
                    let mut future_obj = FutureObj::from(Box::new(fut) as Box<dyn Future<Output=()> + Send>);
                    let _ = Pin::new(&mut future_obj).poll(ctx);
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