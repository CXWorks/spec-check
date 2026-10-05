use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type CoreId = u64;

pub type CorePowerState = u8;

pub type PowerState = u64;

pub struct S {
    pub calling_core: CoreId,
    pub system_suspend_implemented: bool,
    pub system_power_state: PowerState,
}

pub const PSCI_SUCCESS: PsciReturnCode = 0;

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const DENIED: PsciReturnCode = -3;

pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub const OFF: CorePowerState = 1;

pub open spec fn SystemSuspendImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;

pub open spec fn Exists(p: bool) -> bool;

pub open spec fn CallingCore(s: S) -> CoreId;

pub open spec fn CoreState(s: S, core: CoreId) -> CorePowerState;

pub open spec fn SystemPowerState(s: S) -> PowerState;

pub open spec fn DeepestPlatformPowerdownState(s: S) -> PowerState;

pub open spec fn OnWakeup(s: S, p: bool) -> bool;

pub open spec fn resumes_execution_at(core: CoreId, addr: Address) -> bool;

} // verus!
