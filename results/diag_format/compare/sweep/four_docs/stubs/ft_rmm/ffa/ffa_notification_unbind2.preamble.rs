use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FFAFunction = u32;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: FFAFunction = 0x84000061;
pub const FFA_ERROR: FFAFunction = 0x84000060;
pub const FFA_NOTIFICATION_UNBIND2: FFAFunction = 0x84000080;
pub const INVALID_PARAMETERS: FFAFunction = 1;
pub const NOT_SUPPORTED: FFAFunction = 2;
pub const DENIED: FFAFunction = 3;
pub const ABORTED: FFAFunction = 4;

pub open spec fn IsValidPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn ResultEqual(result: FFAFunction, expected: FFAFunction) -> bool;
pub open spec fn IsValidNotificationBitmap(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn ExceedsSupportedNotifications(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn IsEmptyBitmap(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, f: FFAFunction) -> bool;
pub open spec fn AnyBoundToOtherSender(s: S, receiver_id: UInt16, sender_id: UInt16, bitmap: [UInt64; 6]) -> bool;
pub open spec fn AnyNotificationPending(s: S, receiver_id: UInt16, bitmap: [UInt64; 6]) -> bool;
pub open spec fn CallerMayInvoke(s: S, f: FFAFunction) -> bool;
pub open spec fn PartitionAborted(s: S, id: UInt16) -> bool;
pub open spec fn BitmapBit(bitmap: [UInt64; 6], i: int) -> int;
pub open spec fn IsBoundToSender(s: S, receiver_id: UInt16, i: int, sender_id: UInt16) -> bool;

} // verus!
