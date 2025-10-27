#[macro_use]
extern crate afl;
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


fn test_function3() {
    let mut _local0 = bumpalo::Bump::try_new();
    let mut _local1_param0_helper1 = _unwrap_result(_local0);
    bumpalo::Bump::reset(&mut (_local1_param0_helper1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 0 {return;}
        test_function3();
    });
}
