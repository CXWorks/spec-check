use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Aborted,
    Busy,
    NoMemory,
    Retry,
}

pub struct S {
    pub version: u32,
    pub caller_id: u16,
}

pub spec const FFA_NOTIFICATION_UNBIND: u32 = 0x84000080u32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn IsImplementedAtThisInstance(s: S, func_id: u32) -> bool;
pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidNotificationBitmap(s: S, bitmap_hi: UInt32, bitmap_lo: UInt32) -> bool;
pub open spec fn AnyNotificationBoundToOtherSender(s: S, receiver_id: UInt16, bitmap_hi: UInt32, bitmap_lo: UInt32, sender_id: UInt16) -> bool;
pub open spec fn AnyNotificationPending(s: S, receiver_id: UInt16, bitmap_hi: UInt32, bitmap_lo: UInt32) -> bool;
pub open spec fn CallerAllowedToInvoke(s: S, func_id: u32) -> bool;
pub open spec fn SenderPartitionAborted(s: S, sender_id: UInt16) -> bool;
pub open spec fn CanSenderSignal(s: S, receiver_id: UInt16, sender_id: UInt16, notification_id: u32) -> bool;
pub open spec fn ResultEqual(r: Result<(), FfaStatusCode>, expected: Result<(), FfaStatusCode>) -> bool;

} // verus!
