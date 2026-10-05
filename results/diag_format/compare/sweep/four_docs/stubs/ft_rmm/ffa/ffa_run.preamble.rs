use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub type ExecStateKind = u8;

pub struct ExecContext {
    pub state: ExecStateKind,
}

pub struct S {
    pub dummy: int,
}

pub const RUNNING: ExecStateKind = 0;
pub const BLOCKED: ExecStateKind = 1;
pub const PREEMPTED: ExecStateKind = 2;
pub const WAITING: ExecStateKind = 3;
pub const ABORTED_STATE: ExecStateKind = 4;

pub const FFA_RUN: UInt32 = 0x8400006D;

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NO_MEMORY: Int32 = -3;
pub const BUSY: Int32 = -4;
pub const INTERRUPTED: Int32 = -5;
pub const DENIED: Int32 = -6;
pub const RETRY: Int32 = -7;
pub const ABORTED: Int32 = -8;
pub const NO_DATA: Int32 = -9;
pub const NOT_READY: Int32 = -10;

pub const FFA_ERROR: Int32 = 100;
pub const FFA_INTERRUPT: Int32 = 101;
pub const FFA_MSG_WAIT: Int32 = 102;
pub const FFA_YIELD: Int32 = 103;
pub const FFA_MSG_SEND_DIRECT_RESP: Int32 = 104;

pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidVcpuId(s: S, id: UInt16, vcpu: UInt16) -> bool;
pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn IsVcpuPinnedToOtherPe(s: S, id: UInt16, vcpu: UInt16) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func: UInt32) -> bool;
pub open spec fn IsCalleeInStateToHandleRequest(s: S, id: UInt16, vcpu: UInt16) -> bool;
pub open spec fn IsCallerAllowedToInvoke(s: S, func: UInt32) -> bool;
pub open spec fn IsVcpuBusy(s: S, id: UInt16, vcpu: UInt16) -> bool;
pub open spec fn HasAborted(s: S, id: UInt16, vcpu: UInt16) -> bool;
pub open spec fn IsReceiverReady(s: S, id: UInt16) -> bool;
pub open spec fn ExecContextAt(s: S, id: UInt16, vcpu: UInt16) -> ExecContext;
pub open spec fn CallerExecContext(s: S) -> ExecContext;

} // verus!
