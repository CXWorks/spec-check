use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type NotificationId = u32;
pub type FfaFunctionId = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    NoMemory,
    Busy,
    Interrupted,
    Denied,
    Retry,
    Aborted,
    NoData,
}

pub enum FfaInstance {
    NonSecurePhysical,
    NonSecureVirtual,
    SecurePhysical,
    SecureVirtual,
}

pub struct S {
    pub dummy: u64,
}

pub spec const FFA_NOTIFICATION_BIND: FfaFunctionId = 0x8400007Fu32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;
pub open spec fn ResultEqual(r: Result<(), FfaStatusCode>, expected: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsValidSenderId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidReceiverId(s: S, id: UInt16) -> bool;
pub open spec fn Bits(value: UInt32, hi: int, lo: int) -> UInt32;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn BitmapBitSet(s: S, lo: UInt32, hi: UInt32, n: NotificationId) -> bool;
pub open spec fn IsBoundToOtherSender(s: S, receiver_id: UInt16, n: NotificationId, sender_id: UInt16) -> bool;
pub open spec fn IsNotificationPending(s: S, receiver_id: UInt16, n: NotificationId) -> bool;
pub open spec fn CallerAllowedToInvoke(s: S, func: FfaFunctionId) -> bool;
pub open spec fn PartitionHasAborted(s: S, id: UInt16) -> bool;
pub open spec fn IsBoundToSender(s: S, receiver_id: UInt16, n: NotificationId, sender_id: UInt16) -> bool;
pub open spec fn IsPerVcpuNotification(s: S, receiver_id: UInt16, n: NotificationId) -> bool;

} // verus!
