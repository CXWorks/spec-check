use vstd::prelude::*;

verus! {

pub type UInt16 = u16;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub type FunctionId = u32;

pub type Mapping = u64;

pub struct Owner {
    pub id: u16,
}

pub struct S {
    pub dummy: u64,
}

impl S {
    pub open spec fn RxTxBufferPairMapping(self, owner: Owner) -> Mapping;
}

pub const FFA_RXTX_UNMAP: FunctionId = 0x84000067;

pub const FFA_SUCCESS: UInt32 = 0;
pub const NOT_SUPPORTED: UInt32 = 1;
pub const INVALID_PARAMETERS: UInt32 = 2;

pub open spec fn IsImplementedAtInstance(fid: FunctionId) -> bool;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn BufferOwner(id: UInt16) -> Owner;

pub open spec fn IsRxTxBufferPairRegistered(owner: Owner) -> bool;

pub open spec fn IsRxTxBufferPairMappedInCalleeRegime(owner: Owner) -> bool;

pub open spec fn RxTxBufferPairMapping(owner: Owner) -> Mapping;

} // verus!
