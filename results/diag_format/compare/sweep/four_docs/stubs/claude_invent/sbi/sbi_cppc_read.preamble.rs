use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub open spec fn SupervisorXlen(s: S) -> u64;

pub open spec fn CppcRegisterValue(s: S, cppc_reg_id: UInt32) -> u64;

} // verus!
