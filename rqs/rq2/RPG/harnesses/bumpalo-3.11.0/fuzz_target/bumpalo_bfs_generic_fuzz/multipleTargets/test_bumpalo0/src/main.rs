#[macro_use]
extern crate afl;
extern crate bumpalo;
fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            use std::process;
            process::exit(0);
        }
    }
}


fn test_function0(_param0: &str) {
    let _local0 = bumpalo::Bump::new();
    let _local1 = bumpalo::Bump::alloc(&(_local0) ,_param0.to_string());
    bumpalo::Bump::try_alloc(&(_local0) ,*(_local1));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() < 1 {return;}
        let dynamic_length = (data.len() - 0) / 1;
        let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
        test_function0(_param0);
    });
}
