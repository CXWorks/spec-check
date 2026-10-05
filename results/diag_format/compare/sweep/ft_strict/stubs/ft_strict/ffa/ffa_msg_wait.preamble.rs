use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;

pub const FFA_MSG_WAIT: UInt32 = 0x8400006B;

pub const WAITING: u32 = 0;
pub const RUNNING: u32 = 1;

pub const ERET: u32 = 10;
pub const SMC: u32 = 11;
pub const HVC: u32 = 12;
pub const SVC: u32 = 13;

pub open spec fn caller_id() -> int;
pub open spec fn conduit_value() -> u32;

pub spec const caller: int = caller_id();
pub spec const conduit: u32 = conduit_value();

pub open spec fn IsNsPhysicalInstance(s: S) -> bool;
pub open spec fn IsNsVirtualInstance(s: S) -> bool;
pub open spec fn IsVirtualInstance(s: S) -> bool;
pub open spec fn IsSecurePhysicalInstance(s: S) -> bool;
pub open spec fn IsPhysicalInstance(s: S) -> bool;
pub open spec fn IsRecognizedEndpointVcpuId(s: S, endpoint_id: int, vcpu_id: int) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, variant: int) -> bool;
pub open spec fn ExecutionContextState(s: S, c: int) -> u32;
pub open spec fn CallerOwnedRxBufferAtEntry(s: S, c: int) -> bool;
pub open spec fn CallerOwnsRxBuffer(s: S, c: int) -> bool;
pub open spec fn SchedulerInformedOfWaiting(s: S, c: int) -> bool;
pub open spec fn VcpuRunAfterTimeout(s: S, endpoint_id: int, vcpu_id: int, timeout: int) -> bool;
pub open spec fn CompletesWhenAllocatedCpuCycles(s: S, c: int) -> bool;
pub open spec fn CompletesWithAnyFfaAbiInvocation(s: S, c: int) -> bool;

} // verus!
