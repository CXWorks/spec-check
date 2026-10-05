use vstd::prelude::*;
verus! {

pub type int64 = i64;
pub type UInt64 = u64;

pub const SDEI_SUCCESS: int64 = 0;
pub const SDEI_ERROR_NOT_SUPPORTED: int64 = -1;
pub const SDEI_ERROR_INVALID_PARAMETERS: int64 = -2;
pub const SDEI_ERROR_DENIED: int64 = -3;

pub const SDEI_EVENT_HANDLER_STATE_REGISTERED: UInt64 = 1;

pub struct S {
    pub event: UInt64,
    pub routing_mode: UInt64,
    pub affinity: UInt64,
}

impl S {
    pub open spec fn sdei_event_registered(&self, event: UInt64) -> bool;
    pub open spec fn sdei_event_is_shared(&self, event: UInt64) -> bool;
    pub open spec fn sdei_routing_mode_valid(&self, routing_mode: UInt64) -> bool;
    pub open spec fn sdei_affinity_valid(&self, routing_mode: UInt64, affinity: UInt64) -> bool;
    pub open spec fn sdei_event_handler_state(&self, event: UInt64) -> UInt64;
    pub open spec fn sdei_supported(&self) -> bool;
    pub open spec fn sdei_event_routing_mode(&self, event: UInt64) -> UInt64;
    pub open spec fn sdei_event_affinity(&self, event: UInt64) -> UInt64;
}

} // verus!
