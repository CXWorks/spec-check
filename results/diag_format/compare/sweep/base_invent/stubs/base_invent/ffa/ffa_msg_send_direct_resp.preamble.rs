use vstd::prelude::*;

verus! {

pub type int32 = i32;
pub type uint32 = u32;

pub const INVALID_PARAMETERS: int32 = -2;
pub const DENIED: int32 = -6;
pub const NOT_SUPPORTED: int32 = -1;
pub const ABORTED: int32 = -8;

pub struct S {
    pub source_endpoint_id: uint32,
    pub dest_endpoint_id: uint32,
    pub flags: uint32,
}

} // verus!
