use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type FfaErrorCode = i32;

pub const INVALID_PARAMETERS: FfaErrorCode = -2;

pub const NOT_SUPPORTED: FfaErrorCode = -1;

pub const DENIED: FfaErrorCode = -6;

pub const ABORTED: FfaErrorCode = -8;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub use crate::Result::Ok;
pub use crate::Result::Err;

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
}

pub struct S {
    pub dummy: int,
}

pub uninterp spec fn FfaPartitionIdIsRecognized(s: S, id: u16) -> bool;

pub uninterp spec fn FfaNotificationUnbindIsImplemented(s: S) -> bool;

pub uninterp spec fn FfaNotificationBoundToOtherSender(s: S, receiver: u16, sender: u16, bitmap: u64) -> bool;

pub uninterp spec fn FfaNotificationIsPending(s: S, receiver: u16, bitmap: u64) -> bool;

pub uninterp spec fn FfaCallerMayInvokeNotificationUnbind(s: S, sender: u16, receiver: u16) -> bool;

pub uninterp spec fn FfaPartitionHasAborted(s: S, id: u16) -> bool;

pub uninterp spec fn FfaNotificationIsBoundTo(s: S, receiver: u16, sender: u16, i: int) -> bool;

pub uninterp spec fn FfaStateUnchangedExceptNotificationBindings(old_s: S, new_s: S) -> bool;

} // verus!
