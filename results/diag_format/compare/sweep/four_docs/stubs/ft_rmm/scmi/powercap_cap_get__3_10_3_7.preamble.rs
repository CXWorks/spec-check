use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;

#[allow(non_upper_case_globals)]
pub const result: Int32 = 1;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: UInt32, cpli: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn EnforcedPowerCap(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;

pub open spec fn PowerCappingDisabled(s: S, domain_id: UInt32) -> bool;

} // verus!
