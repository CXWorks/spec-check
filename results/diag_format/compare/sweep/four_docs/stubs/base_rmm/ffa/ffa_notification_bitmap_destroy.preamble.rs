use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt16 = u16;

pub struct S {
    pub vm_id: UInt16,
}

pub const FFA_SUCCESS: UInt32 = 0;
pub const INVALID_PARAMETERS: UInt32 = 1;
pub const NOT_SUPPORTED: UInt32 = 2;
pub const DENIED: UInt32 = 3;

pub const FFA_NOTIFICATION_BITMAP_DESTROY: UInt32 = 0x8400007E;

pub open spec fn IsRecognizedPartitionId(id: UInt16) -> bool;
pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn NotificationBitmapRegistered(s: S, id: UInt16) -> bool;
pub open spec fn NotificationBitmapMasked(s: S, id: UInt16) -> bool;
pub open spec fn NotificationBitmapPending(s: S, id: UInt16) -> bool;

} // verus!
