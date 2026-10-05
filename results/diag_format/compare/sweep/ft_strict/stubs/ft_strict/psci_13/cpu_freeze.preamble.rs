use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type Interrupt = u64;

pub struct S {
    pub dummy: int,
}

pub spec const PSCI_SUCCESS: PsciReturnCode = 0i32;
pub spec const NOT_SUPPORTED: PsciReturnCode = -1i32;
pub spec const DENIED: PsciReturnCode = -3i32;

pub open spec fn IsCpuFreezeImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn CpuOffWouldReturnDenied(s: S, cpu: UInt64) -> bool;

pub open spec fn ReturnsToCaller(s: S, cpu: UInt64) -> bool;

pub open spec fn CoreInImplDefinedLowPowerState(s: S, cpu: UInt64) -> bool;

pub open spec fn IsWakeupInterruptFor(s: S, i: Interrupt, cpu: UInt64) -> bool;

pub open spec fn WakesCore(s: S, i: Interrupt, cpu: UInt64) -> bool;

pub open spec fn IsPendingOrActive(s: S, i: Interrupt) -> bool;

pub open spec fn ResumesOnlyViaCpuOn(s: S, cpu: UInt64) -> bool;

pub open spec fn CacheAndCoherencyManagedByImplementation(s: S, cpu: UInt64) -> bool;

pub open spec fn AllCoresExceptOneFrozen(s: S) -> bool;

pub open spec fn PlatformThermallyCritical(s: S) -> bool;

} // verus!
