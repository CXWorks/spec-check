use vstd::prelude::*;

verus! {

pub type VmId = u16;

pub struct S {
    pub current_vm_id: VmId,
}

pub const FFA_ERROR: u32 = 0x84000060;
pub const FFA_SUCCESS: u32 = 0x84000061;
pub const FFA_RX_ACQUIRE: u32 = 0x84000084;

pub const NOT_SUPPORTED: i32 = -1;
pub const INVALID_PARAMETERS: i32 = -2;
pub const DENIED: i32 = -6;

pub open spec fn IsImplementedAtInstance(func_id: u32) -> bool;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;

pub open spec fn vm_id(s: S) -> VmId;

pub open spec fn IsBufferPairRegistered(s: S, id: VmId) -> bool;

pub open spec fn CalleeCanRelinquishRxBuffer(s: S, id: VmId) -> bool;

pub open spec fn RxBufferOwnedByCaller(s: S, id: VmId) -> bool;

} // verus!
