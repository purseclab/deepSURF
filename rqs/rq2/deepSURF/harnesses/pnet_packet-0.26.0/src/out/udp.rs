// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Packet;
use ip::IpNextHeaderProtocols;

use pnet_macros_support::types::*;

use std::net::{Ipv4Addr, Ipv6Addr};
use util;






// Set data





 /* source */
 /* destination */
 /* length */
/* checksum */





// Set data





 /* source */
 /* destination */
 /* length */
/* checksum */
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct UdpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableUdpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> UdpPacket<'a> {
    /// Constructs a new UdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<UdpPacket<'p>> {
        if packet.len() >= UdpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(UdpPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new UdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the UdpPacket will
    /// own its own data and the underlying buffer will be dropped when the UdpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<UdpPacket<'static>> {
        if packet.len() >= UdpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(UdpPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a UdpPacket to a UdpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> UdpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        UdpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a UdpPacket to a UdpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> UdpPacket<'a> {
        UdpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Udp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Udp) -> usize { 8 + _packet.payload.len() }
    /// Get the source field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the destination field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 6;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
}
impl <'a> MutableUdpPacket<'a> {
    /// Constructs a new MutableUdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableUdpPacket<'p>> {
        if packet.len() >= MutableUdpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableUdpPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableUdpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableUdpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableUdpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableUdpPacket<'static>> {
        if packet.len() >= MutableUdpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableUdpPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableUdpPacket to a UdpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> UdpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        UdpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableUdpPacket to a UdpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> UdpPacket<'a> {
        UdpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Udp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Udp) -> usize { 8 + _packet.payload.len() }
    /// Populates a UdpPacket using a Udp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Udp) {
        let _self = self;
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
        _self.set_length(packet.length);
        _self.set_checksum(packet.checksum);
        _self.set_payload(&packet.payload);
    }
    /// Get the source field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the destination field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 6;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Set the source field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: u16be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the destination field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the length field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_length(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the checksum field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_checksum(&mut self, val: u16be) {
        let _self = self;
        let co = 6;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 8;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for UdpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableUdpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableUdpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 8;
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableUdpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for UdpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `UdpPacket`s
pub struct UdpIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for UdpIterable<'a> {
    type
    Item
    =
    UdpPacket<'a>;
    fn next(&mut self) -> Option<UdpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = UdpPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for UdpPacket<'p> {
    type
    T
    =
    Udp;
    #[inline]
    fn from_packet(&self) -> Udp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Udp{source: _self.get_source(),
            destination: _self.get_destination(),
            length: _self.get_length(),
            checksum: _self.get_checksum(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableUdpPacket<'p> {
    type
    T
    =
    Udp;
    #[inline]
    fn from_packet(&self) -> Udp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Udp{source: _self.get_source(),
            destination: _self.get_destination(),
            length: _self.get_length(),
            checksum: _self.get_checksum(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::std::fmt::Debug for UdpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "UdpPacket {{ source : {:?}, destination : {:?}, length : {:?}, checksum : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_length (  ) , _self . get_checksum (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableUdpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableUdpPacket {{ source : {:?}, destination : {:?}, length : {:?}, checksum : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_length (  ) , _self . get_checksum (  ))
    }
}
/// Represents a UDP Packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Udp {
    pub source: u16be,
    pub destination: u16be,
    pub length: u16be,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
/// Calculate a checksum for a packet built on IPv4.
pub fn ipv4_checksum(packet: &UdpPacket, source: &Ipv4Addr,
                     destination: &Ipv4Addr) -> u16be {
    ipv4_checksum_adv(packet, &[], source, destination)
}
/// Calculate a checksum for a packet built on IPv4. Advanced version which
/// accepts an extra slice of data that will be included in the checksum
/// as being part of the data portion of the packet.
///
/// If `packet` contains an odd number of bytes the last byte will not be
/// counted as the first byte of a word together with the first byte of
/// `extra_data`.
pub fn ipv4_checksum_adv(packet: &UdpPacket, extra_data: &[u8],
                         source: &Ipv4Addr, destination: &Ipv4Addr) -> u16be {
    util::ipv4_checksum(packet.packet(), 3, extra_data, source, destination,
                        IpNextHeaderProtocols::Udp)
}
#[test]
fn udp_header_ipv4_test() {
    use ip::IpNextHeaderProtocols;
    use ipv4::MutableIpv4Packet;
    let mut packet = [0u8; 20 + 8 + 4];
    let ipv4_source = Ipv4Addr::new(192, 168, 0, 1);
    let ipv4_destination = Ipv4Addr::new(192, 168, 0, 199);
    {
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Udp);
        ip_header.set_source(ipv4_source);
        ip_header.set_destination(ipv4_destination);
    }
    packet[20 + 8] = 't' as u8;
    packet[20 + 8 + 1] = 'e' as u8;
    packet[20 + 8 + 2] = 's' as u8;
    packet[20 + 8 + 3] = 't' as u8;
    {
        let mut udp_header =
            MutableUdpPacket::new(&mut packet[20..]).unwrap();
        udp_header.set_source(12345);
        assert_eq!(udp_header . get_source (  ) , 12345);
        udp_header.set_destination(54321);
        assert_eq!(udp_header . get_destination (  ) , 54321);
        udp_header.set_length(8 + 4);
        assert_eq!(udp_header . get_length (  ) , 8 + 4);
        let checksum =
            ipv4_checksum(&udp_header.to_immutable(), &ipv4_source,
                          &ipv4_destination);
        udp_header.set_checksum(checksum);
        assert_eq!(udp_header . get_checksum (  ) , 0x9178);
    }
    let ref_packet = [48, 57, 212, 49, 0, 12, 145, 120];
    assert_eq!(& ref_packet [ .. ] , & packet [ 20 .. 28 ]);
}
/// Calculate a checksum for a packet built on IPv6.
pub fn ipv6_checksum(packet: &UdpPacket, source: &Ipv6Addr,
                     destination: &Ipv6Addr) -> u16be {
    ipv6_checksum_adv(packet, &[], source, destination)
}
/// Calculate the checksum for a packet built on IPv6. Advanced version which
/// accepts an extra slice of data that will be included in the checksum
/// as being part of the data portion of the packet.
///
/// If `packet` contains an odd number of bytes the last byte will not be
/// counted as the first byte of a word together with the first byte of
/// `extra_data`.
pub fn ipv6_checksum_adv(packet: &UdpPacket, extra_data: &[u8],
                         source: &Ipv6Addr, destination: &Ipv6Addr) -> u16be {
    util::ipv6_checksum(packet.packet(), 3, extra_data, source, destination,
                        IpNextHeaderProtocols::Udp)
}
#[test]
fn udp_header_ipv6_test() {
    use ip::IpNextHeaderProtocols;
    use ipv6::MutableIpv6Packet;
    let mut packet = [0u8; 40 + 8 + 4];
    let ipv6_source = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    let ipv6_destination = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
    {
        let mut ip_header = MutableIpv6Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_header(IpNextHeaderProtocols::Udp);
        ip_header.set_source(ipv6_source);
        ip_header.set_destination(ipv6_destination);
    }
    packet[40 + 8] = 't' as u8;
    packet[40 + 8 + 1] = 'e' as u8;
    packet[40 + 8 + 2] = 's' as u8;
    packet[40 + 8 + 3] = 't' as u8;
    {
        let mut udp_header =
            MutableUdpPacket::new(&mut packet[40..]).unwrap();
        udp_header.set_source(12345);
        assert_eq!(udp_header . get_source (  ) , 12345);
        udp_header.set_destination(54321);
        assert_eq!(udp_header . get_destination (  ) , 54321);
        udp_header.set_length(8 + 4);
        assert_eq!(udp_header . get_length (  ) , 8 + 4);
        let checksum =
            ipv6_checksum(&udp_header.to_immutable(), &ipv6_source,
                          &ipv6_destination);
        udp_header.set_checksum(checksum);
        assert_eq!(udp_header . get_checksum (  ) , 0x1390);
    }
    let ref_packet = [48, 57, 212, 49, 0, 12, 19, 144];
    assert_eq!(& ref_packet [ .. ] , & packet [ 40 .. 48 ]);
}
