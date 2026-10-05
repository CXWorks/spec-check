use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub enum NotSupported {
    NotSupported,
}

pub struct S {
    pub implemented_functions: Set<u32>,
}

pub struct PsciVersion {
    pub major: int,
    pub minor: int,
}

pub const PSCI_VERSION: u32 = 0x8400_0000;

pub spec const NOT_SUPPORTED: Result<(), NotSupported> = Err(NotSupported::NotSupported);

pub open spec fn IsFunctionImplemented(s: S, function_id: u32) -> bool;

pub open spec fn ImplementedPsciVersion() -> PsciVersion;

} // verus!
