use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_ADDRESS: PsciReturnCode = -9;
pub const PSCI_DENIED: PsciReturnCode = -3;

pub struct S {
    pub dummy: int,
}

pub open spec fn SystemSuspendImplemented(s: S) -> bool;

pub open spec fn EntryPointAddressKnownInvalid(s: S, entry_point_address: UInt64) -> bool;

pub open spec fn OtherCoreNotOff(s: S) -> bool;

} // verus!
