use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;
pub const GENERIC_ERROR: Int32 = -1;

pub const ASSERTED: UInt32 = 1;
pub const DEASSERTED: UInt32 = 0;

pub struct ResetDomain {
    pub reset_autonomous: bool,
    pub reset_signal: UInt32,
    pub state: UInt32,
}

pub struct S {
    pub dummy: UInt32,
}

impl S {
    pub open spec fn ResetDomainAt(self, id: UInt32) -> ResetDomain;
}

pub open spec fn ResetDomainAt(s: S, id: UInt32) -> ResetDomain;

pub open spec fn domain_id(s: S) -> UInt32;

pub open spec fn flags(s: S) -> Seq<UInt32>;

pub open spec fn reset_state(s: S) -> UInt32;

pub open spec fn caller(s: S) -> UInt32;

pub open spec fn ResetDomainExists(s: S, id: UInt32) -> bool;

pub open spec fn IsValidResetFlags(s: S, f: Seq<UInt32>) -> bool;

pub open spec fn IsSupportedResetState(s: S, id: UInt32, st: UInt32) -> bool;

pub open spec fn AgentMayResetDomain(s: S, agent: UInt32, id: UInt32) -> bool;

pub open spec fn ResetOperationFailed(s: S, id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
