use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Endpoint = u64;
pub type ConduitKind = u8;
pub type InstanceKind = u8;
pub type RuntimeStateKind = u8;

pub struct S {
    pub dummy: u8,
}

pub const caller: Endpoint = 1;
pub const callee: Endpoint = 2;

pub const SMC: ConduitKind = 1;
pub const ERET: ConduitKind = 2;

pub const SECURE_PHYSICAL: InstanceKind = 1;
pub const NONSECURE_PHYSICAL: InstanceKind = 2;
pub const NONSECURE_VIRTUAL: InstanceKind = 3;
pub const SECURE_VIRTUAL: InstanceKind = 4;

pub const BLOCKED: RuntimeStateKind = 1;
pub const WAITING: RuntimeStateKind = 2;

pub open spec fn Conduit() -> ConduitKind;
pub open spec fn Instance() -> InstanceKind;
pub open spec fn RuntimeState(e: Endpoint) -> RuntimeStateKind;

pub open spec fn IsSpmc(e: Endpoint) -> bool;
pub open spec fn IsSpmd(e: Endpoint) -> bool;
pub open spec fn IsNsEndpoint(e: Endpoint) -> bool;
pub open spec fn IsHypervisor(e: Endpoint) -> bool;
pub open spec fn IsVm(e: Endpoint) -> bool;
pub open spec fn IsSp(e: Endpoint) -> bool;
pub open spec fn IsPrivilegedPartition(e: Endpoint) -> bool;
pub open spec fn IsSel1OrSel2Spmc(e: Endpoint) -> bool;
pub open spec fn IsEl3Spmc(e: Endpoint) -> bool;
pub open spec fn IsLogicalSel1Sp(e: Endpoint) -> bool;

pub open spec fn IdOfPreemptedSp() -> UInt16;
pub open spec fn IdOfPreemptedSpExecutionContext() -> UInt16;
pub open spec fn IdOfPreemptedPartition() -> UInt16;
pub open spec fn IdOfPreemptedVcpuOrExecutionContext() -> UInt16;
pub open spec fn IdOfPendingInterrupt() -> UInt32;

} // verus!
