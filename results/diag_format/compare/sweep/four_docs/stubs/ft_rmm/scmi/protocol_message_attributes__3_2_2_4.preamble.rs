use vstd::prelude::*;
verus! {

pub type uint32 = u32;
pub type int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: int32 = 0;
pub const NOT_FOUND: int32 = -4;
#[allow(non_upper_case_globals)]
pub const result: int32 = 1;

pub open spec fn IsMessageImplemented(s: S, message_id: uint32) -> bool;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

} // verus!
