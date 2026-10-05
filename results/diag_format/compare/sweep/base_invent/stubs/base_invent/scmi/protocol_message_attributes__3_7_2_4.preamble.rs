use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub struct S {
    pub dummy: u32,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;

pub const message_id: uint32 = 0;

pub open spec fn message_id_not_supported(old_s: S, message_id: uint32) -> bool;

} // verus!
