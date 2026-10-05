use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type RmiStatusCode = u32;

pub const RMI_ERROR_GENERIC: RmiStatusCode = 1;

pub const SUCCESS: Int32 = 0;

pub enum ResetDomainStateValue {
    Idle,
    Quiesced,
    Resetting,
    Reset,
}

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

pub open spec fn ResetDomainHasOtherUsers(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainCanBeQuiesced(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetOperationFailed(s: S, domain_id: UInt32) -> bool;

pub open spec fn AsyncResetRequestedDomain() -> UInt32;

pub open spec fn ResetDomainWasReset(s: S, domain_id: UInt32) -> bool;

pub open spec fn ResetDomainState(s: S, domain_id: UInt32) -> ResetDomainStateValue;

} // verus!
