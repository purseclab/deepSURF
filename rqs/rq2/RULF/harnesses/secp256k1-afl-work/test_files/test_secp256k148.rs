#[macro_use]
extern crate afl;
extern crate secp256k1;
fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

use secp256k1::Context;

fn test_function48(mut _param0 :u8 ,_param1 :usize) {
    unsafe {
        let _ = secp256k1::VerifyOnlyPreallocated::deallocate(&(_param0) as *mut u8 ,_param1);
    }
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 9 {return;}
        let _param0 = _to_u8(data, 0);
        let _param1 = _to_usize(data, 1);
        test_function48(_param0 ,_param1);
    });
}
