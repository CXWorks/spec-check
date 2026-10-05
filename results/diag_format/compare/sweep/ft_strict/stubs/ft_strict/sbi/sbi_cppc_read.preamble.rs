use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: int,
}

pub open spec fn SupervisorXlen(s: S) -> int;

pub open spec fn CppcRegisterValue(s: S, reg_id: UInt32) -> UInt64;

pub open spec fn Bits(x: UInt64, hi: int, lo: int) -> UInt64;

} // verus!
