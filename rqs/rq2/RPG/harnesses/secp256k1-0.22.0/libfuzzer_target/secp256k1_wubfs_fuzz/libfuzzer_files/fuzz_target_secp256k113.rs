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

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function13(_param0: u8) {
    let _local0 = secp256k1::Parity::from_u8(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    secp256k1::Parity::to_i32(_local1_param0_helper1);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 1 {return;}
    let _param0 = _to_u8(data, 0);
    test_function13(_param0);
});
