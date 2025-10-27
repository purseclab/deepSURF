#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 512 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let g = global_data.first_half;

        let op_count = _to_u8(g, 0) % 8;
        for i in 0..op_count {
            let selector = _to_u8(g, 1 + i as usize) % 6;
            match selector {
                0 => {
                    let mut uri_builder = http::uri::Uri::builder();
                    let auth = http::uri::Authority::from_str(_to_str(g, 50, 90)).unwrap();
                    uri_builder.authority(auth.as_str());
                    let _uri = uri_builder.build().unwrap();
                    let scheme = http::uri::Scheme::from_str(_to_str(g, 10, 50)).unwrap();
                    let path = http::uri::PathAndQuery::from_static(_to_str(g, 50, 90));
                    println!("{:?} {:?}", scheme, path);
                },
                1 => {
                    let mut headers = http::header::HeaderMap::new();
                    let hname = http::header::HeaderName::from_static(_to_str(g, 100, 120));
                    let hval = http::header::HeaderValue::from_str(_to_str(g, 120, 160)).unwrap();
                    let hval_clone = hval.clone();
                    headers.insert(hname.clone(), hval.clone());
                    headers.append(hname.clone(), hval);
                    let entry = headers.entry(hname.clone()).unwrap();
                    let req = http::Request::builder()
                        .uri(_to_str(g, 160, 200))
                        .method(http::method::Method::PUT)
                        .header(hname, hval_clone)
                        .body(())
                        .unwrap();
                    let map = headers.iter().collect::<Vec<_>>();
                    println!("{:?}", map);
                },
                2 => {
                    let auth_str = _to_str(g, 200, 250);
                    let auth = http::uri::Authority::from_str(auth_str).unwrap();
                    let port = auth.port_u16();
                    let auth_bytes = auth.as_str().as_bytes();
                    let req = http::Request::builder().uri(auth_str).body(()).unwrap();
                    println!("{:?} {:?}", port, req.uri());
                },
                3 => {
                    let auth = http::uri::Authority::from_str(_to_str(g, 300, 340)).unwrap();
                    let mut req_builder = http::Request::builder();
                    req_builder.uri(auth.as_str()).method("POST");
                    let method = http::method::Method::from_bytes(&g[340..345]).unwrap();
                    let res = http::Response::builder().status(200).body(()).unwrap();
                    println!("{:?} {:?}", res.status(), method);
                },
                4 => {
                    let auth1 = http::uri::Authority::from_static(_to_str(g, 350, 400));
                    let auth2 = http::uri::Authority::from_str(_to_str(g, 400, 450)).unwrap();
                    let cmp = auth1.eq(auth2.as_str());
                    let scheme = http::uri::Scheme::from_str("https").unwrap();
                    let uri = http::Uri::builder().authority(auth2.as_str()).build().unwrap();
                    let status = http::StatusCode::from_u16(_to_u16(g, 450)).unwrap();
                    println!("{:?} {:?}", uri, status);
                },
                5 => {
                    let uri = http::Uri::from_str(_to_str(g, 460, 500)).unwrap().into_parts();
                    let auth_part = uri.authority.unwrap();
                    let cmp = auth_part.eq(_to_str(g, 500, 510));
                    let response = http::Response::builder().status(404).body(()).unwrap();
                    let version = response.version();
                    println!("{:?} {:?}", auth_part, version);
                },
                _ => unreachable!()
            }
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