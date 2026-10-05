use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub sensor_states: Map<uint32, uint32>,
    pub async_queue: Seq<uint32>,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const PROTOCOL_ERROR: int32 = -3;

pub const SENSOR_DISABLED: uint32 = 0;
pub const SENSOR_ENABLED: uint32 = 1;

#[allow(non_upper_case_globals)]
pub const sensor_id: uint32 = 7;
#[allow(non_upper_case_globals)]
pub const flags: uint32 = 3;

pub uninterp spec fn sensor_state(s: S, id: uint32) -> uint32;

pub uninterp spec fn sensor_id_not_found(old_s: S, sid: uint32) -> bool;

pub uninterp spec fn flags_invalid(old_s: S, f: uint32) -> bool;

pub uninterp spec fn sensor_disabled(old_s: S, sid: uint32) -> bool;

pub uninterp spec fn sensor_enabled(old_s: S, sid: uint32) -> bool;

pub uninterp spec fn sensor_enqueued_for_async(old_s: S, sid: uint32) -> bool;

pub uninterp spec fn sensor_reading_valid(old_s: S, sid: uint32) -> bool;

} // verus!
