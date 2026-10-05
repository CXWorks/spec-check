use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaInstance = u32;
pub type FfaConduit = u32;
pub type FfaStatus = i32;

pub const FFA_INSTANCE_NS_VIRTUAL: FfaInstance = 0u32;
pub const FFA_INSTANCE_S_VIRTUAL: FfaInstance = 1u32;
pub const FFA_INSTANCE_NS_PHYSICAL: FfaInstance = 2u32;
pub const FFA_INSTANCE_S_PHYSICAL: FfaInstance = 3u32;

pub const FFA_CONDUIT_SMC: FfaConduit = 0u32;
pub const FFA_CONDUIT_HVC: FfaConduit = 1u32;
pub const FFA_CONDUIT_SVC: FfaConduit = 2u32;
pub const FFA_CONDUIT_ERET: FfaConduit = 3u32;

pub const FFA_SUCCESS: FfaStatus = 0i32;
pub const NOT_SUPPORTED: FfaStatus = -1i32;
pub const INVALID_PARAMETERS: FfaStatus = -2i32;
pub const DENIED: FfaStatus = -3i32;
pub const ABORTED: FfaStatus = -4i32;

pub struct S {
    pub dummy: int,
}

pub open spec fn NotificationSetImplemented(s: S, instance: FfaInstance) -> bool;
pub open spec fn PartitionIdRecognized(s: S, instance: FfaInstance, id: UInt32) -> bool;
pub open spec fn BitmapHasPerVcpuNotification(s: S, sender: UInt32, receiver: UInt32, bitmap: UInt64) -> bool;
pub open spec fn BitmapHasGlobalNotification(s: S, sender: UInt32, receiver: UInt32, bitmap: UInt64) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S, receiver: UInt32) -> bool;
pub open spec fn SenderPermittedToSignal(s: S, sender: UInt32, receiver: UInt32, bitmap: UInt64) -> bool;
pub open spec fn ReceiverSupportsNotifications(s: S, receiver: UInt32) -> bool;
pub open spec fn ReceiverAborted(s: S, receiver: UInt32) -> bool;
pub open spec fn NotificationsSignaled(old_s: S, new_s: S, sender: UInt32, receiver: UInt32, bitmap: UInt64, per_vcpu: bool, vcpu_id: UInt32) -> bool;

} // verus!
