use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type uint32 = u32;
pub type array<T> = Vec<T>;

pub struct S {
    pub dummy: int,
}

pub spec const clock_id: uint32 = 7;

} // verus!
