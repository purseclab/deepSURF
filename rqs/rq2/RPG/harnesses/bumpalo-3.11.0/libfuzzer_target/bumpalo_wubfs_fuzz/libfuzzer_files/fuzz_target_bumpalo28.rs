#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate bumpalo;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
}

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


fn test_function28(_param0: usize) {
    let _local0 = bumpalo::Bump::try_new();
    let _local1_param0_helper1 = _unwrap_result(_local0);
    bumpalo::Bump::set_allocation_limit(&(_local1_param0_helper1) ,Some(_param0));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 8 {return;}
    let _param0 = _to_usize(data, 0);
    test_function28(_param0);
});
