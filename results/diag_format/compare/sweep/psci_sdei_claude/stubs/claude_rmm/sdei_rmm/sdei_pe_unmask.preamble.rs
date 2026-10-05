use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type Int64 = i64;

pub struct S {
    pub dummy: int,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const SUCCESS: Int64 = 0;

pub open spec fn SdeiIsSupported(s: S) -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn PeIsMasked(s: S, client: UInt64, pe: UInt64) -> bool;
pub open spec fn CallingPe(s: S) -> UInt64;
pub open spec fn PendingEventsDispatched(s: S, pe: UInt64) -> bool;

} // verus!
