#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::sync::Arc;

struct DummyWaker;

impl ArcWake for DummyWaker {
    fn wake_by_ref(_arc_self: &Arc<Self>) {}
}

#[derive(Debug, Clone)]
struct CustomType0(String);
struct CustomType1(String);

#[derive(Debug)]
struct DummyFuture(CustomType0);

impl Future for DummyFuture {
    type Output = CustomType0;

    fn poll(self: Pin<&mut Self>, _: &mut core::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(self.0.clone())
    }
}

impl std::ops::Deref for CustomType1 {
    type Target = DummyWaker;

    fn deref(&self) -> &Self::Target {
        static WAKER: DummyWaker = DummyWaker;
        &WAKER
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 128 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        let ops_count = (_to_u8(GLOBAL_DATA, 0) % 5) as usize;
        for i in 0..ops_count {
            let op_select = _to_u8(GLOBAL_DATA, 1 + i) % 3;
            match op_select {
                0 => {
                    let start = 10 + i * 12;
                    if start + 25 >= GLOBAL_DATA.len() { continue; }
                    
                    let t_5 = _to_u8(GLOBAL_DATA, start) % 17;
                    let t_6 = _to_str(GLOBAL_DATA, start + 1, start + 1 + t_5 as usize);
                    let future = DummyFuture(CustomType0(t_6.to_string()));
                    let boxed = Box::pin(future);
                    let mut future_obj = FutureObj::new(boxed);
                    
                    let waker = noop_waker();
                    let mut context = core::task::Context::from_waker(&waker);
                    let _ = Pin::new(&mut future_obj).poll(&mut context);
                },
                1 => {
                    let start = 30 + i * 10;
                    if start + 20 >= GLOBAL_DATA.len() { continue; }
                    
                    let len = _to_u8(GLOBAL_DATA, start) as usize % 65;
                    let data_slice = &GLOBAL_DATA[start + 1..start + 1 + len];
                    let s = core::str::from_utf8(data_slice).unwrap_or("");
                    let arc_wake = Arc::new(DummyWaker);
                    let waker = waker(arc_wake);
                    let mut context = core::task::Context::from_waker(&waker);

                    let mut t_5 = _to_u8(GLOBAL_DATA, start + len) % 17;
                    let t_6 = _to_str(GLOBAL_DATA, start + len + 1, start + len + 1 + t_5 as usize);
                    let future = DummyFuture(CustomType0(t_6.to_string()));
                    let boxed = Box::pin(future) as Pin<Box<dyn Future<Output = CustomType0> + Send>>;
                    let mut future_obj = FutureObj::new(boxed);
                    
                    let _ = Pin::new(&mut future_obj).poll(&mut context);
                },
                2 => {
                    let start = 50 + i * 8;
                    if start + 15 >= GLOBAL_DATA.len() { continue; }
                    
                    let t_5 = _to_u8(GLOBAL_DATA, start) as usize % 65;
                    let t_6 = _to_str(GLOBAL_DATA, start + 1, start + 1 + t_5);
                    let future = DummyFuture(CustomType0(t_6.to_string()));
                    let boxed = Box::pin(future);
                    let mut future_obj = FutureObj::new(boxed);
                    
                    let waker = noop_waker_ref();
                    let mut context = core::task::Context::from_waker(&*waker);
                    let _ = Pin::new(&mut future_obj).poll(&mut context);
                    println!("{:?}", future_obj);
                },
                _ => ()
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