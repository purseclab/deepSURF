#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::pin::Pin;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::sync::Arc;
use std::future::Future;
use std::task::Poll;

struct CustomType0(String);
struct CustomType1(CustomType2);
struct CustomType2(String);

impl Future for CustomType1 {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

impl std::ops::Deref for CustomType1 {
    type Target = CustomType2;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::marker::Unpin for CustomType2 {}

struct MyWaker;

impl ArcWake for MyWaker {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let second_half = global_data.second_half;

        let num_ops = _to_usize(first_half, 0) % 5 + 1;
        for i in 0..num_ops {
            let op_selector = _to_u8(first_half, i) % 4;
            match op_selector {
                0 => {
                    let idx = 10 + i * 8;
                    let len = _to_u8(first_half, idx) % 17;
                    let s = _to_str(first_half, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType1(CustomType2(s.to_string()));
                    let boxed = Box::new(ct);
                    let pinned = Box::pin(boxed);
                    let mut fut_obj = FutureObj::from(pinned);
                    let arc_waker = Arc::new(MyWaker);
                    let w_ref = waker_ref(&arc_waker);
                    println!("{:?}", w_ref);
                    let cx = &mut Context::from_waker(&*w_ref);
                    let _ = Pin::new(&mut fut_obj).poll(cx);
                }
                1 => {
                    let idx = 20 + i * 8;
                    let len = _to_u8(second_half, idx) % 17;
                    let s = _to_str(second_half, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType1(CustomType2(s.to_string()));
                    let boxed = Box::new(ct);
                    let pinned = Box::pin(boxed);
                    let mut fut_obj = FutureObj::from(pinned);
                    let noop_waker = noop_waker();
                    let cx = &mut Context::from_waker(&noop_waker);
                    let _ = Pin::new(&mut fut_obj).poll(cx);
                }
                2 => {
                    let idx = 30 + i * 8;
                    let len = _to_u8(first_half, idx) % 17;
                    let s = _to_str(first_half, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType1(CustomType2(s.to_string()));
                    let boxed = Box::new(ct);
                    let pinned = Box::pin(boxed);
                    let mut fut_obj = FutureObj::from(pinned);
                    let cx = &mut Context::from_waker(noop_waker_ref());
                    let _ = Pin::new(&mut fut_obj).poll(cx);
                }
                3 => {
                    let idx = 40 + i * 8;
                    let len = _to_u8(second_half, idx) % 17;
                    let s = _to_str(second_half, idx + 1, idx + 1 + len as usize);
                    let ct = CustomType1(CustomType2(s.to_string()));
                    let boxed = Box::new(ct);
                    let pinned = Box::pin(boxed);
                    let mut local_fut = LocalFutureObj::from(pinned);
                    let cx = &mut Context::from_waker(noop_waker_ref());
                    let _ = Pin::new(&mut local_fut).poll(cx);
                }
                _ => unreachable!()
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