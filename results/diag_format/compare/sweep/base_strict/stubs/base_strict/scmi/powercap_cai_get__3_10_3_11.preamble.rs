use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const NOT_FOUND: Int32 = -4;

pub const domain_id: UInt32 = 1;
pub const cpli: UInt32 = 2;

pub open spec fn IsValidPowercapDomain(s: S, d: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, d: UInt32, c: UInt32) -> bool;

pub open spec fn DomainSupportsCpc(s: S, d: UInt32) -> bool;

pub open spec fn IsCaiGetSupported(s: S, d: UInt32) -> bool;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

pub open spec fn CurrentCai(s: S, d: UInt32, c: UInt32) -> UInt32;

} // verus!
