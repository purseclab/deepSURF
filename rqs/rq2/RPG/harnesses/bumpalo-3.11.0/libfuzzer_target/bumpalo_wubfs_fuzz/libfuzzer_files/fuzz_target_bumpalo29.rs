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


fn test_function29() {
    let _local0 = bumpalo::Bump::try_new();
    let _local1_param0_helper1 = _unwrap_result(_local0);
    bumpalo::Bump::allocation_limit(&(_local1_param0_helper1));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 0 {return;}
    test_function29();
});
