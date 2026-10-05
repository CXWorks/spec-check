use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Int32 = 0;
pub spec const NOT_FOUND: Int32 = (-2int) as i32;

#[allow(non_upper_case_globals)]
pub spec const message_id: UInt32 = 0;

pub uninterp spec fn IsMessageImplemented(s: S, msg_id: UInt32) -> bool;

pub uninterp spec fn ResultEqual(a: Int32, b: Int32) -> bool;

} // verus!
