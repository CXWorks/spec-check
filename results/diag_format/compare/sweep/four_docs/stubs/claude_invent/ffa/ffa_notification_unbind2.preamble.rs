use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaReturnCode = i32;

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const NOT_SUPPORTED: FfaReturnCode = -1;
pub const INVALID_PARAMETERS: FfaReturnCode = -2;
pub const DENIED: FfaReturnCode = -6;
pub const ABORTED: FfaReturnCode = -8;

pub struct S {
    pub dummy: int,
}

pub open spec fn FfaFunctionImplemented(s: S, fid: u32) -> bool;

pub open spec fn FfaIsValidPartitionId(s: S, id: int) -> bool;

pub open spec fn FfaNumSupportedNotifications(s: S) -> nat;

pub open spec fn FfaCallerAllowedNotificationUnbind2(s: S, sender: int, receiver: int) -> bool;

pub open spec fn FfaNotificationIsBound(s: S, receiver: int, i: int) -> bool;

pub open spec fn FfaNotificationBoundSender(s: S, receiver: int, i: int) -> int;

pub open spec fn FfaNotificationIsPending(s: S, receiver: int, i: int) -> bool;

pub open spec fn FfaPartitionAborted(s: S, id: int) -> bool;

} // verus!
