use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = int;

pub type SuspendMode = u64;

pub type Core = u64;

pub type FunctionId = u64;

pub struct S {
    pub suspend_mode: SuspendMode,
}

pub spec const SUCCEEDED: PsciReturnCode = 0;
pub spec const NOT_SUPPORTED: PsciReturnCode = -1;
pub spec const INVALID_PARAMETERS: PsciReturnCode = -2;
pub spec const DENIED: PsciReturnCode = -3;

pub spec const PLATFORM_COORDINATED: SuspendMode = 0;
pub spec const OS_INITIATED: SuspendMode = 1;

pub spec const PSCI_SET_SUSPEND_MODE: FunctionId = 0x8400000F;

pub uninterp spec fn IsFunctionImplemented(s: S, fid: FunctionId) -> bool;

pub uninterp spec fn CurrentSuspendMode(s: S) -> SuspendMode;

pub uninterp spec fn SuspendModeOf(s: S, mode: UInt64) -> SuspendMode;

pub uninterp spec fn CoreIsRunning(c: Core) -> bool;

pub uninterp spec fn CoreIsOffViaCpuOffOrNotBooted(c: Core) -> bool;

pub uninterp spec fn CoreIsSuspendedViaCpuDefaultSuspend(c: Core) -> bool;

pub uninterp spec fn CpuSuspendCalledSinceLastModeChangeOrBoot(c: Core) -> bool;

pub uninterp spec fn CallingCore() -> Core;

} // verus!
