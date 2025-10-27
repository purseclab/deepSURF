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

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}


fn test_function29(_param0: &[u8] ,_param1: &[u8]) {
    let _local0 = secp256k1::SecretKey::from_slice(_param0);
    let _local1 = secp256k1::PublicKey::from_slice(_param1);
    let _local2_param0_helper1 = _unwrap_result(_local1);
    let _local2_param1_helper1 = _unwrap_result(_local0);
    let _local2 = secp256k1::ecdh::SharedSecret::new(&(_local2_param0_helper1) ,&(_local2_param1_helper1));
    secp256k1::ecdh::SharedSecret::secret_bytes(&(_local2));
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() < 2 {return;}
    let dynamic_length = (data.len() - 0) / 2;
    let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, 0 + 1 * dynamic_length);
    let _param1 = _to_slice::<u8>(data, 0 + 1 * dynamic_length, data.len());
    test_function29(_param0 ,_param1);
});
