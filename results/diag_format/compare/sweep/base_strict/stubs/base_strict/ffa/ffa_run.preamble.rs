use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type ExecState = u32;

pub struct S {
    pub dummy: u32,
}

pub spec const FFA_ERROR: UInt32 = 0x84000060u32;
pub spec const FFA_INTERRUPT: UInt32 = 0x84000062u32;
pub spec const FFA_MSG_WAIT: UInt32 = 0x8400006Bu32;
pub spec const FFA_YIELD: UInt32 = 0x8400006Cu32;
pub spec const FFA_RUN: UInt32 = 0x8400006Du32;
pub spec const FFA_MSG_SEND_DIRECT_RESP: UInt32 = 0x84000070u32;

pub spec const NOT_SUPPORTED: Int32 = -1i32;
pub spec const INVALID_PARAMETERS: Int32 = -2i32;
pub spec const BUSY: Int32 = -4i32;
pub spec const DENIED: Int32 = -6i32;
pub spec const ABORTED: Int32 = -8i32;
pub spec const NOT_READY: Int32 = -10i32;

pub spec const SMC: UInt32 = 0u32;
pub spec const HVC: UInt32 = 1u32;
pub spec const SVC: UInt32 = 2u32;
pub spec const ERET: UInt32 = 3u32;

pub spec const NS_PHYSICAL: UInt32 = 0u32;
pub spec const SECURE_PHYSICAL: UInt32 = 1u32;

pub spec const WAITING: ExecState = 0u32;
pub spec const BLOCKED: ExecState = 1u32;
pub spec const PREEMPTED: ExecState = 2u32;
pub spec const RUNNING: ExecState = 3u32;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;
pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsImplementedAtInstance(func_id: UInt32, ffa_instance: UInt32) -> bool;
pub open spec fn IsRecognizedEndpointId(id: UInt32) -> bool;
pub open spec fn IsRecognizedVcpuId(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn CurrentPe() -> UInt32;
pub open spec fn IsVcpuPinnedToOtherPe(endpoint: UInt32, vcpu: UInt32, pe: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn CpuCycleAllocationMode() -> UInt32;
pub open spec fn CallerAllowedToInvokeRun(caller: UInt32, mode: UInt32) -> bool;
pub open spec fn IsVcpuBusy(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn HasVcpuOrVmAborted(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn IsReceiverReady(endpoint: UInt32) -> bool;
pub open spec fn PreExecCtxState(endpoint: UInt32, vcpu: UInt32) -> ExecState;
pub open spec fn TransitionedToRunning(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn TransitionedToRunningViaEret(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn IsPhysicalInstance(ffa_instance: UInt32) -> bool;
pub open spec fn IsVirtualInstance(ffa_instance: UInt32) -> bool;
pub open spec fn RequestedTransitionFromPartitionManager(endpoint: UInt32, vcpu: UInt32) -> bool;
pub open spec fn CallerWasBlockedDuringRun(caller: UInt32) -> bool;
pub open spec fn ExecCtxState(caller: UInt32) -> ExecState;
pub open spec fn CompletedViaEret(result: UInt32) -> bool;
pub open spec fn CompletedViaSmc(result: UInt32) -> bool;

} // verus!
