use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = u64;

pub type Core = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = 1;
pub spec const INVALID_PARAMETERS: PsciReturnCode = 2;
pub spec const DENIED: PsciReturnCode = 3;
pub spec const RSI_SUCCESS: PsciReturnCode = 0;

pub spec const PLATFORM_COORDINATED: UInt64 = 0;
pub spec const OS_INITIATED: UInt64 = 1;

#[allow(non_upper_case_globals)]
pub spec const c: Core = 7;

pub uninterp spec fn PsciSetSuspendModeImplemented(s: S) -> bool;

pub uninterp spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub uninterp spec fn IsOsInitiatedMode(mode: UInt64) -> bool;

pub uninterp spec fn IsPlatformCoordinatedMode(mode: UInt64) -> bool;

pub uninterp spec fn CurrentCoordinationMode(s: S) -> UInt64;

pub uninterp spec fn ExistsCore(s: S, core: Core, pred: bool) -> bool;

pub uninterp spec fn IsRunning(core: Core) -> bool;

pub uninterp spec fn IsOffViaCpuOffOrNotBooted(core: Core) -> bool;

pub uninterp spec fn IsSuspendedViaCpuDefaultSuspend(core: Core) -> bool;

pub uninterp spec fn AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(s: S) -> bool;

pub uninterp spec fn CallingCore() -> Core;

} // verus!
