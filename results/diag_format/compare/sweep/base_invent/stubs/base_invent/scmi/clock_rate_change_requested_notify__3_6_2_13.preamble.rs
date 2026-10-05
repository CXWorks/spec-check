use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;

pub const clock_id: uint32 = 7;
pub const notify_enable: uint32 = 1;
pub const reserved_bits: uint32 = 2;

pub open spec fn clock_id_invalid(old_s: S, clock_id_arg: uint32) -> bool;

} // verus!
