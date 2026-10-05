use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type EndpointId = u32;
pub type VcpuId = u32;
pub type PeId = u32;
pub type FuncId = u32;
pub type FfaInstance = u32;
pub type Conduit = u32;
pub type CtxState = u32;
pub type AllocMode = u32;

pub struct S {
    pub dummy: int,
}

pub const FFA_ERROR: UInt32 = 0x84000060;
pub const FFA_INTERRUPT: UInt32 = 0x84000062;
pub const FFA_MSG_WAIT: UInt32 = 0x8400006B;
pub const FFA_YIELD: UInt32 = 0x8400006C;
pub const FFA_RUN: FuncId = 0x8400006D;
pub const FFA_MSG_SEND_DIRECT_RESP: UInt32 = 0x84000070;

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NO_MEMORY: Int32 = -3;
pub const BUSY: Int32 = -4;
pub const NOT_READY: Int32 = -5;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;

pub const NS_PHYSICAL: FfaInstance = 101;
pub const SECURE_PHYSICAL: FfaInstance = 102;
pub const ffa_instance: FfaInstance = 103;

pub const SMC: Conduit = 201;
pub const HVC: Conduit = 202;
pub const SVC: Conduit = 203;
pub const ERET: Conduit = 204;
pub const conduit: Conduit = 205;

pub const WAITING: CtxState = 301;
pub const BLOCKED: CtxState = 302;
pub const PREEMPTED: CtxState = 303;
pub const RUNNING: CtxState = 304;

pub const caller: EndpointId = 401;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn CurrentPe() -> PeId;
pub open spec fn CpuCycleAllocationMode() -> AllocMode;
pub open spec fn IsImplementedAtInstance(s: S, func: FuncId, inst: FfaInstance) -> bool;
pub open spec fn IsRecognizedEndpointId(s: S, ep: EndpointId) -> bool;
pub open spec fn IsRecognizedVcpuId(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsVcpuPinnedToOtherPe(s: S, ep: EndpointId, vcpu: VcpuId, pe: PeId) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn CallerAllowedToInvokeRun(s: S, c: EndpointId, mode: AllocMode) -> bool;
pub open spec fn IsVcpuBusy(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn HasVcpuOrVmAborted(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsReceiverReady(s: S, ep: EndpointId) -> bool;
pub open spec fn PreExecCtxState(s: S, ep: EndpointId, vcpu: VcpuId) -> CtxState;
pub open spec fn TransitionedToRunning(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn TransitionedToRunningViaEret(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn IsPhysicalInstance(s: S, inst: FfaInstance) -> bool;
pub open spec fn IsVirtualInstance(s: S, inst: FfaInstance) -> bool;
pub open spec fn RequestedTransitionFromPartitionManager(s: S, ep: EndpointId, vcpu: VcpuId) -> bool;
pub open spec fn CallerWasBlockedDuringRun(s: S, c: EndpointId) -> bool;
pub open spec fn ExecCtxState(s: S, c: EndpointId) -> CtxState;
pub open spec fn CompletedViaEret(result: UInt32) -> bool;
pub open spec fn CompletedViaSmc(result: UInt32) -> bool;

} // verus!
