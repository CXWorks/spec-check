use vstd::prelude::*;
verus! {

pub type int32 = i32;
pub type VmId = u16;

pub struct S {
    pub vm_id: VmId,
}

impl S {
    pub uninterp spec fn vm_rx_buffer_owned(&self, id: VmId) -> bool;
    pub uninterp spec fn vm_rx_buffer_registered(&self, id: VmId) -> bool;
}

pub spec const FFA_SUCCESS: int32 = 0x61i32;
pub spec const FFA_NOT_SUPPORTED: int32 = -1i32;
pub spec const FFA_INVALID_PARAMETERS: int32 = -2i32;
pub spec const FFA_DENIED: int32 = -6i32;

pub uninterp spec fn FFA_ERROR(code: int32) -> int32;

} // verus!
