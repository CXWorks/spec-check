use vstd::prelude::*;
verus! {

pub type int64 = i64;

pub struct S {
    pub dummy: int,
}

pub spec const SDEI_ERROR_NOT_SUPPORTED: int64 = (-1int) as i64;
pub spec const SDEI_ERROR_INVALID_PARAMETERS: int64 = (-2int) as i64;

} // verus!
