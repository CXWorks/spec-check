use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FuncId = u32;
pub type InstanceId = u64;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;
pub const INVALID_PARAMETERS: UInt32 = 0xFFFFFFFE;
pub const DENIED: UInt32 = 0xFFFFFFFD;
pub const ABORTED: UInt32 = 0xFFFFFFF8;

pub const FFA_NOTIFICATION_BIND2: FuncId = 0x8400007F;

pub open spec fn IsImplementedAtInstance(s: S, func: FuncId, inst: InstanceId) -> bool;

pub open spec fn CurrentInstance(s: S) -> InstanceId;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn Bits<T>(x: T, hi: int, lo: int) -> int;

pub open spec fn IsValidSenderId(s: S, id: int) -> bool;

pub open spec fn IsValidReceiverId(s: S, id: int) -> bool;

pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;

pub open spec fn NumSupportedNotifications(s: S) -> UInt64;

pub open spec fn BitmapBit(bitmap: [UInt64; 6], i: UInt64) -> int;

pub open spec fn IsBoundToOtherSender(s: S, receiver: int, i: UInt64, sender: int) -> bool;

pub open spec fn IsNotificationPending(s: S, receiver: int, i: UInt64) -> bool;

pub open spec fn CallerAllowedToInvoke(s: S, func: FuncId) -> bool;

pub open spec fn PartitionHasAborted(s: S, id: int) -> bool;

pub open spec fn NotificationBoundSender(s: S, receiver: int, i: UInt64) -> int;

pub open spec fn NotificationIsPerVcpu(s: S, receiver: int, i: UInt64) -> bool;

} // verus!
