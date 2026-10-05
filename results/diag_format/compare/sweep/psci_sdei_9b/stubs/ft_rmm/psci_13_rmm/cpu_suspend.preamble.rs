use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Address = u64;

pub type PsciReturnCode = i64;

pub struct PowerState {
    pub value: u64,
}

pub struct S {
    pub os_initiated_mode: bool,
    pub context_id_register: u64,
    pub pc: u64,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const ALREADY_ON: PsciReturnCode = -4;
pub const ON_PENDING: PsciReturnCode = -5;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;
pub const NOT_PRESENT: PsciReturnCode = -7;
pub const DISABLED: PsciReturnCode = -8;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub open spec fn IsValidPowerState(s: S, power_state: PowerState) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn RequestsHigherThanCoreLevel(s: S, power_state: PowerState) -> bool;

pub open spec fn AnyChildInIncompatibleLowPowerState(s: S, power_state: PowerState) -> bool;

pub open spec fn AllIncompatibleCoresRunning(s: S, power_state: PowerState) -> bool;

pub open spec fn IsKnownUnavailableAddress(s: S, address: Address) -> bool;

pub open spec fn StateType(power_state: PowerState) -> int;

pub open spec fn CoreStateUnchangedExceptTimersAndCpuInterface(s: S) -> bool;

pub open spec fn PowerdownDowngradedToStandby(s: S, power_state: PowerState) -> bool;

pub open spec fn PowerdownReturnedEarlyOnWakeupEvent(s: S, power_state: PowerState) -> bool;

pub open spec fn ResumedAt(s: S, address: Address) -> bool;

pub open spec fn ReturnedToCaller(s: S) -> bool;

pub open spec fn ContextIdRegister(s: S) -> UInt64;

pub open spec fn CachesCleanedForPoweredDownNodes(s: S, power_state: PowerState) -> bool;

} // verus!
