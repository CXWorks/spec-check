use vstd::prelude::*;
verus! {

pub type UInt = u64;

pub type Int = i64;

pub const XLEN: u64 = 64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn CallSucceeded(error: Int) -> bool;

pub open spec fn Bits(value: UInt, hi: int, lo: int) -> UInt;

pub open spec fn SbiSpecMinorVersion() -> UInt;

pub open spec fn SbiSpecMajorVersion() -> UInt;

} // verus!
