use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub flags: UInt32,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const GENERIC_ERROR: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const domain_id: UInt32 = 1;
#[allow(non_upper_case_globals)]
pub const reset_state: UInt32 = 2;
#[allow(non_upper_case_globals)]
pub const caller: UInt32 = 3;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn ResetDomainExists(s: S, domain: UInt32) -> bool;
pub open spec fn IsValidResetFlags(flags: UInt32) -> bool;
pub open spec fn IsSupportedResetState(s: S, domain: UInt32, state: UInt32) -> bool;
pub open spec fn AgentMayResetDomain(s: S, agent: UInt32, domain: UInt32) -> bool;
pub open spec fn ResetOperationFailed(s: S, domain: UInt32) -> bool;
pub open spec fn ResetDomainResetTo(s: S, domain: UInt32, state: UInt32) -> bool;
pub open spec fn ReturnedOnReceiptOfRequest(s: S, domain: UInt32) -> bool;
pub open spec fn ResetSignalAsserted(s: S, domain: UInt32, state: UInt32) -> bool;

} // verus!
