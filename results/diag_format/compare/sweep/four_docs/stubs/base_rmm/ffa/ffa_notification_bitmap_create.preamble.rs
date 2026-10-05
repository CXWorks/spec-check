use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct Instance {
    pub id: nat,
}

pub struct S {
    pub instance: Instance,
}

pub struct NotificationBitmapT {
    pub raw: nat,
}

pub spec const FFA_SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = (-1int) as i32;
pub spec const INVALID_PARAMETERS: Int32 = (-2int) as i32;
pub spec const NO_MEMORY: Int32 = (-3int) as i32;
pub spec const DENIED: Int32 = (-6int) as i32;

pub spec const FFA_NOTIFICATION_BITMAP_CREATE: UInt32 = 0x8400007D;

pub open spec fn IsRecognizedVmId(vm_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsImplementedAtInstance(func_id: UInt32, instance: Instance) -> bool;

pub open spec fn NotificationBitmapExists(vm_id: UInt32, s: S) -> bool;

pub open spec fn CanAllocateNotificationBitmap(vm_id: UInt32, s: S) -> bool;

pub open spec fn NotificationBitmap(vm_id: UInt32, s: S) -> NotificationBitmapT;

pub open spec fn TotalNotificationCount(bitmap: NotificationBitmapT) -> int;

} // verus!
