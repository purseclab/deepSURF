#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 0 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;

        struct MyWaker;
        impl ArcWake for MyWaker {
            fn wake_by_ref(_: &Arc<Self>) {}
        }

        let steps = _to_u8(GLOBAL_DATA, 0) % 5;
        for i in 0..steps {
            let offset = 1 + i as usize * 2;
            if offset >= GLOBAL_DATA.len() { return; }
            let op = _to_u8(GLOBAL_DATA, offset) % 3;

            match op {
                0 => {
                    let waker = noop_waker();
                    let waker_ref = WakerRef::new(&waker);
                    let ctx = Context::from_waker(&*waker_ref);
                    waker_ref.wake_by_ref();
                    let _ = Arc::new(MyWaker);
                }
                1 => {
                    let waker = futures_task::waker(Arc::new(MyWaker));
                    let waker_ref = WakerRef::new(&waker);
                    let mut ctx = Context::from_waker(&*waker_ref);
                    let mut future = Box::new(MyFuture);
                    let future_obj = LocalFutureObj::new(future);
                    let mut pinned = core::pin::pin!(future_obj);
                    let _ = pinned.as_mut().poll(&mut ctx);
                }
                2 => {
                    let my_arc = Arc::new(MyWaker);
                    let waker_ref = waker_ref(&my_arc);
                    let mut ctx = Context::from_waker(&*waker_ref);
                    let future = Box::new(MyFuture);
                    let mut future_obj = LocalFutureObj::new(future);
                    let _ = core::pin::Pin::new(&mut future_obj).poll(&mut ctx);
                    let _ = std::mem::ManuallyDrop::new(waker_ref);
                }
                _ => {}
            }
            println!("{:?}", steps);
        }
    });
}

struct MyFuture;
impl Future for MyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context) -> Poll<Self::Output> {
        Poll::Pending
    }
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