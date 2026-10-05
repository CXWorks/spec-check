use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct Realm {
    pub power_thresh_low: uint32,
    pub power_thresh_high: uint32,
}

pub struct S {
    pub realms: Map<uint32, Realm>,
}

pub const RMI_SUCCESS: int32 = 0;
pub const RMI_ERROR_INVALID_PARAMETERS: int32 = 1;
pub const RMI_ERROR_NOT_FOUND: int32 = 2;

pub const NOT_FOUND_DOMAIN_ID: uint32 = 0xFFFF_FFFF;
pub const MAX_POWER_THRESHOLD: uint32 = 0x7FFF_FFFF;

pub open spec fn ResultEqual(result: int32, code: int32) -> bool;

pub open spec fn IsDomainValid(s: S, domain_id: uint32) -> bool;

pub open spec fn RealmAt(s: S, domain_id: uint32) -> Realm;

} // verus!
