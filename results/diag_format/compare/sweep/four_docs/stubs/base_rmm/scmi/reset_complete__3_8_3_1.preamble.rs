use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;

pub const RMI_ERROR_GENERIC: RmiStatusCode = 1;

pub struct S {
    pub dummy: u64,
}

pub trait ResEq {
    type Other;
}

impl ResEq for Result<(), RmiStatusCode> {
    type Other = RmiStatusCode;
}

impl ResEq for i32 {
    type Other = i32;
}

pub open spec fn ResetDomainHasOtherUsers(s: S, domain_id: u32) -> bool;

pub open spec fn ResetDomainCanBeQuiesced(s: S, domain_id: u32) -> bool;

pub open spec fn ResetDomainWasReset(s: S, domain_id: u32) -> bool;

pub open spec fn ResultEqual<A: ResEq>(a: A, b: A::Other) -> bool;

} // verus!
