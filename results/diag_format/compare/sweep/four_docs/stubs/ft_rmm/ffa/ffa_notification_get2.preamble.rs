use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int32 = i32;
pub type BitmapId = u8;

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

pub struct S {
    pub dummy: int,
}

pub spec const FFA_NOTIFICATION_GET2: UInt32 = 0x84000082u32;

pub spec const NOT_SUPPORTED: FfaStatusCode = FfaStatusCode::NotSupported;
pub spec const DENIED: FfaStatusCode = FfaStatusCode::Denied;
pub spec const INVALID_PARAMETERS: FfaStatusCode = FfaStatusCode::InvalidParameters;

pub spec const FFA_SUCCESS64: Result<(), FfaStatusCode> = Ok(());

pub spec const NON_SECURE_PHYSICAL: UInt64 = 1u64;

pub spec const SP_BITMAP: BitmapId = 0u8;
pub spec const VM_BITMAP: BitmapId = 1u8;
pub spec const SPMC_FW_BITMAP: BitmapId = 2u8;
pub spec const HYP_FW_BITMAP: BitmapId = 3u8;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32, instance: int) -> bool;

pub open spec fn IsCallerAllowedToInvoke(s: S, func_id: UInt32) -> bool;

pub open spec fn IsRecognizedPartitionId(s: S, id: UInt16) -> bool;

pub open spec fn ResultEqual(result: Result<(), FfaStatusCode>, code: FfaStatusCode) -> bool;

pub open spec fn ExceedsSupportedNotifications<T>(s: S, mask: T) -> bool;

pub open spec fn IsEmptyNotificationBitmapSpecified(s: S, flags: UInt64) -> bool;

pub open spec fn IsPending(s: S, endpoint_id: UInt16, vcpu_id: UInt16, bitmap: BitmapId, i: int) -> UInt64;

pub open spec fn NotificationState(s: S, endpoint_id: UInt16, vcpu_id: UInt16, bitmap: BitmapId, i: int) -> UInt64;

} // verus!
