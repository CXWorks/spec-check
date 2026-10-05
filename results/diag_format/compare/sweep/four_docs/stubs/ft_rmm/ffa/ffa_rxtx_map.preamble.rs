use vstd::prelude::*;
verus! {

pub struct BufferPair {
    pub tx_address: u64,
    pub rx_address: u64,
    pub page_count: u32,
}

pub struct CalleeTranslationRegime {
    pub id: u64,
}

pub type FfaReturnCode = u32;

pub const FFA_SUCCESS: FfaReturnCode = 0;

pub struct S {
    pub mapped: bool,
}

pub open spec fn RxTxBufferPairIsMapped(s: S, regime: CalleeTranslationRegime, buffer_pair: BufferPair) -> bool;

pub open spec fn RxTxBufferPairIsNotMapped(s: S, regime: CalleeTranslationRegime, buffer_pair: BufferPair) -> bool;

} // verus!
