use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;
pub const NOT_SUPPORTED: Int32 = -1;

#[allow(non_upper_case_globals)]
pub const domain_id: UInt32 = 1;
#[allow(non_upper_case_globals)]
pub const cpli: UInt32 = 2;

pub uninterp spec fn IsValidPowercapDomain(s: S, d: UInt32) -> bool;

pub uninterp spec fn IsValidCpli(s: S, d: UInt32, c: UInt32) -> bool;

pub uninterp spec fn IsRequestSupported(s: S, d: UInt32, c: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn EnforcedPowerCap(s: S, d: UInt32, c: UInt32) -> UInt32;

pub uninterp spec fn PowerCappingDisabled(s: S, d: UInt32) -> bool;

} // verus!
