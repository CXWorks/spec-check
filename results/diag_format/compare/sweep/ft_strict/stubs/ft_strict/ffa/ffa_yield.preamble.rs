use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub struct ExecutionContext {
    pub id: int,
}

pub enum ContextState {
    Running,
    Waiting,
    Blocked,
    Preempted,
}

pub spec const RUNNING: ContextState = ContextState::Running;

pub spec const FFA_ERROR: UInt32 = 0x84000060u32;
pub spec const FFA_INTERRUPT: UInt32 = 0x84000062u32;
pub spec const FFA_YIELD: UInt32 = 0x8400006Cu32;
pub spec const FFA_RUN: UInt32 = 0x8400006Du32;
pub spec const NOT_SUPPORTED: UInt32 = 0xFFFFFFFFu32;
pub spec const INVALID_PARAMETERS: UInt32 = 0xFFFFFFFEu32;
pub spec const DENIED: UInt32 = 0xFFFFFFF9u32;

pub open spec fn ConduitIsEret(s: S) -> bool;
pub open spec fn IsRecognizedEndpointId(s: S, id: UInt32) -> bool;
pub open spec fn IsRecognizedVcpuId(s: S, ep: UInt32, vcpu: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn ExecutionYieldedToScheduler(s: S, ctx: ExecutionContext) -> bool;
pub open spec fn CallerContext() -> ExecutionContext;
pub open spec fn ExecutionContextState(s: S, ctx: ExecutionContext) -> ContextState;
pub open spec fn IsSEL0Endpoint(s: S, ctx: ExecutionContext) -> bool;
pub open spec fn IsVmVcpu(s: S, ctx: ExecutionContext) -> bool;
pub open spec fn Timeout64(hi: UInt32, lo: UInt32) -> UInt64;
pub open spec fn VcpuRunAfterTimeout(s: S, ep: UInt32, vcpu: UInt32, timeout: UInt64) -> bool;

} // verus!
