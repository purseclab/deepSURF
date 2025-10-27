// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Packet;
use PrimitiveValues;
use ip::IpNextHeaderProtocols;

use pnet_macros_support::types::*;

use std::net::Ipv4Addr;
use std::net::Ipv6Addr;
use util::{self, Octets};

/// The TCP flags.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod TcpFlags {
    use pnet_macros_support::types::*;
    /// NS – ECN-nonce concealment protection (experimental: see RFC 3540).
    pub const NS: u9be = 0b100000000;
    /// CWR – Congestion Window Reduced (CWR) flag is set by the sending
    /// host to indicate that it received a TCP segment with the ECE flag set
    /// and had responded in congestion control mechanism (added to header by RFC 3168).
    pub const CWR: u9be = 0b010000000;
    /// ECE – ECN-Echo has a dual role, depending on the value of the
    /// SYN flag. It indicates:
    /// If the SYN flag is set (1), that the TCP peer is ECN capable.
    /// If the SYN flag is clear (0), that a packet with Congestion Experienced
    /// flag set (ECN=11) in IP header received during normal transmission
    /// (added to header by RFC 3168).
    pub const ECE: u9be = 0b001000000;
    /// URG – indicates that the Urgent pointer field is significant.
    pub const URG: u9be = 0b000100000;
    /// ACK – indicates that the Acknowledgment field is significant.
    /// All packets after the initial SYN packet sent by the client should have this flag set.
    pub const ACK: u9be = 0b000010000;
    /// PSH – Push function. Asks to push the buffered data to the receiving application.
    pub const PSH: u9be = 0b000001000;
    /// RST – Reset the connection.
    pub const RST: u9be = 0b000000100;
    /// SYN – Synchronize sequence numbers. Only the first packet sent from each end
    /// should have this flag set.
    pub const SYN: u9be = 0b000000010;
    /// FIN – No more data from sender.
    pub const FIN: u9be = 0b000000001;
}











// The length field is an optional field, using a Vec is a way to implement
// it








/* number *//* length */













// Set data









 /* source */
 /* destination */
 /* seq */
 /* ack */
 /* length, flags, win */
 /* checksum */
 /* urg ptr */
 /* options: nop */
 /* timestamp */
 /* "test" */

 // no space for options
 // set invalid offset

 // shouldn't crash here

 // no space for options
 // set invalid offset

 // shouldn't crash here

 // no space for options
 // set invalid offset

 // shouldn't crash here

 // option type
 // option len, not enough space for it


 // set invalid offset

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct TcpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableTcpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> TcpPacket<'a> {
    /// Constructs a new TcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<TcpPacket<'p>> {
        if packet.len() >= TcpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(TcpPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new TcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the TcpPacket will
    /// own its own data and the underlying buffer will be dropped when the TcpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<TcpPacket<'static>> {
        if packet.len() >= TcpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(TcpPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a TcpPacket to a TcpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> TcpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        TcpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a TcpPacket to a TcpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> TcpPacket<'a> {
        TcpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 20 }
    /// The size (in bytes) of a Tcp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Tcp) -> usize {
        20 + _packet.options.len() + _packet.payload.len()
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
    /// Get the sequence field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the acknowledgement field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_acknowledgement(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the data_offset field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_data_offset(&self) -> u4 {
        let _self = self;
        let co = 12;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved(&self) -> u3 {
        let _self = self;
        let co = 12;
        ((_self.packet[co] as u3) & 14) >> 1
    }
    /// Get the flags field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flags(&self) -> u9be {
        let _self = self;
        let co = 12;
        let b0 = (((_self.packet[co + 0] as u9be) & 1) << 8) as u9be;
        let b1 = ((_self.packet[co + 1] as u9be)) as u9be;
        b0 | b1
    }
    /// Get the window field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_window(&self) -> u16be {
        let _self = self;
        let co = 14;
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
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the urgent_ptr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_urgent_ptr(&self) -> u16be {
        let _self = self;
        let co = 18;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
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
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options(&self) -> Vec<TcpOption> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        TcpOptionIterable{buf:
                              &_self.packet[current_offset..end],}.map(|packet|
                                                                           packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the options field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_iter(&self) -> TcpOptionIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        TcpOptionIterable{buf: &_self.packet[current_offset..end],}
    }
}
impl <'a> MutableTcpPacket<'a> {
    /// Constructs a new MutableTcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableTcpPacket<'p>> {
        if packet.len() >= MutableTcpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableTcpPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableTcpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableTcpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableTcpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableTcpPacket<'static>> {
        if packet.len() >= MutableTcpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableTcpPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableTcpPacket to a TcpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> TcpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        TcpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableTcpPacket to a TcpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> TcpPacket<'a> {
        TcpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 20 }
    /// The size (in bytes) of a Tcp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Tcp) -> usize {
        20 + _packet.options.len() + _packet.payload.len()
    }
    /// Populates a TcpPacket using a Tcp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Tcp) {
        let _self = self;
        _self.set_source(packet.source);
        _self.set_destination(packet.destination);
        _self.set_sequence(packet.sequence);
        _self.set_acknowledgement(packet.acknowledgement);
        _self.set_data_offset(packet.data_offset);
        _self.set_reserved(packet.reserved);
        _self.set_flags(packet.flags);
        _self.set_window(packet.window);
        _self.set_checksum(packet.checksum);
        _self.set_urgent_ptr(packet.urgent_ptr);
        _self.set_options(&packet.options);
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
    /// Get the sequence field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence(&self) -> u32be {
        let _self = self;
        let co = 4;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the acknowledgement field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_acknowledgement(&self) -> u32be {
        let _self = self;
        let co = 8;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Get the data_offset field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_data_offset(&self) -> u4 {
        let _self = self;
        let co = 12;
        ((_self.packet[co] as u4) & 240) >> 4
    }
    /// Get the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_reserved(&self) -> u3 {
        let _self = self;
        let co = 12;
        ((_self.packet[co] as u3) & 14) >> 1
    }
    /// Get the flags field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_flags(&self) -> u9be {
        let _self = self;
        let co = 12;
        let b0 = (((_self.packet[co + 0] as u9be) & 1) << 8) as u9be;
        let b1 = ((_self.packet[co + 1] as u9be)) as u9be;
        b0 | b1
    }
    /// Get the window field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_window(&self) -> u16be {
        let _self = self;
        let co = 14;
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
        let co = 16;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the urgent_ptr field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_urgent_ptr(&self) -> u16be {
        let _self = self;
        let co = 18;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
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
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options(&self) -> Vec<TcpOption> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        TcpOptionIterable{buf:
                              &_self.packet[current_offset..end],}.map(|packet|
                                                                           packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the options field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_options_iter(&self) -> TcpOptionIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 20;
        let end =
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        TcpOptionIterable{buf: &_self.packet[current_offset..end],}
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
    /// Set the sequence field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sequence(&mut self, val: u32be) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the acknowledgement field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_acknowledgement(&mut self, val: u32be) {
        let _self = self;
        let co = 8;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the data_offset field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_data_offset(&mut self, val: u4) {
        let _self = self;
        let co = 12;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 15) | (((val & 15) << 4) as u8)) as u8;
    }
    /// Set the reserved field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_reserved(&mut self, val: u3) {
        let _self = self;
        let co = 12;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 241) | (((val & 7) << 1) as u8)) as u8;
    }
    /// Set the flags field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_flags(&mut self, val: u9be) {
        let _self = self;
        let co = 12;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 254) | (((val & 256) >> 8) as u8)) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the window field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_window(&mut self, val: u16be) {
        let _self = self;
        let co = 14;
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
        let co = 16;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the urgent_ptr field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_urgent_ptr(&mut self, val: u16be) {
        let _self = self;
        let co = 18;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
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
            min(current_offset + tcp_options_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the options field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_options(&mut self, vals: &[TcpOption]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset = 20;
        let end = current_offset + tcp_options_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableTcpOptionPacket::new(&mut _self.packet[current_offset..]).unwrap();
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
        let current_offset = 20 + tcp_options_length(&_self.to_immutable());
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for TcpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        20 + tcp_options_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableTcpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        20 + tcp_options_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableTcpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 20 + tcp_options_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableTcpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 20 + tcp_options_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for TcpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 20 + tcp_options_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `TcpPacket`s
pub struct TcpIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for TcpIterable<'a> {
    type
    Item
    =
    TcpPacket<'a>;
    fn next(&mut self) -> Option<TcpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = TcpPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for TcpPacket<'p> {
    type
    T
    =
    Tcp;
    #[inline]
    fn from_packet(&self) -> Tcp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Tcp{source: _self.get_source(),
            destination: _self.get_destination(),
            sequence: _self.get_sequence(),
            acknowledgement: _self.get_acknowledgement(),
            data_offset: _self.get_data_offset(),
            reserved: _self.get_reserved(),
            flags: _self.get_flags(),
            window: _self.get_window(),
            checksum: _self.get_checksum(),
            urgent_ptr: _self.get_urgent_ptr(),
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
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableTcpPacket<'p> {
    type
    T
    =
    Tcp;
    #[inline]
    fn from_packet(&self) -> Tcp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Tcp{source: _self.get_source(),
            destination: _self.get_destination(),
            sequence: _self.get_sequence(),
            acknowledgement: _self.get_acknowledgement(),
            data_offset: _self.get_data_offset(),
            reserved: _self.get_reserved(),
            flags: _self.get_flags(),
            window: _self.get_window(),
            checksum: _self.get_checksum(),
            urgent_ptr: _self.get_urgent_ptr(),
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
impl <'p> ::std::fmt::Debug for TcpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "TcpPacket {{ source : {:?}, destination : {:?}, sequence : {:?}, acknowledgement : {:?}, data_offset : {:?}, reserved : {:?}, flags : {:?}, window : {:?}, checksum : {:?}, urgent_ptr : {:?}, options : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_sequence (  ) , _self . get_acknowledgement (  ) ,
               _self . get_data_offset (  ) , _self . get_reserved (  ) ,
               _self . get_flags (  ) , _self . get_window (  ) , _self .
               get_checksum (  ) , _self . get_urgent_ptr (  ) , _self .
               get_options (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableTcpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableTcpPacket {{ source : {:?}, destination : {:?}, sequence : {:?}, acknowledgement : {:?}, data_offset : {:?}, reserved : {:?}, flags : {:?}, window : {:?}, checksum : {:?}, urgent_ptr : {:?}, options : {:?},  }}"
               , _self . get_source (  ) , _self . get_destination (  ) ,
               _self . get_sequence (  ) , _self . get_acknowledgement (  ) ,
               _self . get_data_offset (  ) , _self . get_reserved (  ) ,
               _self . get_flags (  ) , _self . get_window (  ) , _self .
               get_checksum (  ) , _self . get_urgent_ptr (  ) , _self .
               get_options (  ))
    }
}
/// Represents a TCP packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Tcp {
    pub source: u16be,
    pub destination: u16be,
    pub sequence: u32be,
    pub acknowledgement: u32be,
    pub data_offset: u4,
    pub reserved: u3,
    pub flags: u9be,
    pub window: u16be,
    pub checksum: u16be,
    pub urgent_ptr: u16be,
    pub options: Vec<TcpOption>,
    pub payload: Vec<u8>,
}
/// Represents a TCP option.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct TcpOptionNumber(pub u8);
/// The TCP header options.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod TcpOptionNumbers {
    use super::TcpOptionNumber;
    /// End of Options list.
    pub const EOL: TcpOptionNumber = TcpOptionNumber(0);
    /// No operation.
    pub const NOP: TcpOptionNumber = TcpOptionNumber(1);
    /// Maximum segment size.
    pub const MSS: TcpOptionNumber = TcpOptionNumber(2);
    /// Window scale.
    pub const WSCALE: TcpOptionNumber = TcpOptionNumber(3);
    /// Selective acknowledgements permitted.
    pub const SACK_PERMITTED: TcpOptionNumber = TcpOptionNumber(4);
    /// Selective acknowledgment.
    pub const SACK: TcpOptionNumber = TcpOptionNumber(5);
    /// Timestamps.
    pub const TIMESTAMPS: TcpOptionNumber = TcpOptionNumber(8);
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct TcpOptionPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableTcpOptionPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> TcpOptionPacket<'a> {
    /// Constructs a new TcpOptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<TcpOptionPacket<'p>> {
        if packet.len() >= TcpOptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(TcpOptionPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new TcpOptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the TcpOptionPacket will
    /// own its own data and the underlying buffer will be dropped when the TcpOptionPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<TcpOptionPacket<'static>> {
        if packet.len() >= TcpOptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(TcpOptionPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a TcpOptionPacket to a TcpOptionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> TcpOptionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        TcpOptionPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a TcpOptionPacket to a TcpOptionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> TcpOptionPacket<'a> {
        TcpOptionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 1 }
    /// The size (in bytes) of a TcpOption instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &TcpOption) -> usize {
        1 + _packet.length.len() + _packet.data.len()
    }
    /// Get the value of the number field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> TcpOptionNumber {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &TcpOptionPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        TcpOptionNumber::new(get_arg0(&self))
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
            min(current_offset + tcp_option_length(&_self.to_immutable()),
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
            min(current_offset + tcp_option_length(&_self.to_immutable()),
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
impl <'a> MutableTcpOptionPacket<'a> {
    /// Constructs a new MutableTcpOptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableTcpOptionPacket<'p>> {
        if packet.len() >= MutableTcpOptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableTcpOptionPacket{packet:
                                            MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableTcpOptionPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableTcpOptionPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableTcpOptionPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableTcpOptionPacket<'static>> {
        if packet.len() >= MutableTcpOptionPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableTcpOptionPacket{packet:
                                            MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableTcpOptionPacket to a TcpOptionPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> TcpOptionPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        TcpOptionPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableTcpOptionPacket to a TcpOptionPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> TcpOptionPacket<'a> {
        TcpOptionPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 1 }
    /// The size (in bytes) of a TcpOption instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &TcpOption) -> usize {
        1 + _packet.length.len() + _packet.data.len()
    }
    /// Populates a TcpOptionPacket using a TcpOption structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &TcpOption) {
        let _self = self;
        _self.set_number(packet.number);
        _self.set_length(&packet.length);
        _self.set_data(&packet.data);
    }
    /// Get the value of the number field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> TcpOptionNumber {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableTcpOptionPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        TcpOptionNumber::new(get_arg0(&self))
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
            min(current_offset + tcp_option_length(&_self.to_immutable()),
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
            min(current_offset + tcp_option_length(&_self.to_immutable()),
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
    /// Set the value of the number field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_number(&mut self, val: TcpOptionNumber) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableTcpOptionPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
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
            min(current_offset + tcp_option_length(&_self.to_immutable()),
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
        let len = tcp_option_length(&_self.to_immutable());
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
        let current_offset = 1 + tcp_option_length(&_self.to_immutable());
        let len = tcp_option_payload_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for TcpOptionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1 + tcp_option_length(&_self.to_immutable()) +
            tcp_option_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableTcpOptionPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        1 + tcp_option_length(&_self.to_immutable()) +
            tcp_option_payload_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableTcpOptionPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 1 + tcp_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + tcp_option_length(&_self.to_immutable()) +
                                tcp_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableTcpOptionPacket<'a>
 {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1 + tcp_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + tcp_option_length(&_self.to_immutable()) +
                                tcp_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for TcpOptionPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 1 + tcp_option_length(&_self.to_immutable());
        let end =
            ::std::cmp::min(1 + tcp_option_length(&_self.to_immutable()) +
                                tcp_option_payload_length(&_self.to_immutable()),
                            _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `TcpOptionPacket`s
pub struct TcpOptionIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for TcpOptionIterable<'a> {
    type
    Item
    =
    TcpOptionPacket<'a>;
    fn next(&mut self) -> Option<TcpOptionPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = TcpOptionPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for TcpOptionPacket<'p> {
    type
    T
    =
    TcpOption;
    #[inline]
    fn from_packet(&self) -> TcpOption {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        TcpOption{number: _self.get_number(),
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
 MutableTcpOptionPacket<'p> {
    type
    T
    =
    TcpOption;
    #[inline]
    fn from_packet(&self) -> TcpOption {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        TcpOption{number: _self.get_number(),
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
impl <'p> ::std::fmt::Debug for TcpOptionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt , "TcpOptionPacket {{ number : {:?}, length : {:?},  }}" ,
               _self . get_number (  ) , _self . get_length (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableTcpOptionPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableTcpOptionPacket {{ number : {:?}, length : {:?},  }}" ,
               _self . get_number (  ) , _self . get_length (  ))
    }
}
/// A TCP option.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct TcpOption {
    number: TcpOptionNumber,
    length: Vec<u8>,
    data: Vec<u8>,
}
impl TcpOption {
    /// NOP: This may be used to align option fields on 32-bit boundaries for better performance.
    pub fn nop() -> Self {
        TcpOption{number: TcpOptionNumbers::NOP,
                  length: vec!(),
                  data: vec!(),}
    }
    /// Timestamp: TCP timestamps, defined in RFC 1323, can help TCP determine in which order
    /// packets were sent. TCP timestamps are not normally aligned to the system clock and
    /// start at some random value.
    pub fn timestamp(my: u32, their: u32) -> Self {
        let mut data = vec!();
        data.extend_from_slice(&my.octets()[..]);
        data.extend_from_slice(&their.octets()[..]);
        TcpOption{number: TcpOptionNumbers::TIMESTAMPS,
                  length: vec!(10),
                  data: data,}
    }
    /// MSS: The maximum segment size (MSS) is the largest amount of data, specified in bytes,
    /// that TCP is willing to receive in a single segment.
    pub fn mss(val: u16) -> Self {
        let mut data = vec!();
        data.extend_from_slice(&val.octets()[..]);
        TcpOption{number: TcpOptionNumbers::MSS, length: vec!(4), data: data,}
    }
    /// Window scale: The TCP window scale option, as defined in RFC 1323, is an option used to
    /// increase the maximum window size from 65,535 bytes to 1 gigabyte.
    pub fn wscale(val: u8) -> Self {
        TcpOption{number: TcpOptionNumbers::WSCALE,
                  length: vec!(3),
                  data: vec!(val),}
    }
    /// Selective acknowledgment (SACK) option, defined in RFC 2018 allows the receiver to acknowledge
    /// discontinuous blocks of packets which were received correctly. This options enables use of
    /// SACK during negotiation.
    pub fn sack_perm() -> Self {
        TcpOption{number: TcpOptionNumbers::SACK_PERMITTED,
                  length: vec!(2),
                  data: vec!(),}
    }
    /// Selective acknowledgment (SACK) option, defined in RFC 2018 allows the receiver to acknowledge
    /// discontinuous blocks of packets which were received correctly. The acknowledgement can specify
    /// a number of SACK blocks, where each SACK block is conveyed by the starting and ending sequence
    /// numbers of a contiguous range that the receiver correctly received.
    pub fn selective_ack(acks: &[u32]) -> Self {
        let mut data = vec!();
        for ack in acks { data.extend_from_slice(&ack.octets()[..]); }
        TcpOption{number: TcpOptionNumbers::SACK,
                  length: vec!(1 + 1 + data . len (  ) as u8),
                  data: data,}
    }
}
/// This function gets the 'length' of the length field of the IPv4Option packet
/// Few options (EOL, NOP) are 1 bytes long, and then have a length field equal
/// to 0.
#[inline]
fn tcp_option_length(option: &TcpOptionPacket) -> usize {
    match option.get_number() {
        TcpOptionNumbers::EOL => 0,
        TcpOptionNumbers::NOP => 0,
        _ => 1,
    }
}
fn tcp_option_payload_length(ipv4_option: &TcpOptionPacket) -> usize {
    match ipv4_option.get_length_raw().first() {
        Some(len) if *len >= 2 => (*len as usize) - 2,
        _ => 0,
    }
}
impl TcpOptionNumber {
    /// Create a new `TcpOptionNumber` instance.
    pub fn new(value: u8) -> TcpOptionNumber { TcpOptionNumber(value) }
}
impl PrimitiveValues for TcpOptionNumber {
    type
    T
    =
    (u8,);
    fn to_primitive_values(&self) -> (u8,) { (self.0,) }
}
#[inline]
fn tcp_options_length(tcp: &TcpPacket) -> usize {
    let data_offset = tcp.get_data_offset();
    if data_offset > 5 { (data_offset as usize) * 4 - 20 } else { 0 }
}
/// Calculate a checksum for a packet built on IPv4.
pub fn ipv4_checksum(packet: &TcpPacket, source: &Ipv4Addr,
                     destination: &Ipv4Addr) -> u16 {
    ipv4_checksum_adv(packet, &[], source, destination)
}
/// Calculate the checksum for a packet built on IPv4, Advanced version which
/// accepts an extra slice of data that will be included in the checksum
/// as being part of the data portion of the packet.
///
/// If `packet` contains an odd number of bytes the last byte will not be
/// counted as the first byte of a word together with the first byte of
/// `extra_data`.
pub fn ipv4_checksum_adv(packet: &TcpPacket, extra_data: &[u8],
                         source: &Ipv4Addr, destination: &Ipv4Addr) -> u16 {
    util::ipv4_checksum(packet.packet(), 8, extra_data, source, destination,
                        IpNextHeaderProtocols::Tcp)
}
/// Calculate a checksum for a packet built on IPv6.
pub fn ipv6_checksum(packet: &TcpPacket, source: &Ipv6Addr,
                     destination: &Ipv6Addr) -> u16 {
    ipv6_checksum_adv(packet, &[], source, destination)
}
/// Calculate the checksum for a packet built on IPv6, Advanced version which
/// accepts an extra slice of data that will be included in the checksum
/// as being part of the data portion of the packet.
///
/// If `packet` contains an odd number of bytes the last byte will not be
/// counted as the first byte of a word together with the first byte of
/// `extra_data`.
pub fn ipv6_checksum_adv(packet: &TcpPacket, extra_data: &[u8],
                         source: &Ipv6Addr, destination: &Ipv6Addr) -> u16 {
    util::ipv6_checksum(packet.packet(), 8, extra_data, source, destination,
                        IpNextHeaderProtocols::Tcp)
}
#[test]
fn tcp_header_ipv4_test() {
    use ip::IpNextHeaderProtocols;
    use ipv4::MutableIpv4Packet;
    const IPV4_HEADER_LEN: usize = 20;
    const TCP_HEADER_LEN: usize = 32;
    const TEST_DATA_LEN: usize = 4;
    let mut packet = [0u8; IPV4_HEADER_LEN + TCP_HEADER_LEN + TEST_DATA_LEN];
    let ipv4_source = Ipv4Addr::new(192, 168, 2, 1);
    let ipv4_destination = Ipv4Addr::new(192, 168, 111, 51);
    {
        let mut ip_header = MutableIpv4Packet::new(&mut packet[..]).unwrap();
        ip_header.set_next_level_protocol(IpNextHeaderProtocols::Tcp);
        ip_header.set_source(ipv4_source);
        ip_header.set_destination(ipv4_destination);
    }
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN] = 't' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 1] = 'e' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 2] = 's' as u8;
    packet[IPV4_HEADER_LEN + TCP_HEADER_LEN + 3] = 't' as u8;
    {
        let mut tcp_header =
            MutableTcpPacket::new(&mut packet[IPV4_HEADER_LEN..]).unwrap();
        tcp_header.set_source(49511);
        assert_eq!(tcp_header . get_source (  ) , 49511);
        tcp_header.set_destination(9000);
        assert_eq!(tcp_header . get_destination (  ) , 9000);
        tcp_header.set_sequence(2419577528);
        assert_eq!(tcp_header . get_sequence (  ) , 0x9037d2b8);
        tcp_header.set_acknowledgement(2487988854);
        assert_eq!(tcp_header . get_acknowledgement (  ) , 0x944bb276);
        tcp_header.set_flags(TcpFlags::PSH | TcpFlags::ACK);
        assert_eq!(tcp_header . get_flags (  ) , TcpFlags :: PSH | TcpFlags ::
                   ACK);
        tcp_header.set_window(4015);
        assert_eq!(tcp_header . get_window (  ) , 4015);
        tcp_header.set_data_offset(8);
        assert_eq!(tcp_header . get_data_offset (  ) , 8);
        let ts = TcpOption::timestamp(743951781, 44056978);
        tcp_header.set_options(&vec!(TcpOption :: nop (  ) , TcpOption :: nop
                                     (  ) , ts));
        let checksum =
            ipv4_checksum(&tcp_header.to_immutable(), &ipv4_source,
                          &ipv4_destination);
        tcp_header.set_checksum(checksum);
        assert_eq!(tcp_header . get_checksum (  ) , 0xc031);
    }
    let ref_packet =
        [193, 103, 35, 40, 144, 55, 210, 184, 148, 75, 178, 118, 128, 24, 15,
         175, 192, 49, 0, 0, 1, 1, 8, 10, 44, 87, 205, 165, 2, 160, 65, 146,
         116, 101, 115, 116];
    assert_eq!(& ref_packet [ .. ] , & packet [ 20 .. ]);
}
#[test]
fn tcp_test_options_invalid_offset() {
    let mut buf = [0; 20];
    {
        if let Some(mut tcp) = MutableTcpPacket::new(&mut buf[..]) {
            tcp.set_data_offset(10);
        }
    }
    if let Some(tcp) = TcpPacket::new(&buf[..]) {
        let _options = tcp.get_options_iter();
    }
}
#[test]
fn tcp_test_options_vec_invalid_offset() {
    let mut buf = [0; 20];
    {
        if let Some(mut tcp) = MutableTcpPacket::new(&mut buf[..]) {
            tcp.set_data_offset(10);
        }
    }
    if let Some(tcp) = TcpPacket::new(&buf[..]) {
        let _options = tcp.get_options();
    }
}
#[test]
fn tcp_test_options_slice_invalid_offset() {
    let mut buf = [0; 20];
    {
        if let Some(mut tcp) = MutableTcpPacket::new(&mut buf[..]) {
            tcp.set_data_offset(10);
        }
    }
    if let Some(tcp) = TcpPacket::new(&buf[..]) {
        let _options = tcp.get_options_raw();
    }
}
#[test]
fn tcp_test_option_invalid_len() {
    let mut buf = [0; 24];
    {
        if let Some(mut tcp) = MutableTcpPacket::new(&mut buf[..]) {
            tcp.set_data_offset(6);
        }
        buf[20] = 2;
        buf[21] = 8;
    }
    if let Some(tcp) = TcpPacket::new(&buf[..]) {
        let options = tcp.get_options_iter();
        for opt in options { println!("{:?}" , opt); }
    }
}
#[test]
fn tcp_test_payload_slice_invalid_offset() {
    let mut buf = [0; 20];
    {
        if let Some(mut tcp) = MutableTcpPacket::new(&mut buf[..]) {
            tcp.set_data_offset(10);
        }
    }
    if let Some(tcp) = TcpPacket::new(&buf[..]) {
        assert_eq!(tcp . payload (  ) . len (  ) , 0);
    }
}
