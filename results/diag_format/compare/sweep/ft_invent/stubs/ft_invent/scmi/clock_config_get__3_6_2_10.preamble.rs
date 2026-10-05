use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;

pub struct S {
    pub dummy: u64,
}

} // verus!
