// Copyright (c) 2016 Robert Collins <robertc@robertcollins.net>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(test)]
use Packet;

use pnet_macros_support::types::*;

 // 0x800 for ipv4 [basically an ethertype










/* no flags *//* no flags, version 0 *//* protocol 0x0800 */


/* checksum on *//* no flags, version 0 *//* protocol 0x0000 */
/* 16 bits of checksum *//* 16 bits of offset */

#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct GrePacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableGrePacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> GrePacket<'a> {
    /// Constructs a new GrePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<GrePacket<'p>> {
        if packet.len() >= GrePacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(GrePacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new GrePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the GrePacket will
    /// own its own data and the underlying buffer will be dropped when the GrePacket is.
    pub fn owned(packet: Vec<u8>) -> Option<GrePacket<'static>> {
        if packet.len() >= GrePacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(GrePacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a GrePacket to a GrePacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> GrePacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        GrePacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a GrePacket to a GrePacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> GrePacket<'a> {
        GrePacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Gre instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Gre) -> usize {
        4 + _packet.checksum.len() + _packet.offset.len() + _packet.key.len()
            + _packet.sequence.len() + _packet.routing.len() +
            _packet.payload.len()
    }
    /// Get the checksum_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the routing_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 64) >> 6
    }
    /// Get the key_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the sequence_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the strict_source_route field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_strict_source_route(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 8) >> 3
    }
    /// Get the recursion_control field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_recursion_control(&self) -> u3 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u3) & 7)
    }
    /// Get the zero_flags field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_zero_flags(&self) -> u5 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u5) & 248) >> 3
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u3 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u3) & 7)
    }
    /// Get the protocol_type field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_protocol_type(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the raw &[u8] value of the checksum field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the checksum field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> Vec<U16BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the checksum field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_iter(&self) -> U16BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the offset field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the offset field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset(&self) -> Vec<U16BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the offset field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset_iter(&self) -> U16BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the key field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the key field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key(&self) -> Vec<U32BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the key field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_iter(&self) -> U32BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the sequence field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the sequence field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence(&self) -> Vec<U32BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the sequence field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_iter(&self) -> U32BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the routing field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_routing_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the routing field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing(&self) -> Vec<u8> {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let pkt_len = self.packet.len();
        let end =
            min(current_offset + gre_routing_length(&_self.to_immutable()),
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
impl <'a> MutableGrePacket<'a> {
    /// Constructs a new MutableGrePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableGrePacket<'p>> {
        if packet.len() >= MutableGrePacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableGrePacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableGrePacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableGrePacket will
    /// own its own data and the underlying buffer will be dropped when the MutableGrePacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableGrePacket<'static>> {
        if packet.len() >= MutableGrePacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableGrePacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableGrePacket to a GrePacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> GrePacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        GrePacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableGrePacket to a GrePacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> GrePacket<'a> {
        GrePacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Gre instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Gre) -> usize {
        4 + _packet.checksum.len() + _packet.offset.len() + _packet.key.len()
            + _packet.sequence.len() + _packet.routing.len() +
            _packet.payload.len()
    }
    /// Populates a GrePacket using a Gre structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Gre) {
        let _self = self;
        _self.set_checksum_present(packet.checksum_present);
        _self.set_routing_present(packet.routing_present);
        _self.set_key_present(packet.key_present);
        _self.set_sequence_present(packet.sequence_present);
        _self.set_strict_source_route(packet.strict_source_route);
        _self.set_recursion_control(packet.recursion_control);
        _self.set_zero_flags(packet.zero_flags);
        _self.set_version(packet.version);
        _self.set_protocol_type(packet.protocol_type);
        _self.set_checksum(&packet.checksum);
        _self.set_offset(&packet.offset);
        _self.set_key(&packet.key);
        _self.set_sequence(&packet.sequence);
        _self.set_routing(&packet.routing);
        _self.set_payload(&packet.payload);
    }
    /// Get the checksum_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 128) >> 7
    }
    /// Get the routing_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 64) >> 6
    }
    /// Get the key_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 32) >> 5
    }
    /// Get the sequence_present field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_present(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the strict_source_route field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_strict_source_route(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 8) >> 3
    }
    /// Get the recursion_control field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_recursion_control(&self) -> u3 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u3) & 7)
    }
    /// Get the zero_flags field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_zero_flags(&self) -> u5 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u5) & 248) >> 3
    }
    /// Get the version field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_version(&self) -> u3 {
        let _self = self;
        let co = 1;
        ((_self.packet[co] as u3) & 7)
    }
    /// Get the protocol_type field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_protocol_type(&self) -> u16be {
        let _self = self;
        let co = 2;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Get the raw &[u8] value of the checksum field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the checksum field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum(&self) -> Vec<U16BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the checksum field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_iter(&self) -> U16BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the offset field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the offset field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset(&self) -> Vec<U16BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the offset field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset_iter(&self) -> U16BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        U16BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the key field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the key field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key(&self) -> Vec<U32BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the key field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_iter(&self) -> U32BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the sequence field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the sequence field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence(&self) -> Vec<U32BE> {
        use pnet_macros_support::packet::FromPacket;
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf:
                          &_self.packet[current_offset..end],}.map(|packet|
                                                                       packet.from_packet()).collect::<Vec<_>>()
    }
    /// Get the value of the sequence field as iterator
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_iter(&self) -> U32BEIterable {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        U32BEIterable{buf: &_self.packet[current_offset..end],}
    }
    /// Get the raw &[u8] value of the routing field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_raw(&self) -> &[u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_routing_length(&_self.to_immutable()),
                _self.packet.len());
        &_self.packet[current_offset..end]
    }
    /// Get the value of the routing field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing(&self) -> Vec<u8> {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let pkt_len = self.packet.len();
        let end =
            min(current_offset + gre_routing_length(&_self.to_immutable()),
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
    /// Set the checksum_present field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_checksum_present(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 127) | (((val & 1) << 7) as u8)) as u8;
    }
    /// Set the routing_present field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_routing_present(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 191) | (((val & 1) << 6) as u8)) as u8;
    }
    /// Set the key_present field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_key_present(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 223) | (((val & 1) << 5) as u8)) as u8;
    }
    /// Set the sequence_present field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sequence_present(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 239) | (((val & 1) << 4) as u8)) as u8;
    }
    /// Set the strict_source_route field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_strict_source_route(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 247) | (((val & 1) << 3) as u8)) as u8;
    }
    /// Set the recursion_control field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_recursion_control(&mut self, val: u3) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 248) | (((val & 7)) as u8)) as u8;
    }
    /// Set the zero_flags field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_zero_flags(&mut self, val: u5) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 7) | (((val & 31) << 3) as u8)) as u8;
    }
    /// Set the version field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_version(&mut self, val: u3) {
        let _self = self;
        let co = 1;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 248) | (((val & 7)) as u8)) as u8;
    }
    /// Set the protocol_type field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_protocol_type(&mut self, val: u16be) {
        let _self = self;
        let co = 2;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Get the raw &mut [u8] value of the checksum field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_checksum_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4;
        let end =
            min(current_offset + gre_checksum_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the checksum field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_checksum(&mut self, vals: &[U16BE]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset = 4;
        let end = current_offset + gre_checksum_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableU16BEPacket::new(&mut _self.packet[current_offset..]).unwrap();
            packet.populate(val);
            current_offset += packet.packet_size();
            assert!(current_offset <= end);
        }
    }
    /// Get the raw &mut [u8] value of the offset field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_offset_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset = 4 + gre_checksum_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_offset_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the offset field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_offset(&mut self, vals: &[U16BE]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset =
            4 + gre_checksum_length(&_self.to_immutable());
        let end = current_offset + gre_offset_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableU16BEPacket::new(&mut _self.packet[current_offset..]).unwrap();
            packet.populate(val);
            current_offset += packet.packet_size();
            assert!(current_offset <= end);
        }
    }
    /// Get the raw &mut [u8] value of the key field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_key_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_key_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the key field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_key(&mut self, vals: &[U32BE]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable());
        let end = current_offset + gre_key_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableU32BEPacket::new(&mut _self.packet[current_offset..]).unwrap();
            packet.populate(val);
            current_offset += packet.packet_size();
            assert!(current_offset <= end);
        }
    }
    /// Get the raw &mut [u8] value of the sequence field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sequence_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_sequence_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the sequence field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sequence(&mut self, vals: &[U32BE]) {
        use pnet_macros_support::packet::PacketSize;
        let _self = self;
        let mut current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable());
        let end = current_offset + gre_sequence_length(&_self.to_immutable());
        for val in vals.into_iter() {
            let mut packet =
                MutableU32BEPacket::new(&mut _self.packet[current_offset..]).unwrap();
            packet.populate(val);
            current_offset += packet.packet_size();
            assert!(current_offset <= end);
        }
    }
    /// Get the raw &mut [u8] value of the routing field, without copying
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_routing_raw_mut(&mut self) -> &mut [u8] {
        use std::cmp::min;
        let _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let end =
            min(current_offset + gre_routing_length(&_self.to_immutable()),
                _self.packet.len());
        &mut _self.packet[current_offset..end]
    }
    /// Set the value of the routing field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_routing(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable());
        let len = gre_routing_length(&_self.to_immutable());
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
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
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable()) +
                gre_routing_length(&_self.to_immutable());
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for GrePacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4 + gre_checksum_length(&_self.to_immutable()) +
            gre_offset_length(&_self.to_immutable()) +
            gre_key_length(&_self.to_immutable()) +
            gre_sequence_length(&_self.to_immutable()) +
            gre_routing_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableGrePacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize {
        let _self = self;
        4 + gre_checksum_length(&_self.to_immutable()) +
            gre_offset_length(&_self.to_immutable()) +
            gre_key_length(&_self.to_immutable()) +
            gre_sequence_length(&_self.to_immutable()) +
            gre_routing_length(&_self.to_immutable())
    }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableGrePacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable()) +
                gre_routing_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableGrePacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable()) +
                gre_routing_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for GrePacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start =
            4 + gre_checksum_length(&_self.to_immutable()) +
                gre_offset_length(&_self.to_immutable()) +
                gre_key_length(&_self.to_immutable()) +
                gre_sequence_length(&_self.to_immutable()) +
                gre_routing_length(&_self.to_immutable());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `GrePacket`s
pub struct GreIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for GreIterable<'a> {
    type
    Item
    =
    GrePacket<'a>;
    fn next(&mut self) -> Option<GrePacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = GrePacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for GrePacket<'p> {
    type
    T
    =
    Gre;
    #[inline]
    fn from_packet(&self) -> Gre {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Gre{checksum_present: _self.get_checksum_present(),
            routing_present: _self.get_routing_present(),
            key_present: _self.get_key_present(),
            sequence_present: _self.get_sequence_present(),
            strict_source_route: _self.get_strict_source_route(),
            recursion_control: _self.get_recursion_control(),
            zero_flags: _self.get_zero_flags(),
            version: _self.get_version(),
            protocol_type: _self.get_protocol_type(),
            checksum: _self.get_checksum(),
            offset: _self.get_offset(),
            key: _self.get_key(),
            sequence: _self.get_sequence(),
            routing: _self.get_routing(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableGrePacket<'p> {
    type
    T
    =
    Gre;
    #[inline]
    fn from_packet(&self) -> Gre {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Gre{checksum_present: _self.get_checksum_present(),
            routing_present: _self.get_routing_present(),
            key_present: _self.get_key_present(),
            sequence_present: _self.get_sequence_present(),
            strict_source_route: _self.get_strict_source_route(),
            recursion_control: _self.get_recursion_control(),
            zero_flags: _self.get_zero_flags(),
            version: _self.get_version(),
            protocol_type: _self.get_protocol_type(),
            checksum: _self.get_checksum(),
            offset: _self.get_offset(),
            key: _self.get_key(),
            sequence: _self.get_sequence(),
            routing: _self.get_routing(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::std::fmt::Debug for GrePacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "GrePacket {{ checksum_present : {:?}, routing_present : {:?}, key_present : {:?}, sequence_present : {:?}, strict_source_route : {:?}, recursion_control : {:?}, zero_flags : {:?}, version : {:?}, protocol_type : {:?}, checksum : {:?}, offset : {:?}, key : {:?}, sequence : {:?}, routing : {:?},  }}"
               , _self . get_checksum_present (  ) , _self .
               get_routing_present (  ) , _self . get_key_present (  ) , _self
               . get_sequence_present (  ) , _self . get_strict_source_route (
                ) , _self . get_recursion_control (  ) , _self .
               get_zero_flags (  ) , _self . get_version (  ) , _self .
               get_protocol_type (  ) , _self . get_checksum (  ) , _self .
               get_offset (  ) , _self . get_key (  ) , _self . get_sequence (
                ) , _self . get_routing (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableGrePacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableGrePacket {{ checksum_present : {:?}, routing_present : {:?}, key_present : {:?}, sequence_present : {:?}, strict_source_route : {:?}, recursion_control : {:?}, zero_flags : {:?}, version : {:?}, protocol_type : {:?}, checksum : {:?}, offset : {:?}, key : {:?}, sequence : {:?}, routing : {:?},  }}"
               , _self . get_checksum_present (  ) , _self .
               get_routing_present (  ) , _self . get_key_present (  ) , _self
               . get_sequence_present (  ) , _self . get_strict_source_route (
                ) , _self . get_recursion_control (  ) , _self .
               get_zero_flags (  ) , _self . get_version (  ) , _self .
               get_protocol_type (  ) , _self . get_checksum (  ) , _self .
               get_offset (  ) , _self . get_key (  ) , _self . get_sequence (
                ) , _self . get_routing (  ))
    }
}
/// GRE (Generic Routing Encapsulation) Packet.
///
/// See RFCs 1701, 2784, 2890, 7676, 2637
///
/// Current status of implementation:
///
/// - [RFC 1701](https://tools.ietf.org/html/rfc1701) except for source routing and checksums.
///   Processing a source routed packet will panic. Checksums are able to be inspected, but not
///   calculated or verified.
///
/// - [RFC 2784](https://tools.ietf.org/html/rfc2784) except for checksums (same as 1701 status).
///   Note that it is possible to generate noncompliant packets by setting any of the reserved bits
///   (but see 2890).
///
/// - [RFC 2890](https://tools.ietf.org/html/rfc2890) implemented.
///
/// - [RFC 7676](https://tools.ietf.org/html/rfc7676) has no packet changes - compliance is up to
///   the user.
///
/// - [RFC 2637](https://tools.ietf.org/html/rfc2637) not implemented.
///
/// Note that routing information from RFC 1701 is not implemented, packets
/// with `routing_present` true will currently cause a panic.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Gre {
    pub checksum_present: u1,
    pub routing_present: u1,
    pub key_present: u1,
    pub sequence_present: u1,
    pub strict_source_route: u1,
    pub recursion_control: u3,
    pub zero_flags: u5,
    pub version: u3,
    pub protocol_type: u16be,
    pub checksum: Vec<U16BE>,
    pub offset: Vec<U16BE>,
    pub key: Vec<U32BE>,
    pub sequence: Vec<U32BE>,
    pub routing: Vec<u8>,
    pub payload: Vec<u8>,
}
fn gre_checksum_length(gre: &GrePacket) -> usize {
    ((gre.get_checksum_present() | gre.get_routing_present()) as usize) * 2
}
fn gre_offset_length(gre: &GrePacket) -> usize {
    ((gre.get_checksum_present() | gre.get_routing_present()) as usize) * 2
}
fn gre_key_length(gre: &GrePacket) -> usize {
    (gre.get_key_present() as usize) * 4
}
fn gre_sequence_length(gre: &GrePacket) -> usize {
    (gre.get_sequence_present() as usize) * 4
}
fn gre_routing_length(gre: &GrePacket) -> usize {
    if 0 == gre.get_routing_present() {
        0
    } else { panic!("Source routed GRE packets not supported") }
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct U16BEPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableU16BEPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> U16BEPacket<'a> {
    /// Constructs a new U16BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<U16BEPacket<'p>> {
        if packet.len() >= U16BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U16BEPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new U16BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the U16BEPacket will
    /// own its own data and the underlying buffer will be dropped when the U16BEPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<U16BEPacket<'static>> {
        if packet.len() >= U16BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U16BEPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a U16BEPacket to a U16BEPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U16BEPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U16BEPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a U16BEPacket to a U16BEPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U16BEPacket<'a> {
        U16BEPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 2 }
    /// The size (in bytes) of a U16BE instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U16BE) -> usize { 2 + _packet.unused.len() }
    /// Get the number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
}
impl <'a> MutableU16BEPacket<'a> {
    /// Constructs a new MutableU16BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableU16BEPacket<'p>> {
        if packet.len() >= MutableU16BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU16BEPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableU16BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableU16BEPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableU16BEPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableU16BEPacket<'static>> {
        if packet.len() >= MutableU16BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU16BEPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableU16BEPacket to a U16BEPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U16BEPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U16BEPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableU16BEPacket to a U16BEPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U16BEPacket<'a> {
        U16BEPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 2 }
    /// The size (in bytes) of a U16BE instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U16BE) -> usize { 2 + _packet.unused.len() }
    /// Populates a U16BEPacket using a U16BE structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &U16BE) {
        let _self = self;
        _self.set_number(packet.number);
        _self.set_unused(&packet.unused);
    }
    /// Get the number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> u16be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
        let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
        b0 | b1
    }
    /// Set the number field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_number(&mut self, val: u16be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 1] = (val) as u8;
    }
    /// Set the value of the unused field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_unused(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 2;
        let len = 0;
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for U16BEPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 2 + 0 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableU16BEPacket<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 2 + 0 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableU16BEPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 2;
        let end = ::std::cmp::min(2 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableU16BEPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        let end = ::std::cmp::min(2 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for U16BEPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 2;
        let end = ::std::cmp::min(2 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `U16BEPacket`s
pub struct U16BEIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for U16BEIterable<'a> {
    type
    Item
    =
    U16BEPacket<'a>;
    fn next(&mut self) -> Option<U16BEPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = U16BEPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for U16BEPacket<'p> {
    type
    T
    =
    U16BE;
    #[inline]
    fn from_packet(&self) -> U16BE {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U16BE{number: _self.get_number(),
              unused:
                  {
                      let payload = self.payload();
                      let mut vec = Vec::with_capacity(payload.len());
                      vec.extend_from_slice(payload);
                      vec
                  },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableU16BEPacket<'p>
 {
    type
    T
    =
    U16BE;
    #[inline]
    fn from_packet(&self) -> U16BE {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U16BE{number: _self.get_number(),
              unused:
                  {
                      let payload = self.payload();
                      let mut vec = Vec::with_capacity(payload.len());
                      vec.extend_from_slice(payload);
                      vec
                  },}
    }
}
impl <'p> ::std::fmt::Debug for U16BEPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt , "U16BEPacket {{ number : {:?},  }}" , _self . get_number
               (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableU16BEPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt , "MutableU16BEPacket {{ number : {:?},  }}" , _self .
               get_number (  ))
    }
}
/// `u16be`, but we can't use that directly in a `Vec` :(
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct U16BE {
    number: u16be,
    unused: Vec<u8>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct U32BEPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableU32BEPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> U32BEPacket<'a> {
    /// Constructs a new U32BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<U32BEPacket<'p>> {
        if packet.len() >= U32BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U32BEPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new U32BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the U32BEPacket will
    /// own its own data and the underlying buffer will be dropped when the U32BEPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<U32BEPacket<'static>> {
        if packet.len() >= U32BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(U32BEPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a U32BEPacket to a U32BEPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U32BEPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U32BEPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a U32BEPacket to a U32BEPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U32BEPacket<'a> {
        U32BEPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a U32BE instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U32BE) -> usize { 4 + _packet.unused.len() }
    /// Get the number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
}
impl <'a> MutableU32BEPacket<'a> {
    /// Constructs a new MutableU32BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableU32BEPacket<'p>> {
        if packet.len() >= MutableU32BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU32BEPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableU32BEPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableU32BEPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableU32BEPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableU32BEPacket<'static>> {
        if packet.len() >= MutableU32BEPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableU32BEPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableU32BEPacket to a U32BEPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> U32BEPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        U32BEPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableU32BEPacket to a U32BEPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> U32BEPacket<'a> {
        U32BEPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a U32BE instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &U32BE) -> usize { 4 + _packet.unused.len() }
    /// Populates a U32BEPacket using a U32BE structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &U32BE) {
        let _self = self;
        _self.set_number(packet.number);
        _self.set_unused(&packet.unused);
    }
    /// Get the number field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_number(&self) -> u32be {
        let _self = self;
        let co = 0;
        let b0 = ((_self.packet[co + 0] as u32be) << 24) as u32be;
        let b1 = ((_self.packet[co + 1] as u32be) << 16) as u32be;
        let b2 = ((_self.packet[co + 2] as u32be) << 8) as u32be;
        let b3 = ((_self.packet[co + 3] as u32be)) as u32be;
        b0 | b1 | b2 | b3
    }
    /// Set the number field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_number(&mut self, val: u32be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] = ((val & 4278190080) >> 24) as u8;
        _self.packet[co + 1] = ((val & 16711680) >> 16) as u8;
        _self.packet[co + 2] = ((val & 65280) >> 8) as u8;
        _self.packet[co + 3] = (val) as u8;
    }
    /// Set the value of the unused field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_unused(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 4;
        let len = 0;
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for U32BEPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 + 0 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableU32BEPacket<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 + 0 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableU32BEPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 4;
        let end = ::std::cmp::min(4 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableU32BEPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        let end = ::std::cmp::min(4 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for U32BEPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 4;
        let end = ::std::cmp::min(4 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `U32BEPacket`s
pub struct U32BEIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for U32BEIterable<'a> {
    type
    Item
    =
    U32BEPacket<'a>;
    fn next(&mut self) -> Option<U32BEPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = U32BEPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for U32BEPacket<'p> {
    type
    T
    =
    U32BE;
    #[inline]
    fn from_packet(&self) -> U32BE {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U32BE{number: _self.get_number(),
              unused:
                  {
                      let payload = self.payload();
                      let mut vec = Vec::with_capacity(payload.len());
                      vec.extend_from_slice(payload);
                      vec
                  },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableU32BEPacket<'p>
 {
    type
    T
    =
    U32BE;
    #[inline]
    fn from_packet(&self) -> U32BE {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        U32BE{number: _self.get_number(),
              unused:
                  {
                      let payload = self.payload();
                      let mut vec = Vec::with_capacity(payload.len());
                      vec.extend_from_slice(payload);
                      vec
                  },}
    }
}
impl <'p> ::std::fmt::Debug for U32BEPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt , "U32BEPacket {{ number : {:?},  }}" , _self . get_number
               (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableU32BEPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt , "MutableU32BEPacket {{ number : {:?},  }}" , _self .
               get_number (  ))
    }
}
/// `u32be`, but we can't use that directly in a `Vec` :(
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct U32BE {
    number: u32be,
    unused: Vec<u8>,
}
#[test]
fn gre_packet_test() {
    let mut packet = [0u8; 4];
    {
        let mut gre_packet = MutableGrePacket::new(&mut packet[..]).unwrap();
        gre_packet.set_protocol_type(2048);
        assert_eq!(gre_packet . payload (  ) . len (  ) , 0);
    }
    let ref_packet = [0, 0, 8, 0];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
#[test]
fn gre_checksum_test() {
    let mut packet = [0u8; 8];
    {
        let mut gre_packet = MutableGrePacket::new(&mut packet[..]).unwrap();
        gre_packet.set_checksum_present(1);
        assert_eq!(gre_packet . payload (  ) . len (  ) , 0);
        assert_eq!(gre_packet . get_checksum (  ) . len (  ) , 1);
        assert_eq!(gre_packet . get_offset (  ) . len (  ) , 1);
    }
    let ref_packet = [128, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
#[test]
fn gre_checksum_test1() {
    let x = pnet_packet::icmp::time_exceeded::TimeExceeded;
}
