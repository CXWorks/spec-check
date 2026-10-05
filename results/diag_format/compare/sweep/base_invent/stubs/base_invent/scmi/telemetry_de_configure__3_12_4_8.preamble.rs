use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type UInt64 = u64;

pub struct S {
    pub flags: u64,
    pub identifier_is_de: bool,
    pub de_is_enabled: bool,
    pub identifier_is_event_group: bool,
    pub event_group_has_enabled_de: bool,
    pub total_enabled_count: u64,
    pub max_enabled_count: u64,
}

pub const SUCCESS: int32 = 0;
pub const INVALID_PARAMETERS: int32 = -2;
pub const IN_USE: int32 = -3;
pub const OUT_OF_RANGE: int32 = -4;
pub const NOT_SUPPORTED: int32 = -1;
pub const DENIED: int32 = -5;

} // verus!
