#[macro_use]
extern crate afl;
extern crate pnet_packet;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

use pnet_packet::PrimitiveValues;

fn test_function11(_param0 :u16) {
    let _local0 = pnet_packet::icmp::echo_request::SequenceNumber::new(_param0);
    let _ = pnet_packet::PrimitiveValues::to_primitive_values(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 2 {return;}
        let _param0 = _to_u16(data, 0);
        test_function11(_param0);
    });
}
