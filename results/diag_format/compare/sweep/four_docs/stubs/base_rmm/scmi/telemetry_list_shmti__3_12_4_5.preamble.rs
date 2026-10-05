use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Array<T> = Seq<T>;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_SUPPORTED: Int32 = 1;
pub const NOT_FOUND: Int32 = 2;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsTelemetryListShmtiSupported() -> bool;
pub open spec fn AnyShmtiAvailable() -> bool;
pub open spec fn NumShmtiDescriptorsReturned() -> UInt16;
pub open spec fn NumShmtiRemaining(index: UInt32, num_low: UInt16) -> UInt16;
pub open spec fn ShmtiDescriptor(index: UInt32) -> UInt32;
pub open spec fn IsInCallerMemoryMap(addr: UInt64) -> bool;

} // verus!
