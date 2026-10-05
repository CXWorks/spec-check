use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Address = u64;
pub type PowerState = u32;

pub enum PsciReturnCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: PsciReturnCode = PsciReturnCode::Success;
pub spec const INVALID_PARAMETERS: PsciReturnCode = PsciReturnCode::InvalidParameters;
pub spec const DENIED: PsciReturnCode = PsciReturnCode::Denied;
pub spec const INVALID_ADDRESS: PsciReturnCode = PsciReturnCode::InvalidAddress;

pub open spec fn IsValidPowerState(power_state: PowerState) -> bool;
pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;
pub open spec fn IsOsInitiatedMode() -> bool;
pub open spec fn RequestsHigherThanCoreLevel(power_state: PowerState) -> bool;
pub open spec fn AnyChildInIncompatibleLowPowerState(power_state: PowerState) -> bool;
pub open spec fn AllIncompatibleCoresRunning(power_state: PowerState) -> bool;
pub open spec fn IsKnownUnavailableAddress(addr: Address) -> bool;
pub open spec fn StateType(power_state: PowerState) -> int;
pub open spec fn CoreStateUnchangedExceptTimersAndCpuInterface() -> bool;
pub open spec fn PowerdownDowngradedToStandby(power_state: PowerState) -> bool;
pub open spec fn PowerdownReturnedEarlyOnWakeupEvent(power_state: PowerState) -> bool;
pub open spec fn ResumedAt(addr: Address) -> bool;
pub open spec fn ReturnedToCaller() -> bool;
pub open spec fn ContextIdRegister() -> UInt64;
pub open spec fn CachesCleanedForPoweredDownNodes() -> bool;

} // verus!
