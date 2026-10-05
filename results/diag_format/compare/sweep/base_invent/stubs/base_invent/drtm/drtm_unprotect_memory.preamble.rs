use vstd::prelude::*;

verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub const RSI_INCOMPLETE: RsiCommandReturnCode = 3;
pub const RSI_ERROR_UNKNOWN: RsiCommandReturnCode = 4;
pub const RSI_ERROR_NOT_SUPPORTED: RsiCommandReturnCode = 5;
pub const RSI_ERROR_DENIED: RsiCommandReturnCode = 6;

pub struct S {
    pub drtm_supported: bool,
    pub memory_protected: bool,
}

pub open spec fn DrtmSupported(s: S) -> bool;

pub open spec fn HasMemoryProtections(s: S) -> bool;

} // verus!
