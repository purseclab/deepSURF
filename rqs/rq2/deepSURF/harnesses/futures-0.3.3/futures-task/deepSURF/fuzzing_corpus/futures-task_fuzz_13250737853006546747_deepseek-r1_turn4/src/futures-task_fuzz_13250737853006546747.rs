#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

struct DummyWake;
impl ArcWake for DummyWake {
    fn wake_by_ref(_: &Arc<Self>) {}
}

struct StringFuture(String);
struct DummyFuture;

impl std::future::Future for DummyFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

impl std::future::Future for StringFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        std::task::Poll::Pending
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 300 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;
        let mut idx = 0;

        let num_ops = _to_u8(first_half, idx) % 5 + 1;
        idx += 1;

        let mut futures = Vec::new();
        let mut wakers = vec![futures_task::noop_waker()];

        for _ in 0..num_ops {
            let op = _to_u8(first_half, idx) % 4;
            idx += 1;

            match op {
                0 => {
                    let len = _to_u8(first_half, idx) as usize % 65;
                    idx += 1;
                    let s = _to_str(first_half, idx, idx + len);
                    idx += len;
                    let boxed = Box::new(StringFuture(s.to_string()));
                    let fut = FutureObj::new(boxed);
                    futures.push(fut);
                }
                1 => {
                    let boxed = Box::new(DummyFuture);
                    let fut = FutureObj::new(boxed);
                    futures.push(fut);
                }
                2 => {
                    let dummy_arc = Arc::new(DummyWake);
                    let wk = futures_task::waker(dummy_arc);
                    wakers.push(wk);
                }
                3 => {
                    let md = std::mem::ManuallyDrop::new(futures_task::noop_waker());
                    let wk_ref = WakerRef::new_unowned(md);
                    println!("{:?}", *wk_ref);
                }
                _ => unreachable!(),
            }
        }

        let num_polls = _to_u8(first_half, idx) % (futures.len() as u8 + 1);
        idx += 1;
        
        for i in 0..num_polls {
            let f_idx = _to_usize(first_half, idx) % futures.len();
            idx += 1;
            let w_idx = _to_usize(first_half, idx) % wakers.len();
            idx += 1;

            let ctx = &mut std::task::Context::from_waker(&wakers[w_idx]);
            let _ = Pin::new(&mut futures[f_idx]).poll(ctx);
        }

        let len1 = _to_u8(first_half, idx) % 65;
        idx += 1;
        let s1 = _to_str(first_half, idx, idx + len1 as usize);
        let ft1 = StringFuture(s1.to_string());
        let mut pinned_ft = FutureObj::new(Box::new(ft1));

        let len2 = _to_u8(first_half, idx) % 65;
        idx += 1;
        let s2 = _to_str(first_half, idx, idx + len2 as usize);
        let ft2 = StringFuture(s2.to_string());
        let mut local_fut = LocalFutureObj::new(Box::new(ft2));

        let waker_idx = _to_usize(first_half, idx) % wakers.len();
        let ctx_main = &mut std::task::Context::from_waker(&wakers[waker_idx]);
        let _ = Pin::new(&mut pinned_ft).poll(ctx_main);
        let _ = Pin::new(&mut local_fut).poll(ctx_main);
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