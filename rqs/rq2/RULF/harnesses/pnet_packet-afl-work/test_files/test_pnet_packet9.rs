#[macro_use]
extern crate afl;
extern crate pnet_packet;
fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

use pnet_packet::PrimitiveValues;

fn test_function9(_param0 :u8) {
    let _local0 = pnet_packet::icmp::IcmpCode::new(_param0);
    let _ = pnet_packet::PrimitiveValues::to_primitive_values(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 1 {return;}
        let _param0 = _to_u8(data, 0);
        test_function9(_param0);
    });
}
