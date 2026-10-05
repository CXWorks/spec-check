use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub type ffa_vm_id_t = u16;

pub struct S {
    pub vm_id: ffa_vm_id_t,
}

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_NOT_SUPPORTED: int32 = -1;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_DENIED: int32 = -6;

} // verus!
