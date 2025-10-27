// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PrimitiveValues;

use std::fmt;
use pnet_base::MacAddr;






 /* destination */
 /* source */
/* ethertype */





 //(0x0800)
 //(0x0806)
 //(0x0842)
 //(0x22F3)
 //(0x6003)
 //(0x8035)
 //(0x809B)
 //(0x80F3)
 //(0x8137)
 //(0x8204)
 //(0x86DD)
 //(0x8808)
 //(0x8819)
 //(0x8847)
 //(0x8848)
 //(0x8863)
 //(0x8864)
 //(0x8100)
 //(0x88a8)
 //(0x88cc)
 //(0x88f7)
 //(0x8902)
 //(0x9100)








#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct EthernetPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableEthernetPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> EthernetPacket<'a> {
    /// Constructs a new EthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<EthernetPacket<'p>> {
        if packet.len() >= EthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(EthernetPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new EthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the EthernetPacket will
    /// own its own data and the underlying buffer will be dropped when the EthernetPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<EthernetPacket<'static>> {
        if packet.len() >= EthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(EthernetPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a EthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        EthernetPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a EthernetPacket to a EthernetPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> EthernetPacket<'a> {
        EthernetPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 14 }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &EthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &EthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &EthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &EthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &EthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &EthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
}
impl <'a> MutableEthernetPacket<'a> {
    /// Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8])
     -> Option<MutableEthernetPacket<'p>> {
        if packet.len() >= MutableEthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableEthernetPacket{packet:
                                           MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableEthernetPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableEthernetPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableEthernetPacket<'static>> {
        if packet.len() >= MutableEthernetPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableEthernetPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableEthernetPacket to a EthernetPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> EthernetPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        EthernetPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableEthernetPacket to a EthernetPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> EthernetPacket<'a> {
        EthernetPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 14 }
    /// The size (in bytes) of a Ethernet instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Ethernet) -> usize {
        14 + _packet.payload.len()
    }
    /// Populates a EthernetPacket using a Ethernet structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Ethernet) {
        let _self = self;
        _self.set_destination(packet.destination);
        _self.set_source(packet.source);
        _self.set_ethertype(packet.ethertype);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the destination field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_destination(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 0;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 1;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 2;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 3;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 4;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 5;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the source field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_source(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u8 {
            let co = 6;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableEthernetPacket) -> u8 {
            let co = 7;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableEthernetPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableEthernetPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableEthernetPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableEthernetPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableEthernetPacket) -> u16 {
            let co = 12;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Set the value of the destination field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_destination(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 0;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 1;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 2;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 3;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 4;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 5;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the source field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_source(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 6;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 7;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 8;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 9;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 10;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableEthernetPacket, val: u8) {
            let co = 11;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
        set_arg4(_self, vals.4);
        set_arg5(_self, vals.5);
    }
    /// Set the value of the ethertype field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_ethertype(&mut self, val: EtherType) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableEthernetPacket, val: u16) {
            let co = 12;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 14;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for EthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 14 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for
 MutableEthernetPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 14 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableEthernetPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableEthernetPacket<'a>
 {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for EthernetPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 14;
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..]
    }
}
/// Used to iterate over a slice of `EthernetPacket`s
pub struct EthernetIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for EthernetIterable<'a> {
    type
    Item
    =
    EthernetPacket<'a>;
    fn next(&mut self) -> Option<EthernetPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = EthernetPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for EthernetPacket<'p> {
    type
    T
    =
    Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ethernet{destination: _self.get_destination(),
                 source: _self.get_source(),
                 ethertype: _self.get_ethertype(),
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
 MutableEthernetPacket<'p> {
    type
    T
    =
    Ethernet;
    #[inline]
    fn from_packet(&self) -> Ethernet {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Ethernet{destination: _self.get_destination(),
                 source: _self.get_source(),
                 ethertype: _self.get_ethertype(),
                 payload:
                     {
                         let payload = self.payload();
                         let mut vec = Vec::with_capacity(payload.len());
                         vec.extend_from_slice(payload);
                         vec
                     },}
    }
}
impl <'p> ::std::fmt::Debug for EthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "EthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}"
               , _self . get_destination (  ) , _self . get_source (  ) ,
               _self . get_ethertype (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableEthernetPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableEthernetPacket {{ destination : {:?}, source : {:?}, ethertype : {:?},  }}"
               , _self . get_destination (  ) , _self . get_source (  ) ,
               _self . get_ethertype (  ))
    }
}
/// Represents an Ethernet packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Ethernet {
    pub destination: MacAddr,
    pub source: MacAddr,
    pub ethertype: EtherType,
    pub payload: Vec<u8>,
}
#[test]
fn ethernet_header_test() {
    let mut packet = [0u8; 14];
    {
        let mut ethernet_header =
            MutableEthernetPacket::new(&mut packet[..]).unwrap();
        let source = MacAddr(18, 52, 86, 120, 154, 188);
        ethernet_header.set_source(source);
        assert_eq!(ethernet_header . get_source (  ) , source);
        let dest = MacAddr(222, 240, 18, 52, 69, 103);
        ethernet_header.set_destination(dest);
        assert_eq!(ethernet_header . get_destination (  ) , dest);
        ethernet_header.set_ethertype(EtherTypes::Ipv6);
        assert_eq!(ethernet_header . get_ethertype (  ) , EtherTypes :: Ipv6);
    }
    let ref_packet =
        [222, 240, 18, 52, 69, 103, 18, 52, 86, 120, 154, 188, 134, 221];
    assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
}
/// `EtherTypes` are defined at:
/// http://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml.
/// These values should be used in the `Ethernet` `EtherType` field.
///
/// FIXME Should include all
/// A handful of these have been selected since most are archaic and unused.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod EtherTypes {
    use ethernet::EtherType;
    /// Internet Protocol version 4 (IPv4) [RFC7042].
    pub const Ipv4: EtherType = EtherType(2048);
    /// Address Resolution Protocol (ARP) [RFC7042].
    pub const Arp: EtherType = EtherType(2054);
    /// Wake on Lan.
    pub const WakeOnLan: EtherType = EtherType(2114);
    /// IETF TRILL Protocol [IEEE].
    pub const Trill: EtherType = EtherType(8947);
    /// DECnet Phase IV.
    pub const DECnet: EtherType = EtherType(24579);
    /// Reverse Address Resolution Protocol (RARP) [RFC903].
    pub const Rarp: EtherType = EtherType(32821);
    /// AppleTalk - EtherTalk [Apple].
    pub const AppleTalk: EtherType = EtherType(32923);
    /// AppleTalk Address Resolution Protocol (AARP) [Apple].
    pub const Aarp: EtherType = EtherType(33011);
    /// IPX [Xerox].
    pub const Ipx: EtherType = EtherType(33079);
    /// QNX Qnet [QNX Software Systems].
    pub const Qnx: EtherType = EtherType(33284);
    /// Internet Protocol version 6 (IPv6) [RFC7042].
    pub const Ipv6: EtherType = EtherType(34525);
    /// Ethernet Flow Control [IEEE 802.3x].
    pub const FlowControl: EtherType = EtherType(34824);
    /// CobraNet [CobraNet].
    pub const CobraNet: EtherType = EtherType(34841);
    /// MPLS Unicast [RFC 3032].
    pub const Mpls: EtherType = EtherType(34887);
    /// MPLS Multicast [RFC 5332].
    pub const MplsMcast: EtherType = EtherType(34888);
    /// PPPOE Discovery Stage [RFC 2516].
    pub const PppoeDiscovery: EtherType = EtherType(34915);
    /// PPPoE Session Stage [RFC 2516].
    pub const PppoeSession: EtherType = EtherType(34916);
    /// VLAN-tagged frame (IEEE 802.1Q).
    pub const Vlan: EtherType = EtherType(33024);
    /// Provider Bridging [IEEE 802.1ad / IEEE 802.1aq].
    pub const PBridge: EtherType = EtherType(34984);
    /// Link Layer Discovery Protocol (LLDP) [IEEE 802.1AB].
    pub const Lldp: EtherType = EtherType(35020);
    /// Precision Time Protocol (PTP) over Ethernet [IEEE 1588].
    pub const Ptp: EtherType = EtherType(35063);
    /// CFM / Y.1731 [IEEE 802.1ag].
    pub const Cfm: EtherType = EtherType(35074);
    /// Q-in-Q Vlan Tagging [IEEE 802.1Q].
    pub const QinQ: EtherType = EtherType(37120);
}
/// Represents the `Ethernet::ethertype` field.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Clone, Copy)]
pub struct EtherType(pub u16);
impl EtherType {
    /// Construct a new `EtherType` instance.
    pub fn new(val: u16) -> EtherType { EtherType(val) }
}
impl PrimitiveValues for EtherType {
    type
    T
    =
    (u16,);
    fn to_primitive_values(&self) -> (u16,) { (self.0,) }
}
impl fmt::Display for EtherType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f , "{}" , match self {
               & EtherTypes :: Ipv4 => "Ipv4" , & EtherTypes :: Arp => "Arp" ,
               & EtherTypes :: WakeOnLan => "WakeOnLan" , & EtherTypes ::
               Trill => "Trill" , & EtherTypes :: DECnet => "DECnet" , &
               EtherTypes :: Rarp => "Rarp" , & EtherTypes :: AppleTalk =>
               "AppleTalk" , & EtherTypes :: Aarp => "Aarp" , & EtherTypes ::
               Ipx => "Ipx" , & EtherTypes :: Qnx => "Qnx" , & EtherTypes ::
               Ipv6 => "Ipv6" , & EtherTypes :: FlowControl => "FlowControl" ,
               & EtherTypes :: CobraNet => "CobraNet" , & EtherTypes :: Mpls
               => "Mpls" , & EtherTypes :: MplsMcast => "MplsMcast" , &
               EtherTypes :: PppoeDiscovery => "PppoeDiscovery" , & EtherTypes
               :: PppoeSession => "PppoeSession" , & EtherTypes :: Vlan =>
               "Vlan" , & EtherTypes :: PBridge => "PBridge" , & EtherTypes ::
               Lldp => "Lldp" , & EtherTypes :: Ptp => "Ptp" , & EtherTypes ::
               Cfm => "Cfm" , & EtherTypes :: QinQ => "QinQ" , _ => "unknown"
               , })
    }
}
#[test]
fn ether_type_to_str() {
    let ipv4 = EtherType(2048);
    assert_eq!(format ! ( "{}" , ipv4 ) , "Ipv4");
    let arp = EtherType(2054);
    assert_eq!(format ! ( "{}" , arp ) , "Arp");
    let unknown = EtherType(1638);
    assert_eq!(format ! ( "{}" , unknown ) , "unknown");
}
#[cfg(all(test, feature = "benchmark"))]
mod packet_benchmarks {
    use super::*;
    use test::{Bencher, black_box};
    use util::MacAddr;
    #[bench]
    fn bench_packet_new_constructor(b: &mut Bencher) {
        let buffer = vec!(0 ; 20);
        b.iter(|| EthernetPacket::new(black_box(&buffer)).unwrap());
    }
    #[bench]
    fn bench_packet_get_source(b: &mut Bencher) {
        let buffer = vec!(0 ; 20);
        let packet = EthernetPacket::new(&buffer).unwrap();
        b.iter(|| black_box(packet.get_source()));
    }
    #[bench]
    fn bench_packet_set_source_black_box(b: &mut Bencher) {
        let mut buffer = vec!(0 ; 20);
        let mut packet = MutableEthernetPacket::new(&mut buffer).unwrap();
        let mac = MacAddr::new(1, 2, 3, 4, 5, 6);
        b.iter(|| packet.set_source(black_box(mac)));
    }
    #[bench]
    fn bench_packet_mutable_to_immutable(b: &mut Bencher) {
        let mut buffer = vec!(0 ; 20);
        let mut packet = MutableEthernetPacket::new(&mut buffer).unwrap();
        b.iter(|| black_box(packet.to_immutable()));
    }
    #[bench]
    fn bench_packet_immutable_to_immutable(b: &mut Bencher) {
        let mut buffer = vec!(0 ; 20);
        let mut packet = EthernetPacket::new(&mut buffer).unwrap();
        b.iter(|| black_box(packet.to_immutable()));
    }
}
