use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum RsiCommandReturnCode {
    RsiError,
    RsiIncomplete,
    RsiErrorInput,
    RsiErrorState,
}

pub spec const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub struct S {
    pub dummy: u64,
}

} // verus!
