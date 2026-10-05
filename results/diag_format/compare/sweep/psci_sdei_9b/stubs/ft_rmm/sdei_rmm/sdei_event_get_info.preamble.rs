use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
    pub registered: Map<Int32, bool>,
    pub shared: Map<Int32, bool>,
    pub critical: Map<Int32, bool>,
    pub signalable: Map<Int32, bool>,
    pub routing_mode: Map<Int32, Int64>,
    pub routing_aff: Map<Int32, Int64>,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const EV_TYPE: int = 0;
pub const EV_SIGNALED: int = 1;
pub const EV_PRIORITY: int = 2;
pub const EV_ROUTING_MODE: int = 3;
pub const EV_ROUTING_AFF: int = 4;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn IsValidInfo(s: S, info: UInt32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsEventRegistered(s: S, event: Int32) -> bool;

pub open spec fn RoutingModeHasAffinity(s: S, mode: Int64) -> bool;

pub open spec fn EventRoutingMode(s: S, event: Int32) -> Int64;

pub open spec fn EventRoutingAffinity(s: S, event: Int32) -> Int64;

pub open spec fn CanBeSoftwareSignaled(s: S, event: Int32) -> bool;

pub open spec fn IsCriticalPriority(s: S, event: Int32) -> bool;

} // verus!
