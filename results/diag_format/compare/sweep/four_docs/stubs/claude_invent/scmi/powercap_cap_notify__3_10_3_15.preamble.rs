use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;
pub const INVALID_PARAMETERS: i32 = -2;
pub const NOT_FOUND: i32 = -4;

pub open spec fn PowercapDomainIsValid(s: S, domain_id: u32) -> bool;

pub open spec fn PowercapCapNotifyEnabled(s: S, domain_id: u32) -> bool;

} // verus!
