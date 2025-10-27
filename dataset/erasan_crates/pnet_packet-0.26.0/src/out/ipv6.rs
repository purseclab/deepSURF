// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use ip::IpNextHeaderProtocol;

use pnet_macros_support::types::*;

use std::net::Ipv6Addr;




















































 /* ver/traffic class */
 /* traffic class/flow label */
 /* flow label */
 /* payload length */
 /* next header */
 /* hop limit */
 /* source ip */
/* dest ip */
/* Hop-by-Hop Options */
 // Next Header
 // Hdr Ext Len
/* Destination Options */
 // Next Header
 // Hdr Ext Len
/* Routing */
 // Next Header
 // Hdr Ext Len
 // Routing Type
 // Segments Left
/* Fragment */
 // Next Header
 // Reserved
 // Fragment Offset
 // Identification
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct Ipv6Packet<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableIpv6Packet<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> Ipv6Packet<'a> {
    /// Constructs a new Ipv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<Ipv6Packet<'p>> {
        if packet.len() >= Ipv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv6Packet{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new Ipv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the Ipv6Packet will
    /// own its own data and the underlying buffer will be dropped when the Ipv6Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<Ipv6Packet<'static>> {
        if packet.len() >= Ipv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(Ipv6Packet{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a Ipv6Packet to a Ipv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv6Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv6Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a Ipv6Packet to a Ipv6Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv6Packet<'a> {
        Ipv6Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 40 }
    /// The size (in bytes) of a Ipv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv6) -> usize { 40 + _packet.payload.len() }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u4 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the traffic_class field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_traffic_class(&self) -> u8 {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u8) & 15) << 4) as u8;
        let b1 = (((_self.packet[co + 1] as u8) & 240) >> 4) as u8;
        b0 | b1
    }
    /// Get the flow_label field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flow_label(&self) -> u20be {
        let _self = self;
        let co = 1;
        let b0 = (((_self.packet[co + 0] as u20be) & 15) << 16) as u20be;
        let b1 = ((_self.packet[co + 1] as u20be) << 8) as u20be;
        let b2 = ((_self.packet[co + 2] as u20be)) as u20be;
        b0 | b1 | b2
    }
    /// Get the payload_length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hop_limit field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hop_limit(&self) -> u8 {
        let _self = self;
        let co = 7;
        (_self.packet[co] as u8)
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv6Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u16 {
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv6Packet) -> u16 {
            let co = 10;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv6Packet) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv6Packet) -> u16 {
            let co = 14;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &Ipv6Packet) -> u16 {
            let co = 16;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &Ipv6Packet) -> u16 {
            let co = 18;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &Ipv6Packet) -> u16 {
            let co = 20;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &Ipv6Packet) -> u16 {
            let co = 22;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv6Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &Ipv6Packet) -> u16 {
            let co = 24;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &Ipv6Packet) -> u16 {
            let co = 26;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &Ipv6Packet) -> u16 {
            let co = 28;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &Ipv6Packet) -> u16 {
            let co = 30;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &Ipv6Packet) -> u16 {
            let co = 32;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &Ipv6Packet) -> u16 {
            let co = 34;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &Ipv6Packet) -> u16 {
            let co = 36;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &Ipv6Packet) -> u16 {
            let co = 38;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
}
impl <'a> MutableIpv6Packet<'a> {
    /// Constructs a new MutableIpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableIpv6Packet<'p>> {
        if packet.len() >= MutableIpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv6Packet{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableIpv6Packet. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableIpv6Packet will
    /// own its own data and the underlying buffer will be dropped when the MutableIpv6Packet is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableIpv6Packet<'static>> {
        if packet.len() >= MutableIpv6Packet::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableIpv6Packet{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableIpv6Packet to a Ipv6Packet
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> Ipv6Packet<'p> {
        use ::pnet_macros_support::packet::PacketData;
        Ipv6Packet{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableIpv6Packet to a Ipv6Packet while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> Ipv6Packet<'a> {
        Ipv6Packet{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 40 }
    /// The size (in bytes) of a Ipv6 instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ipv6) -> usize { 40 + _packet.payload.len() }
    /// Populates a Ipv6Packet using a Ipv6 structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ipv6) {
        let _self = self;
        _self.set_version(packet.version);
        _self.set_traffic_class(packet.traffic_class);
        _self.set_flow_label(packet.flow_label);
        _self.set_payload_length(packet.payload_length);
        _self.set_next_header(packet.next_header);
        _self.set_hop_limit(packet.hop_limit);
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
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
    /// Get the traffic_class field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_traffic_class(&self) -> u8 {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u8) & 15) << 4) as u8;
        let b1 = (((_self.packet[co + 1] as u8) & 240) >> 4) as u8;
        b0 | b1
    }
    /// Get the flow_label field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flow_label(&self) -> u20be {
        let _self = self;
        let co = 1;
        let b0 = (((_self.packet[co + 0] as u20be) & 15) << 16) as u20be;
        let b1 = ((_self.packet[co + 1] as u20be) << 8) as u20be;
        let b2 = ((_self.packet[co + 2] as u20be)) as u20be;
        b0 | b1 | b2
    }
    /// Get the payload_length field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_payload_length(&self) -> u16be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hop_limit field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hop_limit(&self) -> u8 {
        let _self = self;
        let co = 7;
        (_self.packet[co] as u8)
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> Ipv6Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u16 {
            let co = 8;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv6Packet) -> u16 {
            let co = 10;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv6Packet) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv6Packet) -> u16 {
            let co = 14;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableIpv6Packet) -> u16 {
            let co = 16;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableIpv6Packet) -> u16 {
            let co = 18;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &MutableIpv6Packet) -> u16 {
            let co = 20;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &MutableIpv6Packet) -> u16 {
            let co = 22;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> Ipv6Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableIpv6Packet) -> u16 {
            let co = 24;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableIpv6Packet) -> u16 {
            let co = 26;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableIpv6Packet) -> u16 {
            let co = 28;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableIpv6Packet) -> u16 {
            let co = 30;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableIpv6Packet) -> u16 {
            let co = 32;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableIpv6Packet) -> u16 {
            let co = 34;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg6(_self: &MutableIpv6Packet) -> u16 {
            let co = 36;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg7(_self: &MutableIpv6Packet) -> u16 {
            let co = 38;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        Ipv6Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self), get_arg4(&self), get_arg5(&self),
                      get_arg6(&self), get_arg7(&self))
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
    /// Set the traffic_class field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_traffic_class(&mut self, val: u8) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 240) >> 4) as u8)) as u8;
        _self.packet[co + 1] =
            ((_self.packet[co + 1] & 15) | (((val & 15) << 4) as u8)) as u8;
    }
    /// Set the flow_label field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_flow_label(&mut self, val: u20be) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 983040) >> 16) as u8)) as
                u8;
        _self.packet[co + 1] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 2] = (val) as u8;
    }
    /// Set the payload_length field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload_length(&mut self, val: u16be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the next_header field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_header(&mut self, val: IpNextHeaderProtocol) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u8) {
            let co = 6;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the hop_limit field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hop_limit(&mut self, val: u8) {
        let _self = self;
        let co = 7;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the source field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: Ipv6Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 8;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 10;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 12;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 14;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 16;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 18;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg6(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 20;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg7(_self: &mut MutableIpv6Packet, val: u16) {
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
    /// Set the value of the destination field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: Ipv6Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 24;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 26;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 28;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 30;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 32;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 34;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg6(_self: &mut MutableIpv6Packet, val: u16) {
            let co = 36;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg7(_self: &mut MutableIpv6Packet, val: u16) {
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
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 40;
        let len = _self.get_payload_length() as usize;
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for Ipv6Packet<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        40 + (_self.get_payload_length() as usize)
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableIpv6Packet<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        40 + (_self.get_payload_length() as usize)
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableIpv6Packet<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 40;
        let end =
            ::std::cmp::min(40 + (_self.get_payload_length() as usize),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableIpv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 40;
        let end =
            ::std::cmp::min(40 + (_self.get_payload_length() as usize),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for Ipv6Packet<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 40;
        let end =
            ::std::cmp::min(40 + (_self.get_payload_length() as usize),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `Ipv6Packet`s
pub struct Ipv6Iterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for Ipv6Iterable<'a> {
    type
    Item
    =
    Ipv6Packet<'a>;
    fn next(&mut self) -> Option<Ipv6Packet<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = Ipv6Packet::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for Ipv6Packet<'p> {
    type
    T
    =
    Ipv6;
    #[inline]
    fn from_packet(&self) -> Ipv6 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv6{version: _self.get_version(),
             traffic_class: _self.get_traffic_class(),
             flow_label: _self.get_flow_label(),
             payload_length: _self.get_payload_length(),
             next_header: _self.get_next_header(),
             hop_limit: _self.get_hop_limit(),
             source: _self.get_source(),
             destination: _self.get_destination(),
             payload:
                 {
                     let payload = self.payload();
                     let mut vec = Vec::with_capacity(payload.len());
                     vec.extend_from_slice(payload);
                     vec
                 },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableIpv6Packet<'p>
 {
    type
    T
    =
    Ipv6;
    #[inline]
    fn from_packet(&self) -> Ipv6 {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ipv6{version: _self.get_version(),
             traffic_class: _self.get_traffic_class(),
             flow_label: _self.get_flow_label(),
             payload_length: _self.get_payload_length(),
             next_header: _self.get_next_header(),
             hop_limit: _self.get_hop_limit(),
             source: _self.get_source(),
             destination: _self.get_destination(),
             payload:
                 {
                     let payload = self.payload();
                     let mut vec = Vec::with_capacity(payload.len());
                     vec.extend_from_slice(payload);
                     vec
                 },}
    }
}
impl <'p> ::std::fmt::Debug for Ipv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "Ipv6Packet {{ version : {:?}, traffic_class : {:?}, flow_label : {:?}, payload_length : {:?}, next_header : {:?}, hop_limit : {:?}, source : {:?}, destination : {:?},  }}"
               , _self . get_version (  ) , _self . get_traffic_class (  ) ,
               _self . get_flow_label (  ) , _self . get_payload_length (  ) ,
               _self . get_next_header (  ) , _self . get_hop_limit (  ) ,
               _self . get_source (  ) , _self . get_destination (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableIpv6Packet<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableIpv6Packet {{ version : {:?}, traffic_class : {:?}, flow_label : {:?}, payload_length : {:?}, next_header : {:?}, hop_limit : {:?}, source : {:?}, destination : {:?},  }}"
               , _self . get_version (  ) , _self . get_traffic_class (  ) ,
               _self . get_flow_label (  ) , _self . get_payload_length (  ) ,
               _self . get_next_header (  ) , _self . get_hop_limit (  ) ,
               _self . get_source (  ) , _self . get_destination (  ))
    }
}
/// Represents an IPv6 Packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Ipv6 {
    pub version: u4,
    pub traffic_class: u8,
    pub flow_label: u20be,
    pub payload_length: u16be,
    pub next_header: IpNextHeaderProtocol,
    pub hop_limit: u8,
    pub source: Ipv6Addr,
    pub destination: Ipv6Addr,
    pub payload: Vec<u8>,
}
impl <'p> ExtensionIterable<'p> {
    pub fn new(buf: &[u8]) -> ExtensionIterable {
        ExtensionIterable{buf: buf,}
    }
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct ExtensionPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableExtensionPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> ExtensionPacket<'a> {
    /// Constructs a new ExtensionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<ExtensionPacket<'p>> {
        if packet.len() >= ExtensionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(ExtensionPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new ExtensionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the ExtensionPacket will
    /// own its own data and the underlying buffer will be dropped when the ExtensionPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<ExtensionPacket<'static>> {
        if packet.len() >= ExtensionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(ExtensionPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a ExtensionPacket to a ExtensionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> ExtensionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        ExtensionPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a ExtensionPacket to a ExtensionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> ExtensionPacket<'a> {
        ExtensionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 2 }
    /// The size (in bytes) of a Extension instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Extension) -> usize {
        2 + _packet.options.len()
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ExtensionPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hdr_ext_len(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
}
impl <'a> MutableExtensionPacket<'a> {
    /// Constructs a new MutableExtensionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableExtensionPacket<'p>> {
        if packet.len() >= MutableExtensionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableExtensionPacket{packet:
                                            MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableExtensionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableExtensionPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableExtensionPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableExtensionPacket<'static>> {
        if packet.len() >= MutableExtensionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableExtensionPacket{packet:
                                            MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableExtensionPacket to a ExtensionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> ExtensionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        ExtensionPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableExtensionPacket to a ExtensionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> ExtensionPacket<'a> {
        ExtensionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 2 }
    /// The size (in bytes) of a Extension instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Extension) -> usize {
        2 + _packet.options.len()
    }
    /// Populates a ExtensionPacket using a Extension structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Extension) {
        let _self = self;
        _self.set_next_header(packet.next_header);
        _self.set_hdr_ext_len(packet.hdr_ext_len);
        _self.set_options(&packet.options);
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableExtensionPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hdr_ext_len(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Set the value of the next_header field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_header(&mut self, val: IpNextHeaderProtocol) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableExtensionPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hdr_ext_len(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_options(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 2;
        let len = ipv6_extension_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for ExtensionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        2 + ipv6_extension_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableExtensionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        2 + ipv6_extension_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableExtensionPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 2;
        let end =
            ::std::cmp::min(2 + ipv6_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableExtensionPacket<'a>
 {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        let end =
            ::std::cmp::min(2 + ipv6_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for ExtensionPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        let end =
            ::std::cmp::min(2 + ipv6_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `ExtensionPacket`s
pub struct ExtensionIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for ExtensionIterable<'a> {
    type
    Item
    =
    ExtensionPacket<'a>;
    fn next(&mut self) -> Option<ExtensionPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = ExtensionPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for ExtensionPacket<'p> {
    type
    T
    =
    Extension;
    #[inline]
    fn from_packet(&self) -> Extension {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Extension{next_header: _self.get_next_header(),
                  hdr_ext_len: _self.get_hdr_ext_len(),
                  options:
                      {
                          let payload = self.payload();
                          let mut vec = Vec::with_capacity(payload.len());
                          vec.extend_from_slice(payload);
                          vec
                      },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for
 MutableExtensionPacket<'p> {
    type
    T
    =
    Extension;
    #[inline]
    fn from_packet(&self) -> Extension {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Extension{next_header: _self.get_next_header(),
                  hdr_ext_len: _self.get_hdr_ext_len(),
                  options:
                      {
                          let payload = self.payload();
                          let mut vec = Vec::with_capacity(payload.len());
                          vec.extend_from_slice(payload);
                          vec
                      },}
    }
}
impl <'p> ::std::fmt::Debug for ExtensionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "ExtensionPacket {{ next_header : {:?}, hdr_ext_len : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_hdr_ext_len (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableExtensionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableExtensionPacket {{ next_header : {:?}, hdr_ext_len : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_hdr_ext_len (  ))
    }
}
/// Represents an IPv6 Extension.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Extension {
    pub next_header: IpNextHeaderProtocol,
    pub hdr_ext_len: u8,
    pub options: Vec<u8>,
}
fn ipv6_extension_length(ext: &ExtensionPacket) -> usize {
    (ext.get_hdr_ext_len() as usize) * 8 + 8 - 2
}
/// Represents an IPv6 Hop-by-Hop Options.
pub type HopByHop = Extension;
/// A structure enabling manipulation of on the wire packets.
pub type HopByHopPacket<'p> = ExtensionPacket<'p>;
/// A structure enabling manipulation of on the wire packets.
pub type MutableHopByHopPacket<'p> = MutableExtensionPacket<'p>;
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct RoutingPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableRoutingPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> RoutingPacket<'a> {
    /// Constructs a new RoutingPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<RoutingPacket<'p>> {
        if packet.len() >= RoutingPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(RoutingPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new RoutingPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the RoutingPacket will
    /// own its own data and the underlying buffer will be dropped when the RoutingPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<RoutingPacket<'static>> {
        if packet.len() >= RoutingPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(RoutingPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a RoutingPacket to a RoutingPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RoutingPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        RoutingPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a RoutingPacket to a RoutingPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RoutingPacket<'a> {
        RoutingPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Routing instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Routing) -> usize { 4 + _packet.data.len() }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &RoutingPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hdr_ext_len(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the routing_type field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_type(&self) -> u8 {
        let _self = self;
        let co = 2;
        (_self.packet[co] as u8)
    }
    /// Get the segments_left field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_segments_left(&self) -> u8 {
        let _self = self;
        let co = 3;
        (_self.packet[co] as u8)
    }
}
impl <'a> MutableRoutingPacket<'a> {
    /// Constructs a new MutableRoutingPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableRoutingPacket<'p>> {
        if packet.len() >= MutableRoutingPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableRoutingPacket{packet:
                                          MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableRoutingPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableRoutingPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableRoutingPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableRoutingPacket<'static>> {
        if packet.len() >= MutableRoutingPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableRoutingPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableRoutingPacket to a RoutingPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> RoutingPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        RoutingPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableRoutingPacket to a RoutingPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> RoutingPacket<'a> {
        RoutingPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Routing instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Routing) -> usize { 4 + _packet.data.len() }
    /// Populates a RoutingPacket using a Routing structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Routing) {
        let _self = self;
        _self.set_next_header(packet.next_header);
        _self.set_hdr_ext_len(packet.hdr_ext_len);
        _self.set_routing_type(packet.routing_type);
        _self.set_segments_left(packet.segments_left);
        _self.set_data(&packet.data);
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableRoutingPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hdr_ext_len(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the routing_type field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_type(&self) -> u8 {
        let _self = self;
        let co = 2;
        (_self.packet[co] as u8)
    }
    /// Get the segments_left field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_segments_left(&self) -> u8 {
        let _self = self;
        let co = 3;
        (_self.packet[co] as u8)
    }
    /// Set the value of the next_header field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_header(&mut self, val: IpNextHeaderProtocol) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableRoutingPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the hdr_ext_len field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hdr_ext_len(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the routing_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_routing_type(&mut self, val: u8) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the segments_left field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_segments_left(&mut self, val: u8) {
        let _self = self;
        let co = 3;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the data field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_data(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 4;
        let len = routing_extension_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for RoutingPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4 + routing_extension_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableRoutingPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4 + routing_extension_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableRoutingPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 4;
        let end =
            ::std::cmp::min(4 +
                                routing_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableRoutingPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        let end =
            ::std::cmp::min(4 +
                                routing_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for RoutingPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        let end =
            ::std::cmp::min(4 +
                                routing_extension_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `RoutingPacket`s
pub struct RoutingIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for RoutingIterable<'a> {
    type
    Item
    =
    RoutingPacket<'a>;
    fn next(&mut self) -> Option<RoutingPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = RoutingPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for RoutingPacket<'p> {
    type
    T
    =
    Routing;
    #[inline]
    fn from_packet(&self) -> Routing {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Routing{next_header: _self.get_next_header(),
                hdr_ext_len: _self.get_hdr_ext_len(),
                routing_type: _self.get_routing_type(),
                segments_left: _self.get_segments_left(),
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
 MutableRoutingPacket<'p> {
    type
    T
    =
    Routing;
    #[inline]
    fn from_packet(&self) -> Routing {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Routing{next_header: _self.get_next_header(),
                hdr_ext_len: _self.get_hdr_ext_len(),
                routing_type: _self.get_routing_type(),
                segments_left: _self.get_segments_left(),
                data:
                    {
                        let payload = self.payload();
                        let mut vec = Vec::with_capacity(payload.len());
                        vec.extend_from_slice(payload);
                        vec
                    },}
    }
}
impl <'p> ::std::fmt::Debug for RoutingPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "RoutingPacket {{ next_header : {:?}, hdr_ext_len : {:?}, routing_type : {:?}, segments_left : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_hdr_ext_len (  ) ,
               _self . get_routing_type (  ) , _self . get_segments_left (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableRoutingPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableRoutingPacket {{ next_header : {:?}, hdr_ext_len : {:?}, routing_type : {:?}, segments_left : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_hdr_ext_len (  ) ,
               _self . get_routing_type (  ) , _self . get_segments_left (  ))
    }
}
/// Represents an IPv6 Routing Extension.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Routing {
    pub next_header: IpNextHeaderProtocol,
    pub hdr_ext_len: u8,
    pub routing_type: u8,
    pub segments_left: u8,
    pub data: Vec<u8>,
}
fn routing_extension_length(ext: &RoutingPacket) -> usize {
    (ext.get_hdr_ext_len() as usize) * 8 + 8 - 4
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct FragmentPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableFragmentPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> FragmentPacket<'a> {
    /// Constructs a new FragmentPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<FragmentPacket<'p>> {
        if packet.len() >= FragmentPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(FragmentPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new FragmentPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the FragmentPacket will
    /// own its own data and the underlying buffer will be dropped when the FragmentPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<FragmentPacket<'static>> {
        if packet.len() >= FragmentPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(FragmentPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a FragmentPacket to a FragmentPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> FragmentPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        FragmentPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a FragmentPacket to a FragmentPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> FragmentPacket<'a> {
        FragmentPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Fragment instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Fragment) -> usize {
        8 + _packet.payload.len()
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &FragmentPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the fragment_offset_with_flags field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fragment_offset_with_flags(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the id field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_id(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl <'a> MutableFragmentPacket<'a> {
    /// Constructs a new MutableFragmentPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableFragmentPacket<'p>> {
        if packet.len() >= MutableFragmentPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableFragmentPacket{packet:
                                           MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableFragmentPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableFragmentPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableFragmentPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableFragmentPacket<'static>> {
        if packet.len() >= MutableFragmentPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableFragmentPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableFragmentPacket to a FragmentPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> FragmentPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        FragmentPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableFragmentPacket to a FragmentPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> FragmentPacket<'a> {
        FragmentPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 8 }
    /// The size (in bytes) of a Fragment instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Fragment) -> usize {
        8 + _packet.payload.len()
    }
    /// Populates a FragmentPacket using a Fragment structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Fragment) {
        let _self = self;
        _self.set_next_header(packet.next_header);
        _self.set_reserved(packet.reserved);
        _self.set_fragment_offset_with_flags(packet.fragment_offset_with_flags);
        _self.set_id(packet.id);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the next_header field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_next_header(&self) -> IpNextHeaderProtocol {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableFragmentPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        IpNextHeaderProtocol::new(get_arg0(&self))
    }
    /// Get the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved(&self) -> u8 {
        let _self = self;
        let co = 1;
        (_self.packet[co] as u8)
    }
    /// Get the fragment_offset_with_flags field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_fragment_offset_with_flags(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the id field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_id(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Set the value of the next_header field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_next_header(&mut self, val: IpNextHeaderProtocol) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableFragmentPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_reserved(&mut self, val: u8) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the fragment_offset_with_flags field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_fragment_offset_with_flags(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the id field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_id(&mut self, val: u32be) {
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
        let len = 0;
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for FragmentPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 + 0 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableFragmentPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 8 + 0 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableFragmentPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 8;
        let end = ::std::cmp::min(8 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableFragmentPacket<'a>
 {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        let end = ::std::cmp::min(8 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for FragmentPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 8;
        let end = ::std::cmp::min(8 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `FragmentPacket`s
pub struct FragmentIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for FragmentIterable<'a> {
    type
    Item
    =
    FragmentPacket<'a>;
    fn next(&mut self) -> Option<FragmentPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = FragmentPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for FragmentPacket<'p> {
    type
    T
    =
    Fragment;
    #[inline]
    fn from_packet(&self) -> Fragment {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Fragment{next_header: _self.get_next_header(),
                 reserved: _self.get_reserved(),
                 fragment_offset_with_flags:
                     _self.get_fragment_offset_with_flags(),
                 id: _self.get_id(),
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
 MutableFragmentPacket<'p> {
    type
    T
    =
    Fragment;
    #[inline]
    fn from_packet(&self) -> Fragment {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Fragment{next_header: _self.get_next_header(),
                 reserved: _self.get_reserved(),
                 fragment_offset_with_flags:
                     _self.get_fragment_offset_with_flags(),
                 id: _self.get_id(),
                 payload:
                     {
                         let payload = self.payload();
                         let mut vec = Vec::with_capacity(payload.len());
                         vec.extend_from_slice(payload);
                         vec
                     },}
    }
}
impl <'p> ::std::fmt::Debug for FragmentPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "FragmentPacket {{ next_header : {:?}, reserved : {:?}, fragment_offset_with_flags : {:?}, id : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_reserved (  ) ,
               _self . get_fragment_offset_with_flags (  ) , _self . get_id (
               ))
    }
}
impl <'p> ::std::fmt::Debug for MutableFragmentPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableFragmentPacket {{ next_header : {:?}, reserved : {:?}, fragment_offset_with_flags : {:?}, id : {:?},  }}"
               , _self . get_next_header (  ) , _self . get_reserved (  ) ,
               _self . get_fragment_offset_with_flags (  ) , _self . get_id (
               ))
    }
}
/// Represents an IPv6 Fragment Extension.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Fragment {
    pub next_header: IpNextHeaderProtocol,
    pub reserved: u8,
    pub fragment_offset_with_flags: u16be,
    pub id: u32be,
    pub payload: Vec<u8>,
}
const FRAGMENT_FLAGS_MASK: u16 = 3;
const FRAGMENT_FLAGS_MORE_FRAGMENTS: u16 = 1;
const FRAGMENT_OFFSET_MASK: u16 = !FRAGMENT_FLAGS_MASK;
impl <'p> FragmentPacket<'p> {
    pub fn get_fragment_offset(&self) -> u16 {
        self.get_fragment_offset_with_flags() & FRAGMENT_OFFSET_MASK
    }
    pub fn is_last_fragment(&self) -> bool {
        (self.get_fragment_offset_with_flags() &
             FRAGMENT_FLAGS_MORE_FRAGMENTS) == 0
    }
}
impl <'p> MutableFragmentPacket<'p> {
    pub fn get_fragment_offset(&self) -> u16 {
        self.get_fragment_offset_with_flags() & FRAGMENT_OFFSET_MASK
    }
    pub fn is_last_fragment(&self) -> bool {
        (self.get_fragment_offset_with_flags() &
             FRAGMENT_FLAGS_MORE_FRAGMENTS) == 0
    }
    pub fn set_fragment_offset(&mut self, offset: u16) {
        let fragment_offset_with_flags =
            self.get_fragment_offset_with_flags();
        self.set_fragment_offset_with_flags((offset & FRAGMENT_OFFSET_MASK) |
                                                (fragment_offset_with_flags &
                                                     FRAGMENT_FLAGS_MASK));
    }
    pub fn set_last_fragment(&mut self, is_last: bool) {
        let fragment_offset_with_flags =
            self.get_fragment_offset_with_flags();
        self.set_fragment_offset_with_flags(if is_last {
                                                fragment_offset_with_flags &
                                                    !FRAGMENT_FLAGS_MORE_FRAGMENTS
                                            } else {
                                                fragment_offset_with_flags |
                                                    FRAGMENT_FLAGS_MORE_FRAGMENTS
                                            });
    }
}
/// Represents an Destination Options.
pub type Destination = Extension;
/// A structure enabling manipulation of on the wire packets.
pub type DestinationPacket<'p> = ExtensionPacket<'p>;
/// A structure enabling manipulation of on the wire packets.
pub type MutableDestinationPacket<'p> = MutableExtensionPacket<'p>;
#[test]
fn ipv6_header_test() {
    use ip::IpNextHeaderProtocols;
    use {MutablePacket, Packet, PacketSize};
    let mut packet = [0u8; 512];
    {
        let mut ip_header = MutableIpv6Packet::new(&mut packet[..]).unwrap();
        ip_header.set_version(6);
        assert_eq!(ip_header . get_version (  ) , 6);
        ip_header.set_traffic_class(17);
        assert_eq!(ip_header . get_traffic_class (  ) , 17);
        ip_header.set_flow_label(65793);
        assert_eq!(ip_header . get_flow_label (  ) , 0x10101);
        ip_header.set_payload_length(257);
        assert_eq!(ip_header . get_payload_length (  ) , 0x0101);
        assert_eq!(0x0101 , ip_header . payload (  ) . len (  ));
        ip_header.set_next_header(IpNextHeaderProtocols::Hopopt);
        assert_eq!(ip_header . get_next_header (  ) , IpNextHeaderProtocols ::
                   Hopopt);
        ip_header.set_hop_limit(1);
        assert_eq!(ip_header . get_hop_limit (  ) , 1);
        let source =
            Ipv6Addr::new(272, 4097, 272, 4097, 272, 4097, 272, 4097);
        ip_header.set_source(source);
        assert_eq!(ip_header . get_source (  ) , source);
        let dest = Ipv6Addr::new(272, 4097, 272, 4097, 272, 4097, 272, 4097);
        ip_header.set_destination(dest);
        assert_eq!(ip_header . get_destination (  ) , dest);
        let mut pos =
            {
                let mut hopopt =
                    MutableHopByHopPacket::new(ip_header.payload_mut()).unwrap();
                hopopt.set_next_header(IpNextHeaderProtocols::Ipv6Opts);
                assert_eq!(hopopt . get_next_header (  ) ,
                           IpNextHeaderProtocols :: Ipv6Opts);
                hopopt.set_hdr_ext_len(1);
                assert_eq!(hopopt . get_hdr_ext_len (  ) , 1);
                hopopt.set_options(&[b'A'; 14][..]);
                assert_eq!(hopopt . payload (  ) , b"AAAAAAAAAAAAAA");
                hopopt.packet_size()
            };
        pos +=
            {
                let mut dstopt =
                    MutableDestinationPacket::new(&mut ip_header.payload_mut()[pos..]).unwrap();
                dstopt.set_next_header(IpNextHeaderProtocols::Ipv6Route);
                assert_eq!(dstopt . get_next_header (  ) ,
                           IpNextHeaderProtocols :: Ipv6Route);
                dstopt.set_hdr_ext_len(1);
                assert_eq!(dstopt . get_hdr_ext_len (  ) , 1);
                dstopt.set_options(&[b'B'; 14][..]);
                assert_eq!(dstopt . payload (  ) , b"BBBBBBBBBBBBBB");
                dstopt.packet_size()
            };
        pos +=
            {
                let mut routing =
                    MutableRoutingPacket::new(&mut ip_header.payload_mut()[pos..]).unwrap();
                routing.set_next_header(IpNextHeaderProtocols::Ipv6Frag);
                assert_eq!(routing . get_next_header (  ) ,
                           IpNextHeaderProtocols :: Ipv6Frag);
                routing.set_hdr_ext_len(1);
                assert_eq!(routing . get_hdr_ext_len (  ) , 1);
                routing.set_routing_type(4);
                assert_eq!(routing . get_routing_type (  ) , 4);
                routing.set_segments_left(2);
                assert_eq!(routing . get_segments_left (  ) , 2);
                routing.set_data(&[b'C'; 12][..]);
                assert_eq!(routing . payload (  ) , b"CCCCCCCCCCCC");
                routing.packet_size()
            };
        pos +=
            {
                let mut frag =
                    MutableFragmentPacket::new(&mut ip_header.payload_mut()[pos..]).unwrap();
                frag.set_next_header(IpNextHeaderProtocols::Udp);
                assert_eq!(frag . get_next_header (  ) , IpNextHeaderProtocols
                           :: Udp);
                frag.set_fragment_offset(1024);
                assert_eq!(frag . get_fragment_offset (  ) , 1024);
                frag.set_last_fragment(false);
                assert!(! frag . is_last_fragment (  ));
                frag.set_id(1234);
                assert_eq!(frag . get_id (  ) , 1234);
                frag.packet_size()
            };
        assert_eq!(ExtensionIterable :: new (
                   & ip_header . payload (  ) [ .. pos ] ) . map (
                   | ext | (
                   ext . get_next_header (  ) , ext . get_hdr_ext_len (  ) ,
                   ext . packet_size (  ) ) ) . collect :: < Vec < _ >> (  ) ,
                   vec ! [
                   ( IpNextHeaderProtocols :: Ipv6Opts , 1 , 16 ) , (
                   IpNextHeaderProtocols :: Ipv6Route , 1 , 16 ) , (
                   IpNextHeaderProtocols :: Ipv6Frag , 1 , 16 ) , (
                   IpNextHeaderProtocols :: Udp , 0 , 8 ) , ]);
    }
    let ref_packet =
        [97, 17, 1, 1, 1, 1, 0, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1,
         1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1, 1, 16, 16, 1,
         60, 1, b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A', b'A',
         b'A', b'A', b'A', b'A', 43, 1, b'B', b'B', b'B', b'B', b'B', b'B',
         b'B', b'B', b'B', b'B', b'B', b'B', b'B', b'B', 44, 1, 4, 2, b'C',
         b'C', b'C', b'C', b'C', b'C', b'C', b'C', b'C', b'C', b'C', b'C', 17,
         0, 4, 1, 0, 0, 4, 210];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ref_packet . len (  ) ]);
}
