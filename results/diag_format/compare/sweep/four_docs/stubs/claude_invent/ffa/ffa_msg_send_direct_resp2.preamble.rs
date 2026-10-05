use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const DENIED: FfaErrorCode = -6;
pub const ABORTED: FfaErrorCode = -8;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }

    pub open spec fn get_Ok_0(self) -> T
        recommends self.is_Ok()
    {
        match self {
            Result::Ok(v) => v,
            Result::Err(_) => arbitrary(),
        }
    }

    pub open spec fn get_Err_0(self) -> E
        recommends self.is_Err()
    {
        match self {
            Result::Ok(_) => arbitrary(),
            Result::Err(e) => e,
        }
    }
}

pub struct S {
    pub state_id: int,
}

pub open spec fn FfaFunctionImplementedAtInstance(s: S, function_id: UInt32) -> bool;

pub open spec fn FfaEndpointIdValid(s: S, id: int) -> bool;

pub open spec fn FfaDirectMsgFlagsValid(s: S, src_dst_ids: UInt32, x2: UInt64, x3: UInt64) -> bool;

pub open spec fn FfaCalleeStateAllowsDirectResp(s: S, id: int) -> bool;

pub open spec fn FfaEndpointSupportsSendDirectResp2(s: S, id: int) -> bool;

pub open spec fn FfaEndpointSupportsReceiveDirectResp2(s: S, id: int) -> bool;

pub open spec fn FfaReceiverAborted(old_s: S, new_s: S, id: int) -> bool;

pub open spec fn FfaDirectResp2MessageDelivered(old_s: S, new_s: S, sender: int, receiver: int) -> bool;

pub open spec fn FfaMsgWaitSuccess(old_s: S, new_s: S, id: int) -> bool;

} // verus!
