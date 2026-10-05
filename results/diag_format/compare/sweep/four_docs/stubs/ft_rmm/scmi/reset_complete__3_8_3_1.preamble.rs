use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type RmiStatusCode = u32;

pub struct ResetDomainAt {
    pub id: UInt32,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const GENERIC_ERROR: Int32 = 1;
pub spec const RMI_SUCCESS: RmiStatusCode = 2;

pub open spec fn ResetDomainHasOtherUsers(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResetDomainCanBeQuiesced(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResetDomainWasReset(s: S, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual<A, B>(a: A, b: B) -> bool;

} // verus!
