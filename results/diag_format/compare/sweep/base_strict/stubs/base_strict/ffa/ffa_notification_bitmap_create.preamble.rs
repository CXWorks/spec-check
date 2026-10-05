use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const NO_MEMORY: Int32 = -5;
pub const DENIED: Int32 = -6;
pub const FFA_SUCCESS: Int32 = 0;

pub const fid: UInt32 = 0x8400007D;
pub const vm_id: UInt64 = 1;
pub const allocated_count: UInt64 = 2;

pub open spec fn FunctionImplementedAtInstance(f: UInt32) -> bool;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;

pub open spec fn IsRecognizedVmId(id: UInt64) -> bool;

pub open spec fn NotificationBitmapExists(id: UInt64) -> bool;

pub open spec fn NotificationBitmapAllocatable(id: UInt64) -> bool;

pub open spec fn AllocatedSpNotificationCount(id: UInt64) -> UInt64;

pub open spec fn ResultEqual(result: Int32, code: Int32) -> bool;

} // verus!
