use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_FOUND: Int32 = 1;

pub spec const NOT_SUPPORTED: Int32 = 2;

pub spec const domain_id: UInt32 = 10;

pub spec const cpli: UInt32 = 11;

pub uninterp spec fn IsValidPowerCapDomain(s: S, d: UInt32) -> bool;

pub uninterp spec fn IsValidCpli(s: S, d: UInt32, c: UInt32) -> bool;

pub uninterp spec fn IsPowerCapGetSupported(s: S, d: UInt32, c: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub uninterp spec fn CurrentEnforcedPowerCap(s: S, d: UInt32, c: UInt32) -> UInt32;

pub uninterp spec fn PowerCappingDisabled(s: S, d: UInt32) -> bool;

} // verus!
