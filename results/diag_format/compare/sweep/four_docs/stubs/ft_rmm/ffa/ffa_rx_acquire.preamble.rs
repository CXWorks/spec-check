use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub type FunctionId = u32;

pub struct S {
    pub dummy: u64,
}

pub enum BufferOwner {
    Caller,
    Producer,
    Hypervisor,
    None,
}

pub use BufferOwner::*;

pub const FFA_RX_ACQUIRE: FunctionId = 0x84000084;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;
pub const DENIED: UInt32 = 3;

pub open spec fn IsImplementedAtInstance(s: S, func: FunctionId) -> bool;

pub open spec fn ResultEqual(result: UInt32, code: UInt32) -> bool;

pub open spec fn IsBufferPairRegistered(s: S, vm_id: UInt16) -> bool;

pub open spec fn CanRelinquishRxBufferOwnership(s: S, vm_id: UInt16) -> bool;

pub open spec fn RxBufferOwner(s: S, vm_id: UInt16) -> BufferOwner;

} // verus!
