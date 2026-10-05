use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub struct FfaReturn {
    pub func: UInt32,
    pub arg1: UInt32,
    pub arg2: UInt32,
    pub arg3: UInt32,
}

pub const NOT_SUPPORTED: Int32 = -1i32;

pub const INVALID_PARAMETERS: Int32 = -2i32;

pub const DENIED: Int32 = -6i32;

pub const ABORTED: Int32 = -8i32;

pub open spec fn FfaFunctionImplemented(s: S, function_id: UInt32) -> bool;

pub open spec fn IsValidEndpointId(s: S, id: UInt32) -> bool;

pub open spec fn IsValidFrameworkMsgType(t: UInt32) -> bool;

pub open spec fn CalleeCanHandleRequest(s: S, src_id: UInt32, dst_id: UInt32) -> bool;

pub open spec fn CallerAllowedToInvoke(s: S, src_id: UInt32, function_id: UInt32) -> bool;

pub open spec fn ReceiverSupportsDirectResp(s: S, dst_id: UInt32) -> bool;

pub open spec fn ReceiverAborted(old_s: S, new_s: S, dst_id: UInt32) -> bool;

pub open spec fn PendingDirectReqIsSmc64(s: S, src_id: UInt32, dst_id: UInt32) -> bool;

pub open spec fn FfaErrorEqual(result: FfaReturn, err: Int32) -> bool;

pub open spec fn DirectRespDelivered(old_s: S, new_s: S, src_id: UInt32, dst_id: UInt32, msg_type: UInt32, low_bits: UInt32) -> bool;

pub open spec fn FfaMsgWaitSuccess(result: FfaReturn, old_s: S, new_s: S, src_id: UInt32) -> bool;

} // verus!
