use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type Core = u64;

pub type SuspendMode = u32;

pub struct S {
    pub mode: u64,
    pub suspend_mode: SuspendMode,
}

pub spec const NOT_SUPPORTED: PsciReturnCode = -1;
pub spec const INVALID_PARAMETERS: PsciReturnCode = -2;
pub spec const DENIED: PsciReturnCode = -3;

pub spec const PLATFORM_COORDINATED: SuspendMode = 0;
pub spec const OS_INITIATED: SuspendMode = 1;

#[allow(non_upper_case_globals)]
pub spec const mode: u64 = 2;

pub open spec fn IsFunctionImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn CurrentSuspendMode(s: S) -> SuspendMode;

pub open spec fn SuspendModeOf(m: u64) -> SuspendMode;

pub open spec fn CoreIsRunning(c: Core) -> bool;

pub open spec fn CoreIsOffViaCpuOffOrNotBooted(c: Core) -> bool;

pub open spec fn CoreIsSuspendedViaCpuDefaultSuspend(c: Core) -> bool;

pub open spec fn CpuSuspendCalledSinceLastModeChangeOrBoot(c: Core) -> bool;

pub open spec fn CallingCore() -> Core;

} // verus!
