use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type CpuId = u64;

pub const SUCCESS: PsciReturnCode = 0;
pub const INVALID_ADDRESS: PsciReturnCode = -9;

pub struct S {
    pub entry_point: UInt64,
    pub context: UInt64,
    pub cpu: CpuId,
}

pub open spec fn entry_point_address(s: S) -> UInt64;

pub open spec fn context_id(s: S) -> UInt64;

pub open spec fn calling_cpu(s: S) -> CpuId;

pub open spec fn IsKnownUnavailableAddress(addr: UInt64) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, code: PsciReturnCode) -> bool;

pub open spec fn ReturnedAtNextInstruction(cpu: CpuId) -> bool;

pub open spec fn ResumedAtEntryPoint(cpu: CpuId) -> bool;

pub open spec fn CoreRestartsAtEntryPoint(cpu: CpuId, addr: UInt64) -> bool;

pub open spec fn ContextIdPresented(cpu: CpuId, ctx: UInt64) -> bool;

pub open spec fn CacheAndCoherencyManagedByImplementation(cpu: CpuId) -> bool;

pub open spec fn AllCoresInDefaultSuspend() -> bool;

pub open spec fn PlatformThermallyCritical() -> bool;

} // verus!
