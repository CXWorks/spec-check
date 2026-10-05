use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FfaInstance = u32;

pub struct S {
    pub dummy: int,
}

pub const FFA_NOTIFICATION_SET2: UInt32 = 0x8400007Eu32;

pub const FFA_SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;
pub const ABORTED: Int32 = -8;

pub open spec fn IsImplementedAtInstance(s: S, func: UInt32, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;
pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn Bits<T>(x: T, hi: int, lo: int) -> int;
pub open spec fn IsRecognizedPartitionId(s: S, id: int) -> bool;
pub open spec fn IsValidNotificationSetFlags(s: S, flags: UInt64) -> bool;
pub open spec fn BitmapContainsPerVcpuNotification(s: S, id: int, bitmap: [UInt64; 6]) -> bool;
pub open spec fn BitmapContainsGlobalNotification(s: S, id: int, bitmap: [UInt64; 6]) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn NumSupportedNotifications(s: S) -> UInt64;
pub open spec fn BitmapBit(bitmap: [UInt64; 6], i: UInt64) -> int;
pub open spec fn SenderMaySignalNotification(s: S, sender: int, receiver: int, i: UInt64) -> bool;
pub open spec fn ReceiverSupportsNotifications(s: S, id: int) -> bool;
pub open spec fn PartitionHasAborted(s: S, id: int) -> bool;
pub open spec fn GlobalNotificationSignaled<A, B, C>(a: A, b: B, c: C) -> bool;
pub open spec fn PerVcpuNotificationSignaled<A, B, C, D>(a: A, b: B, c: C, d: D) -> bool;

} // verus!
