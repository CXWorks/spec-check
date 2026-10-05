use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaFunctionId = u32;
pub type FfaReturnCode = i64;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: FfaReturnCode = 0;
pub const FFA_ERROR_NOT_SUPPORTED: FfaReturnCode = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: FfaReturnCode = -2;
pub const FFA_ERROR_DENIED: FfaReturnCode = -6;
pub const FFA_ERROR_ABORTED: FfaReturnCode = -8;

pub const FFA_NOTIFICATION_UNBIND2: FfaFunctionId = 0x8400007Fu32;

pub open spec fn sender_id(s: S) -> UInt16;
pub open spec fn receiver_id(s: S) -> UInt16;
pub open spec fn bitmap(s: S) -> UInt64;

pub open spec fn ResultEqual(result: FfaReturnCode, expected: FfaReturnCode) -> bool;
pub open spec fn IsValidPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidNotificationBitmap(s: S, bm: UInt64) -> bool;
pub open spec fn ExceedsSupportedNotifications(s: S, bm: UInt64) -> bool;
pub open spec fn IsEmptyBitmap(s: S, bm: UInt64) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, fid: FfaFunctionId) -> bool;
pub open spec fn AnyBoundToOtherSender(s: S, receiver: UInt16, sender: UInt16, bm: UInt64) -> bool;
pub open spec fn AnyNotificationPending(s: S, receiver: UInt16, bm: UInt64) -> bool;
pub open spec fn CallerMayInvoke(s: S, fid: FfaFunctionId) -> bool;
pub open spec fn PartitionAborted(s: S, id: UInt16) -> bool;
pub open spec fn IsBoundToSender(s: S, receiver: UInt16, bm: UInt64, sender: UInt16) -> bool;

} // verus!
