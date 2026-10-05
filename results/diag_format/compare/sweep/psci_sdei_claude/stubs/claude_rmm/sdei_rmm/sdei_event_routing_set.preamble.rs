use vstd::prelude::*;
verus! {

pub type Int64 = i64;
pub type Int32 = i32;
pub type UInt64 = u64;
pub type UInt32 = u32;

pub type RoutingModeValue = u64;
pub type HandlerStateValue = u32;

pub const SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub const RM_ANY: RoutingModeValue = 0;
pub const RM_PE: RoutingModeValue = 1;

pub const HANDLER_UNREGISTERED: HandlerStateValue = 0;
pub const HANDLER_REGISTERED: HandlerStateValue = 1;

pub struct SdeiEvent {
    pub handler_state: HandlerStateValue,
    pub routing_mode: RoutingModeValue,
    pub affinity: UInt64,
}

pub struct S {
    pub events: Map<Int32, SdeiEvent>,
    pub supported: bool,
}

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsValidEventNumber(event: Int32) -> bool;

pub open spec fn IsSharedEvent(s: S, event: Int32) -> bool;

pub open spec fn IsValidRoutingMode(routing_mode: UInt64) -> bool;

pub open spec fn RoutingMode(routing_mode: UInt64) -> RoutingModeValue;

pub open spec fn IsValidMpidr(s: S, affinity: UInt64) -> bool;

pub open spec fn EventAt(s: S, event: Int32) -> SdeiEvent;

} // verus!
