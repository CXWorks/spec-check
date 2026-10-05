use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct S {
    pub direct_request_processing: bool,
}

pub const RETRY: int32 = -7;
pub const DENIED: int32 = -6;
pub const NOT_SUPPORTED: int32 = -1;

pub open spec fn IsDirectRequestProcessing(s: S) -> bool;

} // verus!
