use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct FfaReturn {
    pub function_id: u32,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
    pub arg6: u64,
    pub arg7: u64,
}

pub struct S {
    pub state_id: nat,
}

pub const NOT_SUPPORTED: Int32 = -1i32;

pub const DENIED: Int32 = -6i32;

pub const RETRY: Int32 = -7i32;

pub open spec fn FfaMsgPollImplemented(s: S) -> bool;

pub open spec fn CallerProcessingDirectRequest(s: S) -> bool;

pub open spec fn CalleeReadyForRequest(s: S) -> bool;

pub open spec fn CallerRxBufferMessageAvailable(s: S) -> bool;

pub open spec fn FfaErrorEqual(result: FfaReturn, code: Int32) -> bool;

pub open spec fn FfaResultIsMsgSend(result: FfaReturn) -> bool;

} // verus!
