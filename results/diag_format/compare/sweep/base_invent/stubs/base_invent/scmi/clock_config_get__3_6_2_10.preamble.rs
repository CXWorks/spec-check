use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub attributes: uint32,
    pub config: uint32,
    pub extended_config_val: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;

#[allow(non_upper_case_globals)]
pub const clock_id: uint32 = 1;
#[allow(non_upper_case_globals)]
pub const flags: uint32 = 2;
#[allow(non_upper_case_globals)]
pub const extended_config_type: uint32 = 3;

pub open spec fn clock_id_not_found(old_s: S, clock_id_arg: uint32) -> bool;

pub open spec fn flags_nonzero(old_s: S, flags_arg: uint32) -> bool;

pub open spec fn extended_config_type_unused(old_s: S, extended_config_type_arg: uint32) -> bool;

} // verus!
