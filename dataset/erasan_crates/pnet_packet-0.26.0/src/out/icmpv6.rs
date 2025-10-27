// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ip::IpNextHeaderProtocols;
use PrimitiveValues;
use pnet_macros_support::types::*;
use std::net::Ipv6Addr;

/// Represents the "ICMPv6 type" header field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Icmpv6Type(pub u8);

impl Icmpv6Type {
    /// Create a new `Icmpv6Type` instance.
    pub fn new(val: u8) -> Icmpv6Type { Icmpv6Type(val) }
}

impl PrimitiveValues for Icmpv6Type {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}

/// Represents the "ICMPv6 code" header field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Icmpv6Code(pub u8);

impl Icmpv6Code {
    /// Create a new `Icmpv6Code` instance.
    pub fn new(val: u8) -> Icmpv6Code { Icmpv6Code(val) }
}

impl PrimitiveValues for Icmpv6Code {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}





// The equivalent of your typical ping -6 ::1%lo
 // Icmpv6 Type
 // Code
 // Checksum
 // Id
 // Sequence
 // 56 bytes of "random" data


// Check


// Neighbor Discovery Protocol [RFC4861]




























// Extra bytes to confuse the parsing

 // Type
 // Code
 // Checksum
 // Reserved





 // Type
 // Code
 // Checksum
 // Hop Limit
 // Flags
 // Lifetime
 // Reachable
 // Retrans
 // Source Link-Layer
 // MTU









#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct Icmpv6Packet<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableIcmpv6Packet<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> Icmpv6Packet<'a> {
    /// Constructs a new Icmpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<Icmpv6Packet<'p>> {
        if packet.len() >= Icmpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Icmpv6Packet{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new Icmpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the Icmpv6Packet will
    /// own its own data and the underlying buffer will be dropped when the Icmpv6Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<Icmpv6Packet<'static>> {
        if packet.len() >= Icmpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Icmpv6Packet{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a Icmpv6Packet to a Icmpv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Icmpv6Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Icmpv6Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a Icmpv6Packet to a Icmpv6Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Icmpv6Packet<'a> {
        Icmpv6Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Icmpv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Icmpv6) -> usize {
        4 + _packet.payload.len()
    }
    /// Get the value of the icmpv6_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmpv6_type(&self) -> Icmpv6Type {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Icmpv6Packet) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        Icmpv6Type::new(get_arg0(&self))
    }
    /// Get the value of the icmpv6_code field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmpv6_code(&self) -> Icmpv6Code {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Icmpv6Packet) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        Icmpv6Code::new(get_arg0(&self))
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
}
impl <'a> MutableIcmpv6Packet<'a> {
    /// Constructs a new MutableIcmpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableIcmpv6Packet<'p>> {
        if packet.len() >= MutableIcmpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIcmpv6Packet{packet:
                                         MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableIcmpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableIcmpv6Packet will
    /// own its own data and the underlying buffer will be dropped when the MutableIcmpv6Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableIcmpv6Packet<'static>> {
        if packet.len() >= MutableIcmpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIcmpv6Packet{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableIcmpv6Packet to a Icmpv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Icmpv6Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Icmpv6Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableIcmpv6Packet to a Icmpv6Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Icmpv6Packet<'a> {
        Icmpv6Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Icmpv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Icmpv6) -> usize {
        4 + _packet.payload.len()
    }
    /// Populates a Icmpv6Packet using a Icmpv6 structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Icmpv6) {
        let _self = self;
        _self.set_icmpv6_type(packet.icmpv6_type);
        _self.set_icmpv6_code(packet.icmpv6_code);
        _self.set_checksum(packet.checksum);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the icmpv6_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmpv6_type(&self) -> Icmpv6Type {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIcmpv6Packet) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        Icmpv6Type::new(get_arg0(&self))
    }
    /// Get the value of the icmpv6_code field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmpv6_code(&self) -> Icmpv6Code {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIcmpv6Packet) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        Icmpv6Code::new(get_arg0(&self))
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Set the value of the icmpv6_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIcmpv6Packet, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the icmpv6_code field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIcmpv6Packet, val: u8) {
            let co = 1;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the checksum field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_checksum(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
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
        let current_offset = 4;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for Icmpv6Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableIcmpv6Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableIcmpv6Packet<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableIcmpv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for Icmpv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `Icmpv6Packet`s
pub struct Icmpv6Iterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for Icmpv6Iterable<'a> {
    type
    Item
    =
    Icmpv6Packet<'a>;
    fn next(&mut self) -> Option<Icmpv6Packet<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = Icmpv6Packet::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for Icmpv6Packet<'p> {
    type
    T
    =
    Icmpv6;
    #[inline]
    fn from_packet(&self) -> Icmpv6 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Icmpv6{icmpv6_type: _self.get_icmpv6_type(),
               icmpv6_code: _self.get_icmpv6_code(),
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
impl <'p> ::pnet_macros_support::packet::FromPacket for
 MutableIcmpv6Packet<'p> {
    type
    T
    =
    Icmpv6;
    #[inline]
    fn from_packet(&self) -> Icmpv6 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Icmpv6{icmpv6_type: _self.get_icmpv6_type(),
               icmpv6_code: _self.get_icmpv6_code(),
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
impl <'p> ::std::fmt::Debug for Icmpv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "Icmpv6Packet {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?},  }}"
               , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (  ) ,
               _self . get_checksum (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIcmpv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIcmpv6Packet {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?},  }}"
               , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (  ) ,
               _self . get_checksum (  ))
    }
}
/// Represents a generic ICMPv6 packet [RFC 4443 § 2.1]
///
/// ```text
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |     Type      |     Code      |          Checksum             |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                                                               |
/// +                         Message Body                          +
/// |                                                               |
/// ```
///
/// [RFC 4443 § 2.1]: https://tools.ietf.org/html/rfc4443#section-2.1
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Icmpv6 {
    pub icmpv6_type: Icmpv6Type,
    pub icmpv6_code: Icmpv6Code,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
/// Calculates a checksum of an ICMPv6 packet.
pub fn checksum(packet: &Icmpv6Packet, source: &Ipv6Addr,
                destination: &Ipv6Addr) -> u16be {
    use Packet;
    use util;
    util::ipv6_checksum(packet.packet(), 1, &[], source, destination,
                        IpNextHeaderProtocols::Icmpv6)
}
#[cfg(test)]
mod checksum_tests {
    use super::*;
    #[test]
    fn checksum_echo_request() {
        let lo = &Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);
        let mut data =
            vec!(0x80 , 0x00 , 0xff , 0xff , 0x00 , 0x00 , 0x00 , 0x01 , 0x20
                 , 0x20 , 0x75 , 0x73 , 0x74 , 0x20 , 0x61 , 0x20 , 0x66 ,
                 0x6c , 0x65 , 0x73 , 0x68 , 0x20 , 0x77 , 0x6f , 0x75 , 0x6e
                 , 0x64 , 0x20 , 0x20 , 0x74 , 0x69 , 0x73 , 0x20 , 0x62 ,
                 0x75 , 0x74 , 0x20 , 0x61 , 0x20 , 0x73 , 0x63 , 0x72 , 0x61
                 , 0x74 , 0x63 , 0x68 , 0x20 , 0x20 , 0x6b , 0x6e , 0x69 ,
                 0x67 , 0x68 , 0x74 , 0x73 , 0x20 , 0x6f , 0x66 , 0x20 , 0x6e
                 , 0x69 , 0x20 , 0x20 , 0x20);
        let mut pkg = MutableIcmpv6Packet::new(&mut data[..]).unwrap();
        assert_eq!(checksum ( & pkg . to_immutable (  ) , lo , lo ) , 0x1d2e);
        pkg.set_icmpv6_type(Icmpv6Type(129));
        assert_eq!(checksum ( & pkg . to_immutable (  ) , lo , lo ) , 0x1c2e);
    }
}
/// The enumeration of the recognized ICMPv6 types.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Icmpv6Types {
    use icmpv6::Icmpv6Type;
    /// ICMPv6 type for "destination unreachable".
    pub const DestinationUnreachable: Icmpv6Type = Icmpv6Type(1);
    /// ICMPv6 type for "packet too big".
    pub const PacketTooBig: Icmpv6Type = Icmpv6Type(2);
    /// ICMPv6 type for "time exceeded".
    pub const TimeExceeded: Icmpv6Type = Icmpv6Type(3);
    /// ICMPv6 type for "parameter problem".
    pub const ParameterProblem: Icmpv6Type = Icmpv6Type(4);
    /// ICMPv6 type for "echo request".
    pub const EchoRequest: Icmpv6Type = Icmpv6Type(128);
    /// ICMPv6 type for "echo reply".
    pub const EchoReply: Icmpv6Type = Icmpv6Type(129);
    /// ICMPv6 type for "router solicitation".
    pub const RouterSolicit: Icmpv6Type = Icmpv6Type(133);
    /// ICMPv6 type for "router advertisement".
    pub const RouterAdvert: Icmpv6Type = Icmpv6Type(134);
    /// ICMPv6 type for "neighbor solicitation".
    pub const NeighborSolicit: Icmpv6Type = Icmpv6Type(135);
    /// ICMPv6 type for "neighbor advertisement".
    pub const NeighborAdvert: Icmpv6Type = Icmpv6Type(136);
    /// ICMPv6 type for "redirect".
    pub const Redirect: Icmpv6Type = Icmpv6Type(137);
}
pub mod ndp {
    //! Abstractions for the Neighbor Discovery Protocol [RFC 4861]
    //!
    //! [RFC 4861]: https://tools.ietf.org/html/rfc4861
    use icmpv6::{Icmpv6Code, Icmpv6Type};
    use PrimitiveValues;
    use Packet;
    use pnet_macros_support::types::*;
    use std::net::Ipv6Addr;
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod Icmpv6Codes {
        use icmpv6::Icmpv6Code;
        /// 0 is the only available ICMPv6 Code for the NDP.
        pub const NoCode: Icmpv6Code = Icmpv6Code(0);
    }
    /// Represents a Neighbor Discovery Option Type.
    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
    pub struct NdpOptionType(pub u8);
    impl NdpOptionType {
        /// Create a new `NdpOptionType` instance.
        pub fn new(value: u8) -> NdpOptionType { NdpOptionType(value) }
    }
    impl PrimitiveValues for NdpOptionType {
        type
        T
        =
        (u8,);
        fn to_primitive_values(&self) -> (u8,) { (self.0,) }
    }
    /// Neighbor Discovery Option Types [RFC 4861 § 4.6]
    ///
    /// [RFC 4861 § 4.6]: https://tools.ietf.org/html/rfc4861#section-4.6
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod NdpOptionTypes {
        use super::NdpOptionType;
        /// Source Link-Layer Address Option [RFC 4861 § 4.6.1]
        ///
        /// ```text
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |     Type      |    Length     |    Link-Layer Address ...
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// ```
        ///
        /// [RFC 4861 § 4.6.1]: https://tools.ietf.org/html/rfc4861#section-4.6.1
        pub const SourceLLAddr: NdpOptionType = NdpOptionType(1);
        /// Target Link-Layer Address Option [RFC 4861 § 4.6.1]
        ///
        /// ```text
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |     Type      |    Length     |    Link-Layer Address ...
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// ```
        ///
        /// [RFC 4861 § 4.6.1]: https://tools.ietf.org/html/rfc4861#section-4.6.1
        pub const TargetLLAddr: NdpOptionType = NdpOptionType(2);
        /// Prefix Information Option [RFC 4861 § 4.6.2]
        ///
        /// ```text
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |     Type      |    Length     | Prefix Length |L|A| Reserved1 |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                         Valid Lifetime                        |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                       Preferred Lifetime                      |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                           Reserved2                           |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                                                               |
        /// +                                                               +
        /// |                                                               |
        /// +                            Prefix                             +
        /// |                                                               |
        /// +                                                               +
        /// |                                                               |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// ```
        ///
        /// [RFC 4861 § 4.6.2]: https://tools.ietf.org/html/rfc4861#section-4.6.2
        pub const PrefixInformation: NdpOptionType = NdpOptionType(3);
        /// Redirected Header Option [RFC 4861 § 4.6.3]
        ///
        /// ```text
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |     Type      |    Length     |            Reserved           |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                           Reserved                            |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                                                               |
        /// ~                       IP header + data                        ~
        /// |                                                               |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// ```
        ///
        /// [RFC 4861 § 4.6.3]: https://tools.ietf.org/html/rfc4861#section-4.6.3
        pub const RedirectedHeader: NdpOptionType = NdpOptionType(4);
        /// MTU Option [RFC 4861 § 4.6.4]
        ///
        /// ```text
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |     Type      |    Length     |           Reserved            |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// |                              MTU                              |
        /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
        /// ```
        ///
        /// [RFC 4861 § 4.6.4]: https://tools.ietf.org/html/rfc4861#section-4.6.4
        pub const MTU: NdpOptionType = NdpOptionType(5);
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct NdpOptionPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableNdpOptionPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> NdpOptionPacket<'a> {
        /// Constructs a new NdpOptionPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<NdpOptionPacket<'p>> {
            if packet.len() >= NdpOptionPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NdpOptionPacket{packet: PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new NdpOptionPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the NdpOptionPacket will
        /// own its own data and the underlying buffer will be dropped when the NdpOptionPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<NdpOptionPacket<'static>> {
            if packet.len() >= NdpOptionPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NdpOptionPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a NdpOptionPacket to a NdpOptionPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NdpOptionPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NdpOptionPacket{packet:
                                PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a NdpOptionPacket to a NdpOptionPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NdpOptionPacket<'a> {
            NdpOptionPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 2 }
        /// The size (in bytes) of a NdpOption instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NdpOption) -> usize {
            2 + _packet.data.len()
        }
        /// Get the value of the option_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_option_type(&self) -> NdpOptionType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NdpOptionPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            NdpOptionType::new(get_arg0(&self))
        }
        /// Get the length field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_length(&self) -> u8 {
            let _self = self;
            let co = 1;
            (_self.packet[co] as u8)
        }
    }
    impl <'a> MutableNdpOptionPacket<'a> {
        /// Constructs a new MutableNdpOptionPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableNdpOptionPacket<'p>> {
            if packet.len() >= MutableNdpOptionPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNdpOptionPacket{packet:
                                                MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableNdpOptionPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableNdpOptionPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableNdpOptionPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableNdpOptionPacket<'static>> {
            if packet.len() >= MutableNdpOptionPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNdpOptionPacket{packet:
                                                MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableNdpOptionPacket to a NdpOptionPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NdpOptionPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NdpOptionPacket{packet:
                                PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableNdpOptionPacket to a NdpOptionPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NdpOptionPacket<'a> {
            NdpOptionPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 2 }
        /// The size (in bytes) of a NdpOption instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NdpOption) -> usize {
            2 + _packet.data.len()
        }
        /// Populates a NdpOptionPacket using a NdpOption structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &NdpOption) {
            let _self = self;
            _self.set_option_type(packet.option_type);
            _self.set_length(packet.length);
            _self.set_data(&packet.data);
        }
        /// Get the value of the option_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_option_type(&self) -> NdpOptionType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNdpOptionPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            NdpOptionType::new(get_arg0(&self))
        }
        /// Get the length field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_length(&self) -> u8 {
            let _self = self;
            let co = 1;
            (_self.packet[co] as u8)
        }
        /// Set the value of the option_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_option_type(&mut self, val: NdpOptionType) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNdpOptionPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the length field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_length(&mut self, val: u8) {
            let _self = self;
            let co = 1;
            _self.packet[co + 0] = (val) as u8;
        }
        /// Set the value of the data field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_data(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset = 2;
            let len = ndp_option_payload_length(&_self.to_immutable());
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     NdpOptionPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            2 + ndp_option_payload_length(&_self.to_immutable())
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableNdpOptionPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            2 + ndp_option_payload_length(&_self.to_immutable())
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableNdpOptionPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 2;
            let end =
                ::std::cmp::min(2 +
                                    ndp_option_payload_length(&_self.to_immutable()),
                                _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableNdpOptionPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 2;
            let end =
                ::std::cmp::min(2 +
                                    ndp_option_payload_length(&_self.to_immutable()),
                                _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for NdpOptionPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 2;
            let end =
                ::std::cmp::min(2 +
                                    ndp_option_payload_length(&_self.to_immutable()),
                                _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `NdpOptionPacket`s
    pub struct NdpOptionIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for NdpOptionIterable<'a> {
        type
        Item
        =
        NdpOptionPacket<'a>;
        fn next(&mut self) -> Option<NdpOptionPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = NdpOptionPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     NdpOptionPacket<'p> {
        type
        T
        =
        NdpOption;
        #[inline]
        fn from_packet(&self) -> NdpOption {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NdpOption{option_type: _self.get_option_type(),
                      length: _self.get_length(),
                      data:
                          {
                              let payload = self.payload();
                              let mut vec = Vec::with_capacity(payload.len());
                              vec.extend_from_slice(payload);
                              vec
                          },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableNdpOptionPacket<'p> {
        type
        T
        =
        NdpOption;
        #[inline]
        fn from_packet(&self) -> NdpOption {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NdpOption{option_type: _self.get_option_type(),
                      length: _self.get_length(),
                      data:
                          {
                              let payload = self.payload();
                              let mut vec = Vec::with_capacity(payload.len());
                              vec.extend_from_slice(payload);
                              vec
                          },}
        }
    }
    impl <'p> ::std::fmt::Debug for NdpOptionPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "NdpOptionPacket {{ option_type : {:?}, length : {:?},  }}"
                   , _self . get_option_type (  ) , _self . get_length (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableNdpOptionPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableNdpOptionPacket {{ option_type : {:?}, length : {:?},  }}"
                   , _self . get_option_type (  ) , _self . get_length (  ))
        }
    }
    /// Neighbor Discovery Option [RFC 4861 § 4.6]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |    Length     |              ...              |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// ~                              ...                              ~
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// ```
    ///
    /// [RFC 4861 § 4.6]: https://tools.ietf.org/html/rfc4861#section-4.6
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct NdpOption {
        pub option_type: NdpOptionType,
        pub length: u8,
        pub data: Vec<u8>,
    }
    /// Calculate a length of a `NdpOption`'s payload.
    fn ndp_option_payload_length(option: &NdpOptionPacket) -> usize {
        let len = option.get_length();
        if len > 0 { ((len * 8) - 2) as usize } else { 0 }
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct RouterSolicitPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableRouterSolicitPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> RouterSolicitPacket<'a> {
        /// Constructs a new RouterSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<RouterSolicitPacket<'p>> {
            if packet.len() >= RouterSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RouterSolicitPacket{packet:
                                             PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new RouterSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the RouterSolicitPacket will
        /// own its own data and the underlying buffer will be dropped when the RouterSolicitPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<RouterSolicitPacket<'static>> {
            if packet.len() >= RouterSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RouterSolicitPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a RouterSolicitPacket to a RouterSolicitPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RouterSolicitPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RouterSolicitPacket{packet:
                                    PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a RouterSolicitPacket to a RouterSolicitPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RouterSolicitPacket<'a> {
            RouterSolicitPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a RouterSolicit instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &RouterSolicit) -> usize {
            8 + _packet.options.len() + _packet.payload.len()
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RouterSolicitPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RouterSolicitPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
    }
    impl <'a> MutableRouterSolicitPacket<'a> {
        /// Constructs a new MutableRouterSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableRouterSolicitPacket<'p>> {
            if packet.len() >=
                   MutableRouterSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRouterSolicitPacket{packet:
                                                    MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableRouterSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableRouterSolicitPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableRouterSolicitPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableRouterSolicitPacket<'static>> {
            if packet.len() >=
                   MutableRouterSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRouterSolicitPacket{packet:
                                                    MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableRouterSolicitPacket to a RouterSolicitPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RouterSolicitPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RouterSolicitPacket{packet:
                                    PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableRouterSolicitPacket to a RouterSolicitPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RouterSolicitPacket<'a> {
            RouterSolicitPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a RouterSolicit instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &RouterSolicit) -> usize {
            8 + _packet.options.len() + _packet.payload.len()
        }
        /// Populates a RouterSolicitPacket using a RouterSolicit structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &RouterSolicit) {
            let _self = self;
            _self.set_icmpv6_type(packet.icmpv6_type);
            _self.set_icmpv6_code(packet.icmpv6_code);
            _self.set_checksum(packet.checksum);
            _self.set_reserved(packet.reserved);
            _self.set_options(&packet.options);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRouterSolicitPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRouterSolicitPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
        /// Set the value of the icmpv6_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRouterSolicitPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmpv6_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRouterSolicitPacket, val: u8) {
                let co = 1;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the checksum field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_checksum(&mut self, val: u16be) {
            let _self = self;
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the reserved field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_reserved(&mut self, val: u32be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
        }
        /// Get the raw &mut [u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 8;
            let end =
                min(current_offset +
                        rs_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &mut _self.packet[current_offset..end]
        }
        /// Set the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_options(&mut self, vals: &[NdpOption]) {
            use pnet_macros_support::packet::PacketSize;
            let _self = self;
            let mut current_offset = 8;
            let end =
                current_offset + rs_ndp_options_length(&_self.to_immutable());
            for val in vals.into_iter() {
                let mut packet =
                    MutableNdpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
                packet.populate(val);
                current_offset += packet.packet_size();
                assert!(current_offset <= end);
            }
        }
        /// Set the value of the payload field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_payload(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset =
                8 + rs_ndp_options_length(&_self.to_immutable());
            let len = 0;
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     RouterSolicitPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            8 + rs_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableRouterSolicitPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            8 + rs_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableRouterSolicitPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 8 + rs_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(8 +
                                    rs_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableRouterSolicitPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 8 + rs_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(8 +
                                    rs_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     RouterSolicitPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 8 + rs_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(8 +
                                    rs_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `RouterSolicitPacket`s
    pub struct RouterSolicitIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for RouterSolicitIterable<'a> {
        type
        Item
        =
        RouterSolicitPacket<'a>;
        fn next(&mut self) -> Option<RouterSolicitPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = RouterSolicitPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     RouterSolicitPacket<'p> {
        type
        T
        =
        RouterSolicit;
        #[inline]
        fn from_packet(&self) -> RouterSolicit {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            RouterSolicit{icmpv6_type: _self.get_icmpv6_type(),
                          icmpv6_code: _self.get_icmpv6_code(),
                          checksum: _self.get_checksum(),
                          reserved: _self.get_reserved(),
                          options: _self.get_options(),
                          payload:
                              {
                                  let payload = self.payload();
                                  let mut vec =
                                      Vec::with_capacity(payload.len());
                                  vec.extend_from_slice(payload);
                                  vec
                              },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableRouterSolicitPacket<'p> {
        type
        T
        =
        RouterSolicit;
        #[inline]
        fn from_packet(&self) -> RouterSolicit {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            RouterSolicit{icmpv6_type: _self.get_icmpv6_type(),
                          icmpv6_code: _self.get_icmpv6_code(),
                          checksum: _self.get_checksum(),
                          reserved: _self.get_reserved(),
                          options: _self.get_options(),
                          payload:
                              {
                                  let payload = self.payload();
                                  let mut vec =
                                      Vec::with_capacity(payload.len());
                                  vec.extend_from_slice(payload);
                                  vec
                              },}
        }
    }
    impl <'p> ::std::fmt::Debug for RouterSolicitPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "RouterSolicitPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_options (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableRouterSolicitPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableRouterSolicitPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_options (  ))
        }
    }
    /// Router Solicitation Message [RFC 4861 § 4.1]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |     Code      |          Checksum             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                            Reserved                           |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |   Options ...
    /// ```
    ///
    /// [RFC 4861 § 4.1]: https://tools.ietf.org/html/rfc4861#section-4.1
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct RouterSolicit {
        pub icmpv6_type: Icmpv6Type,
        pub icmpv6_code: Icmpv6Code,
        pub checksum: u16be,
        pub reserved: u32be,
        pub options: Vec<NdpOption>,
        pub payload: Vec<u8>,
    }
    /// Router Solicit packet calculation for the length of the options.
    fn rs_ndp_options_length(pkt: &RouterSolicitPacket) -> usize {
        if pkt.packet().len() > 8 { pkt.packet().len() - 8 } else { 0 }
    }
    /// The enumeration of recognized Router Advert flags.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod RouterAdvertFlags {
        /// "Managed Address Configuration" flag. This is set when
        /// addresses are available via DHCPv6.
        pub const ManagedAddressConf: u8 = 128;
        /// "Other Configuration" flag. This is set when other
        /// configuration information is available via DHCPv6.
        pub const OtherConf: u8 = 64;
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct RouterAdvertPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableRouterAdvertPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> RouterAdvertPacket<'a> {
        /// Constructs a new RouterAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<RouterAdvertPacket<'p>> {
            if packet.len() >= RouterAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RouterAdvertPacket{packet:
                                            PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new RouterAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the RouterAdvertPacket will
        /// own its own data and the underlying buffer will be dropped when the RouterAdvertPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<RouterAdvertPacket<'static>> {
            if packet.len() >= RouterAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RouterAdvertPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a RouterAdvertPacket to a RouterAdvertPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RouterAdvertPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RouterAdvertPacket{packet:
                                   PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a RouterAdvertPacket to a RouterAdvertPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RouterAdvertPacket<'a> {
            RouterAdvertPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 16 }
        /// The size (in bytes) of a RouterAdvert instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &RouterAdvert) -> usize {
            16 + _packet.options.len() + _packet.payload.len()
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RouterAdvertPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RouterAdvertPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the hop_limit field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_hop_limit(&self) -> u8 {
            let _self = self;
            let co = 4;
            (_self.packet[co] as u8)
        }
        /// Get the flags field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_flags(&self) -> u8 {
            let _self = self;
            let co = 5;
            (_self.packet[co] as u8)
        }
        /// Get the lifetime field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_lifetime(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reachable_time field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reachable_time(&self) -> u32be {
            let _self = self;
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the retrans_time field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_retrans_time(&self) -> u32be {
            let _self = self;
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
    }
    impl <'a> MutableRouterAdvertPacket<'a> {
        /// Constructs a new MutableRouterAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableRouterAdvertPacket<'p>> {
            if packet.len() >=
                   MutableRouterAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRouterAdvertPacket{packet:
                                                   MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableRouterAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableRouterAdvertPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableRouterAdvertPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableRouterAdvertPacket<'static>> {
            if packet.len() >=
                   MutableRouterAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRouterAdvertPacket{packet:
                                                   MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableRouterAdvertPacket to a RouterAdvertPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RouterAdvertPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RouterAdvertPacket{packet:
                                   PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableRouterAdvertPacket to a RouterAdvertPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RouterAdvertPacket<'a> {
            RouterAdvertPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 16 }
        /// The size (in bytes) of a RouterAdvert instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &RouterAdvert) -> usize {
            16 + _packet.options.len() + _packet.payload.len()
        }
        /// Populates a RouterAdvertPacket using a RouterAdvert structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &RouterAdvert) {
            let _self = self;
            _self.set_icmpv6_type(packet.icmpv6_type);
            _self.set_icmpv6_code(packet.icmpv6_code);
            _self.set_checksum(packet.checksum);
            _self.set_hop_limit(packet.hop_limit);
            _self.set_flags(packet.flags);
            _self.set_lifetime(packet.lifetime);
            _self.set_reachable_time(packet.reachable_time);
            _self.set_retrans_time(packet.retrans_time);
            _self.set_options(&packet.options);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRouterAdvertPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRouterAdvertPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the hop_limit field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_hop_limit(&self) -> u8 {
            let _self = self;
            let co = 4;
            (_self.packet[co] as u8)
        }
        /// Get the flags field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_flags(&self) -> u8 {
            let _self = self;
            let co = 5;
            (_self.packet[co] as u8)
        }
        /// Get the lifetime field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_lifetime(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reachable_time field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reachable_time(&self) -> u32be {
            let _self = self;
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the retrans_time field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_retrans_time(&self) -> u32be {
            let _self = self;
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
        /// Set the value of the icmpv6_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRouterAdvertPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmpv6_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRouterAdvertPacket, val: u8) {
                let co = 1;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the checksum field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_checksum(&mut self, val: u16be) {
            let _self = self;
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the hop_limit field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_hop_limit(&mut self, val: u8) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = (val) as u8;
        }
        /// Set the flags field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_flags(&mut self, val: u8) {
            let _self = self;
            let co = 5;
            _self.packet[co + 0] = (val) as u8;
        }
        /// Set the lifetime field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_lifetime(&mut self, val: u16be) {
            let _self = self;
            let co = 6;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the reachable_time field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_reachable_time(&mut self, val: u32be) {
            let _self = self;
            let co = 8;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
        }
        /// Set the retrans_time field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_retrans_time(&mut self, val: u32be) {
            let _self = self;
            let co = 12;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
        }
        /// Get the raw &mut [u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 16;
            let end =
                min(current_offset +
                        ra_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &mut _self.packet[current_offset..end]
        }
        /// Set the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_options(&mut self, vals: &[NdpOption]) {
            use pnet_macros_support::packet::PacketSize;
            let _self = self;
            let mut current_offset = 16;
            let end =
                current_offset + ra_ndp_options_length(&_self.to_immutable());
            for val in vals.into_iter() {
                let mut packet =
                    MutableNdpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
                packet.populate(val);
                current_offset += packet.packet_size();
                assert!(current_offset <= end);
            }
        }
        /// Set the value of the payload field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_payload(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset =
                16 + ra_ndp_options_length(&_self.to_immutable());
            let len = 0;
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     RouterAdvertPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            16 + ra_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableRouterAdvertPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            16 + ra_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableRouterAdvertPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 16 + ra_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(16 +
                                    ra_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableRouterAdvertPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 16 + ra_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(16 +
                                    ra_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for RouterAdvertPacket<'a>
     {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 16 + ra_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(16 +
                                    ra_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `RouterAdvertPacket`s
    pub struct RouterAdvertIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for RouterAdvertIterable<'a> {
        type
        Item
        =
        RouterAdvertPacket<'a>;
        fn next(&mut self) -> Option<RouterAdvertPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = RouterAdvertPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     RouterAdvertPacket<'p> {
        type
        T
        =
        RouterAdvert;
        #[inline]
        fn from_packet(&self) -> RouterAdvert {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            RouterAdvert{icmpv6_type: _self.get_icmpv6_type(),
                         icmpv6_code: _self.get_icmpv6_code(),
                         checksum: _self.get_checksum(),
                         hop_limit: _self.get_hop_limit(),
                         flags: _self.get_flags(),
                         lifetime: _self.get_lifetime(),
                         reachable_time: _self.get_reachable_time(),
                         retrans_time: _self.get_retrans_time(),
                         options: _self.get_options(),
                         payload:
                             {
                                 let payload = self.payload();
                                 let mut vec =
                                     Vec::with_capacity(payload.len());
                                 vec.extend_from_slice(payload);
                                 vec
                             },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableRouterAdvertPacket<'p> {
        type
        T
        =
        RouterAdvert;
        #[inline]
        fn from_packet(&self) -> RouterAdvert {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            RouterAdvert{icmpv6_type: _self.get_icmpv6_type(),
                         icmpv6_code: _self.get_icmpv6_code(),
                         checksum: _self.get_checksum(),
                         hop_limit: _self.get_hop_limit(),
                         flags: _self.get_flags(),
                         lifetime: _self.get_lifetime(),
                         reachable_time: _self.get_reachable_time(),
                         retrans_time: _self.get_retrans_time(),
                         options: _self.get_options(),
                         payload:
                             {
                                 let payload = self.payload();
                                 let mut vec =
                                     Vec::with_capacity(payload.len());
                                 vec.extend_from_slice(payload);
                                 vec
                             },}
        }
    }
    impl <'p> ::std::fmt::Debug for RouterAdvertPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "RouterAdvertPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, hop_limit : {:?}, flags : {:?}, lifetime : {:?}, reachable_time : {:?}, retrans_time : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_hop_limit (  )
                   , _self . get_flags (  ) , _self . get_lifetime (  ) ,
                   _self . get_reachable_time (  ) , _self . get_retrans_time
                   (  ) , _self . get_options (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableRouterAdvertPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableRouterAdvertPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, hop_limit : {:?}, flags : {:?}, lifetime : {:?}, reachable_time : {:?}, retrans_time : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_hop_limit (  )
                   , _self . get_flags (  ) , _self . get_lifetime (  ) ,
                   _self . get_reachable_time (  ) , _self . get_retrans_time
                   (  ) , _self . get_options (  ))
        }
    }
    /// Router Advertisement Message Format [RFC 4861 § 4.2]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |     Code      |          Checksum             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// | Cur Hop Limit |M|O|  Reserved |       Router Lifetime         |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                         Reachable Time                        |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                          Retrans Timer                        |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |   Options ...
    /// +-+-+-+-+-+-+-+-+-+-+-+-
    /// ```
    ///
    /// [RFC 4861 § 4.2]: https://tools.ietf.org/html/rfc4861#section-4.2
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct RouterAdvert {
        pub icmpv6_type: Icmpv6Type,
        pub icmpv6_code: Icmpv6Code,
        pub checksum: u16be,
        pub hop_limit: u8,
        pub flags: u8,
        pub lifetime: u16be,
        pub reachable_time: u32be,
        pub retrans_time: u32be,
        pub options: Vec<NdpOption>,
        pub payload: Vec<u8>,
    }
    /// Router Advert packet calculation for the length of the options.
    fn ra_ndp_options_length(pkt: &RouterAdvertPacket) -> usize {
        if pkt.packet().len() > 16 { pkt.packet().len() - 16 } else { 0 }
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct NeighborSolicitPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableNeighborSolicitPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> NeighborSolicitPacket<'a> {
        /// Constructs a new NeighborSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8])
         -> Option<NeighborSolicitPacket<'p>> {
            if packet.len() >= NeighborSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NeighborSolicitPacket{packet:
                                               PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new NeighborSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the NeighborSolicitPacket will
        /// own its own data and the underlying buffer will be dropped when the NeighborSolicitPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<NeighborSolicitPacket<'static>> {
            if packet.len() >= NeighborSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NeighborSolicitPacket{packet:
                                               PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a NeighborSolicitPacket to a NeighborSolicitPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NeighborSolicitPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NeighborSolicitPacket{packet:
                                      PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a NeighborSolicitPacket to a NeighborSolicitPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NeighborSolicitPacket<'a> {
            NeighborSolicitPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 24 }
        /// The size (in bytes) of a NeighborSolicit instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NeighborSolicit) -> usize {
            24 + _packet.options.len() + _packet.payload.len()
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborSolicitPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborSolicitPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborSolicitPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &NeighborSolicitPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &NeighborSolicitPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &NeighborSolicitPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &NeighborSolicitPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &NeighborSolicitPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &NeighborSolicitPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &NeighborSolicitPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
    }
    impl <'a> MutableNeighborSolicitPacket<'a> {
        /// Constructs a new MutableNeighborSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableNeighborSolicitPacket<'p>> {
            if packet.len() >=
                   MutableNeighborSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNeighborSolicitPacket{packet:
                                                      MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableNeighborSolicitPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableNeighborSolicitPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableNeighborSolicitPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableNeighborSolicitPacket<'static>> {
            if packet.len() >=
                   MutableNeighborSolicitPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNeighborSolicitPacket{packet:
                                                      MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableNeighborSolicitPacket to a NeighborSolicitPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NeighborSolicitPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NeighborSolicitPacket{packet:
                                      PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableNeighborSolicitPacket to a NeighborSolicitPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NeighborSolicitPacket<'a> {
            NeighborSolicitPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 24 }
        /// The size (in bytes) of a NeighborSolicit instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NeighborSolicit) -> usize {
            24 + _packet.options.len() + _packet.payload.len()
        }
        /// Populates a NeighborSolicitPacket using a NeighborSolicit structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &NeighborSolicit) {
            let _self = self;
            _self.set_icmpv6_type(packet.icmpv6_type);
            _self.set_icmpv6_code(packet.icmpv6_code);
            _self.set_checksum(packet.checksum);
            _self.set_reserved(packet.reserved);
            _self.set_target_addr(packet.target_addr);
            _self.set_options(&packet.options);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborSolicitPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborSolicitPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &MutableNeighborSolicitPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
        /// Set the value of the icmpv6_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborSolicitPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmpv6_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborSolicitPacket, val: u8) {
                let co = 1;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the checksum field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_checksum(&mut self, val: u16be) {
            let _self = self;
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the reserved field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_reserved(&mut self, val: u32be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
        }
        /// Set the value of the target_addr field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_target_addr(&mut self, val: Ipv6Addr) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 8;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg1(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 10;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg2(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 12;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg3(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 14;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg4(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 16;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg5(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 18;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg6(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 20;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg7(_self: &mut MutableNeighborSolicitPacket, val: u16) {
                let co = 22;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
            set_arg1(_self, vals.1);
            set_arg2(_self, vals.2);
            set_arg3(_self, vals.3);
            set_arg4(_self, vals.4);
            set_arg5(_self, vals.5);
            set_arg6(_self, vals.6);
            set_arg7(_self, vals.7);
        }
        /// Get the raw &mut [u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        ns_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &mut _self.packet[current_offset..end]
        }
        /// Set the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_options(&mut self, vals: &[NdpOption]) {
            use pnet_macros_support::packet::PacketSize;
            let _self = self;
            let mut current_offset = 24;
            let end =
                current_offset + ns_ndp_options_length(&_self.to_immutable());
            for val in vals.into_iter() {
                let mut packet =
                    MutableNdpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
                packet.populate(val);
                current_offset += packet.packet_size();
                assert!(current_offset <= end);
            }
        }
        /// Set the value of the payload field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_payload(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset =
                24 + ns_ndp_options_length(&_self.to_immutable());
            let len = 0;
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     NeighborSolicitPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            24 + ns_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableNeighborSolicitPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            24 + ns_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableNeighborSolicitPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 24 + ns_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    ns_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableNeighborSolicitPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 24 + ns_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    ns_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     NeighborSolicitPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 24 + ns_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    ns_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `NeighborSolicitPacket`s
    pub struct NeighborSolicitIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for NeighborSolicitIterable<'a> {
        type
        Item
        =
        NeighborSolicitPacket<'a>;
        fn next(&mut self) -> Option<NeighborSolicitPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = NeighborSolicitPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     NeighborSolicitPacket<'p> {
        type
        T
        =
        NeighborSolicit;
        #[inline]
        fn from_packet(&self) -> NeighborSolicit {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NeighborSolicit{icmpv6_type: _self.get_icmpv6_type(),
                            icmpv6_code: _self.get_icmpv6_code(),
                            checksum: _self.get_checksum(),
                            reserved: _self.get_reserved(),
                            target_addr: _self.get_target_addr(),
                            options: _self.get_options(),
                            payload:
                                {
                                    let payload = self.payload();
                                    let mut vec =
                                        Vec::with_capacity(payload.len());
                                    vec.extend_from_slice(payload);
                                    vec
                                },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableNeighborSolicitPacket<'p> {
        type
        T
        =
        NeighborSolicit;
        #[inline]
        fn from_packet(&self) -> NeighborSolicit {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NeighborSolicit{icmpv6_type: _self.get_icmpv6_type(),
                            icmpv6_code: _self.get_icmpv6_code(),
                            checksum: _self.get_checksum(),
                            reserved: _self.get_reserved(),
                            target_addr: _self.get_target_addr(),
                            options: _self.get_options(),
                            payload:
                                {
                                    let payload = self.payload();
                                    let mut vec =
                                        Vec::with_capacity(payload.len());
                                    vec.extend_from_slice(payload);
                                    vec
                                },}
        }
    }
    impl <'p> ::std::fmt::Debug for NeighborSolicitPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "NeighborSolicitPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, target_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_target_addr (  ) , _self . get_options (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableNeighborSolicitPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableNeighborSolicitPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, target_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_target_addr (  ) , _self . get_options (  ))
        }
    }
    /// Neighbor Solicitation Message Format [RFC 4861 § 4.3]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |     Code      |          Checksum             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                           Reserved                            |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +                       Target Address                          +
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |   Options ...
    /// +-+-+-+-+-+-+-+-+-+-+-+-
    /// ```
    ///
    /// [RFC 4861 § 4.3]: https://tools.ietf.org/html/rfc4861#section-4.3
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct NeighborSolicit {
        pub icmpv6_type: Icmpv6Type,
        pub icmpv6_code: Icmpv6Code,
        pub checksum: u16be,
        pub reserved: u32be,
        pub target_addr: Ipv6Addr,
        pub options: Vec<NdpOption>,
        pub payload: Vec<u8>,
    }
    /// Neighbor Solicit packet calculation for the length of the options.
    fn ns_ndp_options_length(pkt: &NeighborSolicitPacket) -> usize {
        if pkt.packet().len() > 24 { pkt.packet().len() - 24 } else { 0 }
    }
    /// Enumeration of recognized Neighbor Advert flags.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod NeighborAdvertFlags {
        /// Indicates that the sender is a router.
        pub const Router: u8 = 128;
        /// Indicates that the advertisement was sent due to the receipt of a
        /// Neighbor Solicitation message.
        pub const Solicited: u8 = 64;
        /// Indicates that the advertisement should override an existing cache
        /// entry.
        pub const Override: u8 = 32;
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct NeighborAdvertPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableNeighborAdvertPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> NeighborAdvertPacket<'a> {
        /// Constructs a new NeighborAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<NeighborAdvertPacket<'p>> {
            if packet.len() >= NeighborAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NeighborAdvertPacket{packet:
                                              PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new NeighborAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the NeighborAdvertPacket will
        /// own its own data and the underlying buffer will be dropped when the NeighborAdvertPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<NeighborAdvertPacket<'static>> {
            if packet.len() >= NeighborAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(NeighborAdvertPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a NeighborAdvertPacket to a NeighborAdvertPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NeighborAdvertPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NeighborAdvertPacket{packet:
                                     PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a NeighborAdvertPacket to a NeighborAdvertPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NeighborAdvertPacket<'a> {
            NeighborAdvertPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 24 }
        /// The size (in bytes) of a NeighborAdvert instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NeighborAdvert) -> usize {
            24 + _packet.options.len() + _packet.payload.len()
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborAdvertPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborAdvertPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the flags field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_flags(&self) -> u8 {
            let _self = self;
            let co = 4;
            (_self.packet[co] as u8)
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u24be {
            let _self = self;
            let co = 5;
            let b0 = ((_self.packet[co + 0] as u24be) << 16) as u24be;
            let b1 = ((_self.packet[co + 1] as u24be) << 8) as u24be;
            let b2 = ((_self.packet[co + 2] as u24be)) as u24be;
            b0 | b1 | b2
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &NeighborAdvertPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &NeighborAdvertPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &NeighborAdvertPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &NeighborAdvertPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &NeighborAdvertPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &NeighborAdvertPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &NeighborAdvertPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &NeighborAdvertPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
    }
    impl <'a> MutableNeighborAdvertPacket<'a> {
        /// Constructs a new MutableNeighborAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableNeighborAdvertPacket<'p>> {
            if packet.len() >=
                   MutableNeighborAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNeighborAdvertPacket{packet:
                                                     MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableNeighborAdvertPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableNeighborAdvertPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableNeighborAdvertPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableNeighborAdvertPacket<'static>> {
            if packet.len() >=
                   MutableNeighborAdvertPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableNeighborAdvertPacket{packet:
                                                     MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableNeighborAdvertPacket to a NeighborAdvertPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> NeighborAdvertPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            NeighborAdvertPacket{packet:
                                     PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableNeighborAdvertPacket to a NeighborAdvertPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> NeighborAdvertPacket<'a> {
            NeighborAdvertPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 24 }
        /// The size (in bytes) of a NeighborAdvert instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &NeighborAdvert) -> usize {
            24 + _packet.options.len() + _packet.payload.len()
        }
        /// Populates a NeighborAdvertPacket using a NeighborAdvert structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &NeighborAdvert) {
            let _self = self;
            _self.set_icmpv6_type(packet.icmpv6_type);
            _self.set_icmpv6_code(packet.icmpv6_code);
            _self.set_checksum(packet.checksum);
            _self.set_flags(packet.flags);
            _self.set_reserved(packet.reserved);
            _self.set_target_addr(packet.target_addr);
            _self.set_options(&packet.options);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborAdvertPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborAdvertPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the flags field.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_flags(&self) -> u8 {
            let _self = self;
            let co = 4;
            (_self.packet[co] as u8)
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u24be {
            let _self = self;
            let co = 5;
            let b0 = ((_self.packet[co + 0] as u24be) << 16) as u24be;
            let b1 = ((_self.packet[co + 1] as u24be) << 8) as u24be;
            let b2 = ((_self.packet[co + 2] as u24be)) as u24be;
            b0 | b1 | b2
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &MutableNeighborAdvertPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
        /// Set the value of the icmpv6_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborAdvertPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmpv6_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborAdvertPacket, val: u8) {
                let co = 1;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the checksum field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_checksum(&mut self, val: u16be) {
            let _self = self;
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the flags field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_flags(&mut self, val: u8) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = (val) as u8;
        }
        /// Set the reserved field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_reserved(&mut self, val: u24be) {
            let _self = self;
            let co = 5;
            _self.packet[co + 0] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 1] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 2] = (val) as u8;
        }
        /// Set the value of the target_addr field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_target_addr(&mut self, val: Ipv6Addr) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 8;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg1(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 10;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg2(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 12;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg3(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 14;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg4(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 16;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg5(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 18;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg6(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 20;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg7(_self: &mut MutableNeighborAdvertPacket, val: u16) {
                let co = 22;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
            set_arg1(_self, vals.1);
            set_arg2(_self, vals.2);
            set_arg3(_self, vals.3);
            set_arg4(_self, vals.4);
            set_arg5(_self, vals.5);
            set_arg6(_self, vals.6);
            set_arg7(_self, vals.7);
        }
        /// Get the raw &mut [u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 24;
            let end =
                min(current_offset +
                        na_ndp_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &mut _self.packet[current_offset..end]
        }
        /// Set the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_options(&mut self, vals: &[NdpOption]) {
            use pnet_macros_support::packet::PacketSize;
            let _self = self;
            let mut current_offset = 24;
            let end =
                current_offset + na_ndp_options_length(&_self.to_immutable());
            for val in vals.into_iter() {
                let mut packet =
                    MutableNdpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
                packet.populate(val);
                current_offset += packet.packet_size();
                assert!(current_offset <= end);
            }
        }
        /// Set the value of the payload field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_payload(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset =
                24 + na_ndp_options_length(&_self.to_immutable());
            let len = 0;
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     NeighborAdvertPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            24 + na_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableNeighborAdvertPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            24 + na_ndp_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableNeighborAdvertPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 24 + na_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    na_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableNeighborAdvertPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 24 + na_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    na_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     NeighborAdvertPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 24 + na_ndp_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(24 +
                                    na_ndp_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `NeighborAdvertPacket`s
    pub struct NeighborAdvertIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for NeighborAdvertIterable<'a> {
        type
        Item
        =
        NeighborAdvertPacket<'a>;
        fn next(&mut self) -> Option<NeighborAdvertPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = NeighborAdvertPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     NeighborAdvertPacket<'p> {
        type
        T
        =
        NeighborAdvert;
        #[inline]
        fn from_packet(&self) -> NeighborAdvert {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NeighborAdvert{icmpv6_type: _self.get_icmpv6_type(),
                           icmpv6_code: _self.get_icmpv6_code(),
                           checksum: _self.get_checksum(),
                           flags: _self.get_flags(),
                           reserved: _self.get_reserved(),
                           target_addr: _self.get_target_addr(),
                           options: _self.get_options(),
                           payload:
                               {
                                   let payload = self.payload();
                                   let mut vec =
                                       Vec::with_capacity(payload.len());
                                   vec.extend_from_slice(payload);
                                   vec
                               },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableNeighborAdvertPacket<'p> {
        type
        T
        =
        NeighborAdvert;
        #[inline]
        fn from_packet(&self) -> NeighborAdvert {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            NeighborAdvert{icmpv6_type: _self.get_icmpv6_type(),
                           icmpv6_code: _self.get_icmpv6_code(),
                           checksum: _self.get_checksum(),
                           flags: _self.get_flags(),
                           reserved: _self.get_reserved(),
                           target_addr: _self.get_target_addr(),
                           options: _self.get_options(),
                           payload:
                               {
                                   let payload = self.payload();
                                   let mut vec =
                                       Vec::with_capacity(payload.len());
                                   vec.extend_from_slice(payload);
                                   vec
                               },}
        }
    }
    impl <'p> ::std::fmt::Debug for NeighborAdvertPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "NeighborAdvertPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, flags : {:?}, reserved : {:?}, target_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_flags (  ) ,
                   _self . get_reserved (  ) , _self . get_target_addr (  ) ,
                   _self . get_options (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableNeighborAdvertPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableNeighborAdvertPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, flags : {:?}, reserved : {:?}, target_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_flags (  ) ,
                   _self . get_reserved (  ) , _self . get_target_addr (  ) ,
                   _self . get_options (  ))
        }
    }
    /// Neighbor Advertisement Message Format [RFC 4861 § 4.4]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |     Code      |          Checksum             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |R|S|O|                     Reserved                            |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +                       Target Address                          +
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |   Options ...
    /// +-+-+-+-+-+-+-+-+-+-+-+-
    /// ```
    ///
    /// [RFC 4861 § 4.4]: https://tools.ietf.org/html/rfc4861#section-4.4
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct NeighborAdvert {
        pub icmpv6_type: Icmpv6Type,
        pub icmpv6_code: Icmpv6Code,
        pub checksum: u16be,
        pub flags: u8,
        pub reserved: u24be,
        pub target_addr: Ipv6Addr,
        pub options: Vec<NdpOption>,
        pub payload: Vec<u8>,
    }
    /// Neighbor Advert packet calculation for the length of the options.
    fn na_ndp_options_length(pkt: &NeighborAdvertPacket) -> usize {
        if pkt.packet().len() > 24 { pkt.packet().len() - 24 } else { 0 }
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct RedirectPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableRedirectPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> RedirectPacket<'a> {
        /// Constructs a new RedirectPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<RedirectPacket<'p>> {
            if packet.len() >= RedirectPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RedirectPacket{packet: PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new RedirectPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the RedirectPacket will
        /// own its own data and the underlying buffer will be dropped when the RedirectPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<RedirectPacket<'static>> {
            if packet.len() >= RedirectPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(RedirectPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a RedirectPacket to a RedirectPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RedirectPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RedirectPacket{packet:
                               PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a RedirectPacket to a RedirectPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RedirectPacket<'a> {
            RedirectPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 40 }
        /// The size (in bytes) of a Redirect instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &Redirect) -> usize {
            40 + _packet.options.len() + _packet.payload.len()
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RedirectPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RedirectPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RedirectPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &RedirectPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &RedirectPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &RedirectPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &RedirectPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &RedirectPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &RedirectPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &RedirectPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the value of the dest_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_dest_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &RedirectPacket) -> u16 {
                let co = 24;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &RedirectPacket) -> u16 {
                let co = 26;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &RedirectPacket) -> u16 {
                let co = 28;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &RedirectPacket) -> u16 {
                let co = 30;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &RedirectPacket) -> u16 {
                let co = 32;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &RedirectPacket) -> u16 {
                let co = 34;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &RedirectPacket) -> u16 {
                let co = 36;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &RedirectPacket) -> u16 {
                let co = 38;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
    }
    impl <'a> MutableRedirectPacket<'a> {
        /// Constructs a new MutableRedirectPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableRedirectPacket<'p>> {
            if packet.len() >= MutableRedirectPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRedirectPacket{packet:
                                               MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableRedirectPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableRedirectPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableRedirectPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableRedirectPacket<'static>> {
            if packet.len() >= MutableRedirectPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableRedirectPacket{packet:
                                               MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableRedirectPacket to a RedirectPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> RedirectPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            RedirectPacket{packet:
                               PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableRedirectPacket to a RedirectPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> RedirectPacket<'a> {
            RedirectPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 40 }
        /// The size (in bytes) of a Redirect instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &Redirect) -> usize {
            40 + _packet.options.len() + _packet.payload.len()
        }
        /// Populates a RedirectPacket using a Redirect structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &Redirect) {
            let _self = self;
            _self.set_icmpv6_type(packet.icmpv6_type);
            _self.set_icmpv6_code(packet.icmpv6_code);
            _self.set_checksum(packet.checksum);
            _self.set_reserved(packet.reserved);
            _self.set_target_addr(packet.target_addr);
            _self.set_dest_addr(packet.dest_addr);
            _self.set_options(&packet.options);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmpv6_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_type(&self) -> Icmpv6Type {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRedirectPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            Icmpv6Type::new(get_arg0(&self))
        }
        /// Get the value of the icmpv6_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmpv6_code(&self) -> Icmpv6Code {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRedirectPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            Icmpv6Code::new(get_arg0(&self))
        }
        /// Get the checksum field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_checksum(&self) -> u16be {
            let _self = self;
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the reserved field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_reserved(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Get the value of the target_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_target_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRedirectPacket) -> u16 {
                let co = 8;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &MutableRedirectPacket) -> u16 {
                let co = 10;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &MutableRedirectPacket) -> u16 {
                let co = 12;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &MutableRedirectPacket) -> u16 {
                let co = 14;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &MutableRedirectPacket) -> u16 {
                let co = 16;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &MutableRedirectPacket) -> u16 {
                let co = 18;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &MutableRedirectPacket) -> u16 {
                let co = 20;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &MutableRedirectPacket) -> u16 {
                let co = 22;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the value of the dest_addr field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_dest_addr(&self) -> Ipv6Addr {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableRedirectPacket) -> u16 {
                let co = 24;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg1(_self: &MutableRedirectPacket) -> u16 {
                let co = 26;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg2(_self: &MutableRedirectPacket) -> u16 {
                let co = 28;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg3(_self: &MutableRedirectPacket) -> u16 {
                let co = 30;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg4(_self: &MutableRedirectPacket) -> u16 {
                let co = 32;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg5(_self: &MutableRedirectPacket) -> u16 {
                let co = 34;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg6(_self: &MutableRedirectPacket) -> u16 {
                let co = 36;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg7(_self: &MutableRedirectPacket) -> u16 {
                let co = 38;
                let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
                let b1 = ((_self.packet[co + 1] as u16)) as u16;
                b0 | b1
            }
            Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                          get_arg3(&self), get_arg4(&self), get_arg5(&self),
                          get_arg6(&self), get_arg7(&self))
        }
        /// Get the raw &[u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw(&self) -> &[u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &_self.packet[current_offset..end]
        }
        /// Get the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options(&self) -> Vec<NdpOption> {
            use pnet_macros_support::packet::FromPacket;
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf:
                                  &_self.packet[current_offset..end],}.map(|packet|
                                                                               packet.from_packet()).collect::<Vec<_>>()
        }
        /// Get the value of the options field as iterator
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_iter(&self) -> NdpOptionIterable {
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            NdpOptionIterable{buf: &_self.packet[current_offset..end],}
        }
        /// Set the value of the icmpv6_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_type(&mut self, val: Icmpv6Type) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRedirectPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmpv6_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmpv6_code(&mut self, val: Icmpv6Code) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRedirectPacket, val: u8) {
                let co = 1;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the checksum field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_checksum(&mut self, val: u16be) {
            let _self = self;
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the reserved field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_reserved(&mut self, val: u32be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
        }
        /// Set the value of the target_addr field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_target_addr(&mut self, val: Ipv6Addr) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 8;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg1(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 10;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg2(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 12;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg3(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 14;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg4(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 16;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg5(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 18;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg6(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 20;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg7(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 22;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
            set_arg1(_self, vals.1);
            set_arg2(_self, vals.2);
            set_arg3(_self, vals.3);
            set_arg4(_self, vals.4);
            set_arg5(_self, vals.5);
            set_arg6(_self, vals.6);
            set_arg7(_self, vals.7);
        }
        /// Set the value of the dest_addr field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_dest_addr(&mut self, val: Ipv6Addr) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 24;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg1(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 26;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg2(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 28;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg3(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 30;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg4(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 32;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg5(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 34;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg6(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 36;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg7(_self: &mut MutableRedirectPacket, val: u16) {
                let co = 38;
                _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
                _self.packet[co + 1] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
            set_arg1(_self, vals.1);
            set_arg2(_self, vals.2);
            set_arg3(_self, vals.3);
            set_arg4(_self, vals.4);
            set_arg5(_self, vals.5);
            set_arg6(_self, vals.6);
            set_arg7(_self, vals.7);
        }
        /// Get the raw &mut [u8] value of the options field, without copying
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
            use std::cmp::min;
            let _self = self;
            let current_offset = 40;
            let end =
                min(current_offset +
                        redirect_options_length(&_self.to_immutable()),
                    _self.packet.len());
            &mut _self.packet[current_offset..end]
        }
        /// Set the value of the options field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_options(&mut self, vals: &[NdpOption]) {
            use pnet_macros_support::packet::PacketSize;
            let _self = self;
            let mut current_offset = 40;
            let end =
                current_offset +
                    redirect_options_length(&_self.to_immutable());
            for val in vals.into_iter() {
                let mut packet =
                    MutableNdpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
                packet.populate(val);
                current_offset += packet.packet_size();
                assert!(current_offset <= end);
            }
        }
        /// Set the value of the payload field (copies contents)
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_payload(&mut self, vals: &[u8]) {
            use std::ptr::copy_nonoverlapping;
            let mut _self = self;
            let current_offset =
                40 + redirect_options_length(&_self.to_immutable());
            let len = 0;
            assert!(vals . len (  ) <= len);
            unsafe {
                copy_nonoverlapping(vals[..].as_ptr(),
                                    _self.packet[current_offset..].as_mut_ptr(),
                                    vals.len())
            }
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for RedirectPacket<'a>
     {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            40 + redirect_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableRedirectPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize {
            let _self = self;
            40 + redirect_options_length(&_self.to_immutable()) + 0
        }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableRedirectPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 40 + redirect_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(40 +
                                    redirect_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableRedirectPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 40 + redirect_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(40 +
                                    redirect_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for RedirectPacket<'a> {
        #[inline]
        fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload<'p>(&'p self) -> &'p [u8] {
            let _self = self;
            let start = 40 + redirect_options_length(&_self.to_immutable());
            let end =
                ::std::cmp::min(40 +
                                    redirect_options_length(&_self.to_immutable())
                                    + 0, _self.packet.len());
            if _self.packet.len() <= start { return &[]; }
            &_self.packet[start..end]
        }
    }
    /// Used to iterate over a slice of `RedirectPacket`s
    pub struct RedirectIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for RedirectIterable<'a> {
        type
        Item
        =
        RedirectPacket<'a>;
        fn next(&mut self) -> Option<RedirectPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = RedirectPacket::new(self.buf) {
                    let start = min(ret.packet_size(), self.buf.len());
                    self.buf = &self.buf[start..];
                    return Some(ret);
                }
            }
            None
        }
        fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for RedirectPacket<'p>
     {
        type
        T
        =
        Redirect;
        #[inline]
        fn from_packet(&self) -> Redirect {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            Redirect{icmpv6_type: _self.get_icmpv6_type(),
                     icmpv6_code: _self.get_icmpv6_code(),
                     checksum: _self.get_checksum(),
                     reserved: _self.get_reserved(),
                     target_addr: _self.get_target_addr(),
                     dest_addr: _self.get_dest_addr(),
                     options: _self.get_options(),
                     payload:
                         {
                             let payload = self.payload();
                             let mut vec = Vec::with_capacity(payload.len());
                             vec.extend_from_slice(payload);
                             vec
                         },}
        }
    }
    impl <'p> ::pnet_macros_support::packet::FromPacket for
     MutableRedirectPacket<'p> {
        type
        T
        =
        Redirect;
        #[inline]
        fn from_packet(&self) -> Redirect {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            Redirect{icmpv6_type: _self.get_icmpv6_type(),
                     icmpv6_code: _self.get_icmpv6_code(),
                     checksum: _self.get_checksum(),
                     reserved: _self.get_reserved(),
                     target_addr: _self.get_target_addr(),
                     dest_addr: _self.get_dest_addr(),
                     options: _self.get_options(),
                     payload:
                         {
                             let payload = self.payload();
                             let mut vec = Vec::with_capacity(payload.len());
                             vec.extend_from_slice(payload);
                             vec
                         },}
        }
    }
    impl <'p> ::std::fmt::Debug for RedirectPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "RedirectPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, target_addr : {:?}, dest_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_target_addr (  ) , _self . get_dest_addr (  ) ,
                   _self . get_options (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableRedirectPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableRedirectPacket {{ icmpv6_type : {:?}, icmpv6_code : {:?}, checksum : {:?}, reserved : {:?}, target_addr : {:?}, dest_addr : {:?}, options : {:?},  }}"
                   , _self . get_icmpv6_type (  ) , _self . get_icmpv6_code (
                   ) , _self . get_checksum (  ) , _self . get_reserved (  ) ,
                   _self . get_target_addr (  ) , _self . get_dest_addr (  ) ,
                   _self . get_options (  ))
        }
    }
    /// Redirect Message Format [RFC 4861 § 4.5]
    ///
    /// ```text
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |     Type      |     Code      |          Checksum             |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                           Reserved                            |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +                       Target Address                          +
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +                     Destination Address                       +
    /// |                                                               |
    /// +                                                               +
    /// |                                                               |
    /// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    /// |   Options ...
    /// +-+-+-+-+-+-+-+-+-+-+-+-
    /// ```
    ///
    /// [RFC 4861 § 4.5]: https://tools.ietf.org/html/rfc4861#section-4.5
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct Redirect {
        pub icmpv6_type: Icmpv6Type,
        pub icmpv6_code: Icmpv6Code,
        pub checksum: u16be,
        pub reserved: u32be,
        pub target_addr: Ipv6Addr,
        pub dest_addr: Ipv6Addr,
        pub options: Vec<NdpOption>,
        pub payload: Vec<u8>,
    }
    /// Redirect packet calculation for the length of the options.
    fn redirect_options_length(pkt: &RedirectPacket) -> usize {
        if pkt.packet().len() > 40 { pkt.packet().len() - 40 } else { 0 }
    }
    #[cfg(test)]
    mod ndp_tests {
        use icmpv6::{Icmpv6Types, Icmpv6Code};
        use super::*;
        #[test]
        fn basic_option_parsing() {
            let mut data =
                vec!(0x02 , 0x01 , 0x06 , 0x05 , 0x04 , 0x03 , 0x02 , 0x01 ,
                     0x00 , 0x00 , 0x00);
            let pkg = MutableNdpOptionPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_option_type (  ) , NdpOptionTypes ::
                       TargetLLAddr);
            assert_eq!(pkg . get_length (  ) , 0x01);
            assert_eq!(pkg . payload (  ) . len (  ) , 6);
            assert_eq!(pkg . payload (  ) , & [
                       0x06 , 0x05 , 0x04 , 0x03 , 0x02 , 0x01 ]);
        }
        #[test]
        fn basic_rs_parse() {
            let mut data =
                vec!(0x85 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x02 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x01 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,);
            let pkg = MutableRouterSolicitPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_icmpv6_type (  ) , Icmpv6Types ::
                       RouterSolicit);
            assert_eq!(pkg . get_icmpv6_code (  ) , Icmpv6Code ( 0 ));
            assert_eq!(pkg . get_checksum (  ) , 0);
            assert_eq!(pkg . get_reserved (  ) , 0);
            assert_eq!(pkg . get_options (  ) . len (  ) , 2);
            let option = &pkg.get_options()[0];
            assert_eq!(option . option_type , NdpOptionTypes :: TargetLLAddr);
            assert_eq!(option . length , 0x01);
            assert_eq!(option . data , & [
                       0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ]);
            assert_eq!(option . data . len (  ) , 6);
            let option = &pkg.get_options()[1];
            assert_eq!(option . option_type , NdpOptionTypes :: SourceLLAddr);
            assert_eq!(option . length , 1);
            assert_eq!(option . data , & [
                       0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ]);
        }
        #[test]
        fn basic_rs_create() {
            let ref_packet =
                vec!(0x85 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x01 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00);
            let mut packet = [0u8; 16];
            let options =
                vec!(NdpOption {
                     option_type : NdpOptionTypes :: SourceLLAddr , length : 1
                     , data : vec ! [ 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00
                     ] });
            {
                let mut rs_packet =
                    MutableRouterSolicitPacket::new(&mut packet[..]).unwrap();
                rs_packet.set_icmpv6_type(Icmpv6Types::RouterSolicit);
                rs_packet.set_icmpv6_code(Icmpv6Code(0));
                rs_packet.set_options(&options[..]);
            }
            assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
        }
        #[test]
        fn basic_ra_parse() {
            let mut data =
                vec!(0x86 , 0x00 , 0x00 , 0x00 , 0xff , 0x80 , 0x09 , 0x00 ,
                     0x12 , 0x34 , 0x56 , 0x78 , 0x87 , 0x65 , 0x43 , 0x21 ,
                     0x01 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x05 , 0x01 , 0x00 , 0x00 , 0x57 , 0x68 , 0x61 , 0x74);
            let pkg = MutableRouterAdvertPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_icmpv6_type (  ) , Icmpv6Types ::
                       RouterAdvert);
            assert_eq!(pkg . get_icmpv6_code (  ) , Icmpv6Code ( 0 ));
            assert_eq!(pkg . get_checksum (  ) , 0x00);
            assert_eq!(pkg . get_hop_limit (  ) , 0xff);
            assert_eq!(pkg . get_flags (  ) , RouterAdvertFlags ::
                       ManagedAddressConf);
            assert_eq!(pkg . get_lifetime (  ) , 0x900);
            assert_eq!(pkg . get_reachable_time (  ) , 0x12345678);
            assert_eq!(pkg . get_retrans_time (  ) , 0x87654321);
            assert_eq!(pkg . get_options (  ) . len (  ) , 2);
            let option = &pkg.get_options()[0];
            assert_eq!(option . option_type , NdpOptionTypes :: SourceLLAddr);
            assert_eq!(option . length , 1);
            assert_eq!(option . data , & [
                       0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ]);
            let option = &pkg.get_options()[1];
            assert_eq!(option . option_type , NdpOptionTypes :: MTU);
            assert_eq!(option . length , 1);
            assert_eq!(option . data , & [
                       0x00 , 0x00 , 0x57 , 0x68 , 0x61 , 0x74 ]);
        }
        #[test]
        fn basic_ra_create() {
            let ref_packet =
                vec!(0x86 , 0x00 , 0x00 , 0x00 , 0xff , 0x80 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x05 , 0x01 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00);
            let mut packet = [0u8; 24];
            let options =
                vec!(NdpOption {
                     option_type : NdpOptionTypes :: MTU , length : 1 , data :
                     vec ! [ 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ] });
            {
                let mut ra_packet =
                    MutableRouterAdvertPacket::new(&mut packet[..]).unwrap();
                ra_packet.set_icmpv6_type(Icmpv6Types::RouterAdvert);
                ra_packet.set_icmpv6_code(Icmpv6Code(0));
                ra_packet.set_hop_limit(255);
                ra_packet.set_flags(RouterAdvertFlags::ManagedAddressConf);
                ra_packet.set_options(&options[..]);
            }
            assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
        }
        #[test]
        fn basic_ns_parse() {
            let mut data =
                vec!(0x87 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01);
            let pkg =
                MutableNeighborSolicitPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_icmpv6_type (  ) , Icmpv6Types ::
                       NeighborSolicit);
            assert_eq!(pkg . get_icmpv6_code (  ) , Icmpv6Code ( 0 ));
            assert_eq!(pkg . get_checksum (  ) , 0x00);
            assert_eq!(pkg . get_reserved (  ) , 0x00);
            assert_eq!(pkg . get_target_addr (  ) , Ipv6Addr :: new (
                       0xff02 , 0 , 0 , 0 , 0 , 0 , 0 , 1 ));
        }
        #[test]
        fn basic_ns_create() {
            let ref_packet =
                vec!(0x87 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01 ,);
            let mut packet = [0u8; 24];
            {
                let mut ns_packet =
                    MutableNeighborSolicitPacket::new(&mut packet[..]).unwrap();
                ns_packet.set_icmpv6_type(Icmpv6Types::NeighborSolicit);
                ns_packet.set_icmpv6_code(Icmpv6Code(0));
                ns_packet.set_target_addr(Ipv6Addr::new(65282, 0, 0, 0, 0, 0,
                                                        0, 1));
            }
            assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
        }
        #[test]
        fn basic_na_parse() {
            let mut data =
                vec!(0x88 , 0x00 , 0x00 , 0x00 , 0x80 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01);
            let pkg =
                MutableNeighborAdvertPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_icmpv6_type (  ) , Icmpv6Types ::
                       NeighborAdvert);
            assert_eq!(pkg . get_icmpv6_code (  ) , Icmpv6Code ( 0 ));
            assert_eq!(pkg . get_checksum (  ) , 0x00);
            assert_eq!(pkg . get_reserved (  ) , 0x00);
            assert_eq!(pkg . get_flags (  ) , 0x80);
            assert_eq!(pkg . get_target_addr (  ) , Ipv6Addr :: new (
                       0xff02 , 0 , 0 , 0 , 0 , 0 , 0 , 1 ));
        }
        #[test]
        fn basic_na_create() {
            let ref_packet =
                vec!(0x88 , 0x00 , 0x00 , 0x00 , 0x80 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01 ,);
            let mut packet = [0u8; 24];
            {
                let mut na_packet =
                    MutableNeighborAdvertPacket::new(&mut packet[..]).unwrap();
                na_packet.set_icmpv6_type(Icmpv6Types::NeighborAdvert);
                na_packet.set_icmpv6_code(Icmpv6Code(0));
                na_packet.set_target_addr(Ipv6Addr::new(65282, 0, 0, 0, 0, 0,
                                                        0, 1));
                na_packet.set_flags(NeighborAdvertFlags::Router);
            }
            assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
        }
        #[test]
        fn basic_redirect_parse() {
            let mut data =
                vec!(0x89 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,);
            let pkg = MutableRedirectPacket::new(&mut data[..]).unwrap();
            assert_eq!(pkg . get_icmpv6_type (  ) , Icmpv6Types :: Redirect);
            assert_eq!(pkg . get_icmpv6_code (  ) , Icmpv6Code ( 0 ));
            assert_eq!(pkg . get_checksum (  ) , 0x00);
            assert_eq!(pkg . get_reserved (  ) , 0x00);
            assert_eq!(pkg . get_target_addr (  ) , Ipv6Addr :: new (
                       0xff02 , 0 , 0 , 0 , 0 , 0 , 0 , 1 ));
            assert_eq!(pkg . get_dest_addr (  ) , Ipv6Addr :: new (
                       0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 ));
        }
        #[test]
        fn basic_redirect_create() {
            let ref_packet =
                vec!(0x89 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0xff , 0x02 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x01 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,
                     0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 , 0x00 ,);
            let mut packet = [0u8; 40];
            {
                let mut rdr_packet =
                    MutableRedirectPacket::new(&mut packet[..]).unwrap();
                rdr_packet.set_icmpv6_type(Icmpv6Types::Redirect);
                rdr_packet.set_icmpv6_code(Icmpv6Code(0));
                rdr_packet.set_target_addr(Ipv6Addr::new(65282, 0, 0, 0, 0, 0,
                                                         0, 1));
                rdr_packet.set_dest_addr(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0,
                                                       0));
            }
            assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
        }
    }
}
