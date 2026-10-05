use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;

#[allow(non_camel_case_types)]
pub type timeout_lo = UInt32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Interrupt,
    Error,
}

pub enum VcpuState {
    Running,
    Waiting,
    Blocked,
}

pub struct CallerCtx {
    pub state: VcpuState,
}

pub struct S {
    pub dummy: u64,
}

pub type FfaInstance = u32;
pub type Conduit = u8;

pub spec const ffa_instance: FfaInstance = 0u32;

pub spec const conduit: Conduit = 0u8;
pub spec const ERET: Conduit = 1u8;

pub spec const caller: UInt16 = 1u16;
pub spec const callee: UInt16 = 2u16;

pub spec const RUNNING: VcpuState = VcpuState::Running;

pub spec const FFA_RUN: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const FFA_INTERRUPT: Result<(), FfaStatusCode> = Err(FfaStatusCode::Interrupt);
pub spec const FFA_ERROR: Result<(), FfaStatusCode> = Err(FfaStatusCode::Error);

pub open spec fn IsFfaYieldImplemented(s: S, inst: FfaInstance) -> bool;
pub open spec fn ResultEqual(a: Result<(), FfaStatusCode>, b: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsValidEndpointVcpuId(s: S, endpoint_id: UInt16, vcpu_id: UInt16) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S, callee_id: UInt16) -> bool;
pub open spec fn IsSEl0Endpoint(s: S, id: UInt16) -> bool;
pub open spec fn IsVmVcpu(s: S, id: UInt16) -> bool;
pub open spec fn TimeoutSpecified(s: S, timeout_hi: UInt32, timeout_lo: UInt32) -> bool;
pub open spec fn VcpuScheduledAfter(s: S, endpoint_id: UInt16, vcpu_id: UInt16, timeout: UInt32) -> bool;
pub open spec fn CallerContext(s: S) -> CallerCtx;

} // verus!
