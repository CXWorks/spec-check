use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0i32;
pub spec const NOT_SUPPORTED: Int32 = -1i32;
pub spec const NOT_FOUND: Int32 = -3i32;

pub spec const POWERCAP_MAI_GET: UInt32 = 9;

pub spec const domain_id: UInt32 = 0;

pub uninterp spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn IsRequestSupported(msg_id: UInt32) -> bool;

pub uninterp spec fn EnforcedMai(domain_id: UInt32) -> UInt32;

} // verus!
