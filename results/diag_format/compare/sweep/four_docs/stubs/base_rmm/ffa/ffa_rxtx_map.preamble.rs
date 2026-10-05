use vstd::prelude::*;
verus! {

pub type RsiCommandReturnCode = u64;

pub const RSI_SUCCESS: RsiCommandReturnCode = 0;
pub const RSI_ERROR_INPUT: RsiCommandReturnCode = 1;

pub enum TranslationRegime {
    EL1,
    EL2,
}

pub struct BufferPair {
    pub tx_addr: u64,
    pub rx_addr: u64,
    pub page_count: u64,
}

pub struct S {
    pub dummy: u64,
}

pub spec const callee_translation_regime: TranslationRegime = TranslationRegime::EL1;

pub spec const buffer_pair: BufferPair = BufferPair { tx_addr: 0, rx_addr: 4096, page_count: 1 };

pub open spec fn RxTxBufferPairIsMapped(regime: TranslationRegime, pair: BufferPair) -> bool;

} // verus!
