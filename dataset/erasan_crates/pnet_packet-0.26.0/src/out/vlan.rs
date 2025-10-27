use PrimitiveValues;
use ethernet::EtherType;
use pnet_macros_support::types::*;

/// Represents an IEEE 802.1p class of a service.
#[derive(Hash, Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
pub struct ClassOfService(pub u3);

impl ClassOfService {
    /// Create a new `ClassOfService` instance.
    pub fn new(value: u3) -> ClassOfService { ClassOfService(value) }
}

impl PrimitiveValues for ClassOfService {
    type
    T
    =
    (u3,);
    fn to_primitive_values(&self) -> (u3,) { (self.0,) }
}

/// IEEE 802.1p classes of service as defined in
/// https://en.wikipedia.org/wiki/IEEE_P802.1p.
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod ClassesOfService {
    use super::ClassOfService;

    /// Background
    pub const BK: ClassOfService = ClassOfService(1);

    /// Best Effort
    pub const BE: ClassOfService = ClassOfService(0);

    /// Excellent Effort
    pub const EE: ClassOfService = ClassOfService(2);

    /// Critical Applications
    pub const CA: ClassOfService = ClassOfService(3);

    /// Video, < 100 ms latency
    pub const VI: ClassOfService = ClassOfService(4);

    /// Voice, < 10 ms latency
    pub const VO: ClassOfService = ClassOfService(5);

    /// Internetwork Control
    pub const IC: ClassOfService = ClassOfService(6);

    /// Network Control
    pub const NC: ClassOfService = ClassOfService(7);
}







 // PCP, DEI, and first nibble of VID
 // Remainder of VID
 // First byte of ethertype
 // Second byte of ethertype
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct VlanPacket<'p> {
    packet: ::pnet_macros_support::packet::PacketData<'p>,
}
#[derive(PartialEq)]
/// A structure enabling manipulation of on the wire packets
pub struct MutableVlanPacket<'p> {
    packet: ::pnet_macros_support::packet::MutPacketData<'p>,
}
impl <'a> VlanPacket<'a> {
    /// Constructs a new VlanPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p [u8]) -> Option<VlanPacket<'p>> {
        if packet.len() >= VlanPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(VlanPacket{packet: PacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new VlanPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the VlanPacket will
    /// own its own data and the underlying buffer will be dropped when the VlanPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<VlanPacket<'static>> {
        if packet.len() >= VlanPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::PacketData;
            Some(VlanPacket{packet: PacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a VlanPacket to a VlanPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> VlanPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        VlanPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a VlanPacket to a VlanPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> VlanPacket<'a> {
        VlanPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Vlan instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Vlan) -> usize { 4 + _packet.payload.len() }
    /// Get the value of the priority_code_point field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_priority_code_point(&self) -> ClassOfService {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &VlanPacket) -> u3 {
            let co = 0;
            ((_self.packet[co] as u3) & 224) >> 5
        }
        ClassOfService::new(get_arg0(&self))
    }
    /// Get the drop_eligible_indicator field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_drop_eligible_indicator(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the vlan_identifier field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_vlan_identifier(&self) -> u12be {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u12be) & 15) << 8) as u12be;
        let b1 = ((_self.packet[co + 1] as u12be)) as u12be;
        b0 | b1
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &VlanPacket) -> u16be {
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
}
impl <'a> MutableVlanPacket<'a> {
    /// Constructs a new MutableVlanPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None.
    #[inline]
    pub fn new<'p>(packet: &'p mut [u8]) -> Option<MutableVlanPacket<'p>> {
        if packet.len() >= MutableVlanPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableVlanPacket{packet: MutPacketData::Borrowed(packet),})
        } else { None }
    }
    /// Constructs a new MutableVlanPacket. If the provided buffer is less than the minimum required
    /// packet size, this will return None. With this constructor the MutableVlanPacket will
    /// own its own data and the underlying buffer will be dropped when the MutableVlanPacket is.
    pub fn owned(packet: Vec<u8>) -> Option<MutableVlanPacket<'static>> {
        if packet.len() >= MutableVlanPacket::minimum_packet_size() {
            use ::pnet_macros_support::packet::MutPacketData;
            Some(MutableVlanPacket{packet: MutPacketData::Owned(packet),})
        } else { None }
    }
    /// Maps from a MutableVlanPacket to a VlanPacket
    #[inline]
    pub fn to_immutable<'p>(&'p self) -> VlanPacket<'p> {
        use ::pnet_macros_support::packet::PacketData;
        VlanPacket{packet: PacketData::Borrowed(self.packet.as_slice()),}
    }
    /// Maps from a MutableVlanPacket to a VlanPacket while consuming the source
    #[inline]
    pub fn consume_to_immutable(self) -> VlanPacket<'a> {
        VlanPacket{packet: self.packet.to_immutable(),}
    }
    /// The minimum size (in bytes) a packet of this type can be. It's based on the total size
    /// of the fixed-size fields.
    #[inline]
    pub const fn minimum_packet_size() -> usize { 4 }
    /// The size (in bytes) of a Vlan instance when converted into
    /// a byte-array
    #[inline]
    pub fn packet_size(_packet: &Vlan) -> usize { 4 + _packet.payload.len() }
    /// Populates a VlanPacket using a Vlan structure
    #[inline]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn populate(&mut self, packet: &Vlan) {
        let _self = self;
        _self.set_priority_code_point(packet.priority_code_point);
        _self.set_drop_eligible_indicator(packet.drop_eligible_indicator);
        _self.set_vlan_identifier(packet.vlan_identifier);
        _self.set_ethertype(packet.ethertype);
        _self.set_payload(&packet.payload);
    }
    /// Get the value of the priority_code_point field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_priority_code_point(&self) -> ClassOfService {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableVlanPacket) -> u3 {
            let co = 0;
            ((_self.packet[co] as u3) & 224) >> 5
        }
        ClassOfService::new(get_arg0(&self))
    }
    /// Get the drop_eligible_indicator field.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_drop_eligible_indicator(&self) -> u1 {
        let _self = self;
        let co = 0;
        ((_self.packet[co] as u1) & 16) >> 4
    }
    /// Get the vlan_identifier field. This field is always stored big-endian
    /// within the struct, but this accessor returns host order.
    #[inline]
    #[allow(trivial_numeric_casts, unused_parens)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_vlan_identifier(&self) -> u12be {
        let _self = self;
        let co = 0;
        let b0 = (((_self.packet[co + 0] as u12be) & 15) << 8) as u12be;
        let b1 = ((_self.packet[co + 1] as u12be)) as u12be;
        b0 | b1
    }
    /// Get the value of the ethertype field
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn get_ethertype(&self) -> EtherType {
        #[inline(always)]
        #[allow(trivial_numeric_casts, unused_parens)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn get_arg0(_self: &MutableVlanPacket) -> u16be {
            let co = 2;
            let b0 = ((_self.packet[co + 0] as u16be) << 8) as u16be;
            let b1 = ((_self.packet[co + 1] as u16be)) as u16be;
            b0 | b1
        }
        EtherType::new(get_arg0(&self))
    }
    /// Set the value of the priority_code_point field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_priority_code_point(&mut self, val: ClassOfService) {
        use pnet_macros_support::packet::PrimitiveValues;
        let _self = self;
        #[inline]
        #[allow(trivial_numeric_casts)]
        #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
        fn set_arg0(_self: &mut MutableVlanPacket, val: u3) {
            let co = 0;
            _self.packet[co + 0] =
                ((_self.packet[co + 0] & 31) | (((val & 7) << 5) as u8)) as
                    u8;
        }
        let vals = val.to_primitive_values();
        set_arg0(_self, vals.0);
    }
    /// Set the drop_eligible_indicator field.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_drop_eligible_indicator(&mut self, val: u1) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 239) | (((val & 1) << 4) as u8)) as u8;
    }
    /// Set the vlan_identifier field. This field is always stored big-endian
    /// within the struct, but this mutator wants host order.
    #[inline]
    #[allow(trivial_numeric_casts)]
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    pub fn set_vlan_identifier(&mut self, val: u12be) {
        let _self = self;
        let co = 0;
        _self.packet[co + 0] =
            ((_self.packet[co + 0] & 240) | (((val & 3840) >> 8) as u8)) as
                u8;
        _self.packet[co + 1] = (val) as u8;
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
        fn set_arg0(_self: &mut MutableVlanPacket, val: u16be) {
            let co = 2;
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
        let current_offset = 4;
        unsafe {
            copy_nonoverlapping(vals[..].as_ptr(),
                                _self.packet[current_offset..].as_mut_ptr(),
                                vals.len())
        }
    }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for VlanPacket<'a> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::PacketSize for MutableVlanPacket<'a>
 {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn packet_size(&self) -> usize { let _self = self; 4 }
}
impl <'a> ::pnet_macros_support::packet::MutablePacket for
 MutableVlanPacket<'a> {
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
impl <'a> ::pnet_macros_support::packet::Packet for MutableVlanPacket<'a> {
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
impl <'a> ::pnet_macros_support::packet::Packet for VlanPacket<'a> {
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
/// Used to iterate over a slice of `VlanPacket`s
pub struct VlanIterable<'a> {
    buf: &'a [u8],
}
impl <'a> Iterator for VlanIterable<'a> {
    type
    Item
    =
    VlanPacket<'a>;
    fn next(&mut self) -> Option<VlanPacket<'a>> {
        use pnet_macros_support::packet::PacketSize;
        use std::cmp::min;
        if self.buf.len() > 0 {
            if let Some(ret) = VlanPacket::new(self.buf) {
                let start = min(ret.packet_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) { (0, None) }
}
impl <'p> ::pnet_macros_support::packet::FromPacket for VlanPacket<'p> {
    type
    T
    =
    Vlan;
    #[inline]
    fn from_packet(&self) -> Vlan {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Vlan{priority_code_point: _self.get_priority_code_point(),
             drop_eligible_indicator: _self.get_drop_eligible_indicator(),
             vlan_identifier: _self.get_vlan_identifier(),
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
impl <'p> ::pnet_macros_support::packet::FromPacket for MutableVlanPacket<'p>
 {
    type
    T
    =
    Vlan;
    #[inline]
    fn from_packet(&self) -> Vlan {
        use pnet_macros_support::packet::Packet;
        let _self = self;
        Vlan{priority_code_point: _self.get_priority_code_point(),
             drop_eligible_indicator: _self.get_drop_eligible_indicator(),
             vlan_identifier: _self.get_vlan_identifier(),
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
impl <'p> ::std::fmt::Debug for VlanPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "VlanPacket {{ priority_code_point : {:?}, drop_eligible_indicator : {:?}, vlan_identifier : {:?}, ethertype : {:?},  }}"
               , _self . get_priority_code_point (  ) , _self .
               get_drop_eligible_indicator (  ) , _self . get_vlan_identifier
               (  ) , _self . get_ethertype (  ))
    }
}
impl <'p> ::std::fmt::Debug for MutableVlanPacket<'p> {
    #[cfg_attr(feature = "clippy", allow(used_underscore_binding))]
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let _self = self;
        write!(fmt ,
               "MutableVlanPacket {{ priority_code_point : {:?}, drop_eligible_indicator : {:?}, vlan_identifier : {:?}, ethertype : {:?},  }}"
               , _self . get_priority_code_point (  ) , _self .
               get_drop_eligible_indicator (  ) , _self . get_vlan_identifier
               (  ) , _self . get_ethertype (  ))
    }
}
/// Represents a VLAN-tagged packet.
#[derive(Clone, Debug)]
#[allow(unused_attributes)]
pub struct Vlan {
    pub priority_code_point: ClassOfService,
    pub drop_eligible_indicator: u1,
    pub vlan_identifier: u12be,
    pub ethertype: EtherType,
    pub payload: Vec<u8>,
}
#[cfg(test)]
mod tests {
    use ethernet::EtherTypes;
    use super::*;
    #[test]
    fn vlan_packet_test() {
        let mut packet = [0u8; 4];
        {
            let mut vlan_header =
                MutableVlanPacket::new(&mut packet[..]).unwrap();
            vlan_header.set_priority_code_point(ClassesOfService::BE);
            assert_eq!(vlan_header . get_priority_code_point (  ) ,
                       ClassesOfService :: BE);
            vlan_header.set_drop_eligible_indicator(0);
            assert_eq!(vlan_header . get_drop_eligible_indicator (  ) , 0);
            vlan_header.set_ethertype(EtherTypes::Ipv4);
            assert_eq!(vlan_header . get_ethertype (  ) , EtherTypes :: Ipv4);
            vlan_header.set_vlan_identifier(256);
            assert_eq!(vlan_header . get_vlan_identifier (  ) , 0x100);
        }
        let ref_packet = [1, 0, 8, 0];
        assert_eq!(& ref_packet [ .. ] , & packet [ .. ]);
    }
}
