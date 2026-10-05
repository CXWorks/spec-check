use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type PowerState = u32;

pub type Address = u64;

pub type ContextId = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: PsciReturnCode = 0;
pub spec const INVALID_PARAMETERS: PsciReturnCode = (-2) as i32;
pub spec const DENIED: PsciReturnCode = (-3) as i32;
pub spec const INVALID_ADDRESS: PsciReturnCode = (-9) as i32;

#[allow(non_upper_case_globals)]
pub spec const power_state: PowerState = 0x1234;

#[allow(non_upper_case_globals)]
pub spec const entry_point_address: Address = 0x8000_0000;

#[allow(non_upper_case_globals)]
pub spec const context_id: ContextId = 0x42;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn IsValidPowerState(s: S, ps: PowerState) -> bool;

pub open spec fn IsOsInitiatedMode(s: S) -> bool;

pub open spec fn RequestsHigherThanCoreLevel(s: S, ps: PowerState) -> bool;

pub open spec fn AnyChildInIncompatibleLowPowerState(s: S, ps: PowerState) -> bool;

pub open spec fn AllIncompatibleCoresRunning(s: S, ps: PowerState) -> bool;

pub open spec fn IsKnownUnavailableAddress(s: S, addr: Address) -> bool;

pub open spec fn StateType(s: S, ps: PowerState) -> int;

pub open spec fn CoreStateUnchangedExceptTimersAndCpuInterface(old_s: S, new_s: S) -> bool;

pub open spec fn PowerdownDowngradedToStandby(s: S, ps: PowerState) -> bool;

pub open spec fn PowerdownReturnedEarlyOnWakeupEvent(s: S, ps: PowerState) -> bool;

pub open spec fn ResumedAt(s: S, addr: Address) -> bool;

pub open spec fn ReturnedToCaller(s: S) -> bool;

pub open spec fn ContextIdRegister(s: S) -> ContextId;

pub open spec fn CachesCleanedForPoweredDownNodes(old_s: S, new_s: S, ps: PowerState) -> bool;

} // verus!
