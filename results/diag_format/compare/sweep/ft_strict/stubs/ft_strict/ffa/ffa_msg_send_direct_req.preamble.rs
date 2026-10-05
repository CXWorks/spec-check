use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FfaInstance = u32;
pub type RuntimeState = u8;

pub struct EndpointInfo {
    pub runtime_state: RuntimeState,
}

pub struct S {
    pub dummy: int,
}

pub spec const RUNNING: RuntimeState = 1;
pub spec const BLOCKED: RuntimeState = 2;
pub spec const PREEMPTED: RuntimeState = 3;

pub spec const NOT_SUPPORTED: UInt32 = 0xFFFF_FFFF;
pub spec const INVALID_PARAMETERS: UInt32 = 0xFFFF_FFFE;
pub spec const DENIED: UInt32 = 0xFFFF_FFFC;
pub spec const BUSY: UInt32 = 0xFFFF_FFF8;
pub spec const ABORTED: UInt32 = 0xFFFF_FFF6;
pub spec const NOT_READY: UInt32 = 0xFFFF_FFF5;
pub spec const FFA_SUCCESS: UInt32 = 0x8400_0061;
pub spec const FFA_INTERRUPT: UInt32 = 0x8400_0062;
pub spec const FFA_YIELD: UInt32 = 0x8400_006C;
pub spec const FFA_MSG_SEND_DIRECT_REQ: UInt32 = 0x8400_006F;
pub spec const FFA_MSG_SEND_DIRECT_RESP: UInt32 = 0x8400_0070;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;
pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;
pub open spec fn IsValidEndpointId(s: S, id: UInt32) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn IsValidFrameworkMessageType(s: S, t: UInt32) -> bool;
pub open spec fn CalleeCanHandleRequest(s: S) -> bool;
pub open spec fn SupportsDirectRequestReceipt(s: S, id: UInt32) -> bool;
pub open spec fn Endpoint(s: S, id: UInt32) -> EndpointInfo;
pub open spec fn EndpointHasAborted(s: S, id: UInt32) -> bool;
pub open spec fn EndpointIsReady(s: S, id: UInt32) -> bool;
pub open spec fn IsDirectResponseTo(s: S, sender: UInt32, receiver: UInt32) -> bool;
pub open spec fn MustResumeViaFfaRun(s: S, id: UInt32) -> bool;
pub open spec fn OtherParameterRegistersAreZero(s: S) -> bool;

} // verus!
