use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub const FFA_INSTANCE_NON_SECURE_PHYSICAL: u32 = 1;
pub const FFA_INSTANCE_SECURE_PHYSICAL: u32 = 2;

pub const FFA_SUCCESS: int32 = 0;
pub const FFA_ERROR_INVALID_PARAMETERS: int32 = -2;
pub const FFA_ERROR_DENIED: int32 = -6;

pub struct S {
    pub ffa_instance: u32,
    pub vm_id: u32,
}

} // verus!
