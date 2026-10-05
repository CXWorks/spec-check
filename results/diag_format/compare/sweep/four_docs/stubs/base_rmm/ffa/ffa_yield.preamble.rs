use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type EndpointId = u32;
pub type VcpuId = u32;
pub type FfaInstance = u32;
pub type Conduit = u32;
pub type PartitionId = u32;
pub type VcpuStateCode = u32;

pub const NOT_SUPPORTED: u32 = 1;
pub const INVALID_PARAMETERS: u32 = 2;
pub const DENIED: u32 = 3;
pub const FFA_RUN: u32 = 4;
pub const FFA_INTERRUPT: u32 = 5;

pub const ERET: u32 = 6;
pub const SMC: u32 = 7;

pub const BLOCKED: u32 = 8;
pub const RUNNING: u32 = 9;

pub struct Context {
    pub state: VcpuStateCode,
}

pub struct S {
    pub ffa_instance: FfaInstance,
    pub conduit: Conduit,
    pub endpoint_id: EndpointId,
    pub vcpu_id: VcpuId,
    pub callee: PartitionId,
    pub caller: PartitionId,
    pub timeout_hi: u32,
    pub timeout_lo: u32,
}

pub open spec fn IsFfaYieldImplemented(inst: FfaInstance) -> bool;

pub open spec fn ResultEqual(a: u32, b: u32) -> bool;

pub open spec fn IsValidEndpointVcpuId(endpoint_id: EndpointId, vcpu_id: VcpuId) -> bool;

pub open spec fn CalleeCanHandleRequest(callee: PartitionId) -> bool;

pub open spec fn CallerContext(s: S) -> Context;

pub open spec fn IsSEl0Endpoint(caller: PartitionId) -> bool;

pub open spec fn IsVmVcpu(caller: PartitionId) -> bool;

pub open spec fn TimeoutSpecified(timeout_hi: u32, timeout_lo: u32) -> bool;

pub open spec fn VcpuScheduledAfter(endpoint_id: EndpointId, vcpu_id: VcpuId, timeout_hi: u32, timeout_lo: u32) -> bool;

} // verus!
