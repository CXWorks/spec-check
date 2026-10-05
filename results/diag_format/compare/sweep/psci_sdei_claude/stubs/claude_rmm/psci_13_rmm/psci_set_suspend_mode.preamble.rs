use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i64;

pub type CoordinationMode = u64;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;

pub const PLATFORM_COORDINATED: CoordinationMode = 0;
pub const OS_INITIATED: CoordinationMode = 1;

pub open spec fn PsciSetSuspendModeImplemented(s: S) -> bool;

pub open spec fn IsOsInitiatedMode(mode: UInt64) -> bool;

pub open spec fn IsPlatformCoordinatedMode(mode: UInt64) -> bool;

pub open spec fn CurrentCoordinationMode(s: S) -> CoordinationMode;

pub open spec fn IsRunning(s: S, c: UInt64) -> bool;

pub open spec fn IsOffViaCpuOffOrNotBooted(s: S, c: UInt64) -> bool;

pub open spec fn IsSuspendedViaCpuDefaultSuspend(s: S, c: UInt64) -> bool;

pub open spec fn AnyCoreCalledCpuSuspendSinceLastModeChangeOrBoot(s: S) -> bool;

pub open spec fn CallingCore(s: S) -> UInt64;

} // verus!
