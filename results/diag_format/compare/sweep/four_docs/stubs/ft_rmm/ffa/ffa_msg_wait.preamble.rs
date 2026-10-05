use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type StateT = u32;
pub type ConduitT = u32;
pub type FuncId = u32;

pub struct Bits32Val {
    pub state: StateT,
    pub raw: u32,
}

impl Bits32Val {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub type Bits32 = Bits32Val;

pub struct S {
    pub dummy: int,
}

#[allow(non_camel_case_types)]
pub enum ResultVal {
    result,
    RSI_SUCCESS,
    RSI_ERROR,
}

impl ResultVal {
    pub open spec fn is_Ok(self) -> bool;
}

pub use ResultVal::*;

pub const WAITING: StateT = 1;

pub const ERET: ConduitT = 0;
pub const SMC: ConduitT = 1;
pub const HVC: ConduitT = 2;
pub const SVC: ConduitT = 3;

pub const FFA_MSG_WAIT: FuncId = 0x8400006B;

pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const NOT_SUPPORTED: Int32 = -1;

pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;
pub open spec fn IsNonSecureVirtualInstance(s: S) -> bool;
pub open spec fn IsVirtualInstance(s: S) -> bool;
pub open spec fn IsSecurePhysicalInstance(s: S) -> bool;
pub open spec fn IsPhysicalInstance(s: S) -> bool;
pub open spec fn IsRecognizedEndpointOrVcpuId(s: S, ids: Bits32) -> bool;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, f: FuncId) -> bool;
pub open spec fn IsValidConduit(s: S) -> bool;
pub open spec fn CallerContext(s: S) -> Bits32;
pub open spec fn CallerOwnsRxBuffer(s: S) -> bool;
pub open spec fn SchedulerInformedOfWaitTransition(s: S, c: Bits32) -> bool;
pub open spec fn Conduit(s: S) -> ConduitT;
pub open spec fn IsVmVcpu(s: S, ids: Bits32) -> bool;
pub open spec fn TimeoutSpecified(s: S, hi: UInt32, lo: UInt32) -> bool;
pub open spec fn SchedulerRunsVcpuAfterTimeout(s: S, ids: Bits32, hi: int, lo: int) -> bool;
pub open spec fn CompletesWhenAllocatedCpuCycles(s: S, c: Bits32) -> bool;
pub open spec fn CompletesOnInvocationOfAnyFfaAbi(s: S) -> bool;
pub open spec fn SchedVcpuTimeout(s: S, ids: Bits32) -> int;

} // verus!
