#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use futures_task::*;
use global_data::*;
use std::pin::Pin;
use std::sync::Arc;
use std::future::Future;
use std::ops::Deref;

#[derive(Debug)]
struct DummyWaker;

impl ArcWake for DummyWaker {
    fn wake_by_ref(_: &Arc<Self>) {
        let global_data = get_global_data();
        let selector = _to_u8(global_data.first_half, 42) % 3;
        if selector == 0 {
            panic!("INTENTIONAL PANIC!");
        }
    }
}

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let first_half = global_data.first_half;

        let num_ops = _to_usize(first_half, 0) % 65;
        let mut index = 2;

        for _ in 0..num_ops {
            if index + 3 >= first_half.len() { break; }

            let mode = _to_u8(first_half, index) % 4;
            index += 1;
            let alloc_mode = _to_u8(first_half, index) % 3;
            index += 1;

            let mut future_obj = match mode {
                0 => {
                    let layout_sz = _to_usize(first_half, index);
                    let fut = FutureObj::new(Box::new(async move { layout_sz }));
                    index += 8;
                    fut
                }
                1 => {
                    let str_len = _to_u8(first_half, index) as usize % 64;
                    let s = _to_str(first_half, index + 1, index + 1 + str_len).to_owned();
                    let fut = FutureObj::new(Box::new(async move { s.len() }));
                    index += 1 + str_len;
                    fut
                }
                2 => {
                    let char_val = _to_char(first_half, index);
                    let fut = FutureObj::new(Box::new(async move { char_val as usize }));
                    index += 4;
                    fut
                }
                _ => {
                    let ptr = _to_u64(first_half, index);
                    let boxed_fut = Box::new(async move { ptr as usize });
                    index += 8;
                    FutureObj::new(boxed_fut)
                }
            };

            let waker = match alloc_mode {
                0 => futures_task::noop_waker(),
                1 => {
                    let arc_waker = Arc::new(DummyWaker);
                    futures_task::waker(arc_waker)
                }
                _ => {
                    let weak_waker = Arc::downgrade(&Arc::new(DummyWaker));
                    let man_drop = std::mem::ManuallyDrop::new(futures_task::noop_waker());
                    let waker_ref = WakerRef::new_unowned(man_drop);
                    waker_ref.deref().clone()
                }
            };

            let ctx = &mut Context::from_waker(&waker);
            let mut pinned = Pin::new(&mut future_obj);
            let _ = pinned.as_mut().poll(ctx);

            let layout = std::alloc::Layout::new::<u8>();
            let _ = std::alloc::Layout::array::<u8>(_to_usize(first_half, index));
            index = index.wrapping_add(2);
            
            let _ = futures_task::waker_ref(&Arc::new(DummyWaker));
            let _waker_ref = WakerRef::new(&waker);
            println!("{:?}", _waker_ref);
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