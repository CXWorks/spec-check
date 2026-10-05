use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct PsciVersion {
    pub major: u16,
    pub minor: u16,
}

pub spec const PSCI_VERSION: UInt32 = 0x84000000u32;

pub spec const NOT_SUPPORTED: Int32 = -1i32;

pub open spec fn IsFunctionImplemented(fid: UInt32) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: Int32) -> bool;

pub open spec fn ImplementedPsciVersion() -> PsciVersion;

} // verus!
