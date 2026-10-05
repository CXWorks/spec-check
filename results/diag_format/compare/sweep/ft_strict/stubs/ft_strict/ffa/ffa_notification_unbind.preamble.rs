use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type NotificationId = u32;
pub type FfaFunctionId = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Aborted,
    Other,
}

pub enum FfaInstance {
    Secure,
    NonSecure,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_NOTIFICATION_UNBIND: FfaFunctionId = 0x84000080u32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: FfaInstance) -> bool;
pub open spec fn CurrentFfaInstance(s: S) -> FfaInstance;
pub open spec fn ResultEqual(r: Result<(), FfaStatusCode>, expected: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsValidPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> UInt16;
pub open spec fn IsValidNotificationBitmap(s: S, lo: UInt32, hi: UInt32) -> bool;
pub open spec fn IsCallerAllowedToInvoke(s: S, func: FfaFunctionId) -> bool;
pub open spec fn IsBitSet(s: S, bitmap: UInt64, n: NotificationId) -> bool;
pub open spec fn Bitmap64(hi: UInt32, lo: UInt32) -> UInt64;
pub open spec fn IsNotificationBound(s: S, receiver: UInt16, n: NotificationId) -> bool;
pub open spec fn BoundSender(s: S, receiver: UInt16, n: NotificationId) -> UInt16;
pub open spec fn IsNotificationPending(s: S, receiver: UInt16, n: NotificationId) -> bool;
pub open spec fn HasPartitionAborted(s: S, id: UInt16) -> bool;
pub open spec fn CanSenderSignal(s: S, sender: UInt16, receiver: UInt16, n: NotificationId) -> bool;
pub open spec fn NotificationBinding(s: S, receiver: UInt16, n: NotificationId) -> Option<UInt16>;
pub open spec fn PreNotificationBinding(s: S, receiver: UInt16, n: NotificationId) -> Option<UInt16>;

} // verus!
