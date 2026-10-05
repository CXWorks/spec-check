use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type FfaReturnCode = u32;

pub const FFA_SUCCESS64: FfaReturnCode = 0;
pub const NOT_SUPPORTED: FfaReturnCode = 1;
pub const DENIED: FfaReturnCode = 2;
pub const INVALID_PARAMETERS: FfaReturnCode = 3;

pub struct S {
    pub dummy: u64,
}

pub open spec fn IsNonSecurePhysicalInstance(s: S) -> bool;

pub open spec fn NotificationMaskExceedsSupported(s: S, mask: Seq<UInt64>) -> bool;

pub open spec fn IsEmptyNotificationBitmapSpecified(s: S, endpoint_id: UInt32, vcpu_id: UInt32, flags: UInt64) -> bool;

pub open spec fn IsValidPartitionId(s: S, endpoint_id: UInt32) -> bool;

pub open spec fn IsFfaNotificationGet2Implemented(s: S) -> bool;

pub open spec fn IsCallerAllowedFfaNotificationGet2(s: S) -> bool;

pub open spec fn SpNotificationBitmap(s: S, endpoint_id: UInt32, vcpu_id: UInt32, i: int) -> UInt64;

pub open spec fn VmNotificationBitmap(s: S, endpoint_id: UInt32, vcpu_id: UInt32, i: int) -> UInt64;

pub open spec fn SpmFrameworkNotificationBitmap(s: S, endpoint_id: UInt32, vcpu_id: UInt32) -> UInt64;

pub open spec fn HypervisorFrameworkNotificationBitmap(s: S, endpoint_id: UInt32, vcpu_id: UInt32) -> UInt64;

} // verus!
