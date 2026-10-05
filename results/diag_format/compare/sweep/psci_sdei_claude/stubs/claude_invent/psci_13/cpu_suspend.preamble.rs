use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const PSCI_NOT_SUPPORTED: PsciReturnCode = -1;
pub const PSCI_INVALID_PARAMETERS: PsciReturnCode = -2;
pub const PSCI_DENIED: PsciReturnCode = -3;
pub const PSCI_INVALID_ADDRESS: PsciReturnCode = -9;

pub struct S {
    pub dummy: u64,
}

pub open spec fn PsciExtendedStateIdFormat(s: S) -> bool;

pub open spec fn PowerStateIsValid(s: S, power_state: UInt32) -> bool;

pub open spec fn PsciOsInitiatedMode(s: S) -> bool;

pub open spec fn PowerStateTargetsHigherThanCoreLevel(s: S, power_state: UInt32) -> bool;

pub open spec fn ChildInIncompatibleLowPowerState(s: S, power_state: UInt32) -> bool;

pub open spec fn PowerStateIsPowerdown(s: S, power_state: UInt32) -> bool;

pub open spec fn EntryPointAddressKnownInvalid(s: S, entry_point_address: UInt64) -> bool;

pub open spec fn IncompatibleCoresAllRunning(s: S, power_state: UInt32) -> bool;

pub open spec fn CoreStateUnchangedOnStandbyReturn(old_s: S, new_s: S) -> bool;

pub open spec fn ResumedAtEntryPoint(s: S, entry_point_address: UInt64) -> bool;

pub open spec fn WakeupContextId(s: S) -> UInt64;

} // verus!
