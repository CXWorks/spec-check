use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -6;
pub const NOT_SUPPORTED: i32 = -1;

pub struct S {
    pub endpoint_id: u16,
    pub vcpu_id: u16,
}

} // verus!
