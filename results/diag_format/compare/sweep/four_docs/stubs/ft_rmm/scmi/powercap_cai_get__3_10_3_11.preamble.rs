use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub num_domains: u32,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const NOT_FOUND: Int32 = (-4int) as i32;

pub uninterp spec fn arbitrary_result() -> Int32;

pub spec const result: Int32 = arbitrary_result();

pub uninterp spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub uninterp spec fn IsRequestSupported(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub uninterp spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub uninterp spec fn EnforcedCai(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;

} // verus!
