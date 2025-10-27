#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate secp256k1;
fn _unwrap_result<T, E>(_res: Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            use std::process;
            process::exit(0);
        },
    }
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

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}


fn test_function14(_param0: i32) {
    let _local0 = secp256k1::Parity::from_i32(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    secp256k1::Parity::to_u8(_local1_param0_helper1);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 4 {return;}
    let _param0 = _to_i32(data, 0);
    test_function14(_param0);
});
