use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;

pub const FFA_MSG_WAIT: UInt32 = 0x8400006B;

pub const WAITING: UInt32 = 1;

pub const SMC: UInt32 = 0;
pub const HVC: UInt32 = 1;
pub const SVC: UInt32 = 2;
pub const ERET: UInt32 = 3;

pub struct Context {
    pub state: UInt32,
}

pub struct S {
    pub ids: Context,
    pub flags: UInt32,
    pub timeout_hi: UInt32,
    pub timeout_lo: UInt32,
}

impl S {
    pub open spec fn CallerContext(self) -> Context;
}

pub open spec fn IsNonSecurePhysicalInstance() -> bool;
pub open spec fn IsNonSecureVirtualInstance() -> bool;
pub open spec fn IsVirtualInstance() -> bool;
pub open spec fn IsSecurePhysicalInstance() -> bool;
pub open spec fn IsPhysicalInstance() -> bool;
pub open spec fn IsRecognizedEndpointOrVcpuId(ids: Context) -> bool;
pub open spec fn CalleeCanHandleRequest() -> bool;
pub open spec fn IsImplementedAtInstance(func_id: UInt32) -> bool;
pub open spec fn IsValidConduit(s: S) -> bool;
pub open spec fn CallerOwnsRxBuffer(s: S) -> bool;
pub open spec fn SchedulerInformedOfWaitTransition(ctx: Context) -> bool;
pub open spec fn Conduit(s: S) -> UInt32;
pub open spec fn IsVmVcpu(ids: Context) -> bool;
pub open spec fn TimeoutSpecified(timeout_hi: UInt32, timeout_lo: UInt32) -> bool;
pub open spec fn SchedulerRunsVcpuAfterTimeout(ids: Context, timeout_hi: UInt32, timeout_lo: UInt32) -> bool;
pub open spec fn CompletesWhenAllocatedCpuCycles(ctx: Context) -> bool;
pub open spec fn CompletesOnInvocationOfAnyFfaAbi() -> bool;

} // verus!
