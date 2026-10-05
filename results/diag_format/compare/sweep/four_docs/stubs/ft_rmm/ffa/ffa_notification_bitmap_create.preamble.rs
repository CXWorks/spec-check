use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type FfaCommandReturnCode = u32;
pub type FfaFunctionId = u32;
pub type NotificationBitmapT = u64;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: FfaCommandReturnCode = 0;
pub const INVALID_PARAMETERS: FfaCommandReturnCode = 1;
pub const NOT_SUPPORTED: FfaCommandReturnCode = 2;
pub const DENIED: FfaCommandReturnCode = 3;
pub const NO_MEMORY: FfaCommandReturnCode = 4;

pub const FFA_NOTIFICATION_BITMAP_CREATE: FfaFunctionId = 0x8400007D;

pub open spec fn IsRecognizedVmId(s: S, vm_id: UInt32) -> bool;
pub open spec fn ResultEqual(result: FfaCommandReturnCode, code: FfaCommandReturnCode) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func: FfaFunctionId, instance: int) -> bool;
pub open spec fn NotificationBitmapExists(s: S, vm_id: UInt32) -> bool;
pub open spec fn CanAllocateNotificationBitmap(s: S, vm_id: UInt32) -> bool;
pub open spec fn NotificationBitmap(s: S, vm_id: UInt32) -> NotificationBitmapT;
pub open spec fn TotalNotificationCount(s: S, bitmap: NotificationBitmapT) -> int;

} // verus!
