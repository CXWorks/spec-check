use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt64 = u64;
pub type FfaReturn = i32;
pub type FfaFunctionId = u32;
pub type InstanceId = u32;

pub struct S {
    pub dummy: int,
}

pub const FFA_NOTIFICATION_BIND2: FfaFunctionId = 0x8400007F;

pub const current_instance: InstanceId = 0;

pub const FFA_SUCCESS: FfaReturn = 0;
pub const NOT_SUPPORTED: FfaReturn = -1;
pub const INVALID_PARAMETERS: FfaReturn = -2;
pub const DENIED: FfaReturn = -6;
pub const ABORTED: FfaReturn = -8;

pub open spec fn IsImplementedAtInstance(func: FfaFunctionId, instance: InstanceId) -> bool;
pub open spec fn ResultEqual(result: FfaReturn, code: FfaReturn) -> bool;
pub open spec fn IsValidEndpointId(id: UInt16) -> bool;
pub open spec fn PerVcpuNotificationsSupported() -> bool;
pub open spec fn BitmapExceedsSupportedNotifications(bitmap: UInt64) -> bool;
pub open spec fn IsNotificationBound(receiver_id: UInt16, i: int) -> bool;
pub open spec fn NotificationSender(s: S, receiver_id: UInt16, i: int) -> UInt16;
pub open spec fn IsNotificationPending(receiver_id: UInt16, i: int) -> bool;
pub open spec fn IsCallerAllowedToInvoke(func: FfaFunctionId) -> bool;
pub open spec fn HasAborted(id: UInt16) -> bool;
pub open spec fn IsPerVcpuNotification(s: S, receiver_id: UInt16, i: int) -> bool;

} // verus!
