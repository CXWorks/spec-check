use vstd::prelude::*;

verus! {

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub struct S {
    pub dummy: u32,
}

pub open spec fn IsValidPowerDomain(s: S, domain_id: u32) -> bool;

pub open spec fn PowerStateChangeRequestedNotifyEnabled(s: S, agent_id: u32, domain_id: u32) -> bool;

} // verus!
