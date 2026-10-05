use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt32 = u32;

pub type UInt128 = u128;

pub type UInt64 = u64;

pub struct Caller {
    pub id: u64,
}

pub struct S {
    pub dummy: u64,
}

pub struct RxBufferContents {
    pub data: Seq<u8>,
}

pub const FFA_SUCCESS: Int32 = 0;

pub const NOT_SUPPORTED: Int32 = -1;

pub const INVALID_PARAMETERS: Int32 = -2;

pub const NO_MEMORY: Int32 = -3;

pub const BUSY: Int32 = -4;

pub const DENIED: Int32 = -6;

pub const NOT_READY: Int32 = -10;

pub const FFA_PARTITION_INFO_GET: UInt32 = 0x84000068;

pub const count: UInt32 = 1;

pub const size: UInt32 = 2;

pub open spec fn RxBufferIsFree(caller: Caller) -> bool;

pub open spec fn RxBufferIsMapped(caller: Caller) -> bool;

pub open spec fn IsValidUuid(uuid: UInt128) -> bool;

pub open spec fn ResultsFitInRxBuffer(caller: Caller, uuid: UInt128) -> bool;

pub open spec fn CalleeCanHandleRequest() -> bool;

pub open spec fn IsImplementedAtInstance(func_id: UInt32) -> bool;

pub open spec fn CalleeIsReady() -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn PartitionCount(uuid: UInt128) -> UInt32;

pub open spec fn RxBuffer(caller: Caller) -> RxBufferContents;

} // verus!
