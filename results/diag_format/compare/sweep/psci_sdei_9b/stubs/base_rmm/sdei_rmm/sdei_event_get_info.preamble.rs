use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const EV_TYPE: UInt32 = 0;
pub const EV_SIGNALED: UInt32 = 1;
pub const EV_PRIORITY: UInt32 = 2;
pub const EV_ROUTING_MODE: UInt32 = 3;
pub const EV_ROUTING_AFF: UInt32 = 4;

pub open spec fn SdeiIsSupported() -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn IsValidEventNumber(event: Int32) -> bool;

pub open spec fn IsValidInfo(info: UInt32) -> bool;

pub open spec fn IsSharedEvent(event: Int32) -> bool;

pub open spec fn IsEventRegistered(event: Int32) -> bool;

pub open spec fn EventRoutingMode(event: Int32) -> Int64;

pub open spec fn RoutingModeHasAffinity(mode: Int64) -> bool;

pub open spec fn EventRoutingAffinity(event: Int32) -> Int64;

pub open spec fn CanBeSoftwareSignaled(event: Int32) -> bool;

pub open spec fn IsCriticalPriority(event: Int32) -> bool;

} // verus!
