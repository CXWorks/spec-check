use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const NOT_FOUND: i32 = -3;

pub open spec fn PowerDomainMessageImplementedAndAvailable(s: S, message_id: u32) -> bool;

} // verus!
