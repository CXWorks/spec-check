use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub implemented_messages: Set<UInt32>,
}

pub const SUCCESS: Int32 = 0;
pub const NOT_FOUND: Int32 = -4;
#[allow(non_upper_case_globals)]
pub const result: Int32 = 1;

pub open spec fn IsMessageImplemented(s: S, message_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
