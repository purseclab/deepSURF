// Copyright (c) 2014, 2015 Robert Clipsham <robert@octarineparrot.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use PrimitiveValues;
use ethernet::EtherType;

use std::net::Ipv4Addr;
use pnet_base::MacAddr;

/// Represents an ARP operation.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct ArpOperation(pub u16);

impl ArpOperation {
    /// Create a new `ArpOperation`.
    pub fn new(value: u16) -> Self { ArpOperation(value) }
}

impl PrimitiveValues for ArpOperation {
    type
    T
    =
    (u16,);
    fn to_primitive_values(&self) -> (u16,) { (self.0,) }
}

/// The ARP protocol operations.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ArpOperations {
    use super::ArpOperation;

    /// ARP request
    pub const Request: ArpOperation = ArpOperation(1);

    /// ARP reply
    pub const Reply: ArpOperation = ArpOperation(2);
}

/// Represents the ARP hardware types.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct ArpHardwareType(pub u16);

impl ArpHardwareType {
    /// Create a new `ArpHardwareType`.
    pub fn new(value: u16) -> Self { ArpHardwareType(value) }
}

impl PrimitiveValues for ArpHardwareType {
    type
    T
    =
    (u16,);
    fn to_primitive_values(&self) -> (u16,) { (self.0,) }
}

/// The ARP protocol hardware types.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ArpHardwareTypes {
    use super::ArpHardwareType;

    /// Ethernet
    pub const Ethernet: ArpHardwareType = ArpHardwareType(1);
}

// We completely ignore hw_addr_len and
// proto_addr_len and use values for
// Ipv4 on top of Ethernet as it's the
// most common use case
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct ArpPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableArpPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> ArpPacket<'a> {
    /// Constructs a new ArpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<ArpPacket<'p>> {
        if packet.len() >= ArpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(ArpPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new ArpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the ArpPacket will
    /// own its own data and the underlying buffer will be dropped when the ArpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<ArpPacket<'static>> {
        if packet.len() >= ArpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(ArpPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a ArpPacket to a ArpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> ArpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        ArpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a ArpPacket to a ArpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> ArpPacket<'a> {
        ArpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 28 }
    /// The size (in bytes) of a Arp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Arp) -> usize { 28 + _packet.payload.len() }
    /// Get the value of the hardware_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hardware_type(&self) -> ArpHardwareType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u16 {
            let co = 0;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        ArpHardwareType::new(get_arg0(&self))
    }
    /// Get the value of the protocol_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_protocol_type(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u16 {
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Get the hw_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hw_addr_len(&self) -> u8 {
        let _self = self;
        let co = 4;
        (_self.packet[co] as u8)
    }
    /// Get the proto_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_proto_addr_len(&self) -> u8 {
        let _self = self;
        let co = 5;
        (_self.packet[co] as u8)
    }
    /// Get the value of the operation field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_operation(&self) -> ArpOperation {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u16 {
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        ArpOperation::new(get_arg0(&self))
    }
    /// Get the value of the sender_hw_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sender_hw_addr(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &ArpPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &ArpPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &ArpPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &ArpPacket) -> u8 {
            let co = 12;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &ArpPacket) -> u8 {
            let co = 13;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the sender_proto_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sender_proto_addr(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u8 {
            let co = 14;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &ArpPacket) -> u8 {
            let co = 15;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &ArpPacket) -> u8 {
            let co = 16;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &ArpPacket) -> u8 {
            let co = 17;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the value of the target_hw_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_target_hw_addr(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u8 {
            let co = 18;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &ArpPacket) -> u8 {
            let co = 19;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &ArpPacket) -> u8 {
            let co = 20;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &ArpPacket) -> u8 {
            let co = 21;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &ArpPacket) -> u8 {
            let co = 22;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &ArpPacket) -> u8 {
            let co = 23;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the target_proto_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_target_proto_addr(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &ArpPacket) -> u8 {
            let co = 24;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &ArpPacket) -> u8 {
            let co = 25;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &ArpPacket) -> u8 {
            let co = 26;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &ArpPacket) -> u8 {
            let co = 27;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
}
impl <'a> MutableArpPacket<'a> {
    /// Constructs a new MutableArpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableArpPacket<'p>> {
        if packet.len() >= MutableArpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableArpPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableArpPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableArpPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableArpPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableArpPacket<'static>> {
        if packet.len() >= MutableArpPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableArpPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableArpPacket to a ArpPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> ArpPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        ArpPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableArpPacket to a ArpPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> ArpPacket<'a> {
        ArpPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 28 }
    /// The size (in bytes) of a Arp instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Arp) -> usize { 28 + _packet.payload.len() }
    /// Populates a ArpPacket using a Arp structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Arp) {
        let _self = self;
        _self.set_hardware_type(packet.hardware_type);
        _self.set_protocol_type(packet.protocol_type);
        _self.set_hw_addr_len(packet.hw_addr_len);
        _self.set_proto_addr_len(packet.proto_addr_len);
        _self.set_operation(packet.operation);
        _self.set_sender_hw_addr(packet.sender_hw_addr);
        _self.set_sender_proto_addr(packet.sender_proto_addr);
        _self.set_target_hw_addr(packet.target_hw_addr);
        _self.set_target_proto_addr(packet.target_proto_addr);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the hardware_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hardware_type(&self) -> ArpHardwareType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u16 {
            let co = 0;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        ArpHardwareType::new(get_arg0(&self))
    }
    /// Get the value of the protocol_type field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_protocol_type(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u16 {
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Get the hw_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_hw_addr_len(&self) -> u8 {
        let _self = self;
        let co = 4;
        (_self.packet[co] as u8)
    }
    /// Get the proto_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_proto_addr_len(&self) -> u8 {
        let _self = self;
        let co = 5;
        (_self.packet[co] as u8)
    }
    /// Get the value of the operation field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_operation(&self) -> ArpOperation {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u16 {
            let co = 6;
            let b0 = ((_self.packet[co + 0] as u16) << 8) as u16;
            let b1 = ((_self.packet[co + 1] as u16)) as u16;
            b0 | b1
        }
        ArpOperation::new(get_arg0(&self))
    }
    /// Get the value of the sender_hw_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sender_hw_addr(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u8 {
            let co = 8;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableArpPacket) -> u8 {
            let co = 9;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableArpPacket) -> u8 {
            let co = 10;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableArpPacket) -> u8 {
            let co = 11;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableArpPacket) -> u8 {
            let co = 12;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableArpPacket) -> u8 {
            let co = 13;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the sender_proto_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_sender_proto_addr(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u8 {
            let co = 14;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableArpPacket) -> u8 {
            let co = 15;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableArpPacket) -> u8 {
            let co = 16;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableArpPacket) -> u8 {
            let co = 17;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Get the value of the target_hw_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_target_hw_addr(&self) -> MacAddr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u8 {
            let co = 18;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableArpPacket) -> u8 {
            let co = 19;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableArpPacket) -> u8 {
            let co = 20;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableArpPacket) -> u8 {
            let co = 21;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg4(_self: &MutableArpPacket) -> u8 {
            let co = 22;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg5(_self: &MutableArpPacket) -> u8 {
            let co = 23;
            (_self.packet[co] as u8)
        }
        MacAddr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                     get_arg3(&self), get_arg4(&self), get_arg5(&self))
    }
    /// Get the value of the target_proto_addr field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_target_proto_addr(&self) -> Ipv4Addr {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableArpPacket) -> u8 {
            let co = 24;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg1(_self: &MutableArpPacket) -> u8 {
            let co = 25;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg2(_self: &MutableArpPacket) -> u8 {
            let co = 26;
            (_self.packet[co] as u8)
        }
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg3(_self: &MutableArpPacket) -> u8 {
            let co = 27;
            (_self.packet[co] as u8)
        }
        Ipv4Addr::new(get_arg0(&self), get_arg1(&self), get_arg2(&self),
                      get_arg3(&self))
    }
    /// Set the value of the hardware_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hardware_type(&mut self, val: ArpHardwareType) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u16) {
            let co = 0;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the protocol_type field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_protocol_type(&mut self, val: EtherType) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u16) {
            let co = 2;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the hw_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_hw_addr_len(&mut self, val: u8) {
        let _self = self;
        let co = 4;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the proto_addr_len field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_proto_addr_len(&mut self, val: u8) {
        let _self = self;
        let co = 5;
        _self.packet[co + 0] = (val) as u8;
    }
    /// Set the value of the operation field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_operation(&mut self, val: ArpOperation) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u16) {
            let co = 6;
            _self.packet[co + 0] = ((val & 65280) >> 8) as u8;
            _self.packet[co + 1] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the value of the sender_hw_addr field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sender_hw_addr(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u8) {
            let co = 8;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableArpPacket, val: u8) {
            let co = 9;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableArpPacket, val: u8) {
            let co = 10;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableArpPacket, val: u8) {
            let co = 11;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableArpPacket, val: u8) {
            let co = 12;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableArpPacket, val: u8) {
            let co = 13;
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
    /// Set the value of the sender_proto_addr field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_sender_proto_addr(&mut self, val: Ipv4Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u8) {
            let co = 14;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableArpPacket, val: u8) {
            let co = 15;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableArpPacket, val: u8) {
            let co = 16;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableArpPacket, val: u8) {
            let co = 17;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
    }
    /// Set the value of the target_hw_addr field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_target_hw_addr(&mut self, val: MacAddr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u8) {
            let co = 18;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableArpPacket, val: u8) {
            let co = 19;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableArpPacket, val: u8) {
            let co = 20;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableArpPacket, val: u8) {
            let co = 21;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg4(_self: &mut MutableArpPacket, val: u8) {
            let co = 22;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg5(_self: &mut MutableArpPacket, val: u8) {
            let co = 23;
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
    /// Set the value of the target_proto_addr field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_target_proto_addr(&mut self, val: Ipv4Addr) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableArpPacket, val: u8) {
            let co = 24;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg1(_self: &mut MutableArpPacket, val: u8) {
            let co = 25;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg2(_self: &mut MutableArpPacket, val: u8) {
            let co = 26;
            _self.packet[co + 0] = (val) as u8;
        }
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg3(_self: &mut MutableArpPacket, val: u8) {
            let co = 27;
            _self.packet[co + 0] = (val) as u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
        set_arg1(_self, vals.1);
        set_arg2(_self, vals.2);
        set_arg3(_self, vals.3);
    }
    /// Set the value of the payload field (copies contents)
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_payload(&mut self, vals: &[u8]) {
        use std::ptr::copy_nonoverlapping;
        let mut _self = self;
        let current_offset = 28;
        let len = 0;
        assert!(vals . len (  ) <= len);
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for ArpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 28 + 0 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableArpPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 28 + 0 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableArpPacket<'a> {
    #[inline]
    fn packet_mut<'p>(&'p mut self) -> &'p mut [u8] { &mut self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload_mut<'p>(&'p mut self) -> &'p mut [u8] {
        let _self = self;
        let start = 28;
        let end = ::std::cmp::min(28 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &mut []; }
        &mut _self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for MutableArpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 28;
        let end = ::std::cmp::min(28 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
impl <'a> ::pnet_macros_support::packet::Packet for ArpPacket<'a> {
    #[inline]
    fn packet<'p>(&'p self) -> &'p [u8] { &self.packet[..] }
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn payload<'p>(&'p self) -> &'p [u8] {
        let _self = self;
        let start = 28;
        let end = ::std::cmp::min(28 + 0, _self.packet.len());
        if _self.packet.len() <= start { return &[]; }
        &_self.packet[start..end]
    }
}
/// Used to iterate over a slice of `ArpPacket`s
pub struct ArpIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for ArpIterable<'a> {
    type
    Item
    =
    ArpPacket<'a>;
    fn next(&mut self) -> Option<ArpPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = ArpPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for ArpPacket<'p> {
    type
    T
    =
    Arp;
    #[inline]
    fn from_packet(&self) -> Arp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Arp{hardware_type: _self.get_hardware_type(),
            protocol_type: _self.get_protocol_type(),
            hw_addr_len: _self.get_hw_addr_len(),
            proto_addr_len: _self.get_proto_addr_len(),
            operation: _self.get_operation(),
            sender_hw_addr: _self.get_sender_hw_addr(),
            sender_proto_addr: _self.get_sender_proto_addr(),
            target_hw_addr: _self.get_target_hw_addr(),
            target_proto_addr: _self.get_target_proto_addr(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableArpPacket<'p> {
    type
    T
    =
    Arp;
    #[inline]
    fn from_packet(&self) -> Arp {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Arp{hardware_type: _self.get_hardware_type(),
            protocol_type: _self.get_protocol_type(),
            hw_addr_len: _self.get_hw_addr_len(),
            proto_addr_len: _self.get_proto_addr_len(),
            operation: _self.get_operation(),
            sender_hw_addr: _self.get_sender_hw_addr(),
            sender_proto_addr: _self.get_sender_proto_addr(),
            target_hw_addr: _self.get_target_hw_addr(),
            target_proto_addr: _self.get_target_proto_addr(),
            payload:
                {
                    let payload = self.payload();
                    let mut vec = Vec::with_capacity(payload.len());
                    vec.extend_from_slice(payload);
                    vec
                },}
    }
}
impl <'p> ::std::fmt::Debug for ArpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "ArpPacket {{ hardware_type : {:?}, protocol_type : {:?}, hw_addr_len : {:?}, proto_addr_len : {:?}, operation : {:?}, sender_hw_addr : {:?}, sender_proto_addr : {:?}, target_hw_addr : {:?}, target_proto_addr : {:?},  }}"
               , _self . get_hardware_type (  ) , _self . get_protocol_type (
               ) , _self . get_hw_addr_len (  ) , _self . get_proto_addr_len (
                ) , _self . get_operation (  ) , _self . get_sender_hw_addr (
               ) , _self . get_sender_proto_addr (  ) , _self .
               get_target_hw_addr (  ) , _self . get_target_proto_addr (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableArpPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableArpPacket {{ hardware_type : {:?}, protocol_type : {:?}, hw_addr_len : {:?}, proto_addr_len : {:?}, operation : {:?}, sender_hw_addr : {:?}, sender_proto_addr : {:?}, target_hw_addr : {:?}, target_proto_addr : {:?},  }}"
               , _self . get_hardware_type (  ) , _self . get_protocol_type (
               ) , _self . get_hw_addr_len (  ) , _self . get_proto_addr_len (
                ) , _self . get_operation (  ) , _self . get_sender_hw_addr (
               ) , _self . get_sender_proto_addr (  ) , _self .
               get_target_hw_addr (  ) , _self . get_target_proto_addr (  ))
    }
}
/// Represents an ARP Packet.
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Arp {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: EtherType,
    pub hw_addr_len: u8,
    pub proto_addr_len: u8,
    pub operation: ArpOperation,
    pub sender_hw_addr: MacAddr,
    pub sender_proto_addr: Ipv4Addr,
    pub target_hw_addr: MacAddr,
    pub target_proto_addr: Ipv4Addr,
    pub payload: Vec<u8>,
}
