use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub notify_enable_field: uint32,
    pub domain_id_field: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const INVALID_PARAMETERS: int32 = 2;

pub open spec fn domain_id(s: S) -> uint32;

pub open spec fn notify_enable(s: S) -> uint32;

pub open spec fn IsDomainValid(s: S, id: uint32) -> bool;

} // verus!
