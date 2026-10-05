use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub struct S {
    pub dummy: u64,
}

pub struct SdeiEvent {
    pub handler_state: UInt32,
    pub routing_mode: UInt64,
    pub affinity: UInt64,
}

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const HANDLER_UNREGISTERED: UInt32 = 0;
pub const HANDLER_REGISTERED: UInt32 = 1;

pub const RM_ANY: UInt64 = 100;
pub const RM_PE: UInt64 = 101;

#[allow(non_upper_case_globals)]
pub const event: UInt32 = 7;
#[allow(non_upper_case_globals)]
pub const routing_mode: UInt64 = 11;
#[allow(non_upper_case_globals)]
pub const affinity: UInt64 = 13;

pub open spec fn SdeiIsSupported() -> bool;
pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;
pub open spec fn IsValidEventNumber(ev: UInt32) -> bool;
pub open spec fn IsSharedEvent(ev: UInt32) -> bool;
pub open spec fn IsValidRoutingMode(rm: UInt64) -> bool;
pub open spec fn RoutingMode(rm: UInt64) -> UInt64;
pub open spec fn IsValidMpidr(mpidr: UInt64) -> bool;
pub open spec fn EventAt(ev: UInt32) -> SdeiEvent;

} // verus!
