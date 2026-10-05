use vstd::prelude::*;
verus! {

pub struct S {
    pub dummy: u64,
}

pub struct PsciVersion {
    pub major: u64,
    pub minor: u64,
}

pub type FunctionId = u32;

pub const PSCI_VERSION: FunctionId = 0x8400_0000u32;

pub const NOT_SUPPORTED: u64 = 0xFFFF_FFFF_FFFF_FFFFu64;

pub open spec fn IsFunctionImplemented(fid: FunctionId) -> bool;

pub open spec fn ResultEqual(result: u64, code: u64) -> bool;

pub open spec fn ImplementedPsciVersion() -> PsciVersion;

} // verus!
