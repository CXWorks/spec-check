use vstd::prelude::*;
verus! {

pub type Address = u64;
pub type UInt64 = u64;
pub type CpuId = u64;
pub type PsciReturnCode = i32;
pub type PowerStateValue = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const SUCCESS: PsciReturnCode = 0;
pub spec const INVALID_ADDRESS: PsciReturnCode = (-9int) as i32;

#[allow(non_upper_case_globals)]
pub spec const calling_cpu: CpuId = 0;

pub open spec fn IsKnownUnavailableAddress(s: S, addr: Address) -> bool;
pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;
pub open spec fn ReturnedAtNextInstruction(s: S, cpu: CpuId) -> bool;
pub open spec fn ResumedAtEntryPoint(s: S, cpu: CpuId) -> bool;
pub open spec fn CoreRestartsAtEntryPoint(s: S, cpu: CpuId, addr: Address) -> bool;
pub open spec fn ContextIdPresented(s: S, cpu: CpuId, context_id: UInt64) -> bool;
pub open spec fn CacheAndCoherencyManagedByImplementation(s: S, cpu: CpuId) -> bool;
pub open spec fn AllCoresInDefaultSuspend(s: S) -> bool;
pub open spec fn PlatformThermallyCritical(s: S) -> bool;
pub open spec fn PowerState(s: S, cpu: CpuId) -> PowerStateValue;
pub open spec fn TopologyAncestors(s: S, cpu: CpuId) -> CpuId;

} // verus!
