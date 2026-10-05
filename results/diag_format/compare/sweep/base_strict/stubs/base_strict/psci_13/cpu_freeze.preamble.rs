use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type CpuId = u64;

pub type Interrupt = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const DENIED: PsciReturnCode = -3;

pub spec const calling_cpu: CpuId = 0;

pub open spec fn IsCpuFreezeImplemented(s: S) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn CpuOffWouldReturnDenied(s: S, cpu: CpuId) -> bool;

pub open spec fn ReturnsToCaller(s: S, cpu: CpuId) -> bool;

pub open spec fn CoreInImplDefinedLowPowerState(s: S, cpu: CpuId) -> bool;

pub open spec fn ResumesOnlyViaCpuOn(s: S, cpu: CpuId) -> bool;

pub open spec fn CacheAndCoherencyManagedByImplementation(s: S, cpu: CpuId) -> bool;

pub open spec fn AllCoresExceptOneFrozen(s: S) -> bool;

pub open spec fn PlatformThermallyCritical(s: S) -> bool;

pub open spec fn IsWakeupInterruptFor(i: Interrupt, cpu: CpuId) -> bool;

pub open spec fn WakesCore(s: S, i: Interrupt, cpu: CpuId) -> bool;

pub open spec fn IsPendingOrActive(i: Interrupt) -> bool;

} // verus!
