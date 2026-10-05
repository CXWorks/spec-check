use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const NOT_FOUND: i32 = -4;

pub open spec fn IsValidPerfDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfDomainSupportsLimitsNotify(s: S, domain_id: UInt32) -> bool;

pub open spec fn PerfLimitsNotifyEnabled(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn CallingAgent(s: S) -> UInt32;

} // verus!
