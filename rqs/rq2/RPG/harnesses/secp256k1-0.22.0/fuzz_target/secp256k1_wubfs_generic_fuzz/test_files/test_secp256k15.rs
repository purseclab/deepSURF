#[macro_use]
extern crate afl;
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


fn test_function5(_param0: &[u8]) {
    let _local0 = secp256k1::ecdsa::Signature::from_der(_param0);
    let _local1_param0_helper1 = _unwrap_result(_local0);
    let _local1 = secp256k1::ecdsa::SerializedSignature::from_signature(&(_local1_param0_helper1));
    secp256k1::ecdsa::SerializedSignature::to_signature(&(_local1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_slice::<u8>(data, 0 + 0 * dynamic_length, data.len());
        test_function5(_param0);
    });
}
