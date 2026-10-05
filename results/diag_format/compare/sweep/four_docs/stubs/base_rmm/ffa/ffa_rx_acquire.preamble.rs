use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type VmId = u16;

pub struct S {
    pub current_vm: VmId,
}

pub enum BufferOwner {
    Caller,
    Producer,
    Hypervisor,
}

pub use BufferOwner::*;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;
pub const DENIED: UInt32 = 3;
pub const FFA_RX_ACQUIRE: UInt32 = 0x84000084;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn IsBufferPairRegistered(s: S, id: VmId) -> bool;

pub open spec fn vm_id(s: S) -> VmId;

pub open spec fn CanRelinquishRxBufferOwnership(s: S, id: VmId) -> bool;

pub open spec fn RxBufferOwner(s: S, id: VmId) -> BufferOwner;

} // verus!
