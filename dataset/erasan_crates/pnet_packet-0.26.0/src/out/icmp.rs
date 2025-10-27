// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PrimitiveValues;
use pnet_macros_support::types::*;

/// Represents the "ICMP type" header field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct IcmpType(pub u8);

impl IcmpType {
    /// Create a new `IcmpType` instance.
    pub fn new(val: u8) -> IcmpType { IcmpType(val) }
}

impl PrimitiveValues for IcmpType {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}

/// Represents the "ICMP code" header field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct IcmpCode(pub u8);

impl IcmpCode {
    /// Create a new `IcmpCode` instance.
    pub fn new(val: u8) -> IcmpCode { IcmpCode(val) }
}

impl PrimitiveValues for IcmpCode {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}

// theoretically, the header is 64 bytes long, but since the "Rest Of Header" part depends on
// the ICMP type and ICMP code, we consider it's part of the payload.
// rest_of_header: u32be,







































#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct IcmpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableIcmpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> IcmpPacket<'a> {
    /// Constructs a new IcmpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<IcmpPacket<'p>> {
        if packet.len() >= IcmpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(IcmpPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new IcmpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the IcmpPacket will
    /// own its own data and the underlying buffer will be dropped when the IcmpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<IcmpPacket<'static>> {
        if packet.len() >= IcmpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(IcmpPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a IcmpPacket to a IcmpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> IcmpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        IcmpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a IcmpPacket to a IcmpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> IcmpPacket<'a> {
        IcmpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Icmp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Icmp) -> usize { 4 + _packet.payload.len() }
    /// Get the value of the icmp_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmp_type(&self) -> IcmpType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &IcmpPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IcmpType::new(get_arg0(&self))
    }
    /// Get the value of the icmp_code field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmp_code(&self) -> IcmpCode {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &IcmpPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        IcmpCode::new(get_arg0(&self))
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
impl <'a> MutableIcmpPacket<'a> {
    /// Constructs a new MutableIcmpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableIcmpPacket<'p>> {
        if packet.len() >= MutableIcmpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIcmpPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableIcmpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableIcmpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableIcmpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableIcmpPacket<'static>> {
        if packet.len() >= MutableIcmpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIcmpPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableIcmpPacket to a IcmpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> IcmpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        IcmpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableIcmpPacket to a IcmpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> IcmpPacket<'a> {
        IcmpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Icmp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Icmp) -> usize { 4 + _packet.payload.len() }
    /// Populates a IcmpPacket using a Icmp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Icmp) {
        let _self = self;
        _self.set_icmp_type(packet.icmp_type);
        _self.set_icmp_code(packet.icmp_code);
        _self.set_checksum(packet.checksum);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the icmp_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmp_type(&self) -> IcmpType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIcmpPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IcmpType::new(get_arg0(&self))
    }
    /// Get the value of the icmp_code field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_icmp_code(&self) -> IcmpCode {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIcmpPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        IcmpCode::new(get_arg0(&self))
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
    /// Set the value of the icmp_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_icmp_type(&mut self, val: IcmpType) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIcmpPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the icmp_code field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_icmp_code(&mut self, val: IcmpCode) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIcmpPacket, val: u8) {
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
impl <'a> ::pnet_macros_support::packet::PacketSize for IcmpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableIcmpPacket<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableIcmpPacket<'a> {
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
impl <'a> ::pnet_macros_support::packet::Packet for MutableIcmpPacket<'a> {
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
impl <'a> ::pnet_macros_support::packet::Packet for IcmpPacket<'a> {
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
/// Used to iterate over a slice of `IcmpPacket`s
pub struct IcmpIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for IcmpIterable<'a> {
    type
    Item
    =
    IcmpPacket<'a>;
    fn next(&mut self) -> Option<IcmpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = IcmpPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for IcmpPacket<'p> {
    type
    T
    =
    Icmp;
    #[inline]
    fn from_packet(&self) -> Icmp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Icmp{icmp_type: _self.get_icmp_type(),
             icmp_code: _self.get_icmp_code(),
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
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableIcmpPacket<'p>
 {
    type
    T
    =
    Icmp;
    #[inline]
    fn from_packet(&self) -> Icmp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Icmp{icmp_type: _self.get_icmp_type(),
             icmp_code: _self.get_icmp_code(),
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
impl <'p> ::std::fmt::Debug for IcmpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "IcmpPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?},  }}"
               , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
               _self . get_checksum (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIcmpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIcmpPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?},  }}"
               , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
               _self . get_checksum (  ))
    }
}
/// Represents a generic ICMP packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Icmp {
    pub icmp_type: IcmpType,
    pub icmp_code: IcmpCode,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
/// Calculates a checksum of an ICMP packet.
pub fn checksum(packet: &IcmpPacket) -> u16be {
    use Packet;
    use util;
    util::checksum(packet.packet(), 1)
}
#[cfg(test)]
mod checksum_tests {
    use super::*;
    #[test]
    fn checksum_zeros() {
        let mut data = vec!(0u8 ; 8);
        let expected = 65535;
        let mut pkg = MutableIcmpPacket::new(&mut data[..]).unwrap();
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
        pkg.set_checksum(123);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
    #[test]
    fn checksum_nonzero() {
        let mut data = vec!(255u8 ; 8);
        let expected = 0;
        let mut pkg = MutableIcmpPacket::new(&mut data[..]).unwrap();
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
        pkg.set_checksum(0);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
    #[test]
    fn checksum_odd_bytes() {
        let mut data = vec!(191u8 ; 7);
        let expected = 49535;
        let pkg = IcmpPacket::new(&mut data[..]).unwrap();
        assert_eq!(checksum ( & pkg ) , expected);
    }
}
/// The enumeration of the recognized ICMP types.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod IcmpTypes {
    use icmp::IcmpType;
    /// ICMP type for "echo reply" packet.
    pub const EchoReply: IcmpType = IcmpType(0);
    /// ICMP type for "destination unreachable" packet.
    pub const DestinationUnreachable: IcmpType = IcmpType(3);
    /// ICMP type for "source quench" packet.
    pub const SourceQuench: IcmpType = IcmpType(4);
    /// ICMP type for "redirect message" packet.
    pub const RedirectMessage: IcmpType = IcmpType(5);
    /// ICMP type for "echo request" packet.
    pub const EchoRequest: IcmpType = IcmpType(8);
    /// ICMP type for "router advertisement" packet.
    pub const RouterAdvertisement: IcmpType = IcmpType(9);
    /// ICMP type for "router solicitation" packet.
    pub const RouterSolicitation: IcmpType = IcmpType(10);
    /// ICMP type for "time exceeded" packet.
    pub const TimeExceeded: IcmpType = IcmpType(11);
    /// ICMP type for "parameter problem" packet.
    pub const ParameterProblem: IcmpType = IcmpType(12);
    /// ICMP type for "timestamp" packet.
    pub const Timestamp: IcmpType = IcmpType(13);
    /// ICMP type for "timestamp reply" packet.
    pub const TimestampReply: IcmpType = IcmpType(14);
    /// ICMP type for "information request" packet.
    pub const InformationRequest: IcmpType = IcmpType(15);
    /// ICMP type for "information reply" packet.
    pub const InformationReply: IcmpType = IcmpType(16);
    /// ICMP type for "address mask request" packet.
    pub const AddressMaskRequest: IcmpType = IcmpType(17);
    /// ICMP type for "address mask reply" packet.
    pub const AddressMaskReply: IcmpType = IcmpType(18);
    /// ICMP type for "traceroute" packet.
    pub const Traceroute: IcmpType = IcmpType(30);
}
pub mod echo_reply {
    //! abstraction for ICMP "echo reply" packets.
    //!
    //! ```text
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Type      |     Code      |          Checksum             |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |           Identifier          |        Sequence Number        |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Data ...
    //! +-+-+-+-+-
    //! ```
    use PrimitiveValues;
    use icmp::{IcmpCode, IcmpType};
    use pnet_macros_support::types::*;
    /// Represent the "identifier" field of the ICMP echo replay header.
    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
    pub struct Identifier(pub u16);
    impl Identifier {
        /// Create a new `Identifier` instance.
        pub fn new(val: u16) -> Identifier { Identifier(val) }
    }
    impl PrimitiveValues for Identifier {
        type
        T
        =
        (u16,);
        fn to_primitive_values(&self) -> (u16,) { (self.0,) }
    }
    /// Represent the "sequence number" field of the ICMP echo replay header.
    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
    pub struct SequenceNumber(pub u16);
    impl SequenceNumber {
        /// Create a new `SequenceNumber` instance.
        pub fn new(val: u16) -> SequenceNumber { SequenceNumber(val) }
    }
    impl PrimitiveValues for SequenceNumber {
        type
        T
        =
        (u16,);
        fn to_primitive_values(&self) -> (u16,) { (self.0,) }
    }
    /// Enumeration of available ICMP codes for ICMP echo replay packets. There is actually only
    /// one, since the only valid ICMP code is 0.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod IcmpCodes {
        use icmp::IcmpCode;
        /// 0 is the only available ICMP code for "echo reply" ICMP packets.
        pub const NoCode: IcmpCode = IcmpCode(0);
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct EchoReplyPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableEchoReplyPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> EchoReplyPacket<'a> {
        /// Constructs a new EchoReplyPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<EchoReplyPacket<'p>> {
            if packet.len() >= EchoReplyPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(EchoReplyPacket{packet: PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new EchoReplyPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the EchoReplyPacket will
        /// own its own data and the underlying buffer will be dropped when the EchoReplyPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<EchoReplyPacket<'static>> {
            if packet.len() >= EchoReplyPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(EchoReplyPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a EchoReplyPacket to a EchoReplyPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> EchoReplyPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            EchoReplyPacket{packet:
                                PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a EchoReplyPacket to a EchoReplyPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> EchoReplyPacket<'a> {
            EchoReplyPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a EchoReply instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &EchoReply) -> usize {
            8 + _packet.payload.len()
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &EchoReplyPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &EchoReplyPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the identifier field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_identifier(&self) -> u16be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the sequence_number field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_sequence_number(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
    }
    impl <'a> MutableEchoReplyPacket<'a> {
        /// Constructs a new MutableEchoReplyPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableEchoReplyPacket<'p>> {
            if packet.len() >= MutableEchoReplyPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableEchoReplyPacket{packet:
                                                MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableEchoReplyPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableEchoReplyPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableEchoReplyPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableEchoReplyPacket<'static>> {
            if packet.len() >= MutableEchoReplyPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableEchoReplyPacket{packet:
                                                MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableEchoReplyPacket to a EchoReplyPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> EchoReplyPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            EchoReplyPacket{packet:
                                PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableEchoReplyPacket to a EchoReplyPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> EchoReplyPacket<'a> {
            EchoReplyPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a EchoReply instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &EchoReply) -> usize {
            8 + _packet.payload.len()
        }
        /// Populates a EchoReplyPacket using a EchoReply structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &EchoReply) {
            let _self = self;
            _self.set_icmp_type(packet.icmp_type);
            _self.set_icmp_code(packet.icmp_code);
            _self.set_checksum(packet.checksum);
            _self.set_identifier(packet.identifier);
            _self.set_sequence_number(packet.sequence_number);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableEchoReplyPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableEchoReplyPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the identifier field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_identifier(&self) -> u16be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the sequence_number field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_sequence_number(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Set the value of the icmp_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_type(&mut self, val: IcmpType) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableEchoReplyPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmp_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_code(&mut self, val: IcmpCode) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableEchoReplyPacket, val: u8) {
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
        /// Set the identifier field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_identifier(&mut self, val: u16be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the sequence_number field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_sequence_number(&mut self, val: u16be) {
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
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     EchoReplyPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableEchoReplyPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableEchoReplyPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 8;
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableEchoReplyPacket<'a> {
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
    impl <'a> ::pnet_macros_support::packet::Packet for EchoReplyPacket<'a> {
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
    /// Used to iterate over a slice of `EchoReplyPacket`s
    pub struct EchoReplyIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for EchoReplyIterable<'a> {
        type
        Item
        =
        EchoReplyPacket<'a>;
        fn next(&mut self) -> Option<EchoReplyPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = EchoReplyPacket::new(self.buf) {
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
     EchoReplyPacket<'p> {
        type
        T
        =
        EchoReply;
        #[inline]
        fn from_packet(&self) -> EchoReply {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            EchoReply{icmp_type: _self.get_icmp_type(),
                      icmp_code: _self.get_icmp_code(),
                      checksum: _self.get_checksum(),
                      identifier: _self.get_identifier(),
                      sequence_number: _self.get_sequence_number(),
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
     MutableEchoReplyPacket<'p> {
        type
        T
        =
        EchoReply;
        #[inline]
        fn from_packet(&self) -> EchoReply {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            EchoReply{icmp_type: _self.get_icmp_type(),
                      icmp_code: _self.get_icmp_code(),
                      checksum: _self.get_checksum(),
                      identifier: _self.get_identifier(),
                      sequence_number: _self.get_sequence_number(),
                      payload:
                          {
                              let payload = self.payload();
                              let mut vec = Vec::with_capacity(payload.len());
                              vec.extend_from_slice(payload);
                              vec
                          },}
        }
    }
    impl <'p> ::std::fmt::Debug for EchoReplyPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "EchoReplyPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, identifier : {:?}, sequence_number : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_identifier (  ) ,
                   _self . get_sequence_number (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableEchoReplyPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableEchoReplyPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, identifier : {:?}, sequence_number : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_identifier (  ) ,
                   _self . get_sequence_number (  ))
        }
    }
    /// Represents an ICMP echo reply packet.
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct EchoReply {
        pub icmp_type: IcmpType,
        pub icmp_code: IcmpCode,
        pub checksum: u16be,
        pub identifier: u16be,
        pub sequence_number: u16be,
        pub payload: Vec<u8>,
    }
}
pub mod echo_request {
    //! abstraction for "echo request" ICMP packets.
    //!
    //! ```text
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Type      |     Code      |          Checksum             |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |           Identifier          |        Sequence Number        |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Data ...
    //! +-+-+-+-+-
    //! ```
    use PrimitiveValues;
    use icmp::{IcmpCode, IcmpType};
    use pnet_macros_support::types::*;
    /// Represents the identifier field.
    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
    pub struct Identifier(pub u16);
    impl Identifier {
        /// Create a new `Identifier` instance.
        pub fn new(val: u16) -> Identifier { Identifier(val) }
    }
    impl PrimitiveValues for Identifier {
        type
        T
        =
        (u16,);
        fn to_primitive_values(&self) -> (u16,) { (self.0,) }
    }
    /// Represents the sequence number field.
    #[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
    pub struct SequenceNumber(pub u16);
    impl SequenceNumber {
        /// Create a new `SequenceNumber` instance.
        pub fn new(val: u16) -> SequenceNumber { SequenceNumber(val) }
    }
    impl PrimitiveValues for SequenceNumber {
        type
        T
        =
        (u16,);
        fn to_primitive_values(&self) -> (u16,) { (self.0,) }
    }
    /// Enumeration of available ICMP codes for "echo reply" ICMP packets. There is actually only
    /// one, since the only valid ICMP code is 0.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod IcmpCodes {
        use icmp::IcmpCode;
        /// 0 is the only available ICMP code for "echo reply" ICMP packets.
        pub const NoCode: IcmpCode = IcmpCode(0);
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct EchoRequestPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableEchoRequestPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> EchoRequestPacket<'a> {
        /// Constructs a new EchoRequestPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<EchoRequestPacket<'p>> {
            if packet.len() >= EchoRequestPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(EchoRequestPacket{packet: PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new EchoRequestPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the EchoRequestPacket will
        /// own its own data and the underlying buffer will be dropped when the EchoRequestPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<EchoRequestPacket<'static>> {
            if packet.len() >= EchoRequestPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(EchoRequestPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a EchoRequestPacket to a EchoRequestPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> EchoRequestPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            EchoRequestPacket{packet:
                                  PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a EchoRequestPacket to a EchoRequestPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> EchoRequestPacket<'a> {
            EchoRequestPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a EchoRequest instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &EchoRequest) -> usize {
            8 + _packet.payload.len()
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &EchoRequestPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &EchoRequestPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the identifier field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_identifier(&self) -> u16be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the sequence_number field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_sequence_number(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
    }
    impl <'a> MutableEchoRequestPacket<'a> {
        /// Constructs a new MutableEchoRequestPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableEchoRequestPacket<'p>> {
            if packet.len() >= MutableEchoRequestPacket::minimum_packet_size()
               {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableEchoRequestPacket{packet:
                                                  MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableEchoRequestPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableEchoRequestPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableEchoRequestPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableEchoRequestPacket<'static>> {
            if packet.len() >= MutableEchoRequestPacket::minimum_packet_size()
               {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableEchoRequestPacket{packet:
                                                  MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableEchoRequestPacket to a EchoRequestPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> EchoRequestPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            EchoRequestPacket{packet:
                                  PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableEchoRequestPacket to a EchoRequestPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> EchoRequestPacket<'a> {
            EchoRequestPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a EchoRequest instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &EchoRequest) -> usize {
            8 + _packet.payload.len()
        }
        /// Populates a EchoRequestPacket using a EchoRequest structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &EchoRequest) {
            let _self = self;
            _self.set_icmp_type(packet.icmp_type);
            _self.set_icmp_code(packet.icmp_code);
            _self.set_checksum(packet.checksum);
            _self.set_identifier(packet.identifier);
            _self.set_sequence_number(packet.sequence_number);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableEchoRequestPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableEchoRequestPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the identifier field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_identifier(&self) -> u16be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Get the sequence_number field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_sequence_number(&self) -> u16be {
            let _self = self;
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        /// Set the value of the icmp_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_type(&mut self, val: IcmpType) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableEchoRequestPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmp_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_code(&mut self, val: IcmpCode) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableEchoRequestPacket, val: u8) {
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
        /// Set the identifier field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_identifier(&mut self, val: u16be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        /// Set the sequence_number field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_sequence_number(&mut self, val: u16be) {
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
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     EchoRequestPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableEchoRequestPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableEchoRequestPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 8;
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableEchoRequestPacket<'a> {
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
    impl <'a> ::pnet_macros_support::packet::Packet for EchoRequestPacket<'a>
     {
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
    /// Used to iterate over a slice of `EchoRequestPacket`s
    pub struct EchoRequestIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for EchoRequestIterable<'a> {
        type
        Item
        =
        EchoRequestPacket<'a>;
        fn next(&mut self) -> Option<EchoRequestPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = EchoRequestPacket::new(self.buf) {
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
     EchoRequestPacket<'p> {
        type
        T
        =
        EchoRequest;
        #[inline]
        fn from_packet(&self) -> EchoRequest {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            EchoRequest{icmp_type: _self.get_icmp_type(),
                        icmp_code: _self.get_icmp_code(),
                        checksum: _self.get_checksum(),
                        identifier: _self.get_identifier(),
                        sequence_number: _self.get_sequence_number(),
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
     MutableEchoRequestPacket<'p> {
        type
        T
        =
        EchoRequest;
        #[inline]
        fn from_packet(&self) -> EchoRequest {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            EchoRequest{icmp_type: _self.get_icmp_type(),
                        icmp_code: _self.get_icmp_code(),
                        checksum: _self.get_checksum(),
                        identifier: _self.get_identifier(),
                        sequence_number: _self.get_sequence_number(),
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
    impl <'p> ::std::fmt::Debug for EchoRequestPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "EchoRequestPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, identifier : {:?}, sequence_number : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_identifier (  ) ,
                   _self . get_sequence_number (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableEchoRequestPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableEchoRequestPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, identifier : {:?}, sequence_number : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_identifier (  ) ,
                   _self . get_sequence_number (  ))
        }
    }
    /// Represents an "echo request" ICMP packet.
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct EchoRequest {
        pub icmp_type: IcmpType,
        pub icmp_code: IcmpCode,
        pub checksum: u16be,
        pub identifier: u16be,
        pub sequence_number: u16be,
        pub payload: Vec<u8>,
    }
}
pub mod destination_unreachable {
    //! abstraction for "destination unreachable" ICMP packets.
    //!
    //! ```text
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Type      |     Code      |          Checksum             |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |                             unused                            |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |      Internet Header + 64 bits of Original Data Datagram      |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! ```
    use icmp::{IcmpCode, IcmpType};
    use pnet_macros_support::types::*;
    /// Enumeration of the recognized ICMP codes for "destination unreachable" ICMP packets.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod IcmpCodes {
        use icmp::IcmpCode;
        /// ICMP code for "destination network unreachable" packet.
        pub const DestinationNetworkUnreachable: IcmpCode = IcmpCode(0);
        /// ICMP code for "destination host unreachable" packet.
        pub const DestinationHostUnreachable: IcmpCode = IcmpCode(1);
        /// ICMP code for "destination protocol unreachable" packet.
        pub const DestinationProtocolUnreachable: IcmpCode = IcmpCode(2);
        /// ICMP code for "destination port unreachable" packet.
        pub const DestinationPortUnreachable: IcmpCode = IcmpCode(3);
        /// ICMP code for "fragmentation required and DFF flag set" packet.
        pub const FragmentationRequiredAndDFFlagSet: IcmpCode = IcmpCode(4);
        /// ICMP code for "source route failed" packet.
        pub const SourceRouteFailed: IcmpCode = IcmpCode(5);
        /// ICMP code for "destination network unknown" packet.
        pub const DestinationNetworkUnknown: IcmpCode = IcmpCode(6);
        /// ICMP code for "destination host unknown" packet.
        pub const DestinationHostUnknown: IcmpCode = IcmpCode(7);
        /// ICMP code for "source host isolated" packet.
        pub const SourceHostIsolated: IcmpCode = IcmpCode(8);
        /// ICMP code for "network administrative prohibited" packet.
        pub const NetworkAdministrativelyProhibited: IcmpCode = IcmpCode(9);
        /// ICMP code for "host administrative prohibited" packet.
        pub const HostAdministrativelyProhibited: IcmpCode = IcmpCode(10);
        /// ICMP code for "network unreachable for this Type Of Service" packet.
        pub const NetworkUnreachableForTOS: IcmpCode = IcmpCode(11);
        /// ICMP code for "host unreachable for this Type Of Service" packet.
        pub const HostUnreachableForTOS: IcmpCode = IcmpCode(12);
        /// ICMP code for "communication administratively prohibited" packet.
        pub const CommunicationAdministrativelyProhibited: IcmpCode =
            IcmpCode(13);
        /// ICMP code for "host precedence violation" packet.
        pub const HostPrecedenceViolation: IcmpCode = IcmpCode(14);
        /// ICMP code for "precedence cut off in effect" packet.
        pub const PrecedenceCutoffInEffect: IcmpCode = IcmpCode(15);
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct DestinationUnreachablePacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableDestinationUnreachablePacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> DestinationUnreachablePacket<'a> {
        /// Constructs a new DestinationUnreachablePacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8])
         -> Option<DestinationUnreachablePacket<'p>> {
            if packet.len() >=
                   DestinationUnreachablePacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(DestinationUnreachablePacket{packet:
                                                      PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new DestinationUnreachablePacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the DestinationUnreachablePacket will
        /// own its own data and the underlying buffer will be dropped when the DestinationUnreachablePacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<DestinationUnreachablePacket<'static>> {
            if packet.len() >=
                   DestinationUnreachablePacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(DestinationUnreachablePacket{packet:
                                                      PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a DestinationUnreachablePacket to a DestinationUnreachablePacket
        #[inline]
        pub fn to_immutable<'p>(&'p self)
         -> DestinationUnreachablePacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            DestinationUnreachablePacket{packet:
                                             PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a DestinationUnreachablePacket to a DestinationUnreachablePacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self)
         -> DestinationUnreachablePacket<'a> {
            DestinationUnreachablePacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a DestinationUnreachable instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &DestinationUnreachable) -> usize {
            8 + _packet.payload.len()
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &DestinationUnreachablePacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &DestinationUnreachablePacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the unused field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_unused(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
    }
    impl <'a> MutableDestinationUnreachablePacket<'a> {
        /// Constructs a new MutableDestinationUnreachablePacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableDestinationUnreachablePacket<'p>> {
            if packet.len() >=
                   MutableDestinationUnreachablePacket::minimum_packet_size()
               {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableDestinationUnreachablePacket{packet:
                                                             MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableDestinationUnreachablePacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableDestinationUnreachablePacket will
        /// own its own data and the underlying buffer will be dropped when the MutableDestinationUnreachablePacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableDestinationUnreachablePacket<'static>> {
            if packet.len() >=
                   MutableDestinationUnreachablePacket::minimum_packet_size()
               {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableDestinationUnreachablePacket{packet:
                                                             MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableDestinationUnreachablePacket to a DestinationUnreachablePacket
        #[inline]
        pub fn to_immutable<'p>(&'p self)
         -> DestinationUnreachablePacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            DestinationUnreachablePacket{packet:
                                             PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableDestinationUnreachablePacket to a DestinationUnreachablePacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self)
         -> DestinationUnreachablePacket<'a> {
            DestinationUnreachablePacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a DestinationUnreachable instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &DestinationUnreachable) -> usize {
            8 + _packet.payload.len()
        }
        /// Populates a DestinationUnreachablePacket using a DestinationUnreachable structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &DestinationUnreachable) {
            let _self = self;
            _self.set_icmp_type(packet.icmp_type);
            _self.set_icmp_code(packet.icmp_code);
            _self.set_checksum(packet.checksum);
            _self.set_unused(packet.unused);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableDestinationUnreachablePacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableDestinationUnreachablePacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the unused field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_unused(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Set the value of the icmp_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_type(&mut self, val: IcmpType) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableDestinationUnreachablePacket,
                        val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmp_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_code(&mut self, val: IcmpCode) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableDestinationUnreachablePacket,
                        val: u8) {
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
        /// Set the unused field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_unused(&mut self, val: u32be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
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
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     DestinationUnreachablePacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableDestinationUnreachablePacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableDestinationUnreachablePacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 8;
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableDestinationUnreachablePacket<'a> {
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
    impl <'a> ::pnet_macros_support::packet::Packet for
     DestinationUnreachablePacket<'a> {
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
    /// Used to iterate over a slice of `DestinationUnreachablePacket`s
    pub struct DestinationUnreachableIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for DestinationUnreachableIterable<'a> {
        type
        Item
        =
        DestinationUnreachablePacket<'a>;
        fn next(&mut self) -> Option<DestinationUnreachablePacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = DestinationUnreachablePacket::new(self.buf)
                       {
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
     DestinationUnreachablePacket<'p> {
        type
        T
        =
        DestinationUnreachable;
        #[inline]
        fn from_packet(&self) -> DestinationUnreachable {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            DestinationUnreachable{icmp_type: _self.get_icmp_type(),
                                   icmp_code: _self.get_icmp_code(),
                                   checksum: _self.get_checksum(),
                                   unused: _self.get_unused(),
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
     MutableDestinationUnreachablePacket<'p> {
        type
        T
        =
        DestinationUnreachable;
        #[inline]
        fn from_packet(&self) -> DestinationUnreachable {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            DestinationUnreachable{icmp_type: _self.get_icmp_type(),
                                   icmp_code: _self.get_icmp_code(),
                                   checksum: _self.get_checksum(),
                                   unused: _self.get_unused(),
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
    impl <'p> ::std::fmt::Debug for DestinationUnreachablePacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "DestinationUnreachablePacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, unused : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_unused (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableDestinationUnreachablePacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableDestinationUnreachablePacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, unused : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_unused (  ))
        }
    }
    /// Represents an "echo request" ICMP packet.
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct DestinationUnreachable {
        pub icmp_type: IcmpType,
        pub icmp_code: IcmpCode,
        pub checksum: u16be,
        pub unused: u32be,
        pub payload: Vec<u8>,
    }
}
pub mod time_exceeded {
    //! abstraction for "time exceeded" ICMP packets.
    //!
    //! ```text
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |     Type      |     Code      |          Checksum             |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |                             unused                            |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! |      Internet Header + 64 bits of Original Data Datagram      |
    //! +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    //! ```
    use icmp::{IcmpCode, IcmpType};
    use pnet_macros_support::types::*;
    /// Enumeration of the recognized ICMP codes for "time exceeded" ICMP packets.
    #[allow(non_snake_case)]
    #[allow(non_upper_case_globals)]
    pub mod IcmpCodes {
        use icmp::IcmpCode;
        /// ICMP code for "time to live exceeded in transit" packet.
        pub const TimeToLiveExceededInTransit: IcmpCode = IcmpCode(0);
        /// ICMP code for "fragment reassembly time exceeded" packet.
        pub const FragmentReasemblyTimeExceeded: IcmpCode = IcmpCode(1);
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct TimeExceededPacket<'p> {
        packet: ::pnet_macros_support::packet::PacketData<'p>,
    }
    #[derive(PartialEq)]
    /// A structure enabling manipulation of on the wire packets
    pub struct MutableTimeExceededPacket<'p> {
        packet: ::pnet_macros_support::packet::MutPacketData<'p>,
    }
    impl <'a> TimeExceededPacket<'a> {
        /// Constructs a new TimeExceededPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p [u8]) -> Option<TimeExceededPacket<'p>> {
            if packet.len() >= TimeExceededPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(TimeExceededPacket{packet:
                                            PacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new TimeExceededPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the TimeExceededPacket will
        /// own its own data and the underlying buffer will be dropped when the TimeExceededPacket is.
        pub fn owned(packet: Vec<u8>) -> Option<TimeExceededPacket<'static>> {
            if packet.len() >= TimeExceededPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::PacketData;
                Some(TimeExceededPacket{packet: PacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a TimeExceededPacket to a TimeExceededPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> TimeExceededPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            TimeExceededPacket{packet:
                                   PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a TimeExceededPacket to a TimeExceededPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> TimeExceededPacket<'a> {
            TimeExceededPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a TimeExceeded instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &TimeExceeded) -> usize {
            8 + _packet.payload.len()
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &TimeExceededPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &TimeExceededPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the unused field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_unused(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
    }
    impl <'a> MutableTimeExceededPacket<'a> {
        /// Constructs a new MutableTimeExceededPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None.
        #[inline]
        pub fn new<'p>(packet: &'p mut [u8])
         -> Option<MutableTimeExceededPacket<'p>> {
            if packet.len() >=
                   MutableTimeExceededPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableTimeExceededPacket{packet:
                                                   MutPacketData::Borrowed(packet),})
            } else { None }
        }
        /// Constructs a new MutableTimeExceededPacket. If the provided buffer is less than the minimum required
        /// packet size, this will return None. With this constructor the MutableTimeExceededPacket will
        /// own its own data and the underlying buffer will be dropped when the MutableTimeExceededPacket is.
        pub fn owned(packet: Vec<u8>)
         -> Option<MutableTimeExceededPacket<'static>> {
            if packet.len() >=
                   MutableTimeExceededPacket::minimum_packet_size() {
                use ::pnet_macros_support::packet::MutPacketData;
                Some(MutableTimeExceededPacket{packet:
                                                   MutPacketData::Owned(packet),})
            } else { None }
        }
        /// Maps from a MutableTimeExceededPacket to a TimeExceededPacket
        #[inline]
        pub fn to_immutable<'p>(&'p self) -> TimeExceededPacket<'p> {
            use ::pnet_macros_support::packet::PacketData;
            TimeExceededPacket{packet:
                                   PacketData::Borrowed(self.packet.as_slice()),}
        }
        /// Maps from a MutableTimeExceededPacket to a TimeExceededPacket while consuming the source
        #[inline]
        pub fn consume_to_immutable(self) -> TimeExceededPacket<'a> {
            TimeExceededPacket{packet: self.packet.to_immutable(),}
        }
        /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
        /// of the fixed-size fields.
        #[inline]
        pub const fn minimum_packet_size() -> usize { 8 }
        /// The size (in bytes) of a TimeExceeded instance when converted into
        /// a byte-array
        #[inline]
        pub fn packet_size(_packet: &TimeExceeded) -> usize {
            8 + _packet.payload.len()
        }
        /// Populates a TimeExceededPacket using a TimeExceeded structure
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn populate(&mut self, packet: &TimeExceeded) {
            let _self = self;
            _self.set_icmp_type(packet.icmp_type);
            _self.set_icmp_code(packet.icmp_code);
            _self.set_checksum(packet.checksum);
            _self.set_unused(packet.unused);
            _self.set_payload(&packet.payload);
        }
        /// Get the value of the icmp_type field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_type(&self) -> IcmpType {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableTimeExceededPacket) -> u8 {
                let co = 0;
                (_self.packet[co] as u8)
            }
            IcmpType::new(get_arg0(&self))
        }
        /// Get the value of the icmp_code field
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_icmp_code(&self) -> IcmpCode {
            #[inline(always)]
            #[allow(trivial_numeric_casts, unused_parens)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn get_arg0(_self: &MutableTimeExceededPacket) -> u8 {
                let co = 1;
                (_self.packet[co] as u8)
            }
            IcmpCode::new(get_arg0(&self))
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
        /// Get the unused field. This field is always stored big-endian
        /// within the struct, but this accessor returns host order.
        #[inline]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn get_unused(&self) -> u32be {
            let _self = self;
            let co = 4;
            let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
            let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
            let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
            let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
            b0 | b1 | b2 | b3
        }
        /// Set the value of the icmp_type field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_type(&mut self, val: IcmpType) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableTimeExceededPacket, val: u8) {
                let co = 0;
                _self.packet[co + 0] = (val) as u8;
            }
            let vals = val.to_primitive_values();
            set_arg0(_self, vals.0);
        }
        /// Set the value of the icmp_code field.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_icmp_code(&mut self, val: IcmpCode) {
            use pnet_macros_support::packet::PrimitiveValues;
            let _self = self;
            #[inline]
            #[allow(trivial_numeric_casts)]
            #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
            fn set_arg0(_self: &mut MutableTimeExceededPacket, val: u8) {
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
        /// Set the unused field. This field is always stored big-endian
        /// within the struct, but this mutator wants host order.
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        pub fn set_unused(&mut self, val: u32be) {
            let _self = self;
            let co = 4;
            _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
            _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
            _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 3] = (val) as u8;
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
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     TimeExceededPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::PacketSize for
     MutableTimeExceededPacket<'a> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn packet_size(&self) -> usize { let _self = self; 8 }
    }
    impl <'a> ::pnet_macros_support::packet::MutablePacket for
     MutableTimeExceededPacket<'a> {
        #[inline]
        fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] {
            &mut self.packet[..]
        }
        #[inline]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
            let _self = self;
            let start = 8;
            if _self.packet.len() <= start { return &mut []; }
            &mut _self.packet[start..]
        }
    }
    impl <'a> ::pnet_macros_support::packet::Packet for
     MutableTimeExceededPacket<'a> {
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
    impl <'a> ::pnet_macros_support::packet::Packet for TimeExceededPacket<'a>
     {
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
    /// Used to iterate over a slice of `TimeExceededPacket`s
    pub struct TimeExceededIterable<'a> {
        buf: &'a [u8],
    }
    impl <'a> Iterator for TimeExceededIterable<'a> {
        type
        Item
        =
        TimeExceededPacket<'a>;
        fn next(&mut self) -> Option<TimeExceededPacket<'a>> {
            use pnet_macros_support::packet::PacketSize;
            use std::cmp::min;
            if self.buf.len() > 0 {
                if let Some(ret) = TimeExceededPacket::new(self.buf) {
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
     TimeExceededPacket<'p> {
        type
        T
        =
        TimeExceeded;
        #[inline]
        fn from_packet(&self) -> TimeExceeded {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            TimeExceeded{icmp_type: _self.get_icmp_type(),
                         icmp_code: _self.get_icmp_code(),
                         checksum: _self.get_checksum(),
                         unused: _self.get_unused(),
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
     MutableTimeExceededPacket<'p> {
        type
        T
        =
        TimeExceeded;
        #[inline]
        fn from_packet(&self) -> TimeExceeded {
            use pnet_macros_support::packet::Packet;
            let _self = self;
            TimeExceeded{icmp_type: _self.get_icmp_type(),
                         icmp_code: _self.get_icmp_code(),
                         checksum: _self.get_checksum(),
                         unused: _self.get_unused(),
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
    impl <'p> ::std::fmt::Debug for TimeExceededPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "TimeExceededPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, unused : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_unused (  ))
        }
    }
    impl <'p> ::std::fmt::Debug for MutableTimeExceededPacket<'p> {
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            let _self = self;
            write!(fmt ,
                   "MutableTimeExceededPacket {{ icmp_type : {:?}, icmp_code : {:?}, checksum : {:?}, unused : {:?},  }}"
                   , _self . get_icmp_type (  ) , _self . get_icmp_code (  ) ,
                   _self . get_checksum (  ) , _self . get_unused (  ))
        }
    }
    /// Represents an "echo request" ICMP packet.
    #[derive(Clone, Debug)]
    #[allow(unused_attributes)]
    pub struct TimeExceeded {
        pub icmp_type: IcmpType,
        pub icmp_code: IcmpCode,
        pub checksum: u16be,
        pub unused: u32be,
        pub payload: Vec<u8>,
    }
}
