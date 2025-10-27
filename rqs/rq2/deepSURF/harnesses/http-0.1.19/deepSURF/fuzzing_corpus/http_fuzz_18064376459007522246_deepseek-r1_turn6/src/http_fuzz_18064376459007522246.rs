#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 256 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let gdata = global_data.first_half;
        
        let num_ops = _to_u8(gdata, 0) % 16;
        let mut hmap = header::HeaderMap::new();
        let mut req_builder = request::Request::builder();
        let mut res_builder = response::Response::builder();
        let mut uri_builder = uri::Builder::new();
        
        for i in 0..num_ops {
            let op_byte = _to_u8(gdata, 1 + i as usize);
            match op_byte % 10 {
                0 => {
                    let name_start = 20 + i as usize * 6;
                    let name_len = _to_u8(gdata, name_start) as usize % 32;
                    let name_str = _to_str(gdata, name_start + 1, name_start + 1 + name_len);
                    if let Ok(hname) = header::HeaderName::from_bytes(name_str.as_bytes()) {
                        let val_start = name_start + name_len + 1;
                        let val_len = _to_u8(gdata, val_start) as usize % 64;
                        let val_str = _to_str(gdata, val_start + 1, val_start + 1 + val_len);
                        if let Ok(hval) = header::HeaderValue::from_str(val_str) {
                            hmap.insert(hname.clone(), hval.clone());
                            hmap.append(hname, hval);
                        }
                    }
                },
                1 => {
                    let start = 50 + i as usize * 8;
                    let len = _to_u8(gdata, start) as usize % 96;
                    let s = _to_str(gdata, start + 1, start + 1 + len);
                    let _ = <uri::Uri as HttpTryFrom<String>>::try_from(s.to_string());
                    let _ = uri::Authority::from_static(s);
                },
                2 => {
                    let start = 100 + i as usize * 4;
                    let len = _to_u8(gdata, start) as usize % 48;
                    let bytes = &gdata[start..start+len];
                    let _ = header::HeaderValue::from_bytes(bytes);
                    let _ = header::HeaderValue::from_static("value");
                },
                3 => {
                    let uri_start = 150 + i as usize * 10;
                    let uri_len = _to_u8(gdata, uri_start) as usize % 80;
                    let uri_str = _to_str(gdata, uri_start + 1, uri_start + 1 + uri_len);
                    req_builder.uri(uri_str);
                    uri_builder.path_and_query(uri_str);
                },
                4 => {
                    let meth_start = 200 + i as usize;
                    let meth_byte = _to_u8(gdata, meth_start) % 9;
                    let method = match meth_byte {
                        0 => method::Method::GET,
                        1 => method::Method::POST,
                        2 => method::Method::PUT,
                        3 => method::Method::DELETE,
                        4 => method::Method::HEAD,
                        5 => method::Method::OPTIONS,
                        6 => method::Method::PATCH,
                        7 => method::Method::CONNECT,
                        _ => method::Method::TRACE,
                    };
                    req_builder.method(method.clone());
                    res_builder.status(_to_u16(gdata, meth_start) % 600);
                },
                5 => {
                    for (name, value) in hmap.iter() {
                        println!("Header: {:?} => {:?}", name.as_str(), value.to_str());
                    }
                    let _drain = hmap.drain();
                },
                6 => {
                    let val_start = 250 + i as usize * 4;
                    let status_code = _to_u16(gdata, val_start) % 600;
                    let _ = status::StatusCode::from_u16(status_code);
                    res_builder.version(version::Version::HTTP_11);
                },
                7 => {
                    let auth_start = 300 + i as usize * 12;
                    let auth_len = _to_u8(gdata, auth_start) as usize % 64;
                    let auth_str = _to_str(gdata, auth_start + 1, auth_start + 1 + auth_len);
                    let _auth = uri::Authority::from_static(auth_str);
                },
                8 => {
                    let scheme_start = 350 + i as usize;
                    let scheme_byte = _to_u8(gdata, scheme_start) % 3;
                    let scheme = match scheme_byte {
                        0 => uri::Scheme::HTTP,
                        1 => uri::Scheme::HTTPS,
                        _ => _unwrap_result(uri::Scheme::from_str("custom")),
                    };
                    uri_builder.scheme(scheme);
                },
                9 => {
                    if let Ok(uri) = uri_builder.build() {
                        let _parts = uri.clone().into_parts();
                        req_builder.uri(uri);
                    }
                },
                _ => unreachable!()
            }
        }
        
        if let Ok(req) = req_builder.body(()) {
            let _version = req.version();
        }
        if let Ok(res) = res_builder.body(()) {
            let _status = res.status();
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