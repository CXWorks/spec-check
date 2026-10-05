use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaErrorCode = i32;

pub const NOT_SUPPORTED: FfaErrorCode = -1;
pub const INVALID_PARAMETERS: FfaErrorCode = -2;
pub const NO_MEMORY: FfaErrorCode = -3;
pub const BUSY: FfaErrorCode = -4;
pub const INTERRUPTED: FfaErrorCode = -5;
pub const DENIED: FfaErrorCode = -6;
pub const RETRY: FfaErrorCode = -7;
pub const ABORTED: FfaErrorCode = -8;
pub const NO_DATA: FfaErrorCode = -9;

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub struct S {
    pub state_id: nat,
}

pub open spec fn FfaNotificationInfoGetImplemented(s: S) -> bool;

pub open spec fn HasPendingNotificationInfo(s: S) -> bool;

pub open spec fn ResultEqual(result: Result<(), FfaErrorCode>, code: FfaErrorCode) -> bool;

pub open spec fn IdListsEncodePendingNotificationInfo(s: S, function_id: UInt32, flags: UInt64, id_lists: Seq<UInt64>) -> bool;

pub open spec fn PendingNotificationInfoRetrievedOnce(old_s: S, new_s: S, flags: UInt64, id_lists: Seq<UInt64>) -> bool;

} // verus!
