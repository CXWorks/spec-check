use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const INVALID_PARAMETERS: UInt32 = 0xFFFFFFFE;
pub const NOT_SUPPORTED: UInt32 = 0xFFFFFFFF;
pub const DENIED: UInt32 = 0xFFFFFFF9;
pub const FFA_NOTIFICATION_BITMAP_DESTROY: UInt32 = 0x8400007E;

pub open spec fn IsRecognizedPartitionId(s: S, vm_id: UInt16) -> bool;
pub open spec fn ResultEqual(result: UInt32, expected: UInt32) -> bool;
pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;
pub open spec fn NotificationBitmapRegistered(s: S, vm_id: UInt16) -> bool;
pub open spec fn NotificationBitmapMasked(s: S, vm_id: UInt16) -> bool;
pub open spec fn NotificationBitmapPending(s: S, vm_id: UInt16) -> bool;

} // verus!
