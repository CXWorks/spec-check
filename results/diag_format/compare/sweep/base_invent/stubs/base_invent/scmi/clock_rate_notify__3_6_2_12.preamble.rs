use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;

pub const clock_id: uint32 = 1;
pub const notify_enable: uint32 = 0;

pub open spec fn clock_id_is_invalid(old_s: S, clock_id_arg: uint32) -> bool;

pub open spec fn reserved_bits_are_zero(old_s: S, notify_enable_arg: uint32) -> bool;

} // verus!
