use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub use RsiCommandReturnCode::*;

pub struct Payload {
    pub attributes: u64,
}

pub struct S {
    pub payload: Payload,
}

pub open spec fn Bits64(x: u64, hi: int, lo: int) -> u64;

pub open spec fn NumPowerCappingDomains() -> u64;

} // verus!
