use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub struct S {
    pub rx_buffer_addr: u64,
    pub tx_buffer_addr: u64,
    pub buffer_page_count: u32,
    pub buffers_registered: bool,
    pub rxtx_unmap_supported: bool,
}

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;

pub uninterp spec fn IsBufferPairRegistered(s: S) -> bool;

pub uninterp spec fn IsFfaRxtxUnmapSupported(s: S) -> bool;

} // verus!
