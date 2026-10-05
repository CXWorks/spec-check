use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const DENIED: Int32 = (-3int) as i32;
pub spec const GENERIC_ERROR: Int32 = (-6int) as i32;

pub spec const caller: AgentId = 7;
pub spec const result: Int32 = (-100int) as i32;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn ResetDomainExists(s: S, domain_id: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidResetFlags(s: S, flags: UInt32) -> bool;
pub open spec fn IsSupportedResetState(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;
pub open spec fn AgentMayResetDomain(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub open spec fn ResetOperationFailed(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResetDomainResetTo(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;
pub open spec fn ReturnedOnReceiptOfRequest(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResetSignalAsserted(s: S, domain_id: UInt32, reset_state: UInt32) -> bool;
pub open spec fn ResetDomainState(s: S, domain_id: UInt32) -> int;
pub open spec fn ResetSignal(s: S, domain_id: UInt32) -> int;

} // verus!
