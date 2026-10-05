use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type uint64 = u64;

pub struct S {
    pub cmd_input_clock_id: uint32,
    pub cmd_input_rate: uint64,
    pub cmd_input_flags: uint32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -3;
pub const BUSY: int32 = -6;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn flags_bits_31_4(s: S) -> int;

pub open spec fn clock_id_not_found(s: S, clock_id: uint32) -> bool;

pub open spec fn rate_not_supported(s: S, rate: uint64) -> bool;

pub open spec fn flags_invalid(s: S) -> bool;

pub open spec fn too_many_async_pending(s: S) -> bool;

pub open spec fn clock_rate_cannot_be_set(s: S) -> bool;

pub open spec fn clock_rate_set_succeeded(old_s: S, new_s: S) -> bool;

} // verus!
