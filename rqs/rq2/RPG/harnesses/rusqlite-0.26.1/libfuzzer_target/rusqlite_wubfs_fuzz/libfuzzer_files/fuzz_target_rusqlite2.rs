#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusqlite;
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


fn test_function2(_param0: &str) {
    let _local0 = rusqlite::OpenFlags::empty();
    rusqlite::OpenFlags::is_all(&(_local0));
    rusqlite::Connection::open_in_memory_with_flags_and_vfs(_local0 ,_param0);
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() < 1 {return;}
    let dynamic_length = (data.len() - 0) / 1;
    let _param0 = _to_str(data, 0 + 0 * dynamic_length, data.len());
    test_function2(_param0);
});
