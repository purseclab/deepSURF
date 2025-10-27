#[macro_use]
extern crate afl;
extern crate pnet_packet;
fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}


fn test_function19(_param0 :u32 ,_param1 :u32) {
    let _local0 = pnet_packet::tcp::TcpOption::timestamp(_param0 ,_param1);
    let _ = pnet_packet::tcp::MutableTcpOptionPacket::packet_size(&(_local0));
}

fn main() {
    fuzz!(|data: &[u8]| {
        //actual body emit
        if data.len() != 8 {return;}
        let _param0 = _to_u32(data, 0);
        let _param1 = _to_u32(data, 4);
        test_function19(_param0 ,_param1);
    });
}
