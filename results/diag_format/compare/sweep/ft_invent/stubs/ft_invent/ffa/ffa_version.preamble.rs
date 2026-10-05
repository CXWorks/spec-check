use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type int32 = i64;

pub enum SmcStatusCode {
    SmcSuccess,
    SmcNotSupported,
    SmcInvalidParameters,
    SmcDenied,
}

pub struct S {
    pub version: u32,
}

#[allow(non_snake_case)]
pub mod RsiCommandReturnCode {
    use vstd::prelude::*;
    use super::*;

    verus! {

    pub spec const RSI_SUCCESS: Result<(), SmcStatusCode> = Ok(());

    pub spec const RSI_ERROR_INPUT: bool = true;

    } // verus!
}

} // verus!
