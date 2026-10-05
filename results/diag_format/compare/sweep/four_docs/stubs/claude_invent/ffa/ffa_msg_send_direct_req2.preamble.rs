use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct FfaReturn {
    pub function_id: u32,
    pub error_code: i32,
    pub params: Seq<u64>,
}

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -6;
pub const BUSY: i32 = -4;
pub const ABORTED: i32 = -8;
pub const NOT_READY: i32 = -5;

pub const FFA_MSG_SEND_DIRECT_RESP2: u32 = 0xC400008Eu32;
pub const FFA_INTERRUPT: u32 = 0x84000062u32;
pub const FFA_YIELD: u32 = 0x8400006Cu32;
pub const FFA_SUCCESS: u32 = 0x84000061u32;

pub open spec fn FfaFunctionImplemented(s: S, fid: u32) -> bool;
pub open spec fn FfaReturnIsError(r: FfaReturn, code: i32) -> bool;
pub open spec fn FfaIsValidEndpointId(s: S, id: u16) -> bool;
pub open spec fn FfaIsRecognizedUuid(s: S, id: u16, uuid_lo: u64, uuid_hi: u64) -> bool;
pub open spec fn FfaCalleeCanHandleRequest(s: S) -> bool;
pub open spec fn FfaCallerAllowedToInvoke(s: S, id: u16, fid: u32) -> bool;
pub open spec fn FfaReceiverSupportsDirectReq(s: S, id: u16) -> bool;
pub open spec fn FfaEndpointIsRunning(s: S, id: u16) -> bool;
pub open spec fn FfaEndpointIsBlocked(s: S, id: u16) -> bool;
pub open spec fn FfaEndpointIsPreempted(s: S, id: u16) -> bool;
pub open spec fn FfaEndpointIsAborted(s: S, id: u16) -> bool;
pub open spec fn FfaEndpointIsReady(s: S, id: u16) -> bool;
pub open spec fn FfaValidInstanceConduit(s: S, fid: u32) -> bool;
pub open spec fn FfaReturnFunctionIs(r: FfaReturn, fid: u32) -> bool;
pub open spec fn FfaReturnOtherParamsZero(r: FfaReturn) -> bool;

} // verus!
