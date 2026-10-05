use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type CoreId = u64;
pub type Address = u64;
pub type CoreStateValue = u32;
pub type PowerState = u32;

pub struct S {
    pub system_power_state: PowerState,
    pub calling_core: CoreId,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = -1;
pub spec const INVALID_ADDRESS: PsciReturnCode = -9;
pub spec const DENIED: PsciReturnCode = -3;

pub spec const OFF: CoreStateValue = 1;

pub spec const entry_point_address: Address = 0x8000_0000;

pub open spec fn SystemSuspendImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;

pub open spec fn Exists(b: bool) -> bool;

pub open spec fn CallingCore() -> CoreId;

pub open spec fn CoreState(s: S, core: CoreId) -> CoreStateValue;

pub open spec fn SystemPowerState(s: S) -> PowerState;

pub open spec fn OnWakeup(core: CoreId, s: S) -> Address;

pub open spec fn DeepestPlatformPowerdownState() -> PowerState;

} // verus!
