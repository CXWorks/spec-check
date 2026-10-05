use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const NOT_SUPPORTED: int32 = 2;

pub const domain_id: uint32 = 0;

pub struct S {
    pub dummy: int,
}

impl S {
    pub open spec fn powercap_mai(self, d: uint32) -> uint32;
}

pub open spec fn DomainExists(s: S, d: uint32) -> bool;

pub open spec fn PowerCapDomainSupported(s: S, d: uint32) -> bool;

} // verus!
