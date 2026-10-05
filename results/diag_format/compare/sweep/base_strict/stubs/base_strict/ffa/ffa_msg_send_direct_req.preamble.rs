use vstd::prelude::*;

verus! {

pub type FfaInstance = u32;

pub type RuntimeState = u8;

pub struct S {
    pub sender_receiver_ids: u32,
    pub flags: u32,
}

pub struct EndpointInfo {
    pub runtime_state: RuntimeState,
}

pub const FFA_MSG_SEND_DIRECT_REQ: u32 = 0x8400006F;
pub const FFA_MSG_SEND_DIRECT_RESP: u32 = 0x84000070;
pub const FFA_INTERRUPT: u32 = 0x84000062;
pub const FFA_YIELD: u32 = 0x8400006C;
pub const FFA_SUCCESS: u32 = 0x84000061;
pub const NOT_SUPPORTED: u32 = 0xFFFFFFFF;
pub const INVALID_PARAMETERS: u32 = 0xFFFFFFFE;
pub const DENIED: u32 = 0xFFFFFFFA;
pub const BUSY: u32 = 0xFFFFFFF9;
pub const ABORTED: u32 = 0xFFFFFFF8;
pub const NOT_READY: u32 = 0xFFFFFFF7;

pub const RUNNING: RuntimeState = 1;
pub const BLOCKED: RuntimeState = 2;
pub const PREEMPTED: RuntimeState = 3;

pub open spec fn IsImplementedAtInstance(func_id: u32, instance: FfaInstance) -> bool;

pub open spec fn CurrentFfaInstance() -> FfaInstance;

pub open spec fn ResultEqual(result: u32, code: u32) -> bool;

pub open spec fn IsValidEndpointId(id: u32) -> bool;

pub open spec fn Bits(value: u32, hi: nat, lo: nat) -> u32;

pub open spec fn IsValidFrameworkMessageType(t: u32) -> bool;

pub open spec fn CalleeCanHandleRequest() -> bool;

pub open spec fn SupportsDirectRequestReceipt(id: u32) -> bool;

pub open spec fn Endpoint(value: u32, hi: nat, lo: nat) -> EndpointInfo;

pub open spec fn EndpointHasAborted(id: u32) -> bool;

pub open spec fn EndpointIsReady(id: u32) -> bool;

pub open spec fn IsDirectResponseTo(sender: u32, receiver: u32) -> bool;

pub open spec fn MustResumeViaFfaRun(id: u32) -> bool;

pub open spec fn OtherParameterRegistersAreZero() -> bool;

} // verus!
