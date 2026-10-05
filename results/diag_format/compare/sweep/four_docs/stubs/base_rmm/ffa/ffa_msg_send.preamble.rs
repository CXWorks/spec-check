use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;
pub type PartitionId = u16;
pub type BufferAddr = u64;

pub struct S {
    pub busy: bool,
    pub w1: UInt64,
    pub w2: UInt64,
    pub w3: UInt64,
    pub w4: UInt64,
    pub w5: UInt64,
    pub w6: UInt64,
    pub w7: UInt64,
    pub sender_id: PartitionId,
    pub receiver_id: PartitionId,
}

pub spec const FFA_SUCCESS: UInt32 = 0x84000061u32;
pub spec const FFA_ERROR_BUSY: UInt32 = 0xFFFFFFFCu32;

pub open spec fn ResultEqual(a: UInt32, b: UInt32) -> bool;

pub open spec fn RegistersAreZero(w1: UInt64, w2: UInt64, w3: UInt64, w4: UInt64, w5: UInt64, w6: UInt64, w7: UInt64) -> bool;

pub open spec fn RxBufferOf(s: S, id: PartitionId) -> BufferAddr;

pub open spec fn TxBufferOf(s: S, id: PartitionId) -> BufferAddr;

pub open spec fn SchedulerInformedOfPendingMessage(s: S, id: PartitionId) -> bool;

} // verus!
