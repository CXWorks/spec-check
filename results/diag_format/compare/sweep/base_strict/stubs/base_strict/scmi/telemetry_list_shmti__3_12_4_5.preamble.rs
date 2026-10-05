use vstd::prelude::*;

verus! {

pub type Int32 = i32;

pub type UInt16 = u16;

pub type UInt32 = u32;

pub type Array<T> = Seq<T>;

pub struct ShmtiDescriptor {
    pub words: Seq<UInt32>,
}

pub struct ShmtiAddr {
    pub value: int,
}

pub struct S {
    pub shmti_count: UInt32,
    pub shmti_table: Seq<ShmtiDescriptor>,
}

pub spec const SUCCESS: Int32 = 0;

pub spec const NOT_SUPPORTED: Int32 = -1i32;

pub spec const NOT_FOUND: Int32 = -6i32;

pub spec const message_id: UInt32 = 1;

pub spec const protocol_id: UInt32 = 2;

pub spec const index: UInt32 = 3;

pub open spec fn IsRequestSupported(message_id: UInt32, protocol_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn NumShmtiAvailable() -> UInt32;

pub open spec fn Bits(value: UInt16, hi: int, lo: int) -> UInt32;

pub open spec fn NumDescriptors(desc: Array<UInt32>) -> UInt32;

pub open spec fn RemainingShmtiCount(start_index: UInt32, returned: UInt32) -> UInt32;

pub open spec fn Entry(desc: Array<UInt32>, i: UInt32) -> ShmtiDescriptor;

pub open spec fn DescribesShmti(entry: ShmtiDescriptor, shmti_index: int) -> bool;

pub open spec fn Word(entry: ShmtiDescriptor, n: int) -> UInt32;

pub open spec fn ShmtiAddress(high: UInt32, low: UInt32) -> ShmtiAddr;

pub open spec fn IsInCallerMemoryMap(addr: ShmtiAddr) -> bool;

} // verus!
