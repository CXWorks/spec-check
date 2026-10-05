use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;
pub type Array<T> = T;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_SUPPORTED: Int32 = 1;
pub spec const NOT_FOUND: Int32 = 2;
pub spec const result: Int32 = 3;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;
pub open spec fn IsTelemetryListShmtiSupported(s: S) -> bool;
pub open spec fn AnyShmtiAvailable(s: S) -> bool;
pub open spec fn NumShmtiDescriptorsReturned(s: S) -> UInt16;
pub open spec fn NumShmtiRemaining(s: S, index: UInt32, returned: UInt16) -> UInt16;
pub open spec fn ShmtiDescriptor(s: S, index: UInt32) -> UInt32;
pub open spec fn IsInCallerMemoryMap(addr: UInt32) -> bool;

} // verus!
