use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type InstanceId = u32;
pub type EndpointId = u16;
pub type ConduitKind = u32;
pub type ExecState = u32;
pub type FuncId = u32;

pub struct S {
    pub cmd_input_flags: UInt32,
    pub cmd_input_endpoint_vcpu_ids: UInt64,
    pub cmd_input_timeout_lo: UInt32,
    pub cmd_input_timeout_hi: UInt32,
}

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;

pub const ERET: ConduitKind = 0;
pub const SMC: ConduitKind = 1;
pub const HVC: ConduitKind = 2;
pub const SVC: ConduitKind = 3;

pub const WAITING: ExecState = 0;
pub const RUNNING: ExecState = 1;
pub const BLOCKED: ExecState = 2;
pub const PREEMPTED: ExecState = 3;

pub const FFA_MSG_WAIT: FuncId = 0x8400006B;

pub open spec fn Instance(s: S) -> InstanceId;
pub open spec fn Caller(s: S) -> EndpointId;
pub open spec fn Conduit(s: S) -> ConduitKind;

pub open spec fn IsNsPhysicalInstance(i: InstanceId) -> bool;
pub open spec fn IsNsVirtualInstance(i: InstanceId) -> bool;
pub open spec fn IsVirtualInstance(i: InstanceId) -> bool;
pub open spec fn IsSecurePhysicalInstance(i: InstanceId) -> bool;
pub open spec fn IsPhysicalInstance(i: InstanceId) -> bool;

pub open spec fn IsRecognizedEndpointVcpuId(endpoint: UInt64, vcpu: UInt64) -> bool;
pub open spec fn Bits64(v: UInt64, hi: int, lo: int) -> UInt64;
pub open spec fn Bits32(v: UInt32, hi: int, lo: int) -> UInt32;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;
pub open spec fn CalleeCanHandleRequest(i: InstanceId) -> bool;
pub open spec fn IsImplementedAtInstance(f: FuncId, i: InstanceId) -> bool;

pub open spec fn ExecutionContextState(s: S, caller: EndpointId) -> ExecState;
pub open spec fn CallerOwnedRxBufferAtEntry(s: S, caller: EndpointId) -> bool;
pub open spec fn CallerOwnsRxBuffer(s: S, caller: EndpointId) -> bool;
pub open spec fn SchedulerInformedOfWaiting<A, B>(a: A, b: B) -> bool;
pub open spec fn VcpuRunAfterTimeout(endpoint: UInt64, vcpu: UInt64, timeout: int) -> bool;
pub open spec fn CompletesWhenAllocatedCpuCycles(s: S, caller: EndpointId) -> bool;
pub open spec fn CompletesWithAnyFfaAbiInvocation(s: S, caller: EndpointId) -> bool;

} // verus!
