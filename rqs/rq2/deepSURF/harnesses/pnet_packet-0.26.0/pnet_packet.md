# Crate Documentation

**Version:** 0.26.0

**Format Version:** 39

# Module `pnet_packet`

Provides interfaces for interacting with packets and headers.

## Modules

## Module `arp`

ARP packet abstraction.

```rust
pub mod arp { /* ... */ }
```

### Modules

## Module `ArpOperations`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The ARP protocol operations.

```rust
pub mod ArpOperations { /* ... */ }
```

### Constants and Statics

#### Constant `Request`

ARP request

```rust
pub const Request: super::ArpOperation = _;
```

#### Constant `Reply`

ARP reply

```rust
pub const Reply: super::ArpOperation = _;
```

## Module `ArpHardwareTypes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The ARP protocol hardware types.

```rust
pub mod ArpHardwareTypes { /* ... */ }
```

### Constants and Statics

#### Constant `Ethernet`

Ethernet

```rust
pub const Ethernet: super::ArpHardwareType = _;
```

### Types

#### Struct `ArpOperation`

Represents an ARP operation.

```rust
pub struct ArpOperation(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u16) -> Self { /* ... */ }
  ```
  Create a new `ArpOperation`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArpOperation) -> bool { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArpOperation { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArpOperation) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArpOperation) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `ArpHardwareType`

Represents the ARP hardware types.

```rust
pub struct ArpHardwareType(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u16) -> Self { /* ... */ }
  ```
  Create a new `ArpHardwareType`.

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArpHardwareType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ArpHardwareType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArpHardwareType { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArpHardwareType) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

#### Struct `ArpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct ArpPacket<''p> {
    pub(in ::arp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<ArpPacket<''p>> { /* ... */ }
  ```
  Constructs a new ArpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<ArpPacket<''static>> { /* ... */ }
  ```
  Constructs a new ArpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> ArpPacket<''p> { /* ... */ }
  ```
  Maps from a ArpPacket to a ArpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> ArpPacket<''a> { /* ... */ }
  ```
  Maps from a ArpPacket to a ArpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Arp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Arp instance when converted into

- ```rust
  pub fn get_hardware_type(self: &Self) -> ArpHardwareType { /* ... */ }
  ```
  Get the value of the hardware_type field

- ```rust
  pub fn get_protocol_type(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the protocol_type field

- ```rust
  pub fn get_hw_addr_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hw_addr_len field.

- ```rust
  pub fn get_proto_addr_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the proto_addr_len field.

- ```rust
  pub fn get_operation(self: &Self) -> ArpOperation { /* ... */ }
  ```
  Get the value of the operation field

- ```rust
  pub fn get_sender_hw_addr(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the sender_hw_addr field

- ```rust
  pub fn get_sender_proto_addr(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the sender_proto_addr field

- ```rust
  pub fn get_target_hw_addr(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the target_hw_addr field

- ```rust
  pub fn get_target_proto_addr(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the target_proto_addr field

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArpPacket<''p>) -> bool { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Arp { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `MutableArpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableArpPacket<''p> {
    pub(in ::arp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableArpPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableArpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableArpPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableArpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> ArpPacket<''p> { /* ... */ }
  ```
  Maps from a MutableArpPacket to a ArpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> ArpPacket<''a> { /* ... */ }
  ```
  Maps from a MutableArpPacket to a ArpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Arp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Arp instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Arp) { /* ... */ }
  ```
  Populates a ArpPacket using a Arp structure

- ```rust
  pub fn get_hardware_type(self: &Self) -> ArpHardwareType { /* ... */ }
  ```
  Get the value of the hardware_type field

- ```rust
  pub fn get_protocol_type(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the protocol_type field

- ```rust
  pub fn get_hw_addr_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hw_addr_len field.

- ```rust
  pub fn get_proto_addr_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the proto_addr_len field.

- ```rust
  pub fn get_operation(self: &Self) -> ArpOperation { /* ... */ }
  ```
  Get the value of the operation field

- ```rust
  pub fn get_sender_hw_addr(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the sender_hw_addr field

- ```rust
  pub fn get_sender_proto_addr(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the sender_proto_addr field

- ```rust
  pub fn get_target_hw_addr(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the target_hw_addr field

- ```rust
  pub fn get_target_proto_addr(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the target_proto_addr field

- ```rust
  pub fn set_hardware_type(self: &mut Self, val: ArpHardwareType) { /* ... */ }
  ```
  Set the value of the hardware_type field.

- ```rust
  pub fn set_protocol_type(self: &mut Self, val: EtherType) { /* ... */ }
  ```
  Set the value of the protocol_type field.

- ```rust
  pub fn set_hw_addr_len(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the hw_addr_len field.

- ```rust
  pub fn set_proto_addr_len(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the proto_addr_len field.

- ```rust
  pub fn set_operation(self: &mut Self, val: ArpOperation) { /* ... */ }
  ```
  Set the value of the operation field.

- ```rust
  pub fn set_sender_hw_addr(self: &mut Self, val: MacAddr) { /* ... */ }
  ```
  Set the value of the sender_hw_addr field.

- ```rust
  pub fn set_sender_proto_addr(self: &mut Self, val: Ipv4Addr) { /* ... */ }
  ```
  Set the value of the sender_proto_addr field.

- ```rust
  pub fn set_target_hw_addr(self: &mut Self, val: MacAddr) { /* ... */ }
  ```
  Set the value of the target_hw_addr field.

- ```rust
  pub fn set_target_proto_addr(self: &mut Self, val: Ipv4Addr) { /* ... */ }
  ```
  Set the value of the target_proto_addr field.

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Arp { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableArpPacket<''p>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ArpIterable`

Used to iterate over a slice of `ArpPacket`s

```rust
pub struct ArpIterable<''a> {
    pub(in ::arp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<ArpPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Arp`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(unused_attributes)]`

Represents an ARP Packet.

```rust
pub struct Arp {
    pub hardware_type: ArpHardwareType,
    pub protocol_type: ethernet::EtherType,
    pub hw_addr_len: u8,
    pub proto_addr_len: u8,
    pub operation: ArpOperation,
    pub sender_hw_addr: pnet_base::MacAddr,
    pub sender_proto_addr: std::net::Ipv4Addr,
    pub target_hw_addr: pnet_base::MacAddr,
    pub target_proto_addr: std::net::Ipv4Addr,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `hardware_type` | `ArpHardwareType` |  |
| `protocol_type` | `ethernet::EtherType` |  |
| `hw_addr_len` | `u8` |  |
| `proto_addr_len` | `u8` |  |
| `operation` | `ArpOperation` |  |
| `sender_hw_addr` | `pnet_base::MacAddr` |  |
| `sender_proto_addr` | `std::net::Ipv4Addr` |  |
| `target_hw_addr` | `pnet_base::MacAddr` |  |
| `target_proto_addr` | `std::net::Ipv4Addr` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Arp { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `ethernet`

An ethernet packet abstraction.

```rust
pub mod ethernet { /* ... */ }
```

### Modules

## Module `EtherTypes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

`EtherTypes` are defined at:
http://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml.
These values should be used in the `Ethernet` `EtherType` field.

FIXME Should include all
A handful of these have been selected since most are archaic and unused.

```rust
pub mod EtherTypes { /* ... */ }
```

### Constants and Statics

#### Constant `Ipv4`

Internet Protocol version 4 (IPv4) [RFC7042].

```rust
pub const Ipv4: ethernet::EtherType = _;
```

#### Constant `Arp`

Address Resolution Protocol (ARP) [RFC7042].

```rust
pub const Arp: ethernet::EtherType = _;
```

#### Constant `WakeOnLan`

Wake on Lan.

```rust
pub const WakeOnLan: ethernet::EtherType = _;
```

#### Constant `Trill`

IETF TRILL Protocol [IEEE].

```rust
pub const Trill: ethernet::EtherType = _;
```

#### Constant `DECnet`

DECnet Phase IV.

```rust
pub const DECnet: ethernet::EtherType = _;
```

#### Constant `Rarp`

Reverse Address Resolution Protocol (RARP) [RFC903].

```rust
pub const Rarp: ethernet::EtherType = _;
```

#### Constant `AppleTalk`

AppleTalk - EtherTalk [Apple].

```rust
pub const AppleTalk: ethernet::EtherType = _;
```

#### Constant `Aarp`

AppleTalk Address Resolution Protocol (AARP) [Apple].

```rust
pub const Aarp: ethernet::EtherType = _;
```

#### Constant `Ipx`

IPX [Xerox].

```rust
pub const Ipx: ethernet::EtherType = _;
```

#### Constant `Qnx`

QNX Qnet [QNX Software Systems].

```rust
pub const Qnx: ethernet::EtherType = _;
```

#### Constant `Ipv6`

Internet Protocol version 6 (IPv6) [RFC7042].

```rust
pub const Ipv6: ethernet::EtherType = _;
```

#### Constant `FlowControl`

Ethernet Flow Control [IEEE 802.3x].

```rust
pub const FlowControl: ethernet::EtherType = _;
```

#### Constant `CobraNet`

CobraNet [CobraNet].

```rust
pub const CobraNet: ethernet::EtherType = _;
```

#### Constant `Mpls`

MPLS Unicast [RFC 3032].

```rust
pub const Mpls: ethernet::EtherType = _;
```

#### Constant `MplsMcast`

MPLS Multicast [RFC 5332].

```rust
pub const MplsMcast: ethernet::EtherType = _;
```

#### Constant `PppoeDiscovery`

PPPOE Discovery Stage [RFC 2516].

```rust
pub const PppoeDiscovery: ethernet::EtherType = _;
```

#### Constant `PppoeSession`

PPPoE Session Stage [RFC 2516].

```rust
pub const PppoeSession: ethernet::EtherType = _;
```

#### Constant `Vlan`

VLAN-tagged frame (IEEE 802.1Q).

```rust
pub const Vlan: ethernet::EtherType = _;
```

#### Constant `PBridge`

Provider Bridging [IEEE 802.1ad / IEEE 802.1aq].

```rust
pub const PBridge: ethernet::EtherType = _;
```

#### Constant `Lldp`

Link Layer Discovery Protocol (LLDP) [IEEE 802.1AB].

```rust
pub const Lldp: ethernet::EtherType = _;
```

#### Constant `Ptp`

Precision Time Protocol (PTP) over Ethernet [IEEE 1588].

```rust
pub const Ptp: ethernet::EtherType = _;
```

#### Constant `Cfm`

CFM / Y.1731 [IEEE 802.1ag].

```rust
pub const Cfm: ethernet::EtherType = _;
```

#### Constant `QinQ`

Q-in-Q Vlan Tagging [IEEE 802.1Q].

```rust
pub const QinQ: ethernet::EtherType = _;
```

### Types

#### Struct `EthernetPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct EthernetPacket<''p> {
    pub(in ::ethernet) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<EthernetPacket<''p>> { /* ... */ }
  ```
  Constructs a new EthernetPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<EthernetPacket<''static>> { /* ... */ }
  ```
  Constructs a new EthernetPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EthernetPacket<''p> { /* ... */ }
  ```
  Maps from a EthernetPacket to a EthernetPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EthernetPacket<''a> { /* ... */ }
  ```
  Maps from a EthernetPacket to a EthernetPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ethernet) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ethernet instance when converted into

- ```rust
  pub fn get_destination(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the destination field

- ```rust
  pub fn get_source(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_ethertype(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the ethertype field

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ethernet { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EthernetPacket<''p>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `MutableEthernetPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableEthernetPacket<''p> {
    pub(in ::ethernet) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableEthernetPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableEthernetPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableEthernetPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EthernetPacket<''p> { /* ... */ }
  ```
  Maps from a MutableEthernetPacket to a EthernetPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EthernetPacket<''a> { /* ... */ }
  ```
  Maps from a MutableEthernetPacket to a EthernetPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ethernet) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ethernet instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Ethernet) { /* ... */ }
  ```
  Populates a EthernetPacket using a Ethernet structure

- ```rust
  pub fn get_destination(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the destination field

- ```rust
  pub fn get_source(self: &Self) -> MacAddr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_ethertype(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the ethertype field

- ```rust
  pub fn set_destination(self: &mut Self, val: MacAddr) { /* ... */ }
  ```
  Set the value of the destination field.

- ```rust
  pub fn set_source(self: &mut Self, val: MacAddr) { /* ... */ }
  ```
  Set the value of the source field.

- ```rust
  pub fn set_ethertype(self: &mut Self, val: EtherType) { /* ... */ }
  ```
  Set the value of the ethertype field.

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **StructuralPartialEq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableEthernetPacket<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ethernet { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `EthernetIterable`

Used to iterate over a slice of `EthernetPacket`s

```rust
pub struct EthernetIterable<''a> {
    pub(in ::ethernet) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<EthernetPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Ethernet`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an Ethernet packet.

```rust
pub struct Ethernet {
    pub destination: pnet_base::MacAddr,
    pub source: pnet_base::MacAddr,
    pub ethertype: EtherType,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `destination` | `pnet_base::MacAddr` |  |
| `source` | `pnet_base::MacAddr` |  |
| `ethertype` | `EtherType` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ethernet { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `EtherType`

Represents the `Ethernet::ethertype` field.

```rust
pub struct EtherType(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u16) -> EtherType { /* ... */ }
  ```
  Construct a new `EtherType` instance.

###### Trait Implementations

- **Send**
- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &EtherType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EtherType) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EtherType { /* ... */ }
    ```

- **Copy**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &EtherType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

## Module `gre`

Minimal GRE Packet implementation: suitable for inspection not generation (e.g. checksum not
implemented).

```rust
pub mod gre { /* ... */ }
```

### Types

#### Struct `GrePacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct GrePacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<GrePacket<''p>> { /* ... */ }
  ```
  Constructs a new GrePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<GrePacket<''static>> { /* ... */ }
  ```
  Constructs a new GrePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> GrePacket<''p> { /* ... */ }
  ```
  Maps from a GrePacket to a GrePacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> GrePacket<''a> { /* ... */ }
  ```
  Maps from a GrePacket to a GrePacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Gre) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Gre instance when converted into

- ```rust
  pub fn get_checksum_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the checksum_present field.

- ```rust
  pub fn get_routing_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the routing_present field.

- ```rust
  pub fn get_key_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the key_present field.

- ```rust
  pub fn get_sequence_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the sequence_present field.

- ```rust
  pub fn get_strict_source_route(self: &Self) -> u1 { /* ... */ }
  ```
  Get the strict_source_route field.

- ```rust
  pub fn get_recursion_control(self: &Self) -> u3 { /* ... */ }
  ```
  Get the recursion_control field.

- ```rust
  pub fn get_zero_flags(self: &Self) -> u5 { /* ... */ }
  ```
  Get the zero_flags field.

- ```rust
  pub fn get_version(self: &Self) -> u3 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_protocol_type(self: &Self) -> u16be { /* ... */ }
  ```
  Get the protocol_type field. This field is always stored big-endian

- ```rust
  pub fn get_checksum_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the checksum field, without copying

- ```rust
  pub fn get_checksum(self: &Self) -> Vec<U16BE> { /* ... */ }
  ```
  Get the value of the checksum field (copies contents)

- ```rust
  pub fn get_checksum_iter(self: &Self) -> U16BEIterable<''_> { /* ... */ }
  ```
  Get the value of the checksum field as iterator

- ```rust
  pub fn get_offset_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the offset field, without copying

- ```rust
  pub fn get_offset(self: &Self) -> Vec<U16BE> { /* ... */ }
  ```
  Get the value of the offset field (copies contents)

- ```rust
  pub fn get_offset_iter(self: &Self) -> U16BEIterable<''_> { /* ... */ }
  ```
  Get the value of the offset field as iterator

- ```rust
  pub fn get_key_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the key field, without copying

- ```rust
  pub fn get_key(self: &Self) -> Vec<U32BE> { /* ... */ }
  ```
  Get the value of the key field (copies contents)

- ```rust
  pub fn get_key_iter(self: &Self) -> U32BEIterable<''_> { /* ... */ }
  ```
  Get the value of the key field as iterator

- ```rust
  pub fn get_sequence_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the sequence field, without copying

- ```rust
  pub fn get_sequence(self: &Self) -> Vec<U32BE> { /* ... */ }
  ```
  Get the value of the sequence field (copies contents)

- ```rust
  pub fn get_sequence_iter(self: &Self) -> U32BEIterable<''_> { /* ... */ }
  ```
  Get the value of the sequence field as iterator

- ```rust
  pub fn get_routing_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the routing field, without copying

- ```rust
  pub fn get_routing(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the routing field (copies contents)

###### Trait Implementations

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Gre { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GrePacket<''p>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `MutableGrePacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableGrePacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableGrePacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableGrePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableGrePacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableGrePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> GrePacket<''p> { /* ... */ }
  ```
  Maps from a MutableGrePacket to a GrePacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> GrePacket<''a> { /* ... */ }
  ```
  Maps from a MutableGrePacket to a GrePacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Gre) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Gre instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Gre) { /* ... */ }
  ```
  Populates a GrePacket using a Gre structure

- ```rust
  pub fn get_checksum_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the checksum_present field.

- ```rust
  pub fn get_routing_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the routing_present field.

- ```rust
  pub fn get_key_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the key_present field.

- ```rust
  pub fn get_sequence_present(self: &Self) -> u1 { /* ... */ }
  ```
  Get the sequence_present field.

- ```rust
  pub fn get_strict_source_route(self: &Self) -> u1 { /* ... */ }
  ```
  Get the strict_source_route field.

- ```rust
  pub fn get_recursion_control(self: &Self) -> u3 { /* ... */ }
  ```
  Get the recursion_control field.

- ```rust
  pub fn get_zero_flags(self: &Self) -> u5 { /* ... */ }
  ```
  Get the zero_flags field.

- ```rust
  pub fn get_version(self: &Self) -> u3 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_protocol_type(self: &Self) -> u16be { /* ... */ }
  ```
  Get the protocol_type field. This field is always stored big-endian

- ```rust
  pub fn get_checksum_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the checksum field, without copying

- ```rust
  pub fn get_checksum(self: &Self) -> Vec<U16BE> { /* ... */ }
  ```
  Get the value of the checksum field (copies contents)

- ```rust
  pub fn get_checksum_iter(self: &Self) -> U16BEIterable<''_> { /* ... */ }
  ```
  Get the value of the checksum field as iterator

- ```rust
  pub fn get_offset_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the offset field, without copying

- ```rust
  pub fn get_offset(self: &Self) -> Vec<U16BE> { /* ... */ }
  ```
  Get the value of the offset field (copies contents)

- ```rust
  pub fn get_offset_iter(self: &Self) -> U16BEIterable<''_> { /* ... */ }
  ```
  Get the value of the offset field as iterator

- ```rust
  pub fn get_key_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the key field, without copying

- ```rust
  pub fn get_key(self: &Self) -> Vec<U32BE> { /* ... */ }
  ```
  Get the value of the key field (copies contents)

- ```rust
  pub fn get_key_iter(self: &Self) -> U32BEIterable<''_> { /* ... */ }
  ```
  Get the value of the key field as iterator

- ```rust
  pub fn get_sequence_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the sequence field, without copying

- ```rust
  pub fn get_sequence(self: &Self) -> Vec<U32BE> { /* ... */ }
  ```
  Get the value of the sequence field (copies contents)

- ```rust
  pub fn get_sequence_iter(self: &Self) -> U32BEIterable<''_> { /* ... */ }
  ```
  Get the value of the sequence field as iterator

- ```rust
  pub fn get_routing_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the routing field, without copying

- ```rust
  pub fn get_routing(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the routing field (copies contents)

- ```rust
  pub fn set_checksum_present(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the checksum_present field.

- ```rust
  pub fn set_routing_present(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the routing_present field.

- ```rust
  pub fn set_key_present(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the key_present field.

- ```rust
  pub fn set_sequence_present(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the sequence_present field.

- ```rust
  pub fn set_strict_source_route(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the strict_source_route field.

- ```rust
  pub fn set_recursion_control(self: &mut Self, val: u3) { /* ... */ }
  ```
  Set the recursion_control field.

- ```rust
  pub fn set_zero_flags(self: &mut Self, val: u5) { /* ... */ }
  ```
  Set the zero_flags field.

- ```rust
  pub fn set_version(self: &mut Self, val: u3) { /* ... */ }
  ```
  Set the version field.

- ```rust
  pub fn set_protocol_type(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the protocol_type field. This field is always stored big-endian

- ```rust
  pub fn get_checksum_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the checksum field, without copying

- ```rust
  pub fn set_checksum(self: &mut Self, vals: &[U16BE]) { /* ... */ }
  ```
  Set the value of the checksum field (copies contents)

- ```rust
  pub fn get_offset_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the offset field, without copying

- ```rust
  pub fn set_offset(self: &mut Self, vals: &[U16BE]) { /* ... */ }
  ```
  Set the value of the offset field (copies contents)

- ```rust
  pub fn get_key_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the key field, without copying

- ```rust
  pub fn set_key(self: &mut Self, vals: &[U32BE]) { /* ... */ }
  ```
  Set the value of the key field (copies contents)

- ```rust
  pub fn get_sequence_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the sequence field, without copying

- ```rust
  pub fn set_sequence(self: &mut Self, vals: &[U32BE]) { /* ... */ }
  ```
  Set the value of the sequence field (copies contents)

- ```rust
  pub fn get_routing_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the routing field, without copying

- ```rust
  pub fn set_routing(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the routing field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Send**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Gre { /* ... */ }
    ```

- **Unpin**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableGrePacket<''p>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `GreIterable`

Used to iterate over a slice of `GrePacket`s

```rust
pub struct GreIterable<''a> {
    pub(in ::gre) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<GrePacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Gre`

**Attributes:**

- `#[allow(unused_attributes)]`

GRE (Generic Routing Encapsulation) Packet.

See RFCs 1701, 2784, 2890, 7676, 2637

Current status of implementation:

- [RFC 1701](https://tools.ietf.org/html/rfc1701) except for source routing and checksums.
  Processing a source routed packet will panic. Checksums are able to be inspected, but not
  calculated or verified.

- [RFC 2784](https://tools.ietf.org/html/rfc2784) except for checksums (same as 1701 status).
  Note that it is possible to generate noncompliant packets by setting any of the reserved bits
  (but see 2890).

- [RFC 2890](https://tools.ietf.org/html/rfc2890) implemented.

- [RFC 7676](https://tools.ietf.org/html/rfc7676) has no packet changes - compliance is up to
  the user.

- [RFC 2637](https://tools.ietf.org/html/rfc2637) not implemented.

Note that routing information from RFC 1701 is not implemented, packets
with `routing_present` true will currently cause a panic.

```rust
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
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `checksum_present` | `u1` |  |
| `routing_present` | `u1` |  |
| `key_present` | `u1` |  |
| `sequence_present` | `u1` |  |
| `strict_source_route` | `u1` |  |
| `recursion_control` | `u3` |  |
| `zero_flags` | `u5` |  |
| `version` | `u3` |  |
| `protocol_type` | `u16be` |  |
| `checksum` | `Vec<U16BE>` |  |
| `offset` | `Vec<U16BE>` |  |
| `key` | `Vec<U32BE>` |  |
| `sequence` | `Vec<U32BE>` |  |
| `routing` | `Vec<u8>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Gre { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `U16BEPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct U16BEPacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<U16BEPacket<''p>> { /* ... */ }
  ```
  Constructs a new U16BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<U16BEPacket<''static>> { /* ... */ }
  ```
  Constructs a new U16BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> U16BEPacket<''p> { /* ... */ }
  ```
  Maps from a U16BEPacket to a U16BEPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> U16BEPacket<''a> { /* ... */ }
  ```
  Maps from a U16BEPacket to a U16BEPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &U16BE) -> usize { /* ... */ }
  ```
  The size (in bytes) of a U16BE instance when converted into

- ```rust
  pub fn get_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the number field. This field is always stored big-endian

###### Trait Implementations

- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U16BEPacket<''p>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> U16BE { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `MutableU16BEPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableU16BEPacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableU16BEPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableU16BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableU16BEPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableU16BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> U16BEPacket<''p> { /* ... */ }
  ```
  Maps from a MutableU16BEPacket to a U16BEPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> U16BEPacket<''a> { /* ... */ }
  ```
  Maps from a MutableU16BEPacket to a U16BEPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &U16BE) -> usize { /* ... */ }
  ```
  The size (in bytes) of a U16BE instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &U16BE) { /* ... */ }
  ```
  Populates a U16BEPacket using a U16BE structure

- ```rust
  pub fn get_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the number field. This field is always stored big-endian

- ```rust
  pub fn set_number(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the number field. This field is always stored big-endian

- ```rust
  pub fn set_unused(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the unused field (copies contents)

###### Trait Implementations

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> U16BE { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableU16BEPacket<''p>) -> bool { /* ... */ }
    ```

#### Struct `U16BEIterable`

Used to iterate over a slice of `U16BEPacket`s

```rust
pub struct U16BEIterable<''a> {
    pub(in ::gre) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<U16BEPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `U16BE`

**Attributes:**

- `#[allow(unused_attributes)]`

`u16be`, but we can't use that directly in a `Vec` :(

```rust
pub struct U16BE {
    pub(in ::gre) number: u16be,
    pub(in ::gre) unused: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `number` | `u16be` |  |
| `unused` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> U16BE { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
#### Struct `U32BEPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct U32BEPacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<U32BEPacket<''p>> { /* ... */ }
  ```
  Constructs a new U32BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<U32BEPacket<''static>> { /* ... */ }
  ```
  Constructs a new U32BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> U32BEPacket<''p> { /* ... */ }
  ```
  Maps from a U32BEPacket to a U32BEPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> U32BEPacket<''a> { /* ... */ }
  ```
  Maps from a U32BEPacket to a U32BEPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &U32BE) -> usize { /* ... */ }
  ```
  The size (in bytes) of a U32BE instance when converted into

- ```rust
  pub fn get_number(self: &Self) -> u32be { /* ... */ }
  ```
  Get the number field. This field is always stored big-endian

###### Trait Implementations

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &U32BEPacket<''p>) -> bool { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> U32BE { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `MutableU32BEPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableU32BEPacket<''p> {
    pub(in ::gre) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableU32BEPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableU32BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableU32BEPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableU32BEPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> U32BEPacket<''p> { /* ... */ }
  ```
  Maps from a MutableU32BEPacket to a U32BEPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> U32BEPacket<''a> { /* ... */ }
  ```
  Maps from a MutableU32BEPacket to a U32BEPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &U32BE) -> usize { /* ... */ }
  ```
  The size (in bytes) of a U32BE instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &U32BE) { /* ... */ }
  ```
  Populates a U32BEPacket using a U32BE structure

- ```rust
  pub fn get_number(self: &Self) -> u32be { /* ... */ }
  ```
  Get the number field. This field is always stored big-endian

- ```rust
  pub fn set_number(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the number field. This field is always stored big-endian

- ```rust
  pub fn set_unused(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the unused field (copies contents)

###### Trait Implementations

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableU32BEPacket<''p>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> U32BE { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

#### Struct `U32BEIterable`

Used to iterate over a slice of `U32BEPacket`s

```rust
pub struct U32BEIterable<''a> {
    pub(in ::gre) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<U32BEPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `U32BE`

**Attributes:**

- `#[allow(unused_attributes)]`

`u32be`, but we can't use that directly in a `Vec` :(

```rust
pub struct U32BE {
    pub(in ::gre) number: u32be,
    pub(in ::gre) unused: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `number` | `u32be` |  |
| `unused` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Send**
- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> U32BE { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `gre_checksum_length`

```rust
pub(in ::gre) fn gre_checksum_length(gre: &GrePacket<''_>) -> usize { /* ... */ }
```

#### Function `gre_offset_length`

```rust
pub(in ::gre) fn gre_offset_length(gre: &GrePacket<''_>) -> usize { /* ... */ }
```

#### Function `gre_key_length`

```rust
pub(in ::gre) fn gre_key_length(gre: &GrePacket<''_>) -> usize { /* ... */ }
```

#### Function `gre_sequence_length`

```rust
pub(in ::gre) fn gre_sequence_length(gre: &GrePacket<''_>) -> usize { /* ... */ }
```

#### Function `gre_routing_length`

```rust
pub(in ::gre) fn gre_routing_length(gre: &GrePacket<''_>) -> usize { /* ... */ }
```

## Module `icmp`

An ICMP packet abstraction.

```rust
pub mod icmp { /* ... */ }
```

### Modules

## Module `IcmpTypes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The enumeration of the recognized ICMP types.

```rust
pub mod IcmpTypes { /* ... */ }
```

### Constants and Statics

#### Constant `EchoReply`

ICMP type for "echo reply" packet.

```rust
pub const EchoReply: icmp::IcmpType = _;
```

#### Constant `DestinationUnreachable`

ICMP type for "destination unreachable" packet.

```rust
pub const DestinationUnreachable: icmp::IcmpType = _;
```

#### Constant `SourceQuench`

ICMP type for "source quench" packet.

```rust
pub const SourceQuench: icmp::IcmpType = _;
```

#### Constant `RedirectMessage`

ICMP type for "redirect message" packet.

```rust
pub const RedirectMessage: icmp::IcmpType = _;
```

#### Constant `EchoRequest`

ICMP type for "echo request" packet.

```rust
pub const EchoRequest: icmp::IcmpType = _;
```

#### Constant `RouterAdvertisement`

ICMP type for "router advertisement" packet.

```rust
pub const RouterAdvertisement: icmp::IcmpType = _;
```

#### Constant `RouterSolicitation`

ICMP type for "router solicitation" packet.

```rust
pub const RouterSolicitation: icmp::IcmpType = _;
```

#### Constant `TimeExceeded`

ICMP type for "time exceeded" packet.

```rust
pub const TimeExceeded: icmp::IcmpType = _;
```

#### Constant `ParameterProblem`

ICMP type for "parameter problem" packet.

```rust
pub const ParameterProblem: icmp::IcmpType = _;
```

#### Constant `Timestamp`

ICMP type for "timestamp" packet.

```rust
pub const Timestamp: icmp::IcmpType = _;
```

#### Constant `TimestampReply`

ICMP type for "timestamp reply" packet.

```rust
pub const TimestampReply: icmp::IcmpType = _;
```

#### Constant `InformationRequest`

ICMP type for "information request" packet.

```rust
pub const InformationRequest: icmp::IcmpType = _;
```

#### Constant `InformationReply`

ICMP type for "information reply" packet.

```rust
pub const InformationReply: icmp::IcmpType = _;
```

#### Constant `AddressMaskRequest`

ICMP type for "address mask request" packet.

```rust
pub const AddressMaskRequest: icmp::IcmpType = _;
```

#### Constant `AddressMaskReply`

ICMP type for "address mask reply" packet.

```rust
pub const AddressMaskReply: icmp::IcmpType = _;
```

#### Constant `Traceroute`

ICMP type for "traceroute" packet.

```rust
pub const Traceroute: icmp::IcmpType = _;
```

## Module `echo_reply`

abstraction for ICMP "echo reply" packets.

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|           Identifier          |        Sequence Number        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Data ...
+-+-+-+-+-
```

```rust
pub mod echo_reply { /* ... */ }
```

### Modules

## Module `IcmpCodes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Enumeration of available ICMP codes for ICMP echo replay packets. There is actually only
one, since the only valid ICMP code is 0.

```rust
pub mod IcmpCodes { /* ... */ }
```

### Constants and Statics

#### Constant `NoCode`

0 is the only available ICMP code for "echo reply" ICMP packets.

```rust
pub const NoCode: icmp::IcmpCode = _;
```

### Types

#### Struct `Identifier`

Represent the "identifier" field of the ICMP echo replay header.

```rust
pub struct Identifier(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u16) -> Identifier { /* ... */ }
  ```
  Create a new `Identifier` instance.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identifier { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Identifier) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Identifier) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Identifier) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `SequenceNumber`

Represent the "sequence number" field of the ICMP echo replay header.

```rust
pub struct SequenceNumber(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u16) -> SequenceNumber { /* ... */ }
  ```
  Create a new `SequenceNumber` instance.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SequenceNumber) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SequenceNumber) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SequenceNumber { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SequenceNumber) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **RefUnwindSafe**
- **Freeze**
#### Struct `EchoReplyPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct EchoReplyPacket<''p> {
    pub(in ::icmp::echo_reply) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<EchoReplyPacket<''p>> { /* ... */ }
  ```
  Constructs a new EchoReplyPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<EchoReplyPacket<''static>> { /* ... */ }
  ```
  Constructs a new EchoReplyPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EchoReplyPacket<''p> { /* ... */ }
  ```
  Maps from a EchoReplyPacket to a EchoReplyPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EchoReplyPacket<''a> { /* ... */ }
  ```
  Maps from a EchoReplyPacket to a EchoReplyPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &EchoReply) -> usize { /* ... */ }
  ```
  The size (in bytes) of a EchoReply instance when converted into

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_identifier(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identifier field. This field is always stored big-endian

- ```rust
  pub fn get_sequence_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the sequence_number field. This field is always stored big-endian

###### Trait Implementations

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EchoReplyPacket<''p>) -> bool { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> EchoReply { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
#### Struct `MutableEchoReplyPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableEchoReplyPacket<''p> {
    pub(in ::icmp::echo_reply) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableEchoReplyPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableEchoReplyPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableEchoReplyPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableEchoReplyPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EchoReplyPacket<''p> { /* ... */ }
  ```
  Maps from a MutableEchoReplyPacket to a EchoReplyPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EchoReplyPacket<''a> { /* ... */ }
  ```
  Maps from a MutableEchoReplyPacket to a EchoReplyPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &EchoReply) -> usize { /* ... */ }
  ```
  The size (in bytes) of a EchoReply instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &EchoReply) { /* ... */ }
  ```
  Populates a EchoReplyPacket using a EchoReply structure

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_identifier(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identifier field. This field is always stored big-endian

- ```rust
  pub fn get_sequence_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the sequence_number field. This field is always stored big-endian

- ```rust
  pub fn set_icmp_type(self: &mut Self, val: IcmpType) { /* ... */ }
  ```
  Set the value of the icmp_type field.

- ```rust
  pub fn set_icmp_code(self: &mut Self, val: IcmpCode) { /* ... */ }
  ```
  Set the value of the icmp_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_identifier(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the identifier field. This field is always stored big-endian

- ```rust
  pub fn set_sequence_number(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the sequence_number field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableEchoReplyPacket<''p>) -> bool { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> EchoReply { /* ... */ }
    ```

#### Struct `EchoReplyIterable`

Used to iterate over a slice of `EchoReplyPacket`s

```rust
pub struct EchoReplyIterable<''a> {
    pub(in ::icmp::echo_reply) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<EchoReplyPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `EchoReply`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an ICMP echo reply packet.

```rust
pub struct EchoReply {
    pub icmp_type: icmp::IcmpType,
    pub icmp_code: icmp::IcmpCode,
    pub checksum: u16be,
    pub identifier: u16be,
    pub sequence_number: u16be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmp_type` | `icmp::IcmpType` |  |
| `icmp_code` | `icmp::IcmpCode` |  |
| `checksum` | `u16be` |  |
| `identifier` | `u16be` |  |
| `sequence_number` | `u16be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EchoReply { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `echo_request`

abstraction for "echo request" ICMP packets.

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|           Identifier          |        Sequence Number        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Data ...
+-+-+-+-+-
```

```rust
pub mod echo_request { /* ... */ }
```

### Modules

## Module `IcmpCodes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Enumeration of available ICMP codes for "echo reply" ICMP packets. There is actually only
one, since the only valid ICMP code is 0.

```rust
pub mod IcmpCodes { /* ... */ }
```

### Constants and Statics

#### Constant `NoCode`

0 is the only available ICMP code for "echo reply" ICMP packets.

```rust
pub const NoCode: icmp::IcmpCode = _;
```

### Types

#### Struct `Identifier`

Represents the identifier field.

```rust
pub struct Identifier(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u16) -> Identifier { /* ... */ }
  ```
  Create a new `Identifier` instance.

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Identifier { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Identifier) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **RefUnwindSafe**
- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Identifier) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Identifier) -> bool { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `SequenceNumber`

Represents the sequence number field.

```rust
pub struct SequenceNumber(pub u16);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u16` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u16) -> SequenceNumber { /* ... */ }
  ```
  Create a new `SequenceNumber` instance.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SequenceNumber) -> bool { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &SequenceNumber) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SequenceNumber { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u16) { /* ... */ }
    ```

- **UnwindSafe**
- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &SequenceNumber) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `EchoRequestPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct EchoRequestPacket<''p> {
    pub(in ::icmp::echo_request) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<EchoRequestPacket<''p>> { /* ... */ }
  ```
  Constructs a new EchoRequestPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<EchoRequestPacket<''static>> { /* ... */ }
  ```
  Constructs a new EchoRequestPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EchoRequestPacket<''p> { /* ... */ }
  ```
  Maps from a EchoRequestPacket to a EchoRequestPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EchoRequestPacket<''a> { /* ... */ }
  ```
  Maps from a EchoRequestPacket to a EchoRequestPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &EchoRequest) -> usize { /* ... */ }
  ```
  The size (in bytes) of a EchoRequest instance when converted into

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_identifier(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identifier field. This field is always stored big-endian

- ```rust
  pub fn get_sequence_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the sequence_number field. This field is always stored big-endian

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EchoRequestPacket<''p>) -> bool { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> EchoRequest { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `MutableEchoRequestPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableEchoRequestPacket<''p> {
    pub(in ::icmp::echo_request) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableEchoRequestPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableEchoRequestPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableEchoRequestPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableEchoRequestPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> EchoRequestPacket<''p> { /* ... */ }
  ```
  Maps from a MutableEchoRequestPacket to a EchoRequestPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> EchoRequestPacket<''a> { /* ... */ }
  ```
  Maps from a MutableEchoRequestPacket to a EchoRequestPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &EchoRequest) -> usize { /* ... */ }
  ```
  The size (in bytes) of a EchoRequest instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &EchoRequest) { /* ... */ }
  ```
  Populates a EchoRequestPacket using a EchoRequest structure

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_identifier(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identifier field. This field is always stored big-endian

- ```rust
  pub fn get_sequence_number(self: &Self) -> u16be { /* ... */ }
  ```
  Get the sequence_number field. This field is always stored big-endian

- ```rust
  pub fn set_icmp_type(self: &mut Self, val: IcmpType) { /* ... */ }
  ```
  Set the value of the icmp_type field.

- ```rust
  pub fn set_icmp_code(self: &mut Self, val: IcmpCode) { /* ... */ }
  ```
  Set the value of the icmp_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_identifier(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the identifier field. This field is always stored big-endian

- ```rust
  pub fn set_sequence_number(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the sequence_number field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Freeze**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> EchoRequest { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableEchoRequestPacket<''p>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
#### Struct `EchoRequestIterable`

Used to iterate over a slice of `EchoRequestPacket`s

```rust
pub struct EchoRequestIterable<''a> {
    pub(in ::icmp::echo_request) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<EchoRequestPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `EchoRequest`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an "echo request" ICMP packet.

```rust
pub struct EchoRequest {
    pub icmp_type: icmp::IcmpType,
    pub icmp_code: icmp::IcmpCode,
    pub checksum: u16be,
    pub identifier: u16be,
    pub sequence_number: u16be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmp_type` | `icmp::IcmpType` |  |
| `icmp_code` | `icmp::IcmpCode` |  |
| `checksum` | `u16be` |  |
| `identifier` | `u16be` |  |
| `sequence_number` | `u16be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EchoRequest { /* ... */ }
    ```

## Module `destination_unreachable`

abstraction for "destination unreachable" ICMP packets.

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                             unused                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|      Internet Header + 64 bits of Original Data Datagram      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

```rust
pub mod destination_unreachable { /* ... */ }
```

### Modules

## Module `IcmpCodes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Enumeration of the recognized ICMP codes for "destination unreachable" ICMP packets.

```rust
pub mod IcmpCodes { /* ... */ }
```

### Constants and Statics

#### Constant `DestinationNetworkUnreachable`

ICMP code for "destination network unreachable" packet.

```rust
pub const DestinationNetworkUnreachable: icmp::IcmpCode = _;
```

#### Constant `DestinationHostUnreachable`

ICMP code for "destination host unreachable" packet.

```rust
pub const DestinationHostUnreachable: icmp::IcmpCode = _;
```

#### Constant `DestinationProtocolUnreachable`

ICMP code for "destination protocol unreachable" packet.

```rust
pub const DestinationProtocolUnreachable: icmp::IcmpCode = _;
```

#### Constant `DestinationPortUnreachable`

ICMP code for "destination port unreachable" packet.

```rust
pub const DestinationPortUnreachable: icmp::IcmpCode = _;
```

#### Constant `FragmentationRequiredAndDFFlagSet`

ICMP code for "fragmentation required and DFF flag set" packet.

```rust
pub const FragmentationRequiredAndDFFlagSet: icmp::IcmpCode = _;
```

#### Constant `SourceRouteFailed`

ICMP code for "source route failed" packet.

```rust
pub const SourceRouteFailed: icmp::IcmpCode = _;
```

#### Constant `DestinationNetworkUnknown`

ICMP code for "destination network unknown" packet.

```rust
pub const DestinationNetworkUnknown: icmp::IcmpCode = _;
```

#### Constant `DestinationHostUnknown`

ICMP code for "destination host unknown" packet.

```rust
pub const DestinationHostUnknown: icmp::IcmpCode = _;
```

#### Constant `SourceHostIsolated`

ICMP code for "source host isolated" packet.

```rust
pub const SourceHostIsolated: icmp::IcmpCode = _;
```

#### Constant `NetworkAdministrativelyProhibited`

ICMP code for "network administrative prohibited" packet.

```rust
pub const NetworkAdministrativelyProhibited: icmp::IcmpCode = _;
```

#### Constant `HostAdministrativelyProhibited`

ICMP code for "host administrative prohibited" packet.

```rust
pub const HostAdministrativelyProhibited: icmp::IcmpCode = _;
```

#### Constant `NetworkUnreachableForTOS`

ICMP code for "network unreachable for this Type Of Service" packet.

```rust
pub const NetworkUnreachableForTOS: icmp::IcmpCode = _;
```

#### Constant `HostUnreachableForTOS`

ICMP code for "host unreachable for this Type Of Service" packet.

```rust
pub const HostUnreachableForTOS: icmp::IcmpCode = _;
```

#### Constant `CommunicationAdministrativelyProhibited`

ICMP code for "communication administratively prohibited" packet.

```rust
pub const CommunicationAdministrativelyProhibited: icmp::IcmpCode = _;
```

#### Constant `HostPrecedenceViolation`

ICMP code for "host precedence violation" packet.

```rust
pub const HostPrecedenceViolation: icmp::IcmpCode = _;
```

#### Constant `PrecedenceCutoffInEffect`

ICMP code for "precedence cut off in effect" packet.

```rust
pub const PrecedenceCutoffInEffect: icmp::IcmpCode = _;
```

### Types

#### Struct `DestinationUnreachablePacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct DestinationUnreachablePacket<''p> {
    pub(in ::icmp::destination_unreachable) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<DestinationUnreachablePacket<''p>> { /* ... */ }
  ```
  Constructs a new DestinationUnreachablePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<DestinationUnreachablePacket<''static>> { /* ... */ }
  ```
  Constructs a new DestinationUnreachablePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> DestinationUnreachablePacket<''p> { /* ... */ }
  ```
  Maps from a DestinationUnreachablePacket to a DestinationUnreachablePacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> DestinationUnreachablePacket<''a> { /* ... */ }
  ```
  Maps from a DestinationUnreachablePacket to a DestinationUnreachablePacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &DestinationUnreachable) -> usize { /* ... */ }
  ```
  The size (in bytes) of a DestinationUnreachable instance when converted into

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_unused(self: &Self) -> u32be { /* ... */ }
  ```
  Get the unused field. This field is always stored big-endian

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> DestinationUnreachable { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DestinationUnreachablePacket<''p>) -> bool { /* ... */ }
    ```

#### Struct `MutableDestinationUnreachablePacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableDestinationUnreachablePacket<''p> {
    pub(in ::icmp::destination_unreachable) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableDestinationUnreachablePacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableDestinationUnreachablePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableDestinationUnreachablePacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableDestinationUnreachablePacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> DestinationUnreachablePacket<''p> { /* ... */ }
  ```
  Maps from a MutableDestinationUnreachablePacket to a DestinationUnreachablePacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> DestinationUnreachablePacket<''a> { /* ... */ }
  ```
  Maps from a MutableDestinationUnreachablePacket to a DestinationUnreachablePacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &DestinationUnreachable) -> usize { /* ... */ }
  ```
  The size (in bytes) of a DestinationUnreachable instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &DestinationUnreachable) { /* ... */ }
  ```
  Populates a DestinationUnreachablePacket using a DestinationUnreachable structure

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_unused(self: &Self) -> u32be { /* ... */ }
  ```
  Get the unused field. This field is always stored big-endian

- ```rust
  pub fn set_icmp_type(self: &mut Self, val: IcmpType) { /* ... */ }
  ```
  Set the value of the icmp_type field.

- ```rust
  pub fn set_icmp_code(self: &mut Self, val: IcmpCode) { /* ... */ }
  ```
  Set the value of the icmp_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_unused(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the unused field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableDestinationUnreachablePacket<''p>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> DestinationUnreachable { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `DestinationUnreachableIterable`

Used to iterate over a slice of `DestinationUnreachablePacket`s

```rust
pub struct DestinationUnreachableIterable<''a> {
    pub(in ::icmp::destination_unreachable) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<DestinationUnreachablePacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Struct `DestinationUnreachable`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an "echo request" ICMP packet.

```rust
pub struct DestinationUnreachable {
    pub icmp_type: icmp::IcmpType,
    pub icmp_code: icmp::IcmpCode,
    pub checksum: u16be,
    pub unused: u32be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmp_type` | `icmp::IcmpType` |  |
| `icmp_code` | `icmp::IcmpCode` |  |
| `checksum` | `u16be` |  |
| `unused` | `u32be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DestinationUnreachable { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
## Module `time_exceeded`

abstraction for "time exceeded" ICMP packets.

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                             unused                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|      Internet Header + 64 bits of Original Data Datagram      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

```rust
pub mod time_exceeded { /* ... */ }
```

### Modules

## Module `IcmpCodes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Enumeration of the recognized ICMP codes for "time exceeded" ICMP packets.

```rust
pub mod IcmpCodes { /* ... */ }
```

### Constants and Statics

#### Constant `TimeToLiveExceededInTransit`

ICMP code for "time to live exceeded in transit" packet.

```rust
pub const TimeToLiveExceededInTransit: icmp::IcmpCode = _;
```

#### Constant `FragmentReasemblyTimeExceeded`

ICMP code for "fragment reassembly time exceeded" packet.

```rust
pub const FragmentReasemblyTimeExceeded: icmp::IcmpCode = _;
```

### Types

#### Struct `TimeExceededPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct TimeExceededPacket<''p> {
    pub(in ::icmp::time_exceeded) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<TimeExceededPacket<''p>> { /* ... */ }
  ```
  Constructs a new TimeExceededPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<TimeExceededPacket<''static>> { /* ... */ }
  ```
  Constructs a new TimeExceededPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TimeExceededPacket<''p> { /* ... */ }
  ```
  Maps from a TimeExceededPacket to a TimeExceededPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TimeExceededPacket<''a> { /* ... */ }
  ```
  Maps from a TimeExceededPacket to a TimeExceededPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &TimeExceeded) -> usize { /* ... */ }
  ```
  The size (in bytes) of a TimeExceeded instance when converted into

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_unused(self: &Self) -> u32be { /* ... */ }
  ```
  Get the unused field. This field is always stored big-endian

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> TimeExceeded { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TimeExceededPacket<''p>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

#### Struct `MutableTimeExceededPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableTimeExceededPacket<''p> {
    pub(in ::icmp::time_exceeded) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableTimeExceededPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableTimeExceededPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableTimeExceededPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableTimeExceededPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TimeExceededPacket<''p> { /* ... */ }
  ```
  Maps from a MutableTimeExceededPacket to a TimeExceededPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TimeExceededPacket<''a> { /* ... */ }
  ```
  Maps from a MutableTimeExceededPacket to a TimeExceededPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &TimeExceeded) -> usize { /* ... */ }
  ```
  The size (in bytes) of a TimeExceeded instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &TimeExceeded) { /* ... */ }
  ```
  Populates a TimeExceededPacket using a TimeExceeded structure

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_unused(self: &Self) -> u32be { /* ... */ }
  ```
  Get the unused field. This field is always stored big-endian

- ```rust
  pub fn set_icmp_type(self: &mut Self, val: IcmpType) { /* ... */ }
  ```
  Set the value of the icmp_type field.

- ```rust
  pub fn set_icmp_code(self: &mut Self, val: IcmpCode) { /* ... */ }
  ```
  Set the value of the icmp_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_unused(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the unused field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> TimeExceeded { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableTimeExceededPacket<''p>) -> bool { /* ... */ }
    ```

- **Send**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `TimeExceededIterable`

Used to iterate over a slice of `TimeExceededPacket`s

```rust
pub struct TimeExceededIterable<''a> {
    pub(in ::icmp::time_exceeded) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<TimeExceededPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
#### Struct `TimeExceeded`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an "echo request" ICMP packet.

```rust
pub struct TimeExceeded {
    pub icmp_type: icmp::IcmpType,
    pub icmp_code: icmp::IcmpCode,
    pub checksum: u16be,
    pub unused: u32be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmp_type` | `icmp::IcmpType` |  |
| `icmp_code` | `icmp::IcmpCode` |  |
| `checksum` | `u16be` |  |
| `unused` | `u32be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TimeExceeded { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
### Types

#### Struct `IcmpType`

Represents the "ICMP type" header field.

```rust
pub struct IcmpType(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u8) -> IcmpType { /* ... */ }
  ```
  Create a new `IcmpType` instance.

###### Trait Implementations

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &IcmpType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IcmpType { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &IcmpType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IcmpType) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `IcmpCode`

Represents the "ICMP code" header field.

```rust
pub struct IcmpCode(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u8) -> IcmpCode { /* ... */ }
  ```
  Create a new `IcmpCode` instance.

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &IcmpCode) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IcmpCode { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &IcmpCode) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IcmpCode) -> bool { /* ... */ }
    ```

#### Struct `IcmpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct IcmpPacket<''p> {
    pub(in ::icmp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<IcmpPacket<''p>> { /* ... */ }
  ```
  Constructs a new IcmpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<IcmpPacket<''static>> { /* ... */ }
  ```
  Constructs a new IcmpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> IcmpPacket<''p> { /* ... */ }
  ```
  Maps from a IcmpPacket to a IcmpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> IcmpPacket<''a> { /* ... */ }
  ```
  Maps from a IcmpPacket to a IcmpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Icmp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Icmp instance when converted into

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

###### Trait Implementations

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IcmpPacket<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Icmp { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `MutableIcmpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableIcmpPacket<''p> {
    pub(in ::icmp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableIcmpPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableIcmpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableIcmpPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableIcmpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> IcmpPacket<''p> { /* ... */ }
  ```
  Maps from a MutableIcmpPacket to a IcmpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> IcmpPacket<''a> { /* ... */ }
  ```
  Maps from a MutableIcmpPacket to a IcmpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Icmp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Icmp instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Icmp) { /* ... */ }
  ```
  Populates a IcmpPacket using a Icmp structure

- ```rust
  pub fn get_icmp_type(self: &Self) -> IcmpType { /* ... */ }
  ```
  Get the value of the icmp_type field

- ```rust
  pub fn get_icmp_code(self: &Self) -> IcmpCode { /* ... */ }
  ```
  Get the value of the icmp_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_icmp_type(self: &mut Self, val: IcmpType) { /* ... */ }
  ```
  Set the value of the icmp_type field.

- ```rust
  pub fn set_icmp_code(self: &mut Self, val: IcmpCode) { /* ... */ }
  ```
  Set the value of the icmp_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Icmp { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableIcmpPacket<''p>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `IcmpIterable`

Used to iterate over a slice of `IcmpPacket`s

```rust
pub struct IcmpIterable<''a> {
    pub(in ::icmp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<IcmpPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Sync**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

#### Struct `Icmp`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents a generic ICMP packet.

```rust
pub struct Icmp {
    pub icmp_type: IcmpType,
    pub icmp_code: IcmpCode,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmp_type` | `IcmpType` |  |
| `icmp_code` | `IcmpCode` |  |
| `checksum` | `u16be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Icmp { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `checksum`

Calculates a checksum of an ICMP packet.

```rust
pub fn checksum(packet: &IcmpPacket<''_>) -> u16be { /* ... */ }
```

## Module `icmpv6`

An ICMPv6 packet abstraction.

```rust
pub mod icmpv6 { /* ... */ }
```

### Modules

## Module `Icmpv6Types`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The enumeration of the recognized ICMPv6 types.

```rust
pub mod Icmpv6Types { /* ... */ }
```

### Constants and Statics

#### Constant `DestinationUnreachable`

ICMPv6 type for "destination unreachable".

```rust
pub const DestinationUnreachable: icmpv6::Icmpv6Type = _;
```

#### Constant `PacketTooBig`

ICMPv6 type for "packet too big".

```rust
pub const PacketTooBig: icmpv6::Icmpv6Type = _;
```

#### Constant `TimeExceeded`

ICMPv6 type for "time exceeded".

```rust
pub const TimeExceeded: icmpv6::Icmpv6Type = _;
```

#### Constant `ParameterProblem`

ICMPv6 type for "parameter problem".

```rust
pub const ParameterProblem: icmpv6::Icmpv6Type = _;
```

#### Constant `EchoRequest`

ICMPv6 type for "echo request".

```rust
pub const EchoRequest: icmpv6::Icmpv6Type = _;
```

#### Constant `EchoReply`

ICMPv6 type for "echo reply".

```rust
pub const EchoReply: icmpv6::Icmpv6Type = _;
```

#### Constant `RouterSolicit`

ICMPv6 type for "router solicitation".

```rust
pub const RouterSolicit: icmpv6::Icmpv6Type = _;
```

#### Constant `RouterAdvert`

ICMPv6 type for "router advertisement".

```rust
pub const RouterAdvert: icmpv6::Icmpv6Type = _;
```

#### Constant `NeighborSolicit`

ICMPv6 type for "neighbor solicitation".

```rust
pub const NeighborSolicit: icmpv6::Icmpv6Type = _;
```

#### Constant `NeighborAdvert`

ICMPv6 type for "neighbor advertisement".

```rust
pub const NeighborAdvert: icmpv6::Icmpv6Type = _;
```

#### Constant `Redirect`

ICMPv6 type for "redirect".

```rust
pub const Redirect: icmpv6::Icmpv6Type = _;
```

## Module `ndp`

Abstractions for the Neighbor Discovery Protocol [RFC 4861]

[RFC 4861]: https://tools.ietf.org/html/rfc4861

```rust
pub mod ndp { /* ... */ }
```

### Modules

## Module `Icmpv6Codes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

```rust
pub mod Icmpv6Codes { /* ... */ }
```

### Constants and Statics

#### Constant `NoCode`

0 is the only available ICMPv6 Code for the NDP.

```rust
pub const NoCode: icmpv6::Icmpv6Code = _;
```

## Module `NdpOptionTypes`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Neighbor Discovery Option Types [RFC 4861 § 4.6]

[RFC 4861 § 4.6]: https://tools.ietf.org/html/rfc4861#section-4.6

```rust
pub mod NdpOptionTypes { /* ... */ }
```

### Constants and Statics

#### Constant `SourceLLAddr`

Source Link-Layer Address Option [RFC 4861 § 4.6.1]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     |    Link-Layer Address ...
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6.1]: https://tools.ietf.org/html/rfc4861#section-4.6.1

```rust
pub const SourceLLAddr: super::NdpOptionType = _;
```

#### Constant `TargetLLAddr`

Target Link-Layer Address Option [RFC 4861 § 4.6.1]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     |    Link-Layer Address ...
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6.1]: https://tools.ietf.org/html/rfc4861#section-4.6.1

```rust
pub const TargetLLAddr: super::NdpOptionType = _;
```

#### Constant `PrefixInformation`

Prefix Information Option [RFC 4861 § 4.6.2]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     | Prefix Length |L|A| Reserved1 |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Valid Lifetime                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                       Preferred Lifetime                      |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Reserved2                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                                                               +
|                                                               |
+                            Prefix                             +
|                                                               |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6.2]: https://tools.ietf.org/html/rfc4861#section-4.6.2

```rust
pub const PrefixInformation: super::NdpOptionType = _;
```

#### Constant `RedirectedHeader`

Redirected Header Option [RFC 4861 § 4.6.3]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     |            Reserved           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Reserved                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
~                       IP header + data                        ~
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6.3]: https://tools.ietf.org/html/rfc4861#section-4.6.3

```rust
pub const RedirectedHeader: super::NdpOptionType = _;
```

#### Constant `MTU`

MTU Option [RFC 4861 § 4.6.4]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     |           Reserved            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                              MTU                              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6.4]: https://tools.ietf.org/html/rfc4861#section-4.6.4

```rust
pub const MTU: super::NdpOptionType = _;
```

## Module `RouterAdvertFlags`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The enumeration of recognized Router Advert flags.

```rust
pub mod RouterAdvertFlags { /* ... */ }
```

### Constants and Statics

#### Constant `ManagedAddressConf`

"Managed Address Configuration" flag. This is set when
addresses are available via DHCPv6.

```rust
pub const ManagedAddressConf: u8 = 128;
```

#### Constant `OtherConf`

"Other Configuration" flag. This is set when other
configuration information is available via DHCPv6.

```rust
pub const OtherConf: u8 = 64;
```

## Module `NeighborAdvertFlags`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Enumeration of recognized Neighbor Advert flags.

```rust
pub mod NeighborAdvertFlags { /* ... */ }
```

### Constants and Statics

#### Constant `Router`

Indicates that the sender is a router.

```rust
pub const Router: u8 = 128;
```

#### Constant `Solicited`

Indicates that the advertisement was sent due to the receipt of a
Neighbor Solicitation message.

```rust
pub const Solicited: u8 = 64;
```

#### Constant `Override`

Indicates that the advertisement should override an existing cache
entry.

```rust
pub const Override: u8 = 32;
```

### Types

#### Struct `NdpOptionType`

Represents a Neighbor Discovery Option Type.

```rust
pub struct NdpOptionType(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u8) -> NdpOptionType { /* ... */ }
  ```
  Create a new `NdpOptionType` instance.

###### Trait Implementations

- **Unpin**
- **Eq**
- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &NdpOptionType) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &NdpOptionType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NdpOptionType) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NdpOptionType { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `NdpOptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct NdpOptionPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<NdpOptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new NdpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<NdpOptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new NdpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NdpOptionPacket<''p> { /* ... */ }
  ```
  Maps from a NdpOptionPacket to a NdpOptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NdpOptionPacket<''a> { /* ... */ }
  ```
  Maps from a NdpOptionPacket to a NdpOptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NdpOption) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NdpOption instance when converted into

- ```rust
  pub fn get_option_type(self: &Self) -> NdpOptionType { /* ... */ }
  ```
  Get the value of the option_type field

- ```rust
  pub fn get_length(self: &Self) -> u8 { /* ... */ }
  ```
  Get the length field.

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NdpOption { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NdpOptionPacket<''p>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `MutableNdpOptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableNdpOptionPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableNdpOptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableNdpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableNdpOptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableNdpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NdpOptionPacket<''p> { /* ... */ }
  ```
  Maps from a MutableNdpOptionPacket to a NdpOptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NdpOptionPacket<''a> { /* ... */ }
  ```
  Maps from a MutableNdpOptionPacket to a NdpOptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NdpOption) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NdpOption instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &NdpOption) { /* ... */ }
  ```
  Populates a NdpOptionPacket using a NdpOption structure

- ```rust
  pub fn get_option_type(self: &Self) -> NdpOptionType { /* ... */ }
  ```
  Get the value of the option_type field

- ```rust
  pub fn get_length(self: &Self) -> u8 { /* ... */ }
  ```
  Get the length field.

- ```rust
  pub fn set_option_type(self: &mut Self, val: NdpOptionType) { /* ... */ }
  ```
  Set the value of the option_type field.

- ```rust
  pub fn set_length(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the length field.

- ```rust
  pub fn set_data(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the data field (copies contents)

###### Trait Implementations

- **Sync**
- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableNdpOptionPacket<''p>) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NdpOption { /* ... */ }
    ```

#### Struct `NdpOptionIterable`

Used to iterate over a slice of `NdpOptionPacket`s

```rust
pub struct NdpOptionIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<NdpOptionPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
#### Struct `NdpOption`

**Attributes:**

- `#[allow(unused_attributes)]`

Neighbor Discovery Option [RFC 4861 § 4.6]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |    Length     |              ...              |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
~                              ...                              ~
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

[RFC 4861 § 4.6]: https://tools.ietf.org/html/rfc4861#section-4.6

```rust
pub struct NdpOption {
    pub option_type: NdpOptionType,
    pub length: u8,
    pub data: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `option_type` | `NdpOptionType` |  |
| `length` | `u8` |  |
| `data` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NdpOption { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `RouterSolicitPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct RouterSolicitPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<RouterSolicitPacket<''p>> { /* ... */ }
  ```
  Constructs a new RouterSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<RouterSolicitPacket<''static>> { /* ... */ }
  ```
  Constructs a new RouterSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RouterSolicitPacket<''p> { /* ... */ }
  ```
  Maps from a RouterSolicitPacket to a RouterSolicitPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RouterSolicitPacket<''a> { /* ... */ }
  ```
  Maps from a RouterSolicitPacket to a RouterSolicitPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &RouterSolicit) -> usize { /* ... */ }
  ```
  The size (in bytes) of a RouterSolicit instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> RouterSolicit { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RouterSolicitPacket<''p>) -> bool { /* ... */ }
    ```

- **Send**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Sync**
#### Struct `MutableRouterSolicitPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableRouterSolicitPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableRouterSolicitPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableRouterSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableRouterSolicitPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableRouterSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RouterSolicitPacket<''p> { /* ... */ }
  ```
  Maps from a MutableRouterSolicitPacket to a RouterSolicitPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RouterSolicitPacket<''a> { /* ... */ }
  ```
  Maps from a MutableRouterSolicitPacket to a RouterSolicitPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &RouterSolicit) -> usize { /* ... */ }
  ```
  The size (in bytes) of a RouterSolicit instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &RouterSolicit) { /* ... */ }
  ```
  Populates a RouterSolicitPacket using a RouterSolicit structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_reserved(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[NdpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **StructuralPartialEq**
- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableRouterSolicitPacket<''p>) -> bool { /* ... */ }
    ```

- **Send**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> RouterSolicit { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

#### Struct `RouterSolicitIterable`

Used to iterate over a slice of `RouterSolicitPacket`s

```rust
pub struct RouterSolicitIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<RouterSolicitPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
#### Struct `RouterSolicit`

**Attributes:**

- `#[allow(unused_attributes)]`

Router Solicitation Message [RFC 4861 § 4.1]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                            Reserved                           |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Options ...
```

[RFC 4861 § 4.1]: https://tools.ietf.org/html/rfc4861#section-4.1

```rust
pub struct RouterSolicit {
    pub icmpv6_type: icmpv6::Icmpv6Type,
    pub icmpv6_code: icmpv6::Icmpv6Code,
    pub checksum: u16be,
    pub reserved: u32be,
    pub options: Vec<NdpOption>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `icmpv6::Icmpv6Type` |  |
| `icmpv6_code` | `icmpv6::Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `reserved` | `u32be` |  |
| `options` | `Vec<NdpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RouterSolicit { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `RouterAdvertPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct RouterAdvertPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<RouterAdvertPacket<''p>> { /* ... */ }
  ```
  Constructs a new RouterAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<RouterAdvertPacket<''static>> { /* ... */ }
  ```
  Constructs a new RouterAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RouterAdvertPacket<''p> { /* ... */ }
  ```
  Maps from a RouterAdvertPacket to a RouterAdvertPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RouterAdvertPacket<''a> { /* ... */ }
  ```
  Maps from a RouterAdvertPacket to a RouterAdvertPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &RouterAdvert) -> usize { /* ... */ }
  ```
  The size (in bytes) of a RouterAdvert instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_hop_limit(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hop_limit field.

- ```rust
  pub fn get_flags(self: &Self) -> u8 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_lifetime(self: &Self) -> u16be { /* ... */ }
  ```
  Get the lifetime field. This field is always stored big-endian

- ```rust
  pub fn get_reachable_time(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reachable_time field. This field is always stored big-endian

- ```rust
  pub fn get_retrans_time(self: &Self) -> u32be { /* ... */ }
  ```
  Get the retrans_time field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RouterAdvertPacket<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> RouterAdvert { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `MutableRouterAdvertPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableRouterAdvertPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableRouterAdvertPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableRouterAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableRouterAdvertPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableRouterAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RouterAdvertPacket<''p> { /* ... */ }
  ```
  Maps from a MutableRouterAdvertPacket to a RouterAdvertPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RouterAdvertPacket<''a> { /* ... */ }
  ```
  Maps from a MutableRouterAdvertPacket to a RouterAdvertPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &RouterAdvert) -> usize { /* ... */ }
  ```
  The size (in bytes) of a RouterAdvert instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &RouterAdvert) { /* ... */ }
  ```
  Populates a RouterAdvertPacket using a RouterAdvert structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_hop_limit(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hop_limit field.

- ```rust
  pub fn get_flags(self: &Self) -> u8 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_lifetime(self: &Self) -> u16be { /* ... */ }
  ```
  Get the lifetime field. This field is always stored big-endian

- ```rust
  pub fn get_reachable_time(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reachable_time field. This field is always stored big-endian

- ```rust
  pub fn get_retrans_time(self: &Self) -> u32be { /* ... */ }
  ```
  Get the retrans_time field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_hop_limit(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the hop_limit field.

- ```rust
  pub fn set_flags(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the flags field.

- ```rust
  pub fn set_lifetime(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the lifetime field. This field is always stored big-endian

- ```rust
  pub fn set_reachable_time(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the reachable_time field. This field is always stored big-endian

- ```rust
  pub fn set_retrans_time(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the retrans_time field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[NdpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableRouterAdvertPacket<''p>) -> bool { /* ... */ }
    ```

- **Freeze**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> RouterAdvert { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `RouterAdvertIterable`

Used to iterate over a slice of `RouterAdvertPacket`s

```rust
pub struct RouterAdvertIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<RouterAdvertPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RouterAdvert`

**Attributes:**

- `#[allow(unused_attributes)]`

Router Advertisement Message Format [RFC 4861 § 4.2]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
| Cur Hop Limit |M|O|  Reserved |       Router Lifetime         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                         Reachable Time                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                          Retrans Timer                        |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Options ...
+-+-+-+-+-+-+-+-+-+-+-+-
```

[RFC 4861 § 4.2]: https://tools.ietf.org/html/rfc4861#section-4.2

```rust
pub struct RouterAdvert {
    pub icmpv6_type: icmpv6::Icmpv6Type,
    pub icmpv6_code: icmpv6::Icmpv6Code,
    pub checksum: u16be,
    pub hop_limit: u8,
    pub flags: u8,
    pub lifetime: u16be,
    pub reachable_time: u32be,
    pub retrans_time: u32be,
    pub options: Vec<NdpOption>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `icmpv6::Icmpv6Type` |  |
| `icmpv6_code` | `icmpv6::Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `hop_limit` | `u8` |  |
| `flags` | `u8` |  |
| `lifetime` | `u16be` |  |
| `reachable_time` | `u32be` |  |
| `retrans_time` | `u32be` |  |
| `options` | `Vec<NdpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RouterAdvert { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `NeighborSolicitPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct NeighborSolicitPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<NeighborSolicitPacket<''p>> { /* ... */ }
  ```
  Constructs a new NeighborSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<NeighborSolicitPacket<''static>> { /* ... */ }
  ```
  Constructs a new NeighborSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NeighborSolicitPacket<''p> { /* ... */ }
  ```
  Maps from a NeighborSolicitPacket to a NeighborSolicitPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NeighborSolicitPacket<''a> { /* ... */ }
  ```
  Maps from a NeighborSolicitPacket to a NeighborSolicitPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NeighborSolicit) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NeighborSolicit instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NeighborSolicitPacket<''p>) -> bool { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NeighborSolicit { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `MutableNeighborSolicitPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableNeighborSolicitPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableNeighborSolicitPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableNeighborSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableNeighborSolicitPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableNeighborSolicitPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NeighborSolicitPacket<''p> { /* ... */ }
  ```
  Maps from a MutableNeighborSolicitPacket to a NeighborSolicitPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NeighborSolicitPacket<''a> { /* ... */ }
  ```
  Maps from a MutableNeighborSolicitPacket to a NeighborSolicitPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NeighborSolicit) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NeighborSolicit instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &NeighborSolicit) { /* ... */ }
  ```
  Populates a NeighborSolicitPacket using a NeighborSolicit structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_reserved(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the reserved field. This field is always stored big-endian

- ```rust
  pub fn set_target_addr(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the target_addr field.

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[NdpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NeighborSolicit { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableNeighborSolicitPacket<''p>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

#### Struct `NeighborSolicitIterable`

Used to iterate over a slice of `NeighborSolicitPacket`s

```rust
pub struct NeighborSolicitIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<NeighborSolicitPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

#### Struct `NeighborSolicit`

**Attributes:**

- `#[allow(unused_attributes)]`

Neighbor Solicitation Message Format [RFC 4861 § 4.3]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Reserved                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                                                               +
|                                                               |
+                       Target Address                          +
|                                                               |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Options ...
+-+-+-+-+-+-+-+-+-+-+-+-
```

[RFC 4861 § 4.3]: https://tools.ietf.org/html/rfc4861#section-4.3

```rust
pub struct NeighborSolicit {
    pub icmpv6_type: icmpv6::Icmpv6Type,
    pub icmpv6_code: icmpv6::Icmpv6Code,
    pub checksum: u16be,
    pub reserved: u32be,
    pub target_addr: std::net::Ipv6Addr,
    pub options: Vec<NdpOption>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `icmpv6::Icmpv6Type` |  |
| `icmpv6_code` | `icmpv6::Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `reserved` | `u32be` |  |
| `target_addr` | `std::net::Ipv6Addr` |  |
| `options` | `Vec<NdpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NeighborSolicit { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `NeighborAdvertPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct NeighborAdvertPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<NeighborAdvertPacket<''p>> { /* ... */ }
  ```
  Constructs a new NeighborAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<NeighborAdvertPacket<''static>> { /* ... */ }
  ```
  Constructs a new NeighborAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NeighborAdvertPacket<''p> { /* ... */ }
  ```
  Maps from a NeighborAdvertPacket to a NeighborAdvertPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NeighborAdvertPacket<''a> { /* ... */ }
  ```
  Maps from a NeighborAdvertPacket to a NeighborAdvertPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NeighborAdvert) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NeighborAdvert instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_flags(self: &Self) -> u8 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_reserved(self: &Self) -> u24be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **UnwindSafe**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NeighborAdvert { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NeighborAdvertPacket<''p>) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
#### Struct `MutableNeighborAdvertPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableNeighborAdvertPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableNeighborAdvertPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableNeighborAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableNeighborAdvertPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableNeighborAdvertPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> NeighborAdvertPacket<''p> { /* ... */ }
  ```
  Maps from a MutableNeighborAdvertPacket to a NeighborAdvertPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> NeighborAdvertPacket<''a> { /* ... */ }
  ```
  Maps from a MutableNeighborAdvertPacket to a NeighborAdvertPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &NeighborAdvert) -> usize { /* ... */ }
  ```
  The size (in bytes) of a NeighborAdvert instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &NeighborAdvert) { /* ... */ }
  ```
  Populates a NeighborAdvertPacket using a NeighborAdvert structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_flags(self: &Self) -> u8 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_reserved(self: &Self) -> u24be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_flags(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the flags field.

- ```rust
  pub fn set_reserved(self: &mut Self, val: u24be) { /* ... */ }
  ```
  Set the reserved field. This field is always stored big-endian

- ```rust
  pub fn set_target_addr(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the target_addr field.

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[NdpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableNeighborAdvertPacket<''p>) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> NeighborAdvert { /* ... */ }
    ```

#### Struct `NeighborAdvertIterable`

Used to iterate over a slice of `NeighborAdvertPacket`s

```rust
pub struct NeighborAdvertIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<NeighborAdvertPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
#### Struct `NeighborAdvert`

**Attributes:**

- `#[allow(unused_attributes)]`

Neighbor Advertisement Message Format [RFC 4861 § 4.4]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|R|S|O|                     Reserved                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                                                               +
|                                                               |
+                       Target Address                          +
|                                                               |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Options ...
+-+-+-+-+-+-+-+-+-+-+-+-
```

[RFC 4861 § 4.4]: https://tools.ietf.org/html/rfc4861#section-4.4

```rust
pub struct NeighborAdvert {
    pub icmpv6_type: icmpv6::Icmpv6Type,
    pub icmpv6_code: icmpv6::Icmpv6Code,
    pub checksum: u16be,
    pub flags: u8,
    pub reserved: u24be,
    pub target_addr: std::net::Ipv6Addr,
    pub options: Vec<NdpOption>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `icmpv6::Icmpv6Type` |  |
| `icmpv6_code` | `icmpv6::Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `flags` | `u8` |  |
| `reserved` | `u24be` |  |
| `target_addr` | `std::net::Ipv6Addr` |  |
| `options` | `Vec<NdpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NeighborAdvert { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `RedirectPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct RedirectPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<RedirectPacket<''p>> { /* ... */ }
  ```
  Constructs a new RedirectPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<RedirectPacket<''static>> { /* ... */ }
  ```
  Constructs a new RedirectPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RedirectPacket<''p> { /* ... */ }
  ```
  Maps from a RedirectPacket to a RedirectPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RedirectPacket<''a> { /* ... */ }
  ```
  Maps from a RedirectPacket to a RedirectPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Redirect) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Redirect instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_dest_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the dest_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RedirectPacket<''p>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Redirect { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `MutableRedirectPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableRedirectPacket<''p> {
    pub(in ::icmpv6::ndp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableRedirectPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableRedirectPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableRedirectPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableRedirectPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RedirectPacket<''p> { /* ... */ }
  ```
  Maps from a MutableRedirectPacket to a RedirectPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RedirectPacket<''a> { /* ... */ }
  ```
  Maps from a MutableRedirectPacket to a RedirectPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Redirect) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Redirect instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Redirect) { /* ... */ }
  ```
  Populates a RedirectPacket using a Redirect structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_reserved(self: &Self) -> u32be { /* ... */ }
  ```
  Get the reserved field. This field is always stored big-endian

- ```rust
  pub fn get_target_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the target_addr field

- ```rust
  pub fn get_dest_addr(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the dest_addr field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<NdpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> NdpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_reserved(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the reserved field. This field is always stored big-endian

- ```rust
  pub fn set_target_addr(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the target_addr field.

- ```rust
  pub fn set_dest_addr(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the dest_addr field.

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[NdpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableRedirectPacket<''p>) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Redirect { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `RedirectIterable`

Used to iterate over a slice of `RedirectPacket`s

```rust
pub struct RedirectIterable<''a> {
    pub(in ::icmpv6::ndp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Unpin**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<RedirectPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Redirect`

**Attributes:**

- `#[allow(unused_attributes)]`

Redirect Message Format [RFC 4861 § 4.5]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                           Reserved                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                                                               +
|                                                               |
+                       Target Address                          +
|                                                               |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                                                               +
|                                                               |
+                     Destination Address                       +
|                                                               |
+                                                               +
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|   Options ...
+-+-+-+-+-+-+-+-+-+-+-+-
```

[RFC 4861 § 4.5]: https://tools.ietf.org/html/rfc4861#section-4.5

```rust
pub struct Redirect {
    pub icmpv6_type: icmpv6::Icmpv6Type,
    pub icmpv6_code: icmpv6::Icmpv6Code,
    pub checksum: u16be,
    pub reserved: u32be,
    pub target_addr: std::net::Ipv6Addr,
    pub dest_addr: std::net::Ipv6Addr,
    pub options: Vec<NdpOption>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `icmpv6::Icmpv6Type` |  |
| `icmpv6_code` | `icmpv6::Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `reserved` | `u32be` |  |
| `target_addr` | `std::net::Ipv6Addr` |  |
| `dest_addr` | `std::net::Ipv6Addr` |  |
| `options` | `Vec<NdpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Redirect { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `ndp_option_payload_length`

Calculate a length of a `NdpOption`'s payload.

```rust
pub(in ::icmpv6::ndp) fn ndp_option_payload_length(option: &NdpOptionPacket<''_>) -> usize { /* ... */ }
```

#### Function `rs_ndp_options_length`

Router Solicit packet calculation for the length of the options.

```rust
pub(in ::icmpv6::ndp) fn rs_ndp_options_length(pkt: &RouterSolicitPacket<''_>) -> usize { /* ... */ }
```

#### Function `ra_ndp_options_length`

Router Advert packet calculation for the length of the options.

```rust
pub(in ::icmpv6::ndp) fn ra_ndp_options_length(pkt: &RouterAdvertPacket<''_>) -> usize { /* ... */ }
```

#### Function `ns_ndp_options_length`

Neighbor Solicit packet calculation for the length of the options.

```rust
pub(in ::icmpv6::ndp) fn ns_ndp_options_length(pkt: &NeighborSolicitPacket<''_>) -> usize { /* ... */ }
```

#### Function `na_ndp_options_length`

Neighbor Advert packet calculation for the length of the options.

```rust
pub(in ::icmpv6::ndp) fn na_ndp_options_length(pkt: &NeighborAdvertPacket<''_>) -> usize { /* ... */ }
```

#### Function `redirect_options_length`

Redirect packet calculation for the length of the options.

```rust
pub(in ::icmpv6::ndp) fn redirect_options_length(pkt: &RedirectPacket<''_>) -> usize { /* ... */ }
```

### Types

#### Struct `Icmpv6Type`

Represents the "ICMPv6 type" header field.

```rust
pub struct Icmpv6Type(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u8) -> Icmpv6Type { /* ... */ }
  ```
  Create a new `Icmpv6Type` instance.

###### Trait Implementations

- **Copy**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Icmpv6Type) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Icmpv6Type) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Icmpv6Type { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Icmpv6Type) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **StructuralPartialEq**
- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **Sync**
#### Struct `Icmpv6Code`

Represents the "ICMPv6 code" header field.

```rust
pub struct Icmpv6Code(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(val: u8) -> Icmpv6Code { /* ... */ }
  ```
  Create a new `Icmpv6Code` instance.

###### Trait Implementations

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Icmpv6Code) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Icmpv6Code { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **Unpin**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Icmpv6Code) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Icmpv6Code) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Copy**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Icmpv6Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct Icmpv6Packet<''p> {
    pub(in ::icmpv6) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<Icmpv6Packet<''p>> { /* ... */ }
  ```
  Constructs a new Icmpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<Icmpv6Packet<''static>> { /* ... */ }
  ```
  Constructs a new Icmpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Icmpv6Packet<''p> { /* ... */ }
  ```
  Maps from a Icmpv6Packet to a Icmpv6Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Icmpv6Packet<''a> { /* ... */ }
  ```
  Maps from a Icmpv6Packet to a Icmpv6Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Icmpv6) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Icmpv6 instance when converted into

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Icmpv6Packet<''p>) -> bool { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Icmpv6 { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `MutableIcmpv6Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableIcmpv6Packet<''p> {
    pub(in ::icmpv6) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableIcmpv6Packet<''p>> { /* ... */ }
  ```
  Constructs a new MutableIcmpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableIcmpv6Packet<''static>> { /* ... */ }
  ```
  Constructs a new MutableIcmpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Icmpv6Packet<''p> { /* ... */ }
  ```
  Maps from a MutableIcmpv6Packet to a Icmpv6Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Icmpv6Packet<''a> { /* ... */ }
  ```
  Maps from a MutableIcmpv6Packet to a Icmpv6Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Icmpv6) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Icmpv6 instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Icmpv6) { /* ... */ }
  ```
  Populates a Icmpv6Packet using a Icmpv6 structure

- ```rust
  pub fn get_icmpv6_type(self: &Self) -> Icmpv6Type { /* ... */ }
  ```
  Get the value of the icmpv6_type field

- ```rust
  pub fn get_icmpv6_code(self: &Self) -> Icmpv6Code { /* ... */ }
  ```
  Get the value of the icmpv6_code field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_icmpv6_type(self: &mut Self, val: Icmpv6Type) { /* ... */ }
  ```
  Set the value of the icmpv6_type field.

- ```rust
  pub fn set_icmpv6_code(self: &mut Self, val: Icmpv6Code) { /* ... */ }
  ```
  Set the value of the icmpv6_code field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Icmpv6 { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableIcmpv6Packet<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Send**
- **Sync**
#### Struct `Icmpv6Iterable`

Used to iterate over a slice of `Icmpv6Packet`s

```rust
pub struct Icmpv6Iterable<''a> {
    pub(in ::icmpv6) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Icmpv6Packet<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Icmpv6`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents a generic ICMPv6 packet [RFC 4443 § 2.1]

```text
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|     Type      |     Code      |          Checksum             |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+                         Message Body                          +
|                                                               |
```

[RFC 4443 § 2.1]: https://tools.ietf.org/html/rfc4443#section-2.1

```rust
pub struct Icmpv6 {
    pub icmpv6_type: Icmpv6Type,
    pub icmpv6_code: Icmpv6Code,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `icmpv6_type` | `Icmpv6Type` |  |
| `icmpv6_code` | `Icmpv6Code` |  |
| `checksum` | `u16be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Icmpv6 { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `checksum`

Calculates a checksum of an ICMPv6 packet.

```rust
pub fn checksum(packet: &Icmpv6Packet<''_>, source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr) -> u16be { /* ... */ }
```

## Module `ip`

Defines the type and constants for IP next header/next level protocol
fields.

```rust
pub mod ip { /* ... */ }
```

### Modules

## Module `IpNextHeaderProtocols`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

Protocol numbers as defined at:
http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
Above protocol numbers last updated: 2014-01-16
These values should be used in either the IPv4 Next Level Protocol field
or the IPv6 Next Header field.
NOTE Everything here is pretending to be an enum, but with namespacing by
     default, so we allow breaking style guidelines.

```rust
pub mod IpNextHeaderProtocols { /* ... */ }
```

### Constants and Statics

#### Constant `Hopopt`

IPv6 Hop-by-Hop Option [RFC2460]

```rust
pub const Hopopt: super::IpNextHeaderProtocol = _;
```

#### Constant `Icmp`

Internet Control Message [RFC792]

```rust
pub const Icmp: super::IpNextHeaderProtocol = _;
```

#### Constant `Igmp`

Internet Group Management [RFC1112]

```rust
pub const Igmp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ggp`

Gateway-to-Gateway [RFC823]

```rust
pub const Ggp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv4`

IPv4 encapsulation [RFC2003]

```rust
pub const Ipv4: super::IpNextHeaderProtocol = _;
```

#### Constant `St`

Stream [RFC1190][RFC1819]

```rust
pub const St: super::IpNextHeaderProtocol = _;
```

#### Constant `Tcp`

Transmission Control [RFC793]

```rust
pub const Tcp: super::IpNextHeaderProtocol = _;
```

#### Constant `Cbt`

CBT

```rust
pub const Cbt: super::IpNextHeaderProtocol = _;
```

#### Constant `Egp`

Exterior Gateway Protocol [RFC888]

```rust
pub const Egp: super::IpNextHeaderProtocol = _;
```

#### Constant `Igp`

any private interior gateway (used by Cisco for their IGRP)

```rust
pub const Igp: super::IpNextHeaderProtocol = _;
```

#### Constant `BbnRccMon`

BBN RCC Monitoring

```rust
pub const BbnRccMon: super::IpNextHeaderProtocol = _;
```

#### Constant `NvpII`

Network Voice Protocol [RFC741]

```rust
pub const NvpII: super::IpNextHeaderProtocol = _;
```

#### Constant `Pup`

PUP

```rust
pub const Pup: super::IpNextHeaderProtocol = _;
```

#### Constant `Argus`

ARGUS

```rust
pub const Argus: super::IpNextHeaderProtocol = _;
```

#### Constant `Emcon`

EMCON

```rust
pub const Emcon: super::IpNextHeaderProtocol = _;
```

#### Constant `Xnet`

Cross Net Debugger

```rust
pub const Xnet: super::IpNextHeaderProtocol = _;
```

#### Constant `Chaos`

Chaos

```rust
pub const Chaos: super::IpNextHeaderProtocol = _;
```

#### Constant `Udp`

User Datagram [RFC768]

```rust
pub const Udp: super::IpNextHeaderProtocol = _;
```

#### Constant `Mux`

Multiplexing

```rust
pub const Mux: super::IpNextHeaderProtocol = _;
```

#### Constant `DcnMeas`

DCN Measurement Subsystems

```rust
pub const DcnMeas: super::IpNextHeaderProtocol = _;
```

#### Constant `Hmp`

Host Monitoring [RFC869]

```rust
pub const Hmp: super::IpNextHeaderProtocol = _;
```

#### Constant `Prm`

Packet Radio Measurement

```rust
pub const Prm: super::IpNextHeaderProtocol = _;
```

#### Constant `XnsIdp`

XEROX NS IDP

```rust
pub const XnsIdp: super::IpNextHeaderProtocol = _;
```

#### Constant `Trunk1`

Trunk-1

```rust
pub const Trunk1: super::IpNextHeaderProtocol = _;
```

#### Constant `Trunk2`

Trunk-2

```rust
pub const Trunk2: super::IpNextHeaderProtocol = _;
```

#### Constant `Leaf1`

Leaf-1

```rust
pub const Leaf1: super::IpNextHeaderProtocol = _;
```

#### Constant `Leaf2`

Leaf-2

```rust
pub const Leaf2: super::IpNextHeaderProtocol = _;
```

#### Constant `Rdp`

Reliable Data Protocol [RFC908]

```rust
pub const Rdp: super::IpNextHeaderProtocol = _;
```

#### Constant `Irtp`

Internet Reliable Transaction [RFC938]

```rust
pub const Irtp: super::IpNextHeaderProtocol = _;
```

#### Constant `IsoTp4`

ISO Transport Protocol Class 4 [RFC905]

```rust
pub const IsoTp4: super::IpNextHeaderProtocol = _;
```

#### Constant `Netblt`

Bulk Data Transfer Protocol [RFC969]

```rust
pub const Netblt: super::IpNextHeaderProtocol = _;
```

#### Constant `MfeNsp`

MFE Network Services Protocol

```rust
pub const MfeNsp: super::IpNextHeaderProtocol = _;
```

#### Constant `MeritInp`

MERIT Internodal Protocol

```rust
pub const MeritInp: super::IpNextHeaderProtocol = _;
```

#### Constant `Dccp`

Datagram Congestion Control Protocol [RFC4340]

```rust
pub const Dccp: super::IpNextHeaderProtocol = _;
```

#### Constant `ThreePc`

Third Party Connect Protocol

```rust
pub const ThreePc: super::IpNextHeaderProtocol = _;
```

#### Constant `Idpr`

Inter-Domain Policy Routing Protocol

```rust
pub const Idpr: super::IpNextHeaderProtocol = _;
```

#### Constant `Xtp`

XTP

```rust
pub const Xtp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ddp`

Datagram Delivery Protocol

```rust
pub const Ddp: super::IpNextHeaderProtocol = _;
```

#### Constant `IdprCmtp`

IDPR Control Message Transport Proto

```rust
pub const IdprCmtp: super::IpNextHeaderProtocol = _;
```

#### Constant `TpPlusPlus`

TP++ Transport Protocol

```rust
pub const TpPlusPlus: super::IpNextHeaderProtocol = _;
```

#### Constant `Il`

IL Transport Protocol

```rust
pub const Il: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6`

IPv6 encapsulation [RFC2473]

```rust
pub const Ipv6: super::IpNextHeaderProtocol = _;
```

#### Constant `Sdrp`

Source Demand Routing Protocol

```rust
pub const Sdrp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6Route`

Routing Header for IPv6

```rust
pub const Ipv6Route: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6Frag`

Fragment Header for IPv6

```rust
pub const Ipv6Frag: super::IpNextHeaderProtocol = _;
```

#### Constant `Idrp`

Inter-Domain Routing Protocol

```rust
pub const Idrp: super::IpNextHeaderProtocol = _;
```

#### Constant `Rsvp`

Reservation Protocol [RFC2205][RFC3209]

```rust
pub const Rsvp: super::IpNextHeaderProtocol = _;
```

#### Constant `Gre`

Generic Routing Encapsulation [RFC1701]

```rust
pub const Gre: super::IpNextHeaderProtocol = _;
```

#### Constant `Dsr`

Dynamic Source Routing Protocol [RFC4728]

```rust
pub const Dsr: super::IpNextHeaderProtocol = _;
```

#### Constant `Bna`

BNA

```rust
pub const Bna: super::IpNextHeaderProtocol = _;
```

#### Constant `Esp`

Encap Security Payload [RFC4303]

```rust
pub const Esp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ah`

Authentication Header [RFC4302]

```rust
pub const Ah: super::IpNextHeaderProtocol = _;
```

#### Constant `INlsp`

Integrated Net Layer Security TUBA

```rust
pub const INlsp: super::IpNextHeaderProtocol = _;
```

#### Constant `Swipe`

IP with Encryption

```rust
pub const Swipe: super::IpNextHeaderProtocol = _;
```

#### Constant `Narp`

NBMA Address Resolution Protocol [RFC1735]

```rust
pub const Narp: super::IpNextHeaderProtocol = _;
```

#### Constant `Mobile`

IP Mobility

```rust
pub const Mobile: super::IpNextHeaderProtocol = _;
```

#### Constant `Tlsp`

Transport Layer Security Protocol using Kryptonet key management

```rust
pub const Tlsp: super::IpNextHeaderProtocol = _;
```

#### Constant `Skip`

SKIP

```rust
pub const Skip: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6Icmp`

**Attributes:**

- `#[deprecated(note = "Please use `IpNextHeaderProtocols::Icmpv6` instead")]`

**⚠️ Deprecated**: Please use `IpNextHeaderProtocols::Icmpv6` instead

```rust
pub const Ipv6Icmp: super::IpNextHeaderProtocol = _;
```

#### Constant `Icmpv6`

ICMPv6 [RFC4443]

```rust
pub const Icmpv6: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6NoNxt`

No Next Header for IPv6 [RFC2460]

```rust
pub const Ipv6NoNxt: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipv6Opts`

Destination Options for IPv6 [RFC2460]

```rust
pub const Ipv6Opts: super::IpNextHeaderProtocol = _;
```

#### Constant `HostInternal`

any host internal protocol

```rust
pub const HostInternal: super::IpNextHeaderProtocol = _;
```

#### Constant `Cftp`

CFTP

```rust
pub const Cftp: super::IpNextHeaderProtocol = _;
```

#### Constant `LocalNetwork`

any local network

```rust
pub const LocalNetwork: super::IpNextHeaderProtocol = _;
```

#### Constant `SatExpak`

SATNET and Backroom EXPAK

```rust
pub const SatExpak: super::IpNextHeaderProtocol = _;
```

#### Constant `Kryptolan`

Kryptolan

```rust
pub const Kryptolan: super::IpNextHeaderProtocol = _;
```

#### Constant `Rvd`

MIT Remote Virtual Disk Protocol

```rust
pub const Rvd: super::IpNextHeaderProtocol = _;
```

#### Constant `Ippc`

Internet Pluribus Packet Core

```rust
pub const Ippc: super::IpNextHeaderProtocol = _;
```

#### Constant `DistributedFs`

any distributed file system

```rust
pub const DistributedFs: super::IpNextHeaderProtocol = _;
```

#### Constant `SatMon`

SATNET Monitoring

```rust
pub const SatMon: super::IpNextHeaderProtocol = _;
```

#### Constant `Visa`

VISA Protocol

```rust
pub const Visa: super::IpNextHeaderProtocol = _;
```

#### Constant `Ipcv`

Internet Packet Core Utility

```rust
pub const Ipcv: super::IpNextHeaderProtocol = _;
```

#### Constant `Cpnx`

Computer Protocol Network Executive

```rust
pub const Cpnx: super::IpNextHeaderProtocol = _;
```

#### Constant `Cphb`

Computer Protocol Heart Beat

```rust
pub const Cphb: super::IpNextHeaderProtocol = _;
```

#### Constant `Wsn`

Wang Span Network

```rust
pub const Wsn: super::IpNextHeaderProtocol = _;
```

#### Constant `Pvp`

Packet Video Protocol

```rust
pub const Pvp: super::IpNextHeaderProtocol = _;
```

#### Constant `BrSatMon`

Backroom SATNET Monitoring

```rust
pub const BrSatMon: super::IpNextHeaderProtocol = _;
```

#### Constant `SunNd`

SUN ND PROTOCOL-Temporary

```rust
pub const SunNd: super::IpNextHeaderProtocol = _;
```

#### Constant `WbMon`

WIDEBAND Monitoring

```rust
pub const WbMon: super::IpNextHeaderProtocol = _;
```

#### Constant `WbExpak`

WIDEBAND EXPAK

```rust
pub const WbExpak: super::IpNextHeaderProtocol = _;
```

#### Constant `IsoIp`

ISO Internet Protocol

```rust
pub const IsoIp: super::IpNextHeaderProtocol = _;
```

#### Constant `Vmtp`

VMTP

```rust
pub const Vmtp: super::IpNextHeaderProtocol = _;
```

#### Constant `SecureVmtp`

SECURE-VMTP

```rust
pub const SecureVmtp: super::IpNextHeaderProtocol = _;
```

#### Constant `Vines`

VINES

```rust
pub const Vines: super::IpNextHeaderProtocol = _;
```

#### Constant `TtpOrIptm`

Transaction Transport Protocol/IP Traffic Manager

```rust
pub const TtpOrIptm: super::IpNextHeaderProtocol = _;
```

#### Constant `NsfnetIgp`

NSFNET-IGP

```rust
pub const NsfnetIgp: super::IpNextHeaderProtocol = _;
```

#### Constant `Dgp`

Dissimilar Gateway Protocol

```rust
pub const Dgp: super::IpNextHeaderProtocol = _;
```

#### Constant `Tcf`

TCF

```rust
pub const Tcf: super::IpNextHeaderProtocol = _;
```

#### Constant `Eigrp`

EIGRP

```rust
pub const Eigrp: super::IpNextHeaderProtocol = _;
```

#### Constant `OspfigP`

OSPFIGP [RFC1583][RFC2328][RFC5340]

```rust
pub const OspfigP: super::IpNextHeaderProtocol = _;
```

#### Constant `SpriteRpc`

Sprite RPC Protocol

```rust
pub const SpriteRpc: super::IpNextHeaderProtocol = _;
```

#### Constant `Larp`

Locus Address Resolution Protocol

```rust
pub const Larp: super::IpNextHeaderProtocol = _;
```

#### Constant `Mtp`

Multicast Transport Protocol

```rust
pub const Mtp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ax25`

AX.25 Frames

```rust
pub const Ax25: super::IpNextHeaderProtocol = _;
```

#### Constant `IpIp`

IP-within-IP Encapsulation Protocol

```rust
pub const IpIp: super::IpNextHeaderProtocol = _;
```

#### Constant `Micp`

Mobile Internetworking Control Pro.

```rust
pub const Micp: super::IpNextHeaderProtocol = _;
```

#### Constant `SccSp`

Semaphore Communications Sec. Pro.

```rust
pub const SccSp: super::IpNextHeaderProtocol = _;
```

#### Constant `Etherip`

Ethernet-within-IP Encapsulation [RFC3378]

```rust
pub const Etherip: super::IpNextHeaderProtocol = _;
```

#### Constant `Encap`

Encapsulation Header [RFC1241]

```rust
pub const Encap: super::IpNextHeaderProtocol = _;
```

#### Constant `PrivEncryption`

any private encryption scheme

```rust
pub const PrivEncryption: super::IpNextHeaderProtocol = _;
```

#### Constant `Gmtp`

GMTP

```rust
pub const Gmtp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ifmp`

Ipsilon Flow Management Protocol

```rust
pub const Ifmp: super::IpNextHeaderProtocol = _;
```

#### Constant `Pnni`

PNNI over IP

```rust
pub const Pnni: super::IpNextHeaderProtocol = _;
```

#### Constant `Pim`

Protocol Independent Multicast [RFC4601]

```rust
pub const Pim: super::IpNextHeaderProtocol = _;
```

#### Constant `Aris`

ARIS

```rust
pub const Aris: super::IpNextHeaderProtocol = _;
```

#### Constant `Scps`

SCPS

```rust
pub const Scps: super::IpNextHeaderProtocol = _;
```

#### Constant `Qnx`

QNX

```rust
pub const Qnx: super::IpNextHeaderProtocol = _;
```

#### Constant `AN`

Active Networks

```rust
pub const AN: super::IpNextHeaderProtocol = _;
```

#### Constant `IpComp`

IP Payload Compression Protocol [RFC2393]

```rust
pub const IpComp: super::IpNextHeaderProtocol = _;
```

#### Constant `Snp`

Sitara Networks Protocol

```rust
pub const Snp: super::IpNextHeaderProtocol = _;
```

#### Constant `CompaqPeer`

Compaq Peer Protocol

```rust
pub const CompaqPeer: super::IpNextHeaderProtocol = _;
```

#### Constant `IpxInIp`

IPX in IP

```rust
pub const IpxInIp: super::IpNextHeaderProtocol = _;
```

#### Constant `Vrrp`

Virtual Router Redundancy Protocol [RFC5798]

```rust
pub const Vrrp: super::IpNextHeaderProtocol = _;
```

#### Constant `Pgm`

PGM Reliable Transport Protocol

```rust
pub const Pgm: super::IpNextHeaderProtocol = _;
```

#### Constant `ZeroHop`

any 0-hop protocol

```rust
pub const ZeroHop: super::IpNextHeaderProtocol = _;
```

#### Constant `L2tp`

Layer Two Tunneling Protocol [RFC3931]

```rust
pub const L2tp: super::IpNextHeaderProtocol = _;
```

#### Constant `Ddx`

D-II Data Exchange (DDX)

```rust
pub const Ddx: super::IpNextHeaderProtocol = _;
```

#### Constant `Iatp`

Interactive Agent Transfer Protocol

```rust
pub const Iatp: super::IpNextHeaderProtocol = _;
```

#### Constant `Stp`

Schedule Transfer Protocol

```rust
pub const Stp: super::IpNextHeaderProtocol = _;
```

#### Constant `Srp`

SpectraLink Radio Protocol

```rust
pub const Srp: super::IpNextHeaderProtocol = _;
```

#### Constant `Uti`

UTI

```rust
pub const Uti: super::IpNextHeaderProtocol = _;
```

#### Constant `Smp`

Simple Message Protocol

```rust
pub const Smp: super::IpNextHeaderProtocol = _;
```

#### Constant `Sm`

Simple Multicast Protocol

```rust
pub const Sm: super::IpNextHeaderProtocol = _;
```

#### Constant `Ptp`

Performance Transparency Protocol

```rust
pub const Ptp: super::IpNextHeaderProtocol = _;
```

#### Constant `IsisOverIpv4`



```rust
pub const IsisOverIpv4: super::IpNextHeaderProtocol = _;
```

#### Constant `Fire`



```rust
pub const Fire: super::IpNextHeaderProtocol = _;
```

#### Constant `Crtp`

Combat Radio Transport Protocol

```rust
pub const Crtp: super::IpNextHeaderProtocol = _;
```

#### Constant `Crudp`

Combat Radio User Datagram

```rust
pub const Crudp: super::IpNextHeaderProtocol = _;
```

#### Constant `Sscopmce`



```rust
pub const Sscopmce: super::IpNextHeaderProtocol = _;
```

#### Constant `Iplt`



```rust
pub const Iplt: super::IpNextHeaderProtocol = _;
```

#### Constant `Sps`

Secure Packet Shield

```rust
pub const Sps: super::IpNextHeaderProtocol = _;
```

#### Constant `Pipe`

Private IP Encapsulation within IP

```rust
pub const Pipe: super::IpNextHeaderProtocol = _;
```

#### Constant `Sctp`

Stream Control Transmission Protocol

```rust
pub const Sctp: super::IpNextHeaderProtocol = _;
```

#### Constant `Fc`

Fibre Channel [RFC6172]

```rust
pub const Fc: super::IpNextHeaderProtocol = _;
```

#### Constant `RsvpE2eIgnore`

[RFC3175]

```rust
pub const RsvpE2eIgnore: super::IpNextHeaderProtocol = _;
```

#### Constant `MobilityHeader`

[RFC6275]

```rust
pub const MobilityHeader: super::IpNextHeaderProtocol = _;
```

#### Constant `UdpLite`

[RFC3828]

```rust
pub const UdpLite: super::IpNextHeaderProtocol = _;
```

#### Constant `MplsInIp`

[RFC4023]

```rust
pub const MplsInIp: super::IpNextHeaderProtocol = _;
```

#### Constant `Manet`

MANET Protocols [RFC5498]

```rust
pub const Manet: super::IpNextHeaderProtocol = _;
```

#### Constant `Hip`

Host Identity Protocol [RFC5201]

```rust
pub const Hip: super::IpNextHeaderProtocol = _;
```

#### Constant `Shim6`

Shim6 Protocol [RFC5533]

```rust
pub const Shim6: super::IpNextHeaderProtocol = _;
```

#### Constant `Wesp`

Wrapped Encapsulating Security Payload [RFC5840]

```rust
pub const Wesp: super::IpNextHeaderProtocol = _;
```

#### Constant `Rohc`

Robust Header Compression [RFC5858]

```rust
pub const Rohc: super::IpNextHeaderProtocol = _;
```

#### Constant `Test1`

Use for experimentation and testing [RFC3692]

```rust
pub const Test1: super::IpNextHeaderProtocol = _;
```

#### Constant `Test2`

Use for experimentation and testing [RFC3692]

```rust
pub const Test2: super::IpNextHeaderProtocol = _;
```

#### Constant `Reserved`



```rust
pub const Reserved: super::IpNextHeaderProtocol = _;
```

### Types

#### Struct `IpNextHeaderProtocol`

Represents an IPv4 next level protocol, or an IPv6 next header protocol,
see `IpNextHeaderProtocols` for a list of values.

```rust
pub struct IpNextHeaderProtocol(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u8) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Create a new IpNextHeaderProtocol

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &IpNextHeaderProtocol) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &IpNextHeaderProtocol) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &IpNextHeaderProtocol) -> bool { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `ipv4`

An IPv4 packet abstraction.

```rust
pub mod ipv4 { /* ... */ }
```

### Modules

## Module `Ipv4Flags`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The IPv4 header flags.

```rust
pub mod Ipv4Flags { /* ... */ }
```

### Constants and Statics

#### Constant `DontFragment`

Don't Fragment flag.

```rust
pub const DontFragment: u3 = 0b010;
```

#### Constant `MoreFragments`

More Fragments flag.

```rust
pub const MoreFragments: u3 = 0b001;
```

## Module `Ipv4OptionNumbers`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

IPv4 header options numbers as defined in
http://www.iana.org/assignments/ip-parameters/ip-parameters.xhtml

```rust
pub mod Ipv4OptionNumbers { /* ... */ }
```

### Constants and Statics

#### Constant `EOL`

End of Options List.

```rust
pub const EOL: super::Ipv4OptionNumber = _;
```

#### Constant `NOP`

No Operation.

```rust
pub const NOP: super::Ipv4OptionNumber = _;
```

#### Constant `SEC`

Security.

```rust
pub const SEC: super::Ipv4OptionNumber = _;
```

#### Constant `LSR`

Loose Source Route.

```rust
pub const LSR: super::Ipv4OptionNumber = _;
```

#### Constant `TS`

Time Stamp.

```rust
pub const TS: super::Ipv4OptionNumber = _;
```

#### Constant `ESEC`

Extended Security.

```rust
pub const ESEC: super::Ipv4OptionNumber = _;
```

#### Constant `CIPSO`

Commercial Security.

```rust
pub const CIPSO: super::Ipv4OptionNumber = _;
```

#### Constant `RR`

Record Route.

```rust
pub const RR: super::Ipv4OptionNumber = _;
```

#### Constant `SID`

Stream ID.

```rust
pub const SID: super::Ipv4OptionNumber = _;
```

#### Constant `SSR`

Strict Source Route.

```rust
pub const SSR: super::Ipv4OptionNumber = _;
```

#### Constant `ZSU`

Experimental Measurement.

```rust
pub const ZSU: super::Ipv4OptionNumber = _;
```

#### Constant `MTUP`

MTU Probe.

```rust
pub const MTUP: super::Ipv4OptionNumber = _;
```

#### Constant `MTUR`

MTU Reply.

```rust
pub const MTUR: super::Ipv4OptionNumber = _;
```

#### Constant `FINN`

Experimental Flow Control.

```rust
pub const FINN: super::Ipv4OptionNumber = _;
```

#### Constant `VISA`

Experimental Access Control.

```rust
pub const VISA: super::Ipv4OptionNumber = _;
```

#### Constant `ENCODE`

ENCODE.

```rust
pub const ENCODE: super::Ipv4OptionNumber = _;
```

#### Constant `IMITD`

IMI Traffic Descriptor.

```rust
pub const IMITD: super::Ipv4OptionNumber = _;
```

#### Constant `EIP`

Extended Internet Protocol.

```rust
pub const EIP: super::Ipv4OptionNumber = _;
```

#### Constant `TR`

Traceroute.

```rust
pub const TR: super::Ipv4OptionNumber = _;
```

#### Constant `ADDEXT`

Address Extension.

```rust
pub const ADDEXT: super::Ipv4OptionNumber = _;
```

#### Constant `RTRALT`

Router Alert.

```rust
pub const RTRALT: super::Ipv4OptionNumber = _;
```

#### Constant `SDB`

Selective Directed Broadcast.

```rust
pub const SDB: super::Ipv4OptionNumber = _;
```

#### Constant `DPS`

Dynamic Packet State.

```rust
pub const DPS: super::Ipv4OptionNumber = _;
```

#### Constant `UMP`

Upstream Multicast Pkt.

```rust
pub const UMP: super::Ipv4OptionNumber = _;
```

#### Constant `QS`

Quick-Start.

```rust
pub const QS: super::Ipv4OptionNumber = _;
```

#### Constant `EXP`

RFC3692-style Experiment.

```rust
pub const EXP: super::Ipv4OptionNumber = _;
```

### Types

#### Struct `Ipv4OptionNumber`

Represents an IPv4 option.

```rust
pub struct Ipv4OptionNumber(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u8) -> Ipv4OptionNumber { /* ... */ }
  ```
  Create a new `Ipv4OptionNumber` instance.

###### Trait Implementations

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ipv4OptionNumber { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Copy**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Ipv4OptionNumber) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv4OptionNumber) -> bool { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Ipv4OptionNumber) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
#### Struct `Ipv4Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct Ipv4Packet<''p> {
    pub(in ::ipv4) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<Ipv4Packet<''p>> { /* ... */ }
  ```
  Constructs a new Ipv4Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<Ipv4Packet<''static>> { /* ... */ }
  ```
  Constructs a new Ipv4Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv4Packet<''p> { /* ... */ }
  ```
  Maps from a Ipv4Packet to a Ipv4Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv4Packet<''a> { /* ... */ }
  ```
  Maps from a Ipv4Packet to a Ipv4Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv4) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv4 instance when converted into

- ```rust
  pub fn get_version(self: &Self) -> u4 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_header_length(self: &Self) -> u4 { /* ... */ }
  ```
  Get the header_length field.

- ```rust
  pub fn get_dscp(self: &Self) -> u6 { /* ... */ }
  ```
  Get the dscp field.

- ```rust
  pub fn get_ecn(self: &Self) -> u2 { /* ... */ }
  ```
  Get the ecn field.

- ```rust
  pub fn get_total_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the total_length field. This field is always stored big-endian

- ```rust
  pub fn get_identification(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identification field. This field is always stored big-endian

- ```rust
  pub fn get_flags(self: &Self) -> u3 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_fragment_offset(self: &Self) -> u13be { /* ... */ }
  ```
  Get the fragment_offset field. This field is always stored big-endian

- ```rust
  pub fn get_ttl(self: &Self) -> u8 { /* ... */ }
  ```
  Get the ttl field.

- ```rust
  pub fn get_next_level_protocol(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_level_protocol field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_source(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_destination(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the destination field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<Ipv4Option> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> Ipv4OptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Unpin**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv4Packet<''p>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv4 { /* ... */ }
    ```

#### Struct `MutableIpv4Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableIpv4Packet<''p> {
    pub(in ::ipv4) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableIpv4Packet<''p>> { /* ... */ }
  ```
  Constructs a new MutableIpv4Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableIpv4Packet<''static>> { /* ... */ }
  ```
  Constructs a new MutableIpv4Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv4Packet<''p> { /* ... */ }
  ```
  Maps from a MutableIpv4Packet to a Ipv4Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv4Packet<''a> { /* ... */ }
  ```
  Maps from a MutableIpv4Packet to a Ipv4Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv4) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv4 instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Ipv4) { /* ... */ }
  ```
  Populates a Ipv4Packet using a Ipv4 structure

- ```rust
  pub fn get_version(self: &Self) -> u4 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_header_length(self: &Self) -> u4 { /* ... */ }
  ```
  Get the header_length field.

- ```rust
  pub fn get_dscp(self: &Self) -> u6 { /* ... */ }
  ```
  Get the dscp field.

- ```rust
  pub fn get_ecn(self: &Self) -> u2 { /* ... */ }
  ```
  Get the ecn field.

- ```rust
  pub fn get_total_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the total_length field. This field is always stored big-endian

- ```rust
  pub fn get_identification(self: &Self) -> u16be { /* ... */ }
  ```
  Get the identification field. This field is always stored big-endian

- ```rust
  pub fn get_flags(self: &Self) -> u3 { /* ... */ }
  ```
  Get the flags field.

- ```rust
  pub fn get_fragment_offset(self: &Self) -> u13be { /* ... */ }
  ```
  Get the fragment_offset field. This field is always stored big-endian

- ```rust
  pub fn get_ttl(self: &Self) -> u8 { /* ... */ }
  ```
  Get the ttl field.

- ```rust
  pub fn get_next_level_protocol(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_level_protocol field

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_source(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_destination(self: &Self) -> Ipv4Addr { /* ... */ }
  ```
  Get the value of the destination field

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<Ipv4Option> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> Ipv4OptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_version(self: &mut Self, val: u4) { /* ... */ }
  ```
  Set the version field.

- ```rust
  pub fn set_header_length(self: &mut Self, val: u4) { /* ... */ }
  ```
  Set the header_length field.

- ```rust
  pub fn set_dscp(self: &mut Self, val: u6) { /* ... */ }
  ```
  Set the dscp field.

- ```rust
  pub fn set_ecn(self: &mut Self, val: u2) { /* ... */ }
  ```
  Set the ecn field.

- ```rust
  pub fn set_total_length(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the total_length field. This field is always stored big-endian

- ```rust
  pub fn set_identification(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the identification field. This field is always stored big-endian

- ```rust
  pub fn set_flags(self: &mut Self, val: u3) { /* ... */ }
  ```
  Set the flags field.

- ```rust
  pub fn set_fragment_offset(self: &mut Self, val: u13be) { /* ... */ }
  ```
  Set the fragment_offset field. This field is always stored big-endian

- ```rust
  pub fn set_ttl(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the ttl field.

- ```rust
  pub fn set_next_level_protocol(self: &mut Self, val: IpNextHeaderProtocol) { /* ... */ }
  ```
  Set the value of the next_level_protocol field.

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_source(self: &mut Self, val: Ipv4Addr) { /* ... */ }
  ```
  Set the value of the source field.

- ```rust
  pub fn set_destination(self: &mut Self, val: Ipv4Addr) { /* ... */ }
  ```
  Set the value of the destination field.

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[Ipv4Option]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv4 { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Unpin**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableIpv4Packet<''p>) -> bool { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `Ipv4Iterable`

Used to iterate over a slice of `Ipv4Packet`s

```rust
pub struct Ipv4Iterable<''a> {
    pub(in ::ipv4) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Ipv4Packet<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Ipv4`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an IPv4 Packet.

```rust
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
    pub next_level_protocol: ip::IpNextHeaderProtocol,
    pub checksum: u16be,
    pub source: std::net::Ipv4Addr,
    pub destination: std::net::Ipv4Addr,
    pub options: Vec<Ipv4Option>,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `u4` |  |
| `header_length` | `u4` |  |
| `dscp` | `u6` |  |
| `ecn` | `u2` |  |
| `total_length` | `u16be` |  |
| `identification` | `u16be` |  |
| `flags` | `u3` |  |
| `fragment_offset` | `u13be` |  |
| `ttl` | `u8` |  |
| `next_level_protocol` | `ip::IpNextHeaderProtocol` |  |
| `checksum` | `u16be` |  |
| `source` | `std::net::Ipv4Addr` |  |
| `destination` | `std::net::Ipv4Addr` |  |
| `options` | `Vec<Ipv4Option>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ipv4 { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Ipv4OptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct Ipv4OptionPacket<''p> {
    pub(in ::ipv4) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<Ipv4OptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new Ipv4OptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<Ipv4OptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new Ipv4OptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv4OptionPacket<''p> { /* ... */ }
  ```
  Maps from a Ipv4OptionPacket to a Ipv4OptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv4OptionPacket<''a> { /* ... */ }
  ```
  Maps from a Ipv4OptionPacket to a Ipv4OptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv4Option) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv4Option instance when converted into

- ```rust
  pub fn get_copied(self: &Self) -> u1 { /* ... */ }
  ```
  Get the copied field.

- ```rust
  pub fn get_class(self: &Self) -> u2 { /* ... */ }
  ```
  Get the class field.

- ```rust
  pub fn get_number(self: &Self) -> Ipv4OptionNumber { /* ... */ }
  ```
  Get the value of the number field

- ```rust
  pub fn get_length_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the length field, without copying

- ```rust
  pub fn get_length(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the length field (copies contents)

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv4Option { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv4OptionPacket<''p>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MutableIpv4OptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableIpv4OptionPacket<''p> {
    pub(in ::ipv4) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableIpv4OptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableIpv4OptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableIpv4OptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableIpv4OptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv4OptionPacket<''p> { /* ... */ }
  ```
  Maps from a MutableIpv4OptionPacket to a Ipv4OptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv4OptionPacket<''a> { /* ... */ }
  ```
  Maps from a MutableIpv4OptionPacket to a Ipv4OptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv4Option) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv4Option instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Ipv4Option) { /* ... */ }
  ```
  Populates a Ipv4OptionPacket using a Ipv4Option structure

- ```rust
  pub fn get_copied(self: &Self) -> u1 { /* ... */ }
  ```
  Get the copied field.

- ```rust
  pub fn get_class(self: &Self) -> u2 { /* ... */ }
  ```
  Get the class field.

- ```rust
  pub fn get_number(self: &Self) -> Ipv4OptionNumber { /* ... */ }
  ```
  Get the value of the number field

- ```rust
  pub fn get_length_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the length field, without copying

- ```rust
  pub fn get_length(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the length field (copies contents)

- ```rust
  pub fn set_copied(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the copied field.

- ```rust
  pub fn set_class(self: &mut Self, val: u2) { /* ... */ }
  ```
  Set the class field.

- ```rust
  pub fn set_number(self: &mut Self, val: Ipv4OptionNumber) { /* ... */ }
  ```
  Set the value of the number field.

- ```rust
  pub fn get_length_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the length field, without copying

- ```rust
  pub fn set_length(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the length field (copies contents)

- ```rust
  pub fn set_data(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the data field (copies contents)

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv4Option { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableIpv4OptionPacket<''p>) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Struct `Ipv4OptionIterable`

Used to iterate over a slice of `Ipv4OptionPacket`s

```rust
pub struct Ipv4OptionIterable<''a> {
    pub(in ::ipv4) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Ipv4OptionPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Ipv4Option`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents the IPv4 Option field.

```rust
pub struct Ipv4Option {
    pub(in ::ipv4) copied: u1,
    pub(in ::ipv4) class: u2,
    pub(in ::ipv4) number: Ipv4OptionNumber,
    pub(in ::ipv4) length: Vec<u8>,
    pub(in ::ipv4) data: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `copied` | `u1` |  |
| `class` | `u2` |  |
| `number` | `Ipv4OptionNumber` |  |
| `length` | `Vec<u8>` |  |
| `data` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ipv4Option { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
### Functions

#### Function `checksum`

Calculates a checksum of an IPv4 packet header.
The checksum field of the packet is regarded as zeros during the calculation.

```rust
pub fn checksum(packet: &Ipv4Packet<''_>) -> u16be { /* ... */ }
```

#### Function `ipv4_options_length`

```rust
pub(in ::ipv4) fn ipv4_options_length(ipv4: &Ipv4Packet<''_>) -> usize { /* ... */ }
```

#### Function `ipv4_payload_length`

```rust
pub(in ::ipv4) fn ipv4_payload_length(ipv4: &Ipv4Packet<''_>) -> usize { /* ... */ }
```

#### Function `ipv4_option_length`

This function gets the 'length' of the length field of the IPv4Option packet
Few options (EOL, NOP) are 1 bytes long, and then have a length field equal
to 0.

```rust
pub(in ::ipv4) fn ipv4_option_length(option: &Ipv4OptionPacket<''_>) -> usize { /* ... */ }
```

#### Function `ipv4_option_payload_length`

```rust
pub(in ::ipv4) fn ipv4_option_payload_length(ipv4_option: &Ipv4OptionPacket<''_>) -> usize { /* ... */ }
```

## Module `ipv6`

An IPv6 packet abstraction.

```rust
pub mod ipv6 { /* ... */ }
```

### Types

#### Struct `Ipv6Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct Ipv6Packet<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<Ipv6Packet<''p>> { /* ... */ }
  ```
  Constructs a new Ipv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<Ipv6Packet<''static>> { /* ... */ }
  ```
  Constructs a new Ipv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv6Packet<''p> { /* ... */ }
  ```
  Maps from a Ipv6Packet to a Ipv6Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv6Packet<''a> { /* ... */ }
  ```
  Maps from a Ipv6Packet to a Ipv6Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv6) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv6 instance when converted into

- ```rust
  pub fn get_version(self: &Self) -> u4 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_traffic_class(self: &Self) -> u8 { /* ... */ }
  ```
  Get the traffic_class field.

- ```rust
  pub fn get_flow_label(self: &Self) -> u20be { /* ... */ }
  ```
  Get the flow_label field. This field is always stored big-endian

- ```rust
  pub fn get_payload_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the payload_length field. This field is always stored big-endian

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hop_limit(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hop_limit field.

- ```rust
  pub fn get_source(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_destination(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the destination field

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv6 { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Ipv6Packet<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Freeze**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `MutableIpv6Packet`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableIpv6Packet<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableIpv6Packet<''p>> { /* ... */ }
  ```
  Constructs a new MutableIpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableIpv6Packet<''static>> { /* ... */ }
  ```
  Constructs a new MutableIpv6Packet. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> Ipv6Packet<''p> { /* ... */ }
  ```
  Maps from a MutableIpv6Packet to a Ipv6Packet

- ```rust
  pub fn consume_to_immutable(self: Self) -> Ipv6Packet<''a> { /* ... */ }
  ```
  Maps from a MutableIpv6Packet to a Ipv6Packet while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Ipv6) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Ipv6 instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Ipv6) { /* ... */ }
  ```
  Populates a Ipv6Packet using a Ipv6 structure

- ```rust
  pub fn get_version(self: &Self) -> u4 { /* ... */ }
  ```
  Get the version field.

- ```rust
  pub fn get_traffic_class(self: &Self) -> u8 { /* ... */ }
  ```
  Get the traffic_class field.

- ```rust
  pub fn get_flow_label(self: &Self) -> u20be { /* ... */ }
  ```
  Get the flow_label field. This field is always stored big-endian

- ```rust
  pub fn get_payload_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the payload_length field. This field is always stored big-endian

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hop_limit(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hop_limit field.

- ```rust
  pub fn get_source(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the source field

- ```rust
  pub fn get_destination(self: &Self) -> Ipv6Addr { /* ... */ }
  ```
  Get the value of the destination field

- ```rust
  pub fn set_version(self: &mut Self, val: u4) { /* ... */ }
  ```
  Set the version field.

- ```rust
  pub fn set_traffic_class(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the traffic_class field.

- ```rust
  pub fn set_flow_label(self: &mut Self, val: u20be) { /* ... */ }
  ```
  Set the flow_label field. This field is always stored big-endian

- ```rust
  pub fn set_payload_length(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the payload_length field. This field is always stored big-endian

- ```rust
  pub fn set_next_header(self: &mut Self, val: IpNextHeaderProtocol) { /* ... */ }
  ```
  Set the value of the next_header field.

- ```rust
  pub fn set_hop_limit(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the hop_limit field.

- ```rust
  pub fn set_source(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the source field.

- ```rust
  pub fn set_destination(self: &mut Self, val: Ipv6Addr) { /* ... */ }
  ```
  Set the value of the destination field.

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Ipv6 { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **UnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableIpv6Packet<''p>) -> bool { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Ipv6Iterable`

Used to iterate over a slice of `Ipv6Packet`s

```rust
pub struct Ipv6Iterable<''a> {
    pub(in ::ipv6) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<Ipv6Packet<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Ipv6`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an IPv6 Packet.

```rust
pub struct Ipv6 {
    pub version: u4,
    pub traffic_class: u8,
    pub flow_label: u20be,
    pub payload_length: u16be,
    pub next_header: ip::IpNextHeaderProtocol,
    pub hop_limit: u8,
    pub source: std::net::Ipv6Addr,
    pub destination: std::net::Ipv6Addr,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `version` | `u4` |  |
| `traffic_class` | `u8` |  |
| `flow_label` | `u20be` |  |
| `payload_length` | `u16be` |  |
| `next_header` | `ip::IpNextHeaderProtocol` |  |
| `hop_limit` | `u8` |  |
| `source` | `std::net::Ipv6Addr` |  |
| `destination` | `std::net::Ipv6Addr` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Ipv6 { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ExtensionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct ExtensionPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<ExtensionPacket<''p>> { /* ... */ }
  ```
  Constructs a new ExtensionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<ExtensionPacket<''static>> { /* ... */ }
  ```
  Constructs a new ExtensionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> ExtensionPacket<''p> { /* ... */ }
  ```
  Maps from a ExtensionPacket to a ExtensionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> ExtensionPacket<''a> { /* ... */ }
  ```
  Maps from a ExtensionPacket to a ExtensionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Extension) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Extension instance when converted into

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hdr_ext_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hdr_ext_len field.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Extension { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExtensionPacket<''p>) -> bool { /* ... */ }
    ```

#### Struct `MutableExtensionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableExtensionPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableExtensionPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableExtensionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableExtensionPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableExtensionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> ExtensionPacket<''p> { /* ... */ }
  ```
  Maps from a MutableExtensionPacket to a ExtensionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> ExtensionPacket<''a> { /* ... */ }
  ```
  Maps from a MutableExtensionPacket to a ExtensionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Extension) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Extension instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Extension) { /* ... */ }
  ```
  Populates a ExtensionPacket using a Extension structure

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hdr_ext_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hdr_ext_len field.

- ```rust
  pub fn set_next_header(self: &mut Self, val: IpNextHeaderProtocol) { /* ... */ }
  ```
  Set the value of the next_header field.

- ```rust
  pub fn set_hdr_ext_len(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the hdr_ext_len field.

- ```rust
  pub fn set_options(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Extension { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **StructuralPartialEq**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableExtensionPacket<''p>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

#### Struct `ExtensionIterable`

Used to iterate over a slice of `ExtensionPacket`s

```rust
pub struct ExtensionIterable<''a> {
    pub(in ::ipv6) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(buf: &[u8]) -> ExtensionIterable<''_> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<ExtensionPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
#### Struct `Extension`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an IPv6 Extension.

```rust
pub struct Extension {
    pub next_header: ip::IpNextHeaderProtocol,
    pub hdr_ext_len: u8,
    pub options: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next_header` | `ip::IpNextHeaderProtocol` |  |
| `hdr_ext_len` | `u8` |  |
| `options` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Extension { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
#### Type Alias `HopByHop`

Represents an IPv6 Hop-by-Hop Options.

```rust
pub type HopByHop = Extension;
```

#### Type Alias `HopByHopPacket`

A structure enabling manipulation of on the wire packets.

```rust
pub type HopByHopPacket<''p> = ExtensionPacket<''p>;
```

#### Type Alias `MutableHopByHopPacket`

A structure enabling manipulation of on the wire packets.

```rust
pub type MutableHopByHopPacket<''p> = MutableExtensionPacket<''p>;
```

#### Struct `RoutingPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct RoutingPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<RoutingPacket<''p>> { /* ... */ }
  ```
  Constructs a new RoutingPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<RoutingPacket<''static>> { /* ... */ }
  ```
  Constructs a new RoutingPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RoutingPacket<''p> { /* ... */ }
  ```
  Maps from a RoutingPacket to a RoutingPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RoutingPacket<''a> { /* ... */ }
  ```
  Maps from a RoutingPacket to a RoutingPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Routing) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Routing instance when converted into

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hdr_ext_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hdr_ext_len field.

- ```rust
  pub fn get_routing_type(self: &Self) -> u8 { /* ... */ }
  ```
  Get the routing_type field.

- ```rust
  pub fn get_segments_left(self: &Self) -> u8 { /* ... */ }
  ```
  Get the segments_left field.

###### Trait Implementations

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Routing { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RoutingPacket<''p>) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
#### Struct `MutableRoutingPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableRoutingPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableRoutingPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableRoutingPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableRoutingPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableRoutingPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> RoutingPacket<''p> { /* ... */ }
  ```
  Maps from a MutableRoutingPacket to a RoutingPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> RoutingPacket<''a> { /* ... */ }
  ```
  Maps from a MutableRoutingPacket to a RoutingPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Routing) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Routing instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Routing) { /* ... */ }
  ```
  Populates a RoutingPacket using a Routing structure

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_hdr_ext_len(self: &Self) -> u8 { /* ... */ }
  ```
  Get the hdr_ext_len field.

- ```rust
  pub fn get_routing_type(self: &Self) -> u8 { /* ... */ }
  ```
  Get the routing_type field.

- ```rust
  pub fn get_segments_left(self: &Self) -> u8 { /* ... */ }
  ```
  Get the segments_left field.

- ```rust
  pub fn set_next_header(self: &mut Self, val: IpNextHeaderProtocol) { /* ... */ }
  ```
  Set the value of the next_header field.

- ```rust
  pub fn set_hdr_ext_len(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the hdr_ext_len field.

- ```rust
  pub fn set_routing_type(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the routing_type field.

- ```rust
  pub fn set_segments_left(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the segments_left field.

- ```rust
  pub fn set_data(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the data field (copies contents)

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableRoutingPacket<''p>) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Routing { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
#### Struct `RoutingIterable`

Used to iterate over a slice of `RoutingPacket`s

```rust
pub struct RoutingIterable<''a> {
    pub(in ::ipv6) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<RoutingPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Routing`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an IPv6 Routing Extension.

```rust
pub struct Routing {
    pub next_header: ip::IpNextHeaderProtocol,
    pub hdr_ext_len: u8,
    pub routing_type: u8,
    pub segments_left: u8,
    pub data: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next_header` | `ip::IpNextHeaderProtocol` |  |
| `hdr_ext_len` | `u8` |  |
| `routing_type` | `u8` |  |
| `segments_left` | `u8` |  |
| `data` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Routing { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Send**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `FragmentPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct FragmentPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<FragmentPacket<''p>> { /* ... */ }
  ```
  Constructs a new FragmentPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<FragmentPacket<''static>> { /* ... */ }
  ```
  Constructs a new FragmentPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> FragmentPacket<''p> { /* ... */ }
  ```
  Maps from a FragmentPacket to a FragmentPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> FragmentPacket<''a> { /* ... */ }
  ```
  Maps from a FragmentPacket to a FragmentPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Fragment) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Fragment instance when converted into

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_reserved(self: &Self) -> u8 { /* ... */ }
  ```
  Get the reserved field.

- ```rust
  pub fn get_fragment_offset_with_flags(self: &Self) -> u16be { /* ... */ }
  ```
  Get the fragment_offset_with_flags field. This field is always stored big-endian

- ```rust
  pub fn get_id(self: &Self) -> u32be { /* ... */ }
  ```
  Get the id field. This field is always stored big-endian

- ```rust
  pub fn get_fragment_offset(self: &Self) -> u16 { /* ... */ }
  ```

- ```rust
  pub fn is_last_fragment(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Fragment { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FragmentPacket<''p>) -> bool { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **StructuralPartialEq**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `MutableFragmentPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableFragmentPacket<''p> {
    pub(in ::ipv6) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableFragmentPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableFragmentPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableFragmentPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableFragmentPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> FragmentPacket<''p> { /* ... */ }
  ```
  Maps from a MutableFragmentPacket to a FragmentPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> FragmentPacket<''a> { /* ... */ }
  ```
  Maps from a MutableFragmentPacket to a FragmentPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Fragment) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Fragment instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Fragment) { /* ... */ }
  ```
  Populates a FragmentPacket using a Fragment structure

- ```rust
  pub fn get_next_header(self: &Self) -> IpNextHeaderProtocol { /* ... */ }
  ```
  Get the value of the next_header field

- ```rust
  pub fn get_reserved(self: &Self) -> u8 { /* ... */ }
  ```
  Get the reserved field.

- ```rust
  pub fn get_fragment_offset_with_flags(self: &Self) -> u16be { /* ... */ }
  ```
  Get the fragment_offset_with_flags field. This field is always stored big-endian

- ```rust
  pub fn get_id(self: &Self) -> u32be { /* ... */ }
  ```
  Get the id field. This field is always stored big-endian

- ```rust
  pub fn set_next_header(self: &mut Self, val: IpNextHeaderProtocol) { /* ... */ }
  ```
  Set the value of the next_header field.

- ```rust
  pub fn set_reserved(self: &mut Self, val: u8) { /* ... */ }
  ```
  Set the reserved field.

- ```rust
  pub fn set_fragment_offset_with_flags(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the fragment_offset_with_flags field. This field is always stored big-endian

- ```rust
  pub fn set_id(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the id field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

- ```rust
  pub fn get_fragment_offset(self: &Self) -> u16 { /* ... */ }
  ```

- ```rust
  pub fn is_last_fragment(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn set_fragment_offset(self: &mut Self, offset: u16) { /* ... */ }
  ```

- ```rust
  pub fn set_last_fragment(self: &mut Self, is_last: bool) { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Fragment { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableFragmentPacket<''p>) -> bool { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `FragmentIterable`

Used to iterate over a slice of `FragmentPacket`s

```rust
pub struct FragmentIterable<''a> {
    pub(in ::ipv6) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<FragmentPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **Send**
#### Struct `Fragment`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents an IPv6 Fragment Extension.

```rust
pub struct Fragment {
    pub next_header: ip::IpNextHeaderProtocol,
    pub reserved: u8,
    pub fragment_offset_with_flags: u16be,
    pub id: u32be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `next_header` | `ip::IpNextHeaderProtocol` |  |
| `reserved` | `u8` |  |
| `fragment_offset_with_flags` | `u16be` |  |
| `id` | `u32be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Fragment { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Type Alias `Destination`

Represents an Destination Options.

```rust
pub type Destination = Extension;
```

#### Type Alias `DestinationPacket`

A structure enabling manipulation of on the wire packets.

```rust
pub type DestinationPacket<''p> = ExtensionPacket<''p>;
```

#### Type Alias `MutableDestinationPacket`

A structure enabling manipulation of on the wire packets.

```rust
pub type MutableDestinationPacket<''p> = MutableExtensionPacket<''p>;
```

### Functions

#### Function `ipv6_extension_length`

```rust
pub(in ::ipv6) fn ipv6_extension_length(ext: &ExtensionPacket<''_>) -> usize { /* ... */ }
```

#### Function `routing_extension_length`

```rust
pub(in ::ipv6) fn routing_extension_length(ext: &RoutingPacket<''_>) -> usize { /* ... */ }
```

### Constants and Statics

#### Constant `FRAGMENT_FLAGS_MASK`

```rust
pub(in ::ipv6) const FRAGMENT_FLAGS_MASK: u16 = 3;
```

#### Constant `FRAGMENT_FLAGS_MORE_FRAGMENTS`

```rust
pub(in ::ipv6) const FRAGMENT_FLAGS_MORE_FRAGMENTS: u16 = 1;
```

#### Constant `FRAGMENT_OFFSET_MASK`

```rust
pub(in ::ipv6) const FRAGMENT_OFFSET_MASK: u16 = _;
```

## Module `tcp`

An TCP packet abstraction.

```rust
pub mod tcp { /* ... */ }
```

### Modules

## Module `TcpFlags`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The TCP flags.

```rust
pub mod TcpFlags { /* ... */ }
```

### Constants and Statics

#### Constant `NS`

NS – ECN-nonce concealment protection (experimental: see RFC 3540).

```rust
pub const NS: u9be = 0b100000000;
```

#### Constant `CWR`

CWR – Congestion Window Reduced (CWR) flag is set by the sending
host to indicate that it received a TCP segment with the ECE flag set
and had responded in congestion control mechanism (added to header by RFC 3168).

```rust
pub const CWR: u9be = 0b010000000;
```

#### Constant `ECE`

ECE – ECN-Echo has a dual role, depending on the value of the
SYN flag. It indicates:
If the SYN flag is set (1), that the TCP peer is ECN capable.
If the SYN flag is clear (0), that a packet with Congestion Experienced
flag set (ECN=11) in IP header received during normal transmission
(added to header by RFC 3168).

```rust
pub const ECE: u9be = 0b001000000;
```

#### Constant `URG`

URG – indicates that the Urgent pointer field is significant.

```rust
pub const URG: u9be = 0b000100000;
```

#### Constant `ACK`

ACK – indicates that the Acknowledgment field is significant.
All packets after the initial SYN packet sent by the client should have this flag set.

```rust
pub const ACK: u9be = 0b000010000;
```

#### Constant `PSH`

PSH – Push function. Asks to push the buffered data to the receiving application.

```rust
pub const PSH: u9be = 0b000001000;
```

#### Constant `RST`

RST – Reset the connection.

```rust
pub const RST: u9be = 0b000000100;
```

#### Constant `SYN`

SYN – Synchronize sequence numbers. Only the first packet sent from each end
should have this flag set.

```rust
pub const SYN: u9be = 0b000000010;
```

#### Constant `FIN`

FIN – No more data from sender.

```rust
pub const FIN: u9be = 0b000000001;
```

## Module `TcpOptionNumbers`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

The TCP header options.

```rust
pub mod TcpOptionNumbers { /* ... */ }
```

### Constants and Statics

#### Constant `EOL`

End of Options list.

```rust
pub const EOL: super::TcpOptionNumber = _;
```

#### Constant `NOP`

No operation.

```rust
pub const NOP: super::TcpOptionNumber = _;
```

#### Constant `MSS`

Maximum segment size.

```rust
pub const MSS: super::TcpOptionNumber = _;
```

#### Constant `WSCALE`

Window scale.

```rust
pub const WSCALE: super::TcpOptionNumber = _;
```

#### Constant `SACK_PERMITTED`

Selective acknowledgements permitted.

```rust
pub const SACK_PERMITTED: super::TcpOptionNumber = _;
```

#### Constant `SACK`

Selective acknowledgment.

```rust
pub const SACK: super::TcpOptionNumber = _;
```

#### Constant `TIMESTAMPS`

Timestamps.

```rust
pub const TIMESTAMPS: super::TcpOptionNumber = _;
```

### Types

#### Struct `TcpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct TcpPacket<''p> {
    pub(in ::tcp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<TcpPacket<''p>> { /* ... */ }
  ```
  Constructs a new TcpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<TcpPacket<''static>> { /* ... */ }
  ```
  Constructs a new TcpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TcpPacket<''p> { /* ... */ }
  ```
  Maps from a TcpPacket to a TcpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TcpPacket<''a> { /* ... */ }
  ```
  Maps from a TcpPacket to a TcpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Tcp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Tcp instance when converted into

- ```rust
  pub fn get_source(self: &Self) -> u16be { /* ... */ }
  ```
  Get the source field. This field is always stored big-endian

- ```rust
  pub fn get_destination(self: &Self) -> u16be { /* ... */ }
  ```
  Get the destination field. This field is always stored big-endian

- ```rust
  pub fn get_sequence(self: &Self) -> u32be { /* ... */ }
  ```
  Get the sequence field. This field is always stored big-endian

- ```rust
  pub fn get_acknowledgement(self: &Self) -> u32be { /* ... */ }
  ```
  Get the acknowledgement field. This field is always stored big-endian

- ```rust
  pub fn get_data_offset(self: &Self) -> u4 { /* ... */ }
  ```
  Get the data_offset field.

- ```rust
  pub fn get_reserved(self: &Self) -> u3 { /* ... */ }
  ```
  Get the reserved field.

- ```rust
  pub fn get_flags(self: &Self) -> u9be { /* ... */ }
  ```
  Get the flags field. This field is always stored big-endian

- ```rust
  pub fn get_window(self: &Self) -> u16be { /* ... */ }
  ```
  Get the window field. This field is always stored big-endian

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_urgent_ptr(self: &Self) -> u16be { /* ... */ }
  ```
  Get the urgent_ptr field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<TcpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> TcpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TcpPacket<''p>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Tcp { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `MutableTcpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableTcpPacket<''p> {
    pub(in ::tcp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableTcpPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableTcpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableTcpPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableTcpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TcpPacket<''p> { /* ... */ }
  ```
  Maps from a MutableTcpPacket to a TcpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TcpPacket<''a> { /* ... */ }
  ```
  Maps from a MutableTcpPacket to a TcpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Tcp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Tcp instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Tcp) { /* ... */ }
  ```
  Populates a TcpPacket using a Tcp structure

- ```rust
  pub fn get_source(self: &Self) -> u16be { /* ... */ }
  ```
  Get the source field. This field is always stored big-endian

- ```rust
  pub fn get_destination(self: &Self) -> u16be { /* ... */ }
  ```
  Get the destination field. This field is always stored big-endian

- ```rust
  pub fn get_sequence(self: &Self) -> u32be { /* ... */ }
  ```
  Get the sequence field. This field is always stored big-endian

- ```rust
  pub fn get_acknowledgement(self: &Self) -> u32be { /* ... */ }
  ```
  Get the acknowledgement field. This field is always stored big-endian

- ```rust
  pub fn get_data_offset(self: &Self) -> u4 { /* ... */ }
  ```
  Get the data_offset field.

- ```rust
  pub fn get_reserved(self: &Self) -> u3 { /* ... */ }
  ```
  Get the reserved field.

- ```rust
  pub fn get_flags(self: &Self) -> u9be { /* ... */ }
  ```
  Get the flags field. This field is always stored big-endian

- ```rust
  pub fn get_window(self: &Self) -> u16be { /* ... */ }
  ```
  Get the window field. This field is always stored big-endian

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn get_urgent_ptr(self: &Self) -> u16be { /* ... */ }
  ```
  Get the urgent_ptr field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the options field, without copying

- ```rust
  pub fn get_options(self: &Self) -> Vec<TcpOption> { /* ... */ }
  ```
  Get the value of the options field (copies contents)

- ```rust
  pub fn get_options_iter(self: &Self) -> TcpOptionIterable<''_> { /* ... */ }
  ```
  Get the value of the options field as iterator

- ```rust
  pub fn set_source(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the source field. This field is always stored big-endian

- ```rust
  pub fn set_destination(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the destination field. This field is always stored big-endian

- ```rust
  pub fn set_sequence(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the sequence field. This field is always stored big-endian

- ```rust
  pub fn set_acknowledgement(self: &mut Self, val: u32be) { /* ... */ }
  ```
  Set the acknowledgement field. This field is always stored big-endian

- ```rust
  pub fn set_data_offset(self: &mut Self, val: u4) { /* ... */ }
  ```
  Set the data_offset field.

- ```rust
  pub fn set_reserved(self: &mut Self, val: u3) { /* ... */ }
  ```
  Set the reserved field.

- ```rust
  pub fn set_flags(self: &mut Self, val: u9be) { /* ... */ }
  ```
  Set the flags field. This field is always stored big-endian

- ```rust
  pub fn set_window(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the window field. This field is always stored big-endian

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_urgent_ptr(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the urgent_ptr field. This field is always stored big-endian

- ```rust
  pub fn get_options_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the options field, without copying

- ```rust
  pub fn set_options(self: &mut Self, vals: &[TcpOption]) { /* ... */ }
  ```
  Set the value of the options field (copies contents)

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Tcp { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableTcpPacket<''p>) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `TcpIterable`

Used to iterate over a slice of `TcpPacket`s

```rust
pub struct TcpIterable<''a> {
    pub(in ::tcp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<TcpPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Tcp`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents a TCP packet.

```rust
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
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `u16be` |  |
| `destination` | `u16be` |  |
| `sequence` | `u32be` |  |
| `acknowledgement` | `u32be` |  |
| `data_offset` | `u4` |  |
| `reserved` | `u3` |  |
| `flags` | `u9be` |  |
| `window` | `u16be` |  |
| `checksum` | `u16be` |  |
| `urgent_ptr` | `u16be` |  |
| `options` | `Vec<TcpOption>` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Tcp { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `TcpOptionNumber`

Represents a TCP option.

```rust
pub struct TcpOptionNumber(pub u8);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u8` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u8) -> TcpOptionNumber { /* ... */ }
  ```
  Create a new `TcpOptionNumber` instance.

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &TcpOptionNumber) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &TcpOptionNumber) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TcpOptionNumber) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TcpOptionNumber { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `TcpOptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct TcpOptionPacket<''p> {
    pub(in ::tcp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<TcpOptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new TcpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<TcpOptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new TcpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TcpOptionPacket<''p> { /* ... */ }
  ```
  Maps from a TcpOptionPacket to a TcpOptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TcpOptionPacket<''a> { /* ... */ }
  ```
  Maps from a TcpOptionPacket to a TcpOptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &TcpOption) -> usize { /* ... */ }
  ```
  The size (in bytes) of a TcpOption instance when converted into

- ```rust
  pub fn get_number(self: &Self) -> TcpOptionNumber { /* ... */ }
  ```
  Get the value of the number field

- ```rust
  pub fn get_length_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the length field, without copying

- ```rust
  pub fn get_length(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the length field (copies contents)

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> TcpOption { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TcpOptionPacket<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `MutableTcpOptionPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableTcpOptionPacket<''p> {
    pub(in ::tcp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableTcpOptionPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableTcpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableTcpOptionPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableTcpOptionPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> TcpOptionPacket<''p> { /* ... */ }
  ```
  Maps from a MutableTcpOptionPacket to a TcpOptionPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> TcpOptionPacket<''a> { /* ... */ }
  ```
  Maps from a MutableTcpOptionPacket to a TcpOptionPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &TcpOption) -> usize { /* ... */ }
  ```
  The size (in bytes) of a TcpOption instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &TcpOption) { /* ... */ }
  ```
  Populates a TcpOptionPacket using a TcpOption structure

- ```rust
  pub fn get_number(self: &Self) -> TcpOptionNumber { /* ... */ }
  ```
  Get the value of the number field

- ```rust
  pub fn get_length_raw(self: &Self) -> &[u8] { /* ... */ }
  ```
  Get the raw &[u8] value of the length field, without copying

- ```rust
  pub fn get_length(self: &Self) -> Vec<u8> { /* ... */ }
  ```
  Get the value of the length field (copies contents)

- ```rust
  pub fn set_number(self: &mut Self, val: TcpOptionNumber) { /* ... */ }
  ```
  Set the value of the number field.

- ```rust
  pub fn get_length_raw_mut(self: &mut Self) -> &mut [u8] { /* ... */ }
  ```
  Get the raw &mut [u8] value of the length field, without copying

- ```rust
  pub fn set_length(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the length field (copies contents)

- ```rust
  pub fn set_data(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the data field (copies contents)

###### Trait Implementations

- **Sync**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableTcpOptionPacket<''p>) -> bool { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> TcpOption { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `TcpOptionIterable`

Used to iterate over a slice of `TcpOptionPacket`s

```rust
pub struct TcpOptionIterable<''a> {
    pub(in ::tcp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<TcpOptionPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `TcpOption`

**Attributes:**

- `#[allow(unused_attributes)]`

A TCP option.

```rust
pub struct TcpOption {
    pub(in ::tcp) number: TcpOptionNumber,
    pub(in ::tcp) length: Vec<u8>,
    pub(in ::tcp) data: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `number` | `TcpOptionNumber` |  |
| `length` | `Vec<u8>` |  |
| `data` | `Vec<u8>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn nop() -> Self { /* ... */ }
  ```
  NOP: This may be used to align option fields on 32-bit boundaries for better performance.

- ```rust
  pub fn timestamp(my: u32, their: u32) -> Self { /* ... */ }
  ```
  Timestamp: TCP timestamps, defined in RFC 1323, can help TCP determine in which order

- ```rust
  pub fn mss(val: u16) -> Self { /* ... */ }
  ```
  MSS: The maximum segment size (MSS) is the largest amount of data, specified in bytes,

- ```rust
  pub fn wscale(val: u8) -> Self { /* ... */ }
  ```
  Window scale: The TCP window scale option, as defined in RFC 1323, is an option used to

- ```rust
  pub fn sack_perm() -> Self { /* ... */ }
  ```
  Selective acknowledgment (SACK) option, defined in RFC 2018 allows the receiver to acknowledge

- ```rust
  pub fn selective_ack(acks: &[u32]) -> Self { /* ... */ }
  ```
  Selective acknowledgment (SACK) option, defined in RFC 2018 allows the receiver to acknowledge

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TcpOption { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

### Functions

#### Function `tcp_option_length`

**Attributes:**

- `#[inline]`

This function gets the 'length' of the length field of the IPv4Option packet
Few options (EOL, NOP) are 1 bytes long, and then have a length field equal
to 0.

```rust
pub(in ::tcp) fn tcp_option_length(option: &TcpOptionPacket<''_>) -> usize { /* ... */ }
```

#### Function `tcp_option_payload_length`

```rust
pub(in ::tcp) fn tcp_option_payload_length(ipv4_option: &TcpOptionPacket<''_>) -> usize { /* ... */ }
```

#### Function `tcp_options_length`

**Attributes:**

- `#[inline]`

```rust
pub(in ::tcp) fn tcp_options_length(tcp: &TcpPacket<''_>) -> usize { /* ... */ }
```

#### Function `ipv4_checksum`

Calculate a checksum for a packet built on IPv4.

```rust
pub fn ipv4_checksum(packet: &TcpPacket<''_>, source: &std::net::Ipv4Addr, destination: &std::net::Ipv4Addr) -> u16 { /* ... */ }
```

#### Function `ipv4_checksum_adv`

Calculate the checksum for a packet built on IPv4, Advanced version which
accepts an extra slice of data that will be included in the checksum
as being part of the data portion of the packet.

If `packet` contains an odd number of bytes the last byte will not be
counted as the first byte of a word together with the first byte of
`extra_data`.

```rust
pub fn ipv4_checksum_adv(packet: &TcpPacket<''_>, extra_data: &[u8], source: &std::net::Ipv4Addr, destination: &std::net::Ipv4Addr) -> u16 { /* ... */ }
```

#### Function `ipv6_checksum`

Calculate a checksum for a packet built on IPv6.

```rust
pub fn ipv6_checksum(packet: &TcpPacket<''_>, source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr) -> u16 { /* ... */ }
```

#### Function `ipv6_checksum_adv`

Calculate the checksum for a packet built on IPv6, Advanced version which
accepts an extra slice of data that will be included in the checksum
as being part of the data portion of the packet.

If `packet` contains an odd number of bytes the last byte will not be
counted as the first byte of a word together with the first byte of
`extra_data`.

```rust
pub fn ipv6_checksum_adv(packet: &TcpPacket<''_>, extra_data: &[u8], source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr) -> u16 { /* ... */ }
```

## Module `udp`

A UDP packet abstraction.

```rust
pub mod udp { /* ... */ }
```

### Types

#### Struct `UdpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct UdpPacket<''p> {
    pub(in ::udp) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<UdpPacket<''p>> { /* ... */ }
  ```
  Constructs a new UdpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<UdpPacket<''static>> { /* ... */ }
  ```
  Constructs a new UdpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> UdpPacket<''p> { /* ... */ }
  ```
  Maps from a UdpPacket to a UdpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> UdpPacket<''a> { /* ... */ }
  ```
  Maps from a UdpPacket to a UdpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Udp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Udp instance when converted into

- ```rust
  pub fn get_source(self: &Self) -> u16be { /* ... */ }
  ```
  Get the source field. This field is always stored big-endian

- ```rust
  pub fn get_destination(self: &Self) -> u16be { /* ... */ }
  ```
  Get the destination field. This field is always stored big-endian

- ```rust
  pub fn get_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the length field. This field is always stored big-endian

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

###### Trait Implementations

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Udp { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UdpPacket<''p>) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Struct `MutableUdpPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableUdpPacket<''p> {
    pub(in ::udp) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableUdpPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableUdpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableUdpPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableUdpPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> UdpPacket<''p> { /* ... */ }
  ```
  Maps from a MutableUdpPacket to a UdpPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> UdpPacket<''a> { /* ... */ }
  ```
  Maps from a MutableUdpPacket to a UdpPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Udp) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Udp instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Udp) { /* ... */ }
  ```
  Populates a UdpPacket using a Udp structure

- ```rust
  pub fn get_source(self: &Self) -> u16be { /* ... */ }
  ```
  Get the source field. This field is always stored big-endian

- ```rust
  pub fn get_destination(self: &Self) -> u16be { /* ... */ }
  ```
  Get the destination field. This field is always stored big-endian

- ```rust
  pub fn get_length(self: &Self) -> u16be { /* ... */ }
  ```
  Get the length field. This field is always stored big-endian

- ```rust
  pub fn get_checksum(self: &Self) -> u16be { /* ... */ }
  ```
  Get the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_source(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the source field. This field is always stored big-endian

- ```rust
  pub fn set_destination(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the destination field. This field is always stored big-endian

- ```rust
  pub fn set_length(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the length field. This field is always stored big-endian

- ```rust
  pub fn set_checksum(self: &mut Self, val: u16be) { /* ... */ }
  ```
  Set the checksum field. This field is always stored big-endian

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableUdpPacket<''p>) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Udp { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `UdpIterable`

Used to iterate over a slice of `UdpPacket`s

```rust
pub struct UdpIterable<''a> {
    pub(in ::udp) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<UdpPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `Udp`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents a UDP Packet.

```rust
pub struct Udp {
    pub source: u16be,
    pub destination: u16be,
    pub length: u16be,
    pub checksum: u16be,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `u16be` |  |
| `destination` | `u16be` |  |
| `length` | `u16be` |  |
| `checksum` | `u16be` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Udp { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `ipv4_checksum`

Calculate a checksum for a packet built on IPv4.

```rust
pub fn ipv4_checksum(packet: &UdpPacket<''_>, source: &std::net::Ipv4Addr, destination: &std::net::Ipv4Addr) -> u16be { /* ... */ }
```

#### Function `ipv4_checksum_adv`

Calculate a checksum for a packet built on IPv4. Advanced version which
accepts an extra slice of data that will be included in the checksum
as being part of the data portion of the packet.

If `packet` contains an odd number of bytes the last byte will not be
counted as the first byte of a word together with the first byte of
`extra_data`.

```rust
pub fn ipv4_checksum_adv(packet: &UdpPacket<''_>, extra_data: &[u8], source: &std::net::Ipv4Addr, destination: &std::net::Ipv4Addr) -> u16be { /* ... */ }
```

#### Function `ipv6_checksum`

Calculate a checksum for a packet built on IPv6.

```rust
pub fn ipv6_checksum(packet: &UdpPacket<''_>, source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr) -> u16be { /* ... */ }
```

#### Function `ipv6_checksum_adv`

Calculate the checksum for a packet built on IPv6. Advanced version which
accepts an extra slice of data that will be included in the checksum
as being part of the data portion of the packet.

If `packet` contains an odd number of bytes the last byte will not be
counted as the first byte of a word together with the first byte of
`extra_data`.

```rust
pub fn ipv6_checksum_adv(packet: &UdpPacket<''_>, extra_data: &[u8], source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr) -> u16be { /* ... */ }
```

## Module `vlan`

A VLAN packet abstraction.

```rust
pub mod vlan { /* ... */ }
```

### Modules

## Module `ClassesOfService`

**Attributes:**

- `#[allow(non_snake_case)]`
- `#[allow(non_upper_case_globals)]`

IEEE 802.1p classes of service as defined in
https://en.wikipedia.org/wiki/IEEE_P802.1p.

```rust
pub mod ClassesOfService { /* ... */ }
```

### Constants and Statics

#### Constant `BK`

Background

```rust
pub const BK: super::ClassOfService = _;
```

#### Constant `BE`

Best Effort

```rust
pub const BE: super::ClassOfService = _;
```

#### Constant `EE`

Excellent Effort

```rust
pub const EE: super::ClassOfService = _;
```

#### Constant `CA`

Critical Applications

```rust
pub const CA: super::ClassOfService = _;
```

#### Constant `VI`

Video, < 100 ms latency

```rust
pub const VI: super::ClassOfService = _;
```

#### Constant `VO`

Voice, < 10 ms latency

```rust
pub const VO: super::ClassOfService = _;
```

#### Constant `IC`

Internetwork Control

```rust
pub const IC: super::ClassOfService = _;
```

#### Constant `NC`

Network Control

```rust
pub const NC: super::ClassOfService = _;
```

### Types

#### Struct `ClassOfService`

Represents an IEEE 802.1p class of a service.

```rust
pub struct ClassOfService(pub u3);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `u3` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(value: u3) -> ClassOfService { /* ... */ }
  ```
  Create a new `ClassOfService` instance.

###### Trait Implementations

- **Copy**
- **PrimitiveValues**
  - ```rust
    fn to_primitive_values(self: &Self) -> (u3) { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **StructuralPartialEq**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ClassOfService) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ClassOfService) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ClassOfService { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ClassOfService) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `VlanPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct VlanPacket<''p> {
    pub(in ::vlan) packet: ::pnet_macros_support::packet::PacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::PacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p [u8]) -> Option<VlanPacket<''p>> { /* ... */ }
  ```
  Constructs a new VlanPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<VlanPacket<''static>> { /* ... */ }
  ```
  Constructs a new VlanPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> VlanPacket<''p> { /* ... */ }
  ```
  Maps from a VlanPacket to a VlanPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> VlanPacket<''a> { /* ... */ }
  ```
  Maps from a VlanPacket to a VlanPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Vlan) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Vlan instance when converted into

- ```rust
  pub fn get_priority_code_point(self: &Self) -> ClassOfService { /* ... */ }
  ```
  Get the value of the priority_code_point field

- ```rust
  pub fn get_drop_eligible_indicator(self: &Self) -> u1 { /* ... */ }
  ```
  Get the drop_eligible_indicator field.

- ```rust
  pub fn get_vlan_identifier(self: &Self) -> u12be { /* ... */ }
  ```
  Get the vlan_identifier field. This field is always stored big-endian

- ```rust
  pub fn get_ethertype(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the ethertype field

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VlanPacket<''p>) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **UnwindSafe**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Vlan { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `MutableVlanPacket`

A structure enabling manipulation of on the wire packets

```rust
pub struct MutableVlanPacket<''p> {
    pub(in ::vlan) packet: ::pnet_macros_support::packet::MutPacketData<''p>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `packet` | `::pnet_macros_support::packet::MutPacketData<''p>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new<''p>(packet: &''p mut [u8]) -> Option<MutableVlanPacket<''p>> { /* ... */ }
  ```
  Constructs a new MutableVlanPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn owned(packet: Vec<u8>) -> Option<MutableVlanPacket<''static>> { /* ... */ }
  ```
  Constructs a new MutableVlanPacket. If the provided buffer is less than the minimum required

- ```rust
  pub fn to_immutable<''p>(self: &''p Self) -> VlanPacket<''p> { /* ... */ }
  ```
  Maps from a MutableVlanPacket to a VlanPacket

- ```rust
  pub fn consume_to_immutable(self: Self) -> VlanPacket<''a> { /* ... */ }
  ```
  Maps from a MutableVlanPacket to a VlanPacket while consuming the source

- ```rust
  pub const fn minimum_packet_size() -> usize { /* ... */ }
  ```
  The minimum size (in bytes) a packet of this type can be. It's based on the total size

- ```rust
  pub fn packet_size(_packet: &Vlan) -> usize { /* ... */ }
  ```
  The size (in bytes) of a Vlan instance when converted into

- ```rust
  pub fn populate(self: &mut Self, packet: &Vlan) { /* ... */ }
  ```
  Populates a VlanPacket using a Vlan structure

- ```rust
  pub fn get_priority_code_point(self: &Self) -> ClassOfService { /* ... */ }
  ```
  Get the value of the priority_code_point field

- ```rust
  pub fn get_drop_eligible_indicator(self: &Self) -> u1 { /* ... */ }
  ```
  Get the drop_eligible_indicator field.

- ```rust
  pub fn get_vlan_identifier(self: &Self) -> u12be { /* ... */ }
  ```
  Get the vlan_identifier field. This field is always stored big-endian

- ```rust
  pub fn get_ethertype(self: &Self) -> EtherType { /* ... */ }
  ```
  Get the value of the ethertype field

- ```rust
  pub fn set_priority_code_point(self: &mut Self, val: ClassOfService) { /* ... */ }
  ```
  Set the value of the priority_code_point field.

- ```rust
  pub fn set_drop_eligible_indicator(self: &mut Self, val: u1) { /* ... */ }
  ```
  Set the drop_eligible_indicator field.

- ```rust
  pub fn set_vlan_identifier(self: &mut Self, val: u12be) { /* ... */ }
  ```
  Set the vlan_identifier field. This field is always stored big-endian

- ```rust
  pub fn set_ethertype(self: &mut Self, val: EtherType) { /* ... */ }
  ```
  Set the value of the ethertype field.

- ```rust
  pub fn set_payload(self: &mut Self, vals: &[u8]) { /* ... */ }
  ```
  Set the value of the payload field (copies contents)

###### Trait Implementations

- **Packet**
  - ```rust
    fn packet<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

  - ```rust
    fn payload<''p>(self: &''p Self) -> &''p [u8] { /* ... */ }
    ```

- **Unpin**
- **FromPacket**
  - ```rust
    fn from_packet(self: &Self) -> Vlan { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PacketSize**
  - ```rust
    fn packet_size(self: &Self) -> usize { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &MutableVlanPacket<''p>) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, fmt: &mut ::std::fmt::Formatter<''_>) -> ::std::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **MutablePacket**
  - ```rust
    fn packet_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

  - ```rust
    fn payload_mut<''p>(self: &''p mut Self) -> &''p mut [u8] { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
#### Struct `VlanIterable`

Used to iterate over a slice of `VlanPacket`s

```rust
pub struct VlanIterable<''a> {
    pub(in ::vlan) buf: &''a [u8],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buf` | `&''a [u8]` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Iterator**
  - ```rust
    fn next(self: &mut Self) -> Option<VlanPacket<''a>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> I { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `Vlan`

**Attributes:**

- `#[allow(unused_attributes)]`

Represents a VLAN-tagged packet.

```rust
pub struct Vlan {
    pub priority_code_point: ClassOfService,
    pub drop_eligible_indicator: u1,
    pub vlan_identifier: u12be,
    pub ethertype: ethernet::EtherType,
    pub payload: Vec<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `priority_code_point` | `ClassOfService` |  |
| `drop_eligible_indicator` | `u1` |  |
| `vlan_identifier` | `u12be` |  |
| `ethertype` | `ethernet::EtherType` |  |
| `payload` | `Vec<u8>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Vlan { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `util`

Utilities for working with packets, eg. checksumming.

```rust
pub mod util { /* ... */ }
```

### Traits

#### Trait `Octets`

Convert a value to a byte array.

```rust
pub trait Octets {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Output`: Output type - bytes array.

###### Required Methods

- `octets`: Return a value as bytes (big-endian order).

##### Implementations

This trait is implemented for the following types:

- `u64`
- `u32`
- `u16`
- `u8`

### Functions

#### Function `checksum`

Calculates a checksum. Used by ipv4 and icmp. The two bytes starting at `skipword * 2` will be
ignored. Supposed to be the checksum field, which is regarded as zero during calculation.

```rust
pub fn checksum(data: &[u8], skipword: usize) -> pnet_macros_support::types::u16be { /* ... */ }
```

#### Function `finalize_checksum`

```rust
pub(in ::util) fn finalize_checksum(sum: u32) -> pnet_macros_support::types::u16be { /* ... */ }
```

#### Function `ipv4_checksum`

Calculate the checksum for a packet built on IPv4. Used by UDP and TCP.

```rust
pub fn ipv4_checksum(data: &[u8], skipword: usize, extra_data: &[u8], source: &std::net::Ipv4Addr, destination: &std::net::Ipv4Addr, next_level_protocol: ip::IpNextHeaderProtocol) -> pnet_macros_support::types::u16be { /* ... */ }
```

#### Function `ipv4_word_sum`

```rust
pub(in ::util) fn ipv4_word_sum(ip: &std::net::Ipv4Addr) -> u32 { /* ... */ }
```

#### Function `ipv6_checksum`

Calculate the checksum for a packet built on IPv6.

```rust
pub fn ipv6_checksum(data: &[u8], skipword: usize, extra_data: &[u8], source: &std::net::Ipv6Addr, destination: &std::net::Ipv6Addr, next_level_protocol: ip::IpNextHeaderProtocol) -> pnet_macros_support::types::u16be { /* ... */ }
```

#### Function `ipv6_word_sum`

```rust
pub(in ::util) fn ipv6_word_sum(ip: &std::net::Ipv6Addr) -> u32 { /* ... */ }
```

#### Function `sum_be_words`

Sum all words (16 bit chunks) in the given data. The word at word offset
`skipword` will be skipped. Each word is treated as big endian.

```rust
pub(in ::util) fn sum_be_words(data: &[u8], skipword: usize) -> u32 { /* ... */ }
```

## Re-exports

### Re-export `pnet_macros_support::packet::*`

```rust
pub use pnet_macros_support::packet::*;
```

