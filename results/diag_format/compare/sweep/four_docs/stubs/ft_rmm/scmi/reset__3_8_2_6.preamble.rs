use vstd::prelude::*;

verus! {

pub struct UInt32 {
    pub value: u32,
}

impl UInt32 {
    pub open spec fn spec_index(self, i: int) -> u32;
}

pub type Int32 = i32;

pub type AgentId = u32;

pub type ResetSignal = u32;

pub struct S {
    pub dummy: u32,
}

pub struct ResetDomain {
    pub reset_signal: ResetSignal,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = -1;
pub spec const INVALID_PARAMETERS: Int32 = -2;
pub spec const DENIED: Int32 = -3;
pub spec const NOT_FOUND: Int32 = -4;
pub spec const GENERIC_ERROR: Int32 = -8;

pub spec const ASSERTED: ResetSignal = 1;
pub spec const DEASSERTED: ResetSignal = 0;

pub spec const caller: AgentId = 0;
pub spec const result: bool = true;

pub open spec fn ResetDomainExists(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn IsValidResetFlags(s: S, flags: UInt32) -> bool;

pub open spec fn IsSupportedResetState(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;

pub open spec fn AgentMayResetDomain(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn ResetOperationFailed(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainAt(s: S, domain_id: UInt32) -> ResetDomain;

} // verus!
