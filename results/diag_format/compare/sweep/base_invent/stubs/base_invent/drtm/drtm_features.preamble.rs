use vstd::prelude::*;
verus! {

pub type int64 = i64;

pub struct S {
    pub drtm_function_id: u64,
    pub drtm_function_id_bit63: u64,
    pub drtm_function_id_bit62_32: u64,
    pub drtm_function_id_bit62_8: u64,
}

pub const NOT_SUPPORTED: int64 = -1;

} // verus!
