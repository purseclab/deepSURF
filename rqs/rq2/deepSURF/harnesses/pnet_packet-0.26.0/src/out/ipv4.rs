// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PrimitiveValues;
use ip::IpNextHeaderProtocol;

use pnet_macros_support::types::*;

use std::net::Ipv4Addr;

/// The IPv4 header flags.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Ipv4Flags {
    use pnet_macros_support::types::*;

    /// Don't Fragment flag.
    pub const DontFragment: u3 = 0b010;
    /// More Fragments flag.
    pub const MoreFragments: u3 = 0b001;
}

/// IPv4 header options numbers as defined in
/// http://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod Ipv4OptionNumbers {
    use super::Ipv4OptionNumber;

    /// End of Options List.
    pub const EOL: Ipv4OptionNumber = Ipv4OptionNumber(0);

    /// No Operation.
    pub const NOP: Ipv4OptionNumber = Ipv4OptionNumber(1);

    /// Security.
    pub const SEC: Ipv4OptionNumber = Ipv4OptionNumber(2);

    /// Loose Source Route.
    pub const LSR: Ipv4OptionNumber = Ipv4OptionNumber(3);

    /// Time Stamp.
    pub const TS: Ipv4OptionNumber = Ipv4OptionNumber(4);

    /// Extended Security.
    pub const ESEC: Ipv4OptionNumber = Ipv4OptionNumber(5);

    /// Commercial Security.
    pub const CIPSO: Ipv4OptionNumber = Ipv4OptionNumber(6);

    /// Record Route.
    pub const RR: Ipv4OptionNumber = Ipv4OptionNumber(7);

    /// Stream ID.
    pub const SID: Ipv4OptionNumber = Ipv4OptionNumber(8);

    /// Strict Source Route.
    pub const SSR: Ipv4OptionNumber = Ipv4OptionNumber(9);

    /// Experimental Measurement.
    pub const ZSU: Ipv4OptionNumber = Ipv4OptionNumber(10);

    /// MTU Probe.
    pub const MTUP: Ipv4OptionNumber = Ipv4OptionNumber(11);

    /// MTU Reply.
    pub const MTUR: Ipv4OptionNumber = Ipv4OptionNumber(12);

    /// Experimental Flow Control.
    pub const FINN: Ipv4OptionNumber = Ipv4OptionNumber(13);

    /// Experimental Access Control.
    pub const VISA: Ipv4OptionNumber = Ipv4OptionNumber(14);

    /// ENCODE.
    pub const ENCODE: Ipv4OptionNumber = Ipv4OptionNumber(15);

    /// IMI Traffic Descriptor.
    pub const IMITD: Ipv4OptionNumber = Ipv4OptionNumber(16);

    /// Extended Internet Protocol.
    pub const EIP: Ipv4OptionNumber = Ipv4OptionNumber(17);

    /// Traceroute.
    pub const TR: Ipv4OptionNumber = Ipv4OptionNumber(18);

    /// Address Extension.
    pub const ADDEXT: Ipv4OptionNumber = Ipv4OptionNumber(19);

    /// Router Alert.
    pub const RTRALT: Ipv4OptionNumber = Ipv4OptionNumber(20);

    /// Selective Directed Broadcast.
    pub const SDB: Ipv4OptionNumber = Ipv4OptionNumber(21);

    /// Dynamic Packet State.
    pub const DPS: Ipv4OptionNumber = Ipv4OptionNumber(23);

    /// Upstream Multicast Pkt.
    pub const UMP: Ipv4OptionNumber = Ipv4OptionNumber(24);

    /// Quick-Start.
    pub const QS: Ipv4OptionNumber = Ipv4OptionNumber(25);

    /// RFC3692-style Experiment.
    pub const EXP: Ipv4OptionNumber = Ipv4OptionNumber(30);
}

/// Represents an IPv4 option.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct Ipv4OptionNumber(pub u8);

impl Ipv4OptionNumber {
    /// Create a new `Ipv4OptionNumber` instance.
    pub fn new(value: u8) -> Ipv4OptionNumber { Ipv4OptionNumber(value) }
}

impl PrimitiveValues for Ipv4OptionNumber {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}









// the header_length unit is the "word"
// - and a word is made of 4 bytes,
// - and the header length (without the options) is 5 words long



// just comparing with 0 is prone to false positives in this case.
// for instance if one forgets to set total_length, one always gets 0

// The length field is an optional field, using a Vec is a way to implement
// it

















 /* ver/ihl */
 /* dscp/ecn */
 /* total len */
 /* identification */
 /* flags/frag offset */
 /* ttl */
 /* proto */
 /* checksum */
 /* source ip */
/* dest ip */








 /* copy / class / number */
 /* length */
/* data */
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct Ipv4Packet<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableIpv4Packet<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> Ipv4Packet<'a> {
    /// Constructs a new Ipv4Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<Ipv4Packet<'p>> {
        if packet.len() >= Ipv4Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv4Packet{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new Ipv4Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the Ipv4Packet will
    /// own its own data and the underlying buffer will be dropped when the Ipv4Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<Ipv4Packet<'static>> {
        if packet.len() >= Ipv4Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv4Packet{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a Ipv4Packet to a Ipv4Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv4Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv4Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a Ipv4Packet to a Ipv4Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv4Packet<'a> {
        Ipv4Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 20 }
    /// The size (in bytes) of a Ipv4 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv4) -> usize {
        20 + _packet.options.len() + _packet.payload.len()
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the header_length field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_header_length(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 15)
    }
    /// Get the dscp field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_dscp(&self) -> u6 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u6) & 252) >> 2
    }
    /// Get the ecn field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ecn(&self) -> u2 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u2) & 3)
    }
    /// Get the total_length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_total_length(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the identification field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_identification(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the flags field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flags(&self) -> u3 {
        let _self = self;
        let co = 6;
        ((_self.packet[co] as u3) & 224) >> 5
    }
    /// Get the fragment_offset field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fragment_offset(&self) -> u13be {
        let _self = self;
        let co = 6;
        let b0 = (((_self.packet[co + 0] as u13be) & 31) << 8) as u13be;
        let b1 = ((_self.packet[co + 1] as u13be)) as u13be;
        b0 | b1
    }
    /// Get the ttl field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ttl(&self) -> u8 {
        let _self = self;
        let co = 8;
        (_self.packet[co] as u8)
    }
    /// Get the value of the next_level_protocol field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_level_protocol(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv4Packet) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 10;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv4Packet) -> u8 {
            let co = 12;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv4Packet) -> u8 {
            let co = 13;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv4Packet) -> u8 {
            let co = 14;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv4Packet) -> u8 {
            let co = 15;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv4Packet) -> u8 {
            let co = 16;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv4Packet) -> u8 {
            let co = 17;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv4Packet) -> u8 {
            let co = 18;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv4Packet) -> u8 {
            let co = 19;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the raw &[u8] value of the options field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options(&self) -> Vec<Ipv4Option> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        Ipv4OptionIterable{buf:
                               &_self.packet[current_offset..end],}.map(|packet|
                                                                            packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the options field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_iter(&self) -> Ipv4OptionIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        Ipv4OptionIterable{buf: &_self.packet[current_offset..end],}
    }
}
impl <'a> MutableIpv4Packet<'a> {
    /// Constructs a new MutableIpv4Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableIpv4Packet<'p>> {
        if packet.len() >= MutableIpv4Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv4Packet{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableIpv4Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableIpv4Packet will
    /// own its own data and the underlying buffer will be dropped when the MutableIpv4Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableIpv4Packet<'static>> {
        if packet.len() >= MutableIpv4Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv4Packet{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableIpv4Packet to a Ipv4Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv4Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv4Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableIpv4Packet to a Ipv4Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv4Packet<'a> {
        Ipv4Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 20 }
    /// The size (in bytes) of a Ipv4 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv4) -> usize {
        20 + _packet.options.len() + _packet.payload.len()
    }
    /// Populates a Ipv4Packet using a Ipv4 structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ipv4) {
        let _self = self;
        _self.set_version(packet.version);
        _self.set_header_length(packet.header_length);
        _self.set_dscp(packet.dscp);
        _self.set_ecn(packet.ecn);
        _self.set_total_length(packet.total_length);
        _self.set_identification(packet.identification);
        _self.set_flags(packet.flags);
        _self.set_fragment_offset(packet.fragment_offset);
        _self.set_ttl(packet.ttl);
        _self.set_next_level_protocol(packet.next_level_protocol);
        _self.set_checksum(packet.checksum);
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
        _self.set_options(&packet.options);
        _self.set_payload(&packet.payload);
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the header_length field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_header_length(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 15)
    }
    /// Get the dscp field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_dscp(&self) -> u6 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u6) & 252) >> 2
    }
    /// Get the ecn field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ecn(&self) -> u2 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u2) & 3)
    }
    /// Get the total_length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_total_length(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the identification field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_identification(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the flags field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flags(&self) -> u3 {
        let _self = self;
        let co = 6;
        ((_self.packet[co] as u3) & 224) >> 5
    }
    /// Get the fragment_offset field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fragment_offset(&self) -> u13be {
        let _self = self;
        let co = 6;
        let b0 = (((_self.packet[co + 0] as u13be) & 31) << 8) as u13be;
        let b1 = ((_self.packet[co + 1] as u13be)) as u13be;
        b0 | b1
    }
    /// Get the ttl field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ttl(&self) -> u8 {
        let _self = self;
        let co = 8;
        (_self.packet[co] as u8)
    }
    /// Get the value of the next_level_protocol field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_level_protocol(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv4Packet) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the checksum field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> u16be {
        let _self = self;
        let co = 10;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv4Packet) -> u8 {
            let co = 12;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv4Packet) -> u8 {
            let co = 13;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv4Packet) -> u8 {
            let co = 14;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv4Packet) -> u8 {
            let co = 15;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv4Packet) -> u8 {
            let co = 16;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv4Packet) -> u8 {
            let co = 17;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv4Packet) -> u8 {
            let co = 18;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv4Packet) -> u8 {
            let co = 19;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the raw &[u8] value of the options field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options(&self) -> Vec<Ipv4Option> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        Ipv4OptionIterable{buf:
                               &_self.packet[current_offset..end],}.map(|packet|
                                                                            packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the options field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_iter(&self) -> Ipv4OptionIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        Ipv4OptionIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Set the version field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_version(&mut self, val: u4) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 15) | (((val & 15) << 4) as u8)) as u8;
    }
    /// Set the header_length field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_header_length(&mut self, val: u4) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 15)) as u8)) as u8;
    }
    /// Set the dscp field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_dscp(&mut self, val: u6) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 3) | (((val & 63) << 2) as u8)) as u8;
    }
    /// Set the ecn field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ecn(&mut self, val: u2) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 252) | (((val & 3)) as u8)) as u8;
    }
    /// Set the total_length field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_total_length(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the identification field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_identification(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the flags field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_flags(&mut self, val: u3) {
        let _self = self;
        let co = 6;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 31) | (((val & 7) << 5) as u8)) as u8;
    }
    /// Set the fragment_offset field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_fragment_offset(&mut self, val: u13be) {
        let _self = self;
        let co = 6;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 224) | (((val & 7936) >> 8) as u8)) as
                u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the ttl field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ttl(&mut self, val: u8) {
        let _self = self;
        let co = 8;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the next_level_protocol field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_level_protocol(&mut self, val: IpNextHeaderProtocol) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 9;
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
        let co = 10;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the source field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: Ipv4Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 12;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 13;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 14;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 15;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
    }
    /// Set the value of the destination field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: Ipv4Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 16;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 17;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 18;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv4Packet, val: u8) {
            let co = 19;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
    }
    /// Get the raw &mut [u8] value of the options field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + ipv4_options_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_options(&mut self, vals: &[Ipv4Option]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset = 20;
        let end = current_offset + ipv4_options_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableIpv4OptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
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
        let current_offset = 20 + ipv4_options_length(&_self.to_immutable());
        let len = ipv4_payload_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for Ipv4Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        20 + ipv4_options_length(&_self.to_immutable()) +
            ipv4_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableIpv4Packet<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        20 + ipv4_options_length(&_self.to_immutable()) +
            ipv4_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableIpv4Packet<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 20 + ipv4_options_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(20 + ipv4_options_length(&_self.to_immutable()) +
                                ipv4_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableIpv4Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 20 + ipv4_options_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(20 + ipv4_options_length(&_self.to_immutable()) +
                                ipv4_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for Ipv4Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 20 + ipv4_options_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(20 + ipv4_options_length(&_self.to_immutable()) +
                                ipv4_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `Ipv4Packet`s
pub struct Ipv4Iterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for Ipv4Iterable<'a> {
    type
    Item
    =
    Ipv4Packet<'a>;
    fn next(&mut self) -> Option<Ipv4Packet<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = Ipv4Packet::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for Ipv4Packet<'p> {
    type
    T
    =
    Ipv4;
    #[inline]
    fn from_packet(&self) -> Ipv4 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv4{version: _self.get_version(),
             header_length: _self.get_header_length(),
             dscp: _self.get_dscp(),
             ecn: _self.get_ecn(),
             total_length: _self.get_total_length(),
             identification: _self.get_identification(),
             flags: _self.get_flags(),
             fragment_offset: _self.get_fragment_offset(),
             ttl: _self.get_ttl(),
             next_level_protocol: _self.get_next_level_protocol(),
             checksum: _self.get_checksum(),
             source: _self.get_source(),
             destination: _self.get_destination(),
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
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableIpv4Packet<'p>
 {
    type
    T
    =
    Ipv4;
    #[inline]
    fn from_packet(&self) -> Ipv4 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv4{version: _self.get_version(),
             header_length: _self.get_header_length(),
             dscp: _self.get_dscp(),
             ecn: _self.get_ecn(),
             total_length: _self.get_total_length(),
             identification: _self.get_identification(),
             flags: _self.get_flags(),
             fragment_offset: _self.get_fragment_offset(),
             ttl: _self.get_ttl(),
             next_level_protocol: _self.get_next_level_protocol(),
             checksum: _self.get_checksum(),
             source: _self.get_source(),
             destination: _self.get_destination(),
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
impl <'p> ::std::fmt::Debug for Ipv4Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "Ipv4Packet {{ version : {:?}, header_length : {:?}, dscp : {:?}, ecn : {:?}, total_length : {:?}, identification : {:?}, flags : {:?}, fragment_offset : {:?}, ttl : {:?}, next_level_protocol : {:?}, checksum : {:?}, source : {:?}, destination : {:?}, options : {:?},  }}"
               , _self . get_version (  ) , _self . get_header_length (  ) ,
               _self . get_dscp (  ) , _self . get_ecn (  ) , _self .
               get_total_length (  ) , _self . get_identification (  ) , _self
               . get_flags (  ) , _self . get_fragment_offset (  ) , _self .
               get_ttl (  ) , _self . get_next_level_protocol (  ) , _self .
               get_checksum (  ) , _self . get_source (  ) , _self .
               get_destination (  ) , _self . get_options (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIpv4Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIpv4Packet {{ version : {:?}, header_length : {:?}, dscp : {:?}, ecn : {:?}, total_length : {:?}, identification : {:?}, flags : {:?}, fragment_offset : {:?}, ttl : {:?}, next_level_protocol : {:?}, checksum : {:?}, source : {:?}, destination : {:?}, options : {:?},  }}"
               , _self . get_version (  ) , _self . get_header_length (  ) ,
               _self . get_dscp (  ) , _self . get_ecn (  ) , _self .
               get_total_length (  ) , _self . get_identification (  ) , _self
               . get_flags (  ) , _self . get_fragment_offset (  ) , _self .
               get_ttl (  ) , _self . get_next_level_protocol (  ) , _self .
               get_checksum (  ) , _self . get_source (  ) , _self .
               get_destination (  ) , _self . get_options (  ))
    }
}
/// Represents an IPv4 Packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Ipv4 {
    pub version: u4,
    pub header_length: u4,
    pub dscp: u6,
    pub ecn: u2,
    pub total_length: u16be,
    pub identification: u16be,
    pub flags: u3,
    pub fragment_offset: u13be,
    pub ttl: u8,
    pub next_level_protocol: IpNextHeaderProtocol,
    pub checksum: u16be,
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
    pub options: Vec<Ipv4Option>,
    pub payload: Vec<u8>,
}
/// Calculates a checksum of an IPv4 packet header.
/// The checksum field of the packet is regarded as zeros during the calculation.
pub fn checksum(packet: &Ipv4Packet) -> u16be {
    use Packet;
    use util;
    let min = Ipv4Packet::minimum_packet_size();
    let max = packet.packet().len();
    let header_length =
        match (packet.get_header_length() as usize) * 4 {
            length if length < min => min,
            length if length > max => max,
            length => length,
        };
    let data = &packet.packet()[..header_length];
    util::checksum(data, 5)
}
#[cfg(test)]
mod checksum_tests {
    use super::*;
    #[test]
    fn checksum_zeros() {
        let mut data = vec!(0 ; 20);
        let expected = 64255;
        let mut pkg = MutableIpv4Packet::new(&mut data[..]).unwrap();
        pkg.set_header_length(5);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
        pkg.set_checksum(123);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
    #[test]
    fn checksum_nonzero() {
        let mut data = vec!(255 ; 20);
        let expected = 2560;
        let mut pkg = MutableIpv4Packet::new(&mut data[..]).unwrap();
        pkg.set_header_length(5);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
        pkg.set_checksum(123);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
    #[test]
    fn checksum_too_small_header_length() {
        let mut data = vec!(148 ; 20);
        let expected = 51910;
        let mut pkg = MutableIpv4Packet::new(&mut data[..]).unwrap();
        pkg.set_header_length(0);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
    #[test]
    fn checksum_too_large_header_length() {
        let mut data = vec!(148 ; 20);
        let expected = 51142;
        let mut pkg = MutableIpv4Packet::new(&mut data[..]).unwrap();
        pkg.set_header_length(99);
        assert_eq!(checksum ( & pkg . to_immutable (  ) ) , expected);
    }
}
fn ipv4_options_length(ipv4: &Ipv4Packet) -> usize {
    ((ipv4.get_header_length() as usize) * 4).saturating_sub(20)
}
#[test]
fn ipv4_options_length_test() {
    let mut packet = [0u8; 20];
    let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
    ip_header.set_header_length(5);
    assert_eq!(ipv4_options_length ( & ip_header . to_immutable (  ) ) , 0);
}
fn ipv4_payload_length(ipv4: &Ipv4Packet) -> usize {
    (ipv4.get_total_length() as
         usize).saturating_sub((ipv4.get_header_length() as usize) * 4)
}
#[test]
fn ipv4_payload_length_test() {
    let mut packet = [0u8; 30];
    let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
    ip_header.set_header_length(5);
    ip_header.set_total_length(20);
    assert_eq!(ipv4_payload_length ( & ip_header . to_immutable (  ) ) , 0);
    ip_header.set_total_length(30);
    assert_eq!(ipv4_payload_length ( & ip_header . to_immutable (  ) ) , 10);
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct Ipv4OptionPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableIpv4OptionPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> Ipv4OptionPacket<'a> {
    /// Constructs a new Ipv4OptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<Ipv4OptionPacket<'p>> {
        if packet.len() >= Ipv4OptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv4OptionPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new Ipv4OptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the Ipv4OptionPacket will
    /// own its own data and the underlying buffer will be dropped when the Ipv4OptionPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<Ipv4OptionPacket<'static>> {
        if packet.len() >= Ipv4OptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv4OptionPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a Ipv4OptionPacket to a Ipv4OptionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv4OptionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv4OptionPacket{packet:
                             PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a Ipv4OptionPacket to a Ipv4OptionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv4OptionPacket<'a> {
        Ipv4OptionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 1 }
    /// The size (in bytes) of a Ipv4Option instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv4Option) -> usize {
        1 + _packet.length.len() + _packet.data.len()
    }
    /// Get the copied field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_copied(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the class field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_class(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 96) >> 5
    }
    /// Get the value of the number field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> Ipv4OptionNumber {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv4OptionPacket) -> u5 {
            let co = 0;
            ((_self.packet[co] as u5) & 31)
        }
        Ipv4OptionNumber::new(get_arg0(&self))
    }
    /// Get the raw &[u8] value of the length field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 1;
        let end =
            min(current_offset + ipv4_option_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the length field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> Vec<u8> {
        use std::cmp::min;
        let _self = self;
        let current_offset = 1;
        let pkt_len = self.packet.len();
        let end =
            min(current_offset + ipv4_option_length(&_self.to_immutable()),
                pkt_len);
        let packet = &_self.packet[current_offset..end];
        let mut vec: Vec<u8> = Vec::with_capacity(packet.len());
        let mut co = 0;
        for _ in 0..vec.capacity() {
            vec.push({ (packet[co] as u8) });
            co += 1;
        }
        vec
    }
}
impl <'a> MutableIpv4OptionPacket<'a> {
    /// Constructs a new MutableIpv4OptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableIpv4OptionPacket<'p>> {
        if packet.len() >= MutableIpv4OptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv4OptionPacket{packet:
                                             MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableIpv4OptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableIpv4OptionPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableIpv4OptionPacket is.
    pub fn owned(packet: Vec<u8>)
     -> Option<MutableIpv4OptionPacket<'static>> {
        if packet.len() >= MutableIpv4OptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv4OptionPacket{packet:
                                             MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableIpv4OptionPacket to a Ipv4OptionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv4OptionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv4OptionPacket{packet:
                             PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableIpv4OptionPacket to a Ipv4OptionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv4OptionPacket<'a> {
        Ipv4OptionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 1 }
    /// The size (in bytes) of a Ipv4Option instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv4Option) -> usize {
        1 + _packet.length.len() + _packet.data.len()
    }
    /// Populates a Ipv4OptionPacket using a Ipv4Option structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ipv4Option) {
        let _self = self;
        _self.set_copied(packet.copied);
        _self.set_class(packet.class);
        _self.set_number(packet.number);
        _self.set_length(&packet.length);
        _self.set_data(&packet.data);
    }
    /// Get the copied field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_copied(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the class field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_class(&self) -> u2 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u2) & 96) >> 5
    }
    /// Get the value of the number field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> Ipv4OptionNumber {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv4OptionPacket) -> u5 {
            let co = 0;
            ((_self.packet[co] as u5) & 31)
        }
        Ipv4OptionNumber::new(get_arg0(&self))
    }
    /// Get the raw &[u8] value of the length field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 1;
        let end =
            min(current_offset + ipv4_option_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the length field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length(&self) -> Vec<u8> {
        use std::cmp::min;
        let _self = self;
        let current_offset = 1;
        let pkt_len = self.packet.len();
        let end =
            min(current_offset + ipv4_option_length(&_self.to_immutable()),
                pkt_len);
        let packet = &_self.packet[current_offset..end];
        let mut vec: Vec<u8> = Vec::with_capacity(packet.len());
        let mut co = 0;
        for _ in 0..vec.capacity() {
            vec.push({ (packet[co] as u8) });
            co += 1;
        }
        vec
    }
    /// Set the copied field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_copied(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the class field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_class(&mut self, val: u2) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 159) | (((val & 3) << 5) as u8)) as u8;
    }
    /// Set the value of the number field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_number(&mut self, val: Ipv4OptionNumber) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv4OptionPacket, val: u5) {
            let co = 0;
            _self.packet[co + 0] =
                ((_self.packet[co + 0] & 224) | (((val & 31)) as u8)) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Get the raw &mut [u8] value of the length field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_length_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 1;
        let end =
            min(current_offset + ipv4_option_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the length field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_length(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 1;
        let len = ipv4_option_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
    /// Set the value of the data field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_data(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 1 + ipv4_option_length(&_self.to_immutable());
        let len = ipv4_option_payload_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for Ipv4OptionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1 + ipv4_option_length(&_self.to_immutable()) +
            ipv4_option_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableIpv4OptionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1 + ipv4_option_length(&_self.to_immutable()) +
            ipv4_option_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableIpv4OptionPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 1 + ipv4_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + ipv4_option_length(&_self.to_immutable()) +
                                ipv4_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for
 MutableIpv4OptionPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1 + ipv4_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + ipv4_option_length(&_self.to_immutable()) +
                                ipv4_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for Ipv4OptionPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1 + ipv4_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + ipv4_option_length(&_self.to_immutable()) +
                                ipv4_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `Ipv4OptionPacket`s
pub struct Ipv4OptionIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for Ipv4OptionIterable<'a> {
    type
    Item
    =
    Ipv4OptionPacket<'a>;
    fn next(&mut self) -> Option<Ipv4OptionPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = Ipv4OptionPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for Ipv4OptionPacket<'p> {
    type
    T
    =
    Ipv4Option;
    #[inline]
    fn from_packet(&self) -> Ipv4Option {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv4Option{copied: _self.get_copied(),
                   class: _self.get_class(),
                   number: _self.get_number(),
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
 MutableIpv4OptionPacket<'p> {
    type
    T
    =
    Ipv4Option;
    #[inline]
    fn from_packet(&self) -> Ipv4Option {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv4Option{copied: _self.get_copied(),
                   class: _self.get_class(),
                   number: _self.get_number(),
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
impl <'p> ::std::fmt::Debug for Ipv4OptionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "Ipv4OptionPacket {{ copied : {:?}, class : {:?}, number : {:?}, length : {:?},  }}"
               , _self . get_copied (  ) , _self . get_class (  ) , _self .
               get_number (  ) , _self . get_length (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIpv4OptionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIpv4OptionPacket {{ copied : {:?}, class : {:?}, number : {:?}, length : {:?},  }}"
               , _self . get_copied (  ) , _self . get_class (  ) , _self .
               get_number (  ) , _self . get_length (  ))
    }
}
/// Represents the IPv4 Option field.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Ipv4Option {
    copied: u1,
    class: u2,
    number: Ipv4OptionNumber,
    length: Vec<u8>,
    data: Vec<u8>,
}
/// This function gets the 'length' of the length field of the IPv4Option packet
/// Few options (EOL, NOP) are 1 bytes long, and then have a length field equal
/// to 0.
fn ipv4_option_length(option: &Ipv4OptionPacket) -> usize {
    match option.get_number() {
        Ipv4OptionNumbers::EOL => 0,
        Ipv4OptionNumbers::NOP => 0,
        _ => 1,
    }
}
fn ipv4_option_payload_length(ipv4_option: &Ipv4OptionPacket) -> usize {
    match ipv4_option.get_length().first() {
        Some(len) => (*len as usize) - 2,
        None => 0,
    }
}
#[test]
fn ipv4_packet_test() {
    use ip::IpNextHeaderProtocols;
    use Packet;
    use PacketSize;
    let mut packet = [0u8; 200];
    {
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_version(4);
        assert_eq!(ip_header . get_version (  ) , 4);
        ip_header.set_header_length(5);
        assert_eq!(ip_header . get_header_length (  ) , 5);
        ip_header.set_dscp(4);
        assert_eq!(ip_header . get_dscp (  ) , 4);
        ip_header.set_ecn(1);
        assert_eq!(ip_header . get_ecn (  ) , 1);
        ip_header.set_total_length(115);
        assert_eq!(ip_header . get_total_length (  ) , 115);
        assert_eq!(95 , ip_header . payload (  ) . len (  ));
        assert_eq!(ip_header . get_total_length (  ) , ip_header . packet_size
                   (  ) as u16);
        ip_header.set_identification(257);
        assert_eq!(ip_header . get_identification (  ) , 257);
        ip_header.set_flags(Ipv4Flags::DontFragment);
        assert_eq!(ip_header . get_flags (  ) , 2);
        ip_header.set_fragment_offset(257);
        assert_eq!(ip_header . get_fragment_offset (  ) , 257);
        ip_header.set_ttl(64);
        assert_eq!(ip_header . get_ttl (  ) , 64);
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Udp);
        assert_eq!(ip_header . get_next_level_protocol (  ) ,
                   IpNextHeaderProtocols :: Udp);
        ip_header.set_source(Ipv4Addr::new(192, 168, 0, 1));
        assert_eq!(ip_header . get_source (  ) , Ipv4Addr :: new (
                   192 , 168 , 0 , 1 ));
        ip_header.set_destination(Ipv4Addr::new(192, 168, 0, 199));
        assert_eq!(ip_header . get_destination (  ) , Ipv4Addr :: new (
                   192 , 168 , 0 , 199 ));
        let imm_header = checksum(&ip_header.to_immutable());
        ip_header.set_checksum(imm_header);
        assert_eq!(ip_header . get_checksum (  ) , 0xb64e);
    }
    let ref_packet =
        [69, 17, 0, 115, 1, 1, 65, 1, 64, 17, 182, 78, 192, 168, 0, 1, 192,
         168, 0, 199];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ref_packet . len (  ) ]);
}
#[test]
fn ipv4_packet_option_test() {
    let mut packet = [0u8; 3];
    {
        let mut ipv4_options =
            MutableIpv4OptionPacket::new(&mut packet[..]).unwrap();
        ipv4_options.set_copied(1);
        assert_eq!(ipv4_options . get_copied (  ) , 1);
        ipv4_options.set_class(0);
        assert_eq!(ipv4_options . get_class (  ) , 0);
        ipv4_options.set_number(Ipv4OptionNumber(3));
        assert_eq!(ipv4_options . get_number (  ) , Ipv4OptionNumbers :: LSR);
        ipv4_options.set_length(&vec!(3));
        assert_eq!(ipv4_options . get_length (  ) , vec ! [ 3 ]);
        ipv4_options.set_data(&vec!(16));
    }
    let ref_packet = [131, 3, 16];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
