use vstd::prelude::*;
verus! {

pub struct RxTxBuffer {
    pub base_addr: u64,
    pub page_count: u64,
}

pub struct S {
    pub rx_buffer: RxTxBuffer,
    pub tx_buffer: RxTxBuffer,
    pub mapped: bool,
}

pub open spec fn RxTxBufferPairMappedInCalleeRegime(s: S, rx_buffer: RxTxBuffer, tx_buffer: RxTxBuffer) -> bool;

} // verus!
