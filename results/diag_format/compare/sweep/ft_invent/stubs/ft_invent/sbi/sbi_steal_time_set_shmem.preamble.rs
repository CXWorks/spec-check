use vstd::prelude::*;
verus! {

pub type UInt64 = int;

pub type SbiStatusCode = i64;

pub const SBI_SUCCESS: SbiStatusCode = 0;
pub const SBI_ERROR_INVALID_ARGS: SbiStatusCode = -3;

pub struct S {
    pub dummy: int,
}

} // verus!
