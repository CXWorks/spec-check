use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type CoreId = u64;
pub type CoordinationMode = u32;
pub type SuspendMode = u32;

pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const SUCCESS: PsciReturnCode = 0;

pub const PLATFORM_COORDINATED: CoordinationMode = 0;
pub const OS_INITIATED: CoordinationMode = 1;

#[allow(non_upper_case_globals)]
pub const mode: SuspendMode = 7;

#[allow(non_upper_case_globals)]
pub const c: CoreId = 3;

pub struct S {
    pub coordination_mode: CoordinationMode,
}

impl S {
    pub uninterp spec fn CurrentCoordinationMode(self) -> CoordinationMode;
}

pub uninterp spec fn PsciSetSuspendModeImplemented() -> bool;
pub uninterp spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub uninterp spec fn IsOsInitiatedMode(m: SuspendMode) -> bool;
pub uninterp spec fn IsPlatformCoordinatedMode(m: SuspendMode) -> bool;
pub uninterp spec fn CurrentCoordinationMode() -> CoordinationMode;
pub uninterp spec fn ExistsCore(core: CoreId, cond: bool) -> bool;
pub uninterp spec fn IsRunning(core: CoreId) -> bool;
pub uninterp spec fn IsOffViaCpuOffOrNotBooted(core: CoreId) -> bool;
pub uninterp spec fn IsSuspendedViaCpuDefaultSuspend(core: CoreId) -> bool;
pub uninterp spec fn AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot() -> bool;
pub uninterp spec fn CallingCore() -> CoreId;

} // verus!
