use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -2;

pub const domain_id: uint32 = 0;

pub uninterp spec fn PowerDomainExists(s: S, d: uint32) -> bool;

pub uninterp spec fn PowerDomainState(s: S, d: uint32) -> uint32;

} // verus!
