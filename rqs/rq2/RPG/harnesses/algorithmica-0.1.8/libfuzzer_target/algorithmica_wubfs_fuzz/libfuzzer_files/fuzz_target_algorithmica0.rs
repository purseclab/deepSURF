#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate algorithmica;
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

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}


fn test_function0(_param0: i32 ,_param1: i32) {
    let _local0 = algorithmica::tree::Node::create(_param0);
    algorithmica::tree::Node::add_new(Some(_local0) ,_param1);
    algorithmica::tree::bst::BST::new();
}

fuzz_target!(|data: &[u8]| {
    //actual body emit
    if data.len() != 8 {return;}
    let _param0 = _to_i32(data, 0);
    let _param1 = _to_i32(data, 4);
    test_function0(_param0 ,_param1);
});
