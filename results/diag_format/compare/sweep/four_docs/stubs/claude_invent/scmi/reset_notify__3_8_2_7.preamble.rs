use vstd::prelude::*;
verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsValidResetDomain(s: S, domain_id: u32) -> bool;

pub open spec fn ResetNotifyEnabled(s: S, domain_id: u32) -> bool;

} // verus!
