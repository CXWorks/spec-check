use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt = u64;

pub struct S {
    pub supervisor_xlen: u64,
}

pub open spec fn SupervisorXlen(s: S) -> u64;

pub open spec fn CppcRegister(s: S, cppc_reg_id: UInt32) -> UInt;

} // verus!
