use vstd::prelude::*;

verus! {

pub type UInt = u64;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;

pub struct S {
    pub cppc_reg_id: UInt,
}

pub open spec fn SupervisorXlen(s: S) -> UInt;

pub open spec fn CppcRegister(reg_id: UInt) -> UInt;

} // verus!
