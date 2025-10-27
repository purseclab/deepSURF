#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use http::*;
use http::header::{HeaderName, HeaderValue};
use http::uri::{Authority, PathAndQuery};
use global_data::*;
use std::str::FromStr;

fn main() {
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 1024 { return; }
        set_global_data(data);
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let mut index = 0;

        let constructor_type = _to_u8(GLOBAL_DATA, index) % 4;
        index += 1;

        let uri = match constructor_type {
            0 => {
                let len = _to_u8(GLOBAL_DATA, index) as usize % 50;
                index += 1;
                let s = _to_str(GLOBAL_DATA, index, index + len);
                index += len;
                Uri::from_static(s)
            }
            1 => {
                let temp_uri = Uri::from_static("http://example.com");
                let parts = temp_uri.into_parts();
                let new_uri = Uri::from_parts(parts).unwrap();
                new_uri
            }
            2 => {
                let len = _to_u8(GLOBAL_DATA, index) as usize % 50;
                index += 1;
                let bytes = &GLOBAL_DATA[index..index + len];
                index += len;
                _unwrap_result(Uri::from_shared(bytes.into()))
            }
            3 => {
                let authority_len = _to_u8(GLOBAL_DATA, index) as usize % 50;
                index += 1;
                let authority_bytes = &GLOBAL_DATA[index..index + authority_len];
                index += authority_len;
                let authority = _unwrap_result(Authority::from_shared(authority_bytes.into()));
                Uri::builder()
                    .authority(authority)
                    .path_and_query("/")
                    .build()
                    .unwrap()
            }
            _ => unreachable!(),
        };

        let path_len = _to_u8(GLOBAL_DATA, index) as usize % 50;
        index += 1;
        let path_bytes = &GLOBAL_DATA[index..index + path_len];
        index += path_len;
        let path = _unwrap_result(PathAndQuery::from_shared(path_bytes.into()));

        let n_headers = _to_u8(GLOBAL_DATA, index) % 65;
        index += 1;

        let mut headers = http::header::HeaderMap::new();
        for _ in 0..n_headers {
            let method_selector = _to_u8(GLOBAL_DATA, index) % 3;
            index += 1;

            let (name, value) = match method_selector {
                0 => {
                    let name_len = _to_u8(GLOBAL_DATA, index) as usize % 20;
                    index += 1;
                    let name_str = _to_str(GLOBAL_DATA, index, index + name_len);
                    index += name_len;

                    let value_len = _to_u8(GLOBAL_DATA, index) as usize % 20;
                    index += 1;
                    let value_str = _to_str(GLOBAL_DATA, index, index + value_len);
                    index += value_len;

                    (
                        _unwrap_result(HeaderName::from_str(name_str)),
                        _unwrap_result(HeaderValue::from_str(value_str))
                    )
                }
                1 => {
                    let name_len = _to_u8(GLOBAL_DATA, index) as usize % 20;
                    index += 1;
                    let name_bytes = &GLOBAL_DATA[index..index + name_len];
                    index += name_len;

                    let value_len = _to_u8(GLOBAL_DATA, index) as usize % 20;
                    index += 1;
                    let value_bytes = &GLOBAL_DATA[index..index + value_len];
                    index += value_len;

                    (
                        _unwrap_result(HeaderName::from_bytes(name_bytes)),
                        _unwrap_result(HeaderValue::from_bytes(value_bytes))
                    )
                }
                2 => {
                    let value_static = if _to_u8(GLOBAL_DATA, index) % 2 == 0 {
                        "value1"
                    } else {
                        "value2"
                    };
                    index += 1;

                    (
                        HeaderName::from_static("x-custom-header"),
                        HeaderValue::from_static(value_static)
                    )
                }
                _ => unreachable!(),
            };

            headers.append(&name, value);
        }

        let req = _unwrap_result(Request::builder()
            .method(_unwrap_result(Method::from_str(_to_str(GLOBAL_DATA, index, index + 4))))
            .uri(&uri)
            .version(Version::HTTP_11)
            .header("host", _unwrap_result(Authority::from_shared(GLOBAL_DATA.into())).as_str())
            .body(()));

        let res = _unwrap_result(Response::builder()
            .status(_unwrap_result(StatusCode::from_u16(_to_u16(GLOBAL_DATA, index) % 500)))
            .version(Version::HTTP_2)
            .header("server", HeaderValue::from_static("fuzz-server"))
            .body(()));

        println!("URI path: {:?}", uri.path());
        println!("Request path: {:?}", req.uri().path());
        println!("Response status: {:?}", res.status().as_str());
        println!("Header map: {:?}", headers);
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