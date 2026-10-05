use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = 1;
pub const NOT_SUPPORTED: int32 = 2;
pub const INVALID_PARAMETERS: int32 = 3;

#[allow(non_upper_case_globals)]
pub const sensor_id: uint32 = 7;

#[allow(non_upper_case_globals)]
pub const notify_enable: uint32 = 1;

pub open spec fn sensor_id_is_valid(old_s: S, sid: uint32) -> bool;

pub open spec fn sensor_id_is_invalid(old_s: S, sid: uint32) -> bool;

pub open spec fn sensor_supports_continuous_update(old_s: S, sid: uint32) -> bool;

pub open spec fn notify_enable_is_invalid(old_s: S, ne: uint32) -> bool;

pub open spec fn notify_enable_is_valid(old_s: S, ne: uint32) -> bool;

} // verus!
