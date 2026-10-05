use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type FfaCommandReturnCode = u32;

pub spec const FFA_SUCCESS: FfaCommandReturnCode = 0;

pub spec const FFA_NOT_SUPPORTED: FfaCommandReturnCode = 1;

pub spec const FFA_INVALID_PARAMETERS: FfaCommandReturnCode = 2;

pub spec const FFA_DENIED: FfaCommandReturnCode = 3;

pub spec const FFA_BUSY: FfaCommandReturnCode = 4;

pub struct S {
    pub dummy: u32,
}

} // verus!
