use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type UInt16 = u32;
pub type UInt64 = u64;
pub type RsiCommandReturnCode = u32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: RsiCommandReturnCode = 0;
pub const NOT_SUPPORTED: RsiCommandReturnCode = 1;
pub const NOT_FOUND: RsiCommandReturnCode = 2;

pub const RSI_SUCCESS: Result<(), RsiCommandReturnCode> = Ok(());

pub open spec fn IsRequestSupported(s: S, a: u32, b: u32) -> bool;

pub open spec fn ResultEqual(r: Result<(), RsiCommandReturnCode>, c: RsiCommandReturnCode) -> bool;

pub open spec fn NumShmtiAvailable(s: S) -> int;

pub open spec fn NumDescriptors(desc: [UInt32; 5]) -> UInt16;

pub open spec fn RemainingShmtiCount(s: S, n: UInt16) -> UInt16;

pub open spec fn Entry(desc: [UInt32; 5], i: UInt32) -> UInt32;

pub open spec fn DescribesShmti(e: UInt32, idx: int) -> bool;

pub open spec fn Word(e: UInt32, w: int) -> UInt32;

pub open spec fn ShmtiAddress(hi: UInt32, lo: UInt32) -> UInt64;

pub open spec fn IsInCallerMemoryMap(addr: UInt64) -> bool;

} // verus!
