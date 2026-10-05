use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type UInt32 = u32;

pub const SUCCESS: Int32 = 0;

pub struct S {
    pub dummy: int,
}

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits(x: UInt32, hi: int, lo: int) -> int;

pub open spec fn NumVoltageDomains() -> int;

} // verus!
