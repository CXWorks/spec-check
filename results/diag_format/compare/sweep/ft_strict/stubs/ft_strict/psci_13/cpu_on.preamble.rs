use vstd::prelude::*;

verus! {

pub type Mpidr = u64;
pub type Address = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;
pub type CoreStateValue = i32;

pub struct S {
    pub dummy: int,
}

pub const PSCI_SUCCESS: PsciReturnCode = 0;
pub const INVALID_PARAMETERS: PsciReturnCode = -2;
pub const DENIED: PsciReturnCode = -3;
pub const ALREADY_ON: PsciReturnCode = -4;
pub const ON_PENDING: PsciReturnCode = -5;
pub const INTERNAL_FAILURE: PsciReturnCode = -6;
pub const INVALID_ADDRESS: PsciReturnCode = -9;
pub const ON: CoreStateValue = 1;

pub open spec fn IsValidMpidr(s: S, cpu: Mpidr) -> bool;
pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;
pub open spec fn IsKnownUnavailableToCaller(s: S, addr: Address) -> bool;
pub open spec fn CoreState(s: S, cpu: Mpidr) -> CoreStateValue;
pub open spec fn CanBePhysicallyPoweredUp(s: S, cpu: Mpidr) -> bool;
pub open spec fn IsAvailableForOsUsage(s: S, cpu: Mpidr) -> bool;
pub open spec fn CoreRestartsAtEntryPoint(s: S, cpu: Mpidr, addr: Address) -> bool;
pub open spec fn ContextIdPresentedAtReturnExceptionLevel(s: S, cpu: Mpidr, context_id: UInt64) -> bool;
pub open spec fn CachesInvalidatedOnBoot(s: S, cpu: Mpidr) -> bool;
pub open spec fn HardwareInvalidatesCachesOnBoot(s: S, cpu: Mpidr) -> bool;
pub open spec fn CoherencyManaged(s: S, cpu: Mpidr) -> bool;
pub open spec fn CorePowerState(s: S, cpu: Mpidr) -> int;
pub open spec fn CoreCaches(s: S, cpu: Mpidr) -> int;

} // verus!
