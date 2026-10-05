use vstd::prelude::*;
verus! {

pub type uint32 = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_ERROR_INPUT,
    RSI_ERROR_STATE,
    RSI_INCOMPLETE,
}

pub use RsiCommandReturnCode::*;

pub struct S {
    pub dummy: int,
}

pub open spec fn Bits(x: uint32, hi: int, lo: int) -> int;

pub open spec fn NumPowerCappingDomains() -> int;

} // verus!
