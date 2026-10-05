use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type Address = u64;

pub type RsiCommandReturnCode = u64;

pub spec const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub spec const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;
pub spec const RSI_ERROR_STATE: RsiCommandReturnCode = 2;
pub spec const RSI_INCOMPLETE: RsiCommandReturnCode = 3;

pub spec const rx_buffer: Address = 0x1000;
pub spec const tx_buffer: Address = 0x2000;

pub struct S {
    pub dummy: u64,
}

pub open spec fn RxTxBufferPairMappedInCalleeRegime(s: S, rx: Address, tx: Address) -> bool;

} // verus!
