use vstd::prelude::*;
verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type Int32 = i32;

pub struct S {
    pub dummy: int,
}

pub const FFA_ERROR: UInt32 = 0x84000060;
pub const FFA_SUCCESS: UInt32 = 0x84000061;
pub const FFA_RX_ACQUIRE: UInt32 = 0x84000084;

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -6;

pub open spec fn IsImplementedAtInstance(s: S, func_id: UInt32) -> bool;

pub open spec fn ResultEqual<T>(a: T, b: T) -> bool;

pub open spec fn IsBufferPairRegistered(s: S, vm_id: UInt16) -> bool;

pub open spec fn CalleeCanRelinquishRxBuffer(s: S, vm_id: UInt16) -> bool;

pub open spec fn RxBufferOwnedByCaller(s: S, vm_id: UInt16) -> bool;

} // verus!
