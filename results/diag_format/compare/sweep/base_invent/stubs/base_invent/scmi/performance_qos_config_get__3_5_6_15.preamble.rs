use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint64 = u64;

pub struct S {
    pub domain_id: uint64,
    pub valid_domain_id: uint64,
    pub capability: uint32,
    pub valid_capability: uint32,
    pub multiple_type_bits_set: bool,
    pub multiple_subtype_bits_set: bool,
    pub qos_value: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -5;
pub const INVALID_PARAMETERS: int32 = -2;

pub open spec fn domain_id(s: S) -> uint64;
pub open spec fn valid_domain_id(s: S) -> uint64;
pub open spec fn capability(s: S) -> uint32;
pub open spec fn valid_capability(s: S) -> uint32;
pub open spec fn multiple_type_bits_set(s: S) -> bool;
pub open spec fn multiple_subtype_bits_set(s: S) -> bool;
pub open spec fn qos_value(s: S) -> uint32;

} // verus!
