use vstd::prelude::*;

verus! {

pub type int64 = i64;

pub struct S {
    pub dummy: u64,
}

pub const SDEI_ERROR_NOT_SUPPORTED: int64 = -1;

pub open spec fn ResultEqual(a: int64, b: int64) -> bool;

} // verus!
