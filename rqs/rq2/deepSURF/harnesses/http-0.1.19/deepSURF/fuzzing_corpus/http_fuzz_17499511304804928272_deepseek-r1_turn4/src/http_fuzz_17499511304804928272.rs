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
        let GLOBAL_DATA = global_data.first_half;
        let num_ops = _to_u8(GLOBAL_DATA, 0) % 8;
        let mut offset = 1;

        let mut map = header::HeaderMap::new();
        let mut names = Vec::new();
        let mut response_builder = response::Response::builder();
        let mut status_code = status::StatusCode::from_u16(200).unwrap();

        for _ in 0..num_ops {
            if offset + 2 >= GLOBAL_DATA.len() { break; }
            let op = _to_u8(GLOBAL_DATA, offset) % 8;
            offset += 1;

            match op {
                0 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    let start = offset;
                    offset += len as usize;
                    if start >= GLOBAL_DATA.len() || offset >= GLOBAL_DATA.len() { continue; }
                    let slice = &GLOBAL_DATA[start..offset];
                    if let Ok(name) = header::HeaderName::from_bytes(slice) {
                        let value = header::HeaderValue::from_bytes(slice).unwrap_or(header::HeaderValue::from_static(""));
                        map.append(&name, value.clone());
                        println!("{:?} {:?}", name, value);
                    }
                }
                1 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 32;
                    offset += 1;
                    let start = offset;
                    offset += len as usize;
                    if start >= GLOBAL_DATA.len() || offset >= GLOBAL_DATA.len() { continue; }
                    if let Ok(s) = std::str::from_utf8(&GLOBAL_DATA[start..offset]) {
                        if let Ok(name) = header::HeaderName::from_str(s) {
                            names.push(name);
                        }
                    }
                }
                2 => {
                    let capacity = _to_usize(GLOBAL_DATA, offset) % 8;
                    offset += 8;
                    let mut builder = uri::Builder::new();
                    for _ in 0..capacity {
                        let len = _to_u8(GLOBAL_DATA, offset) % 16;
                        offset += 1;
                        let start = offset;
                        offset += len as usize;
                        if start >= GLOBAL_DATA.len() || offset >= GLOBAL_DATA.len() { break; }
                        if let Ok(s) = std::str::from_utf8(&GLOBAL_DATA[start..offset]) {
                            let _ = builder.path_and_query(s);
                        }
                    }
                    if let Ok(uri) = builder.build() {
                        let _ = uri.path().eq(uri.query().unwrap_or(""));
                    }
                }
                3 => {
                    let len = _to_u8(GLOBAL_DATA, offset) % 16;
                    offset += 1;
                    let start = offset;
                    offset += len as usize;
                    if start >= GLOBAL_DATA.len() || offset >= GLOBAL_DATA.len() { continue; }
                    if let Ok(auth) = <uri::Authority as HttpTryFrom<&[u8]>>::try_from(&GLOBAL_DATA[start..offset]) {
                        let port = auth.port_u16().unwrap_or(0);
                        let _ = port.cmp(&80);
                        let _ = uri::Uri::builder().authority(auth).build();
                    }
                }
                4 => {
                    if let Some((key, value)) = map.iter().next() {
                        let _ = key.eq("content-length");
                        println!("{:?}", value);
                        let _ = response_builder.header(key, value);
                    }
                }
                5 => {
                    if names.len() >= 2 {
                        let a = &names[_to_usize(GLOBAL_DATA, offset) % names.len()];
                        let b = &names[_to_usize(GLOBAL_DATA, offset + 8) % names.len()];
                        let _ = a.eq(b);
                        let _ = a.eq("host");
                    }
                }
                6 => {
                    let code = _to_u16(GLOBAL_DATA, offset) % 600;
                    offset += 2;
                    if let Ok(sc) = status::StatusCode::from_u16(code) {
                        status_code = sc;
                        let _ = response_builder.status(sc);
                    }
                }
                7 => {
                    let scheme_bytes = &GLOBAL_DATA[offset..offset+4];
                    offset += 4;
                    if let Ok(scheme) = <uri::Scheme as HttpTryFrom<&[u8]>>::try_from(scheme_bytes) {
                        let _ = uri::Uri::builder().scheme(scheme).build();
                    }
                }
                _ => {}
            }
        }

        if names.len() >= 1 && !map.is_empty() {
            let req = request::Request::builder()
                .method(method::Method::GET)
                .uri(uri::Uri::from_static("http://example.com"))
                .body(())
                .unwrap();
            let _ = req.version();

            let res = response_builder
                .version(version::Version::HTTP_11)
                .body(());
            
            if let Ok(res) = res {
                let _ = res.status();
                let _ = res.version();
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