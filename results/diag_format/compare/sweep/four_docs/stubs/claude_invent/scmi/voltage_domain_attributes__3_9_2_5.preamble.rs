use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: i32 = 0i32;
pub const NOT_FOUND: i32 = -4i32;

pub open spec fn VoltageDomainExists(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageDomainAsyncLevelSetSupported(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageDomainExtendedNameSupported(s: S, domain_id: u32) -> bool;

pub open spec fn VoltageDomainNameEqual(s: S, domain_id: u32, name: Seq<u8>) -> bool;

pub open spec fn VoltageDomainExtendedNameLower15BytesEqual(s: S, domain_id: u32, name: Seq<u8>) -> bool;

} // verus!
