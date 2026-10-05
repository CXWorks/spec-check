use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -4;

pub open spec fn ResetDomainExists(s: S, domain_id: u32) -> bool;

pub open spec fn ResetDomainExtendedNameSupported(s: S, domain_id: u32) -> bool;

pub open spec fn ResetDomainExtendedName(s: S, domain_id: u32) -> Seq<u8>;

} // verus!
