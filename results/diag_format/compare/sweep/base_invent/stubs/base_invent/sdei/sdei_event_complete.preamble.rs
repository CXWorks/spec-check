use vstd::prelude::*;
verus! {

pub type int64 = i64;

pub struct S {
    pub pe_handler_running: bool,
}

pub const SDEI_ERROR_NOT_SUPPORTED: i64 = -1;
pub const SDEI_ERROR_DENIED: i64 = -3;

} // verus!
