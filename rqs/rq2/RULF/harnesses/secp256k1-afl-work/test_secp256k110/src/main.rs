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

fn _to_slice<T>(data:&[u8], start_index: usize, end_index: usize)->&[T] {
    let data_slice = &data[start_index..end_index];
    let (_, shorts, _) = unsafe {data_slice.align_to::<T>()};
    shorts
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function10(_param0 :&[u8] ,_param1 :&[u8] ,_param2 :usize) {
    let _local0 = secp256k1::Secp256k1::gen_new();
    let _local1 = secp256k1::Message::from_slice(_param0);
    let _local2 = secp256k1::SecretKey::from_slice(_param1);
    let _local3_param1_helper1 = _unwrap_result(_local1);
    let _local3_param2_helper1 = _unwrap_result(_local2);
    let _ = secp256k1::Secp256k1::sign_ecdsa_grind_r(&(_local0) ,&(_local3_param1_helper1) ,&(_local3_param2_helper1) ,_param2);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 10 {return;}
        let dynamic_length = (data.len() - 8) / 2;
        let _param0 = _to_slice::<u8>(data, 8 + 0 * dynamic_length, 8 + 1 * dynamic_length);
        let _param1 = _to_slice::<u8>(data, 8 + 1 * dynamic_length, data.len());
        let _param2 = _to_usize(data, 0);
        test_function10(_param0 ,_param1 ,_param2);
    });
}
