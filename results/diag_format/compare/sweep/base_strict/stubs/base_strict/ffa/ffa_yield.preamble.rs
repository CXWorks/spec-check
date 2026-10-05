use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type ContextState = u32;

pub struct S {
    pub cmd_input_ids: u64,
    pub cmd_input_timeout_hi: u32,
    pub cmd_input_timeout_lo: u32,
}

pub struct ExecutionContext {
    pub id: u64,
}

pub spec const INVALID_PARAMETERS: u32 = 0xFFFF_FFFEu32;
pub spec const DENIED: u32 = 0xFFFF_FFFAu32;
pub spec const NOT_SUPPORTED: u32 = 0xFFFF_FFFFu32;
pub spec const FFA_RUN: u32 = 0x8400_006Du32;
pub spec const FFA_INTERRUPT: u32 = 0x8400_0062u32;
pub spec const FFA_YIELD: u32 = 0x8400_006Cu32;

pub spec const RUNNING: ContextState = 1u32;

pub open spec fn ConduitIsEret() -> bool;

pub open spec fn Bits(x: u64, hi: int, lo: int) -> u64;

pub open spec fn IsRecognizedEndpointId(id: u64) -> bool;

pub open spec fn IsRecognizedVcpuId(endpoint_id: u64, vcpu_id: u64) -> bool;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub open spec fn CalleeCanHandleRequest() -> bool;

pub open spec fn IsImplementedAtInstance(func_id: u32) -> bool;

pub open spec fn CallerContext(s: S) -> ExecutionContext;

pub open spec fn ExecutionYieldedToScheduler(ctx: ExecutionContext) -> bool;

pub open spec fn ExecutionContextState(ctx: ExecutionContext) -> ContextState;

pub open spec fn IsSEL0Endpoint(ctx: ExecutionContext) -> bool;

pub open spec fn IsVmVcpu(ctx: ExecutionContext) -> bool;

pub open spec fn Timeout64(hi: u32, lo: u32) -> u64;

pub open spec fn VcpuRunAfterTimeout(endpoint_id: u64, vcpu_id: u64, timeout: u64) -> bool;

} // verus!
