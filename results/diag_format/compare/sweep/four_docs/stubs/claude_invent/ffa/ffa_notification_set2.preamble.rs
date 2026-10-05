use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub type FfaReturnCode = int;

pub spec const FFA_SUCCESS: int = 0;
pub spec const NOT_SUPPORTED: int = -1;
pub spec const INVALID_PARAMETERS: int = -2;
pub spec const DENIED: int = -3;
pub spec const ABORTED: int = -8;

pub struct S {
    pub dummy: int,
}

pub uninterp spec fn FfaNotificationSet2ImplementedAtInstance(s: S) -> bool;
pub uninterp spec fn IsRecognizedPartitionId(s: S, id: int) -> bool;
pub uninterp spec fn NotificationBitmapContainsPerVcpuNotification(s: S, receiver: int, b0: UInt64, b1: UInt64, b2: UInt64, b3: UInt64, b4: UInt64, b5: UInt64) -> bool;
pub uninterp spec fn NotificationBitmapContainsGlobalNotification(s: S, receiver: int, b0: UInt64, b1: UInt64, b2: UInt64, b3: UInt64, b4: UInt64, b5: UInt64) -> bool;
pub uninterp spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub uninterp spec fn NotificationBitmapExceedsSupportedNotifications(s: S, b0: UInt64, b1: UInt64, b2: UInt64, b3: UInt64, b4: UInt64, b5: UInt64) -> bool;
pub uninterp spec fn SenderPermittedToSignalNotifications(s: S, sender: int, receiver: int, b0: UInt64, b1: UInt64, b2: UInt64, b3: UInt64, b4: UInt64, b5: UInt64) -> bool;
pub uninterp spec fn ReceiverSupportsNotifications(s: S, receiver: int) -> bool;
pub uninterp spec fn ReceiverPartitionAborted(s: S, receiver: int) -> bool;
pub uninterp spec fn NotificationsSignaledToReceiver(old_s: S, new_s: S, sender: int, receiver: int, per_vcpu: bool, receiver_vcpu_id: int, flags: UInt64, b0: UInt64, b1: UInt64, b2: UInt64, b3: UInt64, b4: UInt64, b5: UInt64) -> bool;

} // verus!
