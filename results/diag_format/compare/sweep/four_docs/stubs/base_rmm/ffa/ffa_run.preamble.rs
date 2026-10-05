use vstd::prelude::*;

macro_rules! ExecContextAt {
    ($s:expr, $id:expr, $vcpu:expr) => {
        ExecContextAtVcpu($s, $id, $vcpu)
    };
    ($s:expr, $ctx:expr) => {
        ExecContextAtId($s, $ctx)
    };
}

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt16 = u16;
pub type EndpointId = u16;
pub type VcpuId = u16;
pub type FunctionId = u32;
pub type VcpuState = u8;

pub struct ExecContextId {
    pub endpoint: EndpointId,
    pub vcpu: VcpuId,
}

pub struct ExecContext {
    pub state: VcpuState,
}

pub struct S {
    pub dummy: nat,
}

pub const target_id: EndpointId = 1;
pub const target_vcpu: VcpuId = 2;

pub const WAITING: VcpuState = 0;
pub const RUNNING: VcpuState = 1;
pub const BLOCKED: VcpuState = 2;
pub const PREEMPTED: VcpuState = 3;

pub const INVALID_PARAMETERS: Int32 = -2;
pub const NOT_SUPPORTED: Int32 = -1;
pub const DENIED: Int32 = -6;
pub const BUSY: Int32 = -4;
pub const ABORTED: Int32 = -8;
pub const NOT_READY: Int32 = -10;

pub const FFA_RUN: FunctionId = 0x8400006D;
pub const FFA_INTERRUPT: FunctionId = 0x84000062;
pub const FFA_MSG_WAIT: FunctionId = 0x8400006B;
pub const FFA_YIELD: FunctionId = 0x8400006C;
pub const FFA_MSG_SEND_DIRECT_RESP: FunctionId = 0x84000070;

pub open spec fn IsValidEndpointId(s: S, id: EndpointId) -> bool;
pub open spec fn IsValidVcpuId(s: S, id: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsVcpuPinnedToOtherPe(s: S, id: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, fid: FunctionId) -> bool;
pub open spec fn IsCalleeInStateToHandleRequest(s: S, id: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsCallerAllowedToInvoke(s: S, fid: FunctionId) -> bool;
pub open spec fn IsVcpuBusy(s: S, id: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn HasAborted(s: S, id: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsReceiverReady(s: S, id: EndpointId) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn ExecContextAtVcpu(s: S, id: EndpointId, vcpu: VcpuId) -> ExecContext;
pub open spec fn ExecContextAtId(s: S, ctx: ExecContextId) -> ExecContext;
pub open spec fn CallerExecContext() -> ExecContextId;

} // verus!
