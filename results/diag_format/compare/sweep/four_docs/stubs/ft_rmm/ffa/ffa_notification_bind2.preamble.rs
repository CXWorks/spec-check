use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = Seq<int>;
pub type Int32 = i32;
pub type Bitmap = u64;
pub type Result = i32;
pub type FunctionId = u32;
pub type Instance = u8;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Result = -1;
pub const INVALID_PARAMETERS: Result = -2;
pub const DENIED: Result = -6;
pub const ABORTED: Result = -8;
pub const FFA_SUCCESS: Result = 0;

pub const FFA_NOTIFICATION_BIND2: FunctionId = 0x8400007F;

pub const current_instance: Instance = 1;

pub open spec fn IsImplementedAtInstance(s: S, func: FunctionId, inst: Instance) -> bool;
pub open spec fn ResultEqual(r1: Result, r2: Result) -> bool;
pub open spec fn IsValidEndpointId(s: S, id: UInt16) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn BitmapExceedsSupportedNotifications(s: S, bitmap: Bitmap) -> bool;
pub open spec fn IsNotificationBound(s: S, receiver_id: UInt16, i: int) -> bool;
pub open spec fn NotificationSender(s: S, receiver_id: UInt16, i: int) -> UInt16;
pub open spec fn IsNotificationPending(s: S, receiver_id: UInt16, i: int) -> bool;
pub open spec fn IsCallerAllowedToInvoke(s: S, func: FunctionId) -> bool;
pub open spec fn HasAborted(s: S, id: UInt16) -> bool;
pub open spec fn IsPerVcpuNotification(s: S, receiver_id: UInt16, i: int) -> bool;

} // verus!
