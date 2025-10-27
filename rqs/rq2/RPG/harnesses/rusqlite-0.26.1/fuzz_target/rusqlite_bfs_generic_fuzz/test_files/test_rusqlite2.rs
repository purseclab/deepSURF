#[macro_use]
extern crate afl;
extern crate rusqlite;
fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}


fn test_function2(_param0: bool) {
    let mut _local0 = rusqlite::OpenFlags::empty();
    let _local1 = rusqlite::OpenFlags::all();
    rusqlite::OpenFlags::set(&mut (_local0) ,_local1 ,_param0);
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_bool(data, 0);
        test_function2(_param0);
    });
}
