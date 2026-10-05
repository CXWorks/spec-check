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

pub const POWERCAP_MAI_GET: UInt32 = 0x9;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, msg_id: UInt32) -> bool;

pub open spec fn ResultEqual(status: Int32, code: Int32) -> bool;

pub open spec fn EnforcedMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!
