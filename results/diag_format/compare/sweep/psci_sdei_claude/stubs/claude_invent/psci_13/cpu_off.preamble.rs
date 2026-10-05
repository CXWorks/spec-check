use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn IsTrustedOsResidentCore(s: S) -> bool;

pub open spec fn IsCallingCorePoweredDown(s: S) -> bool;

} // verus!
