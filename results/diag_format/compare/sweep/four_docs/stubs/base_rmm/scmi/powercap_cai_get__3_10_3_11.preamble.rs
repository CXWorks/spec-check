use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub domains: Seq<UInt32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -3;
pub const NOT_SUPPORTED: Int32 = -1;

pub const domain_id: UInt32 = 1;
pub const cpli: UInt32 = 2;

pub open spec fn IsValidPowercapDomain(s: S, domain_id_arg: UInt32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id_arg: UInt32, cpli_arg: UInt32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id_arg: UInt32, cpli_arg: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn EnforcedCai(s: S, domain_id_arg: UInt32, cpli_arg: UInt32) -> UInt32;

} // verus!
