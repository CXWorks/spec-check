use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_INVALID_ADDRESS: PsciReturnCode = -9;

pub struct S {
    pub dummy: u64,
}

pub open spec fn EntryPointAddressIsInvalid(s: S, entry_point_address: UInt64) -> bool;

} // verus!
