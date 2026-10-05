use vstd::prelude::*;

verus! {

pub struct S {
    pub dummy: u64,
}

pub const XLEN: u64 = 64;

pub open spec fn CallSucceeded(error: i64) -> bool;

pub open spec fn Bits(value: u64, hi: int, lo: int) -> u64;

pub open spec fn SbiSpecMinorVersion() -> u64;

pub open spec fn SbiSpecMajorVersion() -> u64;

} // verus!
