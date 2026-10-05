use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;

pub const FLAG_PLATFORM_RESET: uint32 = 16;
pub const FLAG_SIBLING_RESET: uint32 = 8;
pub const FLAG_DOMAIN_RESET: uint32 = 4;

pub open spec fn ResultEqual(result: int32, code: int32) -> bool;

pub open spec fn DomainIsValid(s: S, domain_id: uint32) -> bool;

pub open spec fn CapabilityDescriptorIsValid(s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn CapabilityTypeBitsSet(capability: uint32) -> int;

pub open spec fn CapabilitySubtypeBitsSet(capability: uint32) -> int;

pub open spec fn ReservedBitsNonZero(capability: uint32) -> bool;

pub open spec fn ReservedBitsZero(capability: uint32) -> bool;

pub open spec fn InvalidFlags(flags: uint32) -> bool;

pub open spec fn ValidFlags(flags: uint32) -> bool;

pub open spec fn QosValueNotSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool;

pub open spec fn QosValueSupported(s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool;

pub open spec fn AgentPermitted(s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn SiblingDomains(s: S, domain_id: uint32) -> uint32;

pub open spec fn AllDomains(s: S) -> uint32;

pub open spec fn QosValueResetToDefault(old_s: S, new_s: S, domain_id: uint32, capability: uint32) -> bool;

pub open spec fn QosValueSet(old_s: S, new_s: S, domain_id: uint32, capability: uint32, qos_value: uint32) -> bool;

} // verus!
