use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt = u64;
pub type UInt48 = u64;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type FfaFunctionId = u32;
pub type FfaInstance = u32;
pub type FfaFlags = u32;

pub enum FfaStatusCode {
    NotSupported,
    InvalidParameters,
    Denied,
    Aborted,
    Busy,
    Retry,
}

pub struct S {
    pub dummy: int,
}

pub spec const FFA_NOTIFICATION_SET2: FfaFunctionId = 0x84000081u32;
pub spec const ffa_instance: FfaInstance = 0u32;
pub spec const flags: FfaFlags = 0u32;

pub spec const FFA_SUCCESS: Result<(), FfaStatusCode> = Ok(());
pub spec const NOT_SUPPORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<(), FfaStatusCode> = Err(FfaStatusCode::InvalidParameters);
pub spec const DENIED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Denied);
pub spec const ABORTED: Result<(), FfaStatusCode> = Err(FfaStatusCode::Aborted);

pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, inst: FfaInstance) -> bool;
pub open spec fn ResultEqual(result: Result<(), FfaStatusCode>, expected: Result<(), FfaStatusCode>) -> bool;
pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;
pub open spec fn IsValidFlags(s: S, f: FfaFlags) -> bool;
pub open spec fn BitmapContainsPerVcpuNotification(s: S, receiver_id: UInt16, bitmap: [UInt64; 6]) -> bool;
pub open spec fn BitmapContainsGlobalNotification(s: S, receiver_id: UInt16, bitmap: [UInt64; 6]) -> bool;
pub open spec fn PerVcpuNotificationsSupported(s: S) -> bool;
pub open spec fn BitmapExceedsSupportedNotifications(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn IsEmptyBitmap(s: S, bitmap: [UInt64; 6]) -> bool;
pub open spec fn SenderPermittedToSignalAll(s: S, sender_id: UInt16, receiver_id: UInt16, bitmap: [UInt64; 6]) -> bool;
pub open spec fn SupportsNotificationReceipt(s: S, receiver_id: UInt16) -> bool;
pub open spec fn HasAborted(s: S, receiver_id: UInt16) -> bool;
pub open spec fn NotificationSignaled(s: S, receiver_id: UInt16, i: int) -> bool;

} // verus!
