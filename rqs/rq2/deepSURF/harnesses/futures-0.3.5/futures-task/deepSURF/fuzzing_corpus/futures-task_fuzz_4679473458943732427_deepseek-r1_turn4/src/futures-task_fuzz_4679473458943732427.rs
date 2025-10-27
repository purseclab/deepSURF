#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::ops::{Deref, DerefMut};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CustomType2(String);
struct CustomType1(String);
struct CustomType0(String);

impl Future for CustomType1 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Pending
    }
}

impl Future for CustomType0 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

impl std::marker::Unpin for CustomType2 {}

impl Deref for CustomType1 {
    type Target = CustomType2;

    fn deref(&self) -> &Self::Target {
        static CT2: CustomType2 = CustomType2(String::new());
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 0);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
        &CT2
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 {
            return;
        }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let num_ops = _to_u8(GLOBAL_DATA, 0) % 5;
        let mut future_objects = Vec::new();

        for i in 0..num_ops {
            let op_selector = _to_u8(GLOBAL_DATA, 1 + i as usize) % 7;
            match op_selector {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, 10) % 17;
                    let s = _to_str(GLOBAL_DATA, 20, 20 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let future_obj = LocalFutureObj::from(Box::new(custom));
                    future_objects.push(future_obj);
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, 30) % 17;
                    let s = _to_str(GLOBAL_DATA, 40, 40 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let converted = LocalFutureObj::from(Box::new(custom));
                    future_objects.push(converted);
                }
                2 => {
                    let len = _to_u8(GLOBAL_DATA, 50) % 17;
                    let s = _to_str(GLOBAL_DATA, 60, 60 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let boxed = Box::new(custom);
                    let future_obj = FutureObj::from(boxed);
                    let local_future = LocalFutureObj::from(future_obj);
                    future_objects.push(local_future);
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, 70) % 17;
                    let s = _to_str(GLOBAL_DATA, 80, 80 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let boxed = Box::new(custom);
                    let future_obj = FutureObj::from(boxed);
                    future_objects.push(future_obj.into());
                }
                4 => {
                    let waker = noop_waker();
                    let mut context = Context::from_waker(&waker);
                    for fut in &mut future_objects {
                        let pinned = Pin::new(fut);
                        let _ = pinned.poll(&mut context);
                    }
                }
                5 => {
                    let len = _to_u8(GLOBAL_DATA, 100) % 17;
                    let s = _to_str(GLOBAL_DATA, 110, 110 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let boxed = Box::new(LocalFutureObj::from(Box::new(custom)));
                    let converted = LocalFutureObj::from(boxed);
                    future_objects.push(converted);
                }
                6 => {
                    let len = _to_u8(GLOBAL_DATA, 150) % 17;
                    let s = _to_str(GLOBAL_DATA, 160, 160 + len as usize);
                    let custom = CustomType1(s.to_string());
                    let boxed = Box::new(custom);
                    let future_obj = LocalFutureObj::from(boxed);
                    let reboxed = Box::new(future_obj);
                    let final_convert = LocalFutureObj::from(reboxed);
                    future_objects.push(final_convert);
                }
                _ => {}
            }
        }

        let target_len = _to_u8(GLOBAL_DATA, 200) % 65;
        let s = _to_str(GLOBAL_DATA, 210, 210 + target_len as usize);
        let target_obj = CustomType1(s.to_string());
        let boxed_target = Box::new(target_obj);
        let intermediate = LocalFutureObj::<()>::from(boxed_target);
        let boxed_target = Box::new(intermediate);
        let _final_target = LocalFutureObj::<()>::from(boxed_target);
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