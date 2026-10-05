use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: int32 = 0;
pub spec const NOT_FOUND: int32 = -1i32;
pub spec const NOT_SUPPORTED: int32 = -2i32;
pub spec const INVALID_PARAMETERS: int32 = -3i32;

pub spec const domain_id: uint32 = 7;
pub spec const notify_enable: uint32 = 11;

pub uninterp spec fn DomainExists(s: S, domain_id: uint32) -> bool;
pub uninterp spec fn DomainSupportsNotifications(s: S, domain_id: uint32) -> bool;

} // verus!
