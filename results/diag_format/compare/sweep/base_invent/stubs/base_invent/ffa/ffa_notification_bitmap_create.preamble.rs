use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type VmId = u16;

pub const FFA_SUCCESS: int32 = 0;
pub const NOT_SUPPORTED: int32 = -1;
pub const INVALID_PARAMETERS: int32 = -2;
pub const NO_MEMORY: int32 = -3;
pub const DENIED: int32 = -6;

pub struct S {
    pub vm_id: VmId,
}

impl S {
    pub open spec fn vm_id_is_valid(&self, id: VmId) -> bool;

    pub open spec fn notification_bitmap_created(&self, id: VmId) -> bool;

    pub open spec fn notification_count(&self, id: VmId) -> u64;
}

} // verus!
