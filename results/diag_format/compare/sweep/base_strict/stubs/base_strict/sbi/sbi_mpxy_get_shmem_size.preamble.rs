use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type SbiReturnCode = i64;

pub const SBI_SUCCESS: SbiReturnCode = 0;

pub type Hart = u64;

pub type MpxyChannel = u32;

pub struct S {
    pub dummy: u64,
}

pub open spec fn ResultEqual(a: SbiReturnCode, b: SbiReturnCode) -> bool;

pub open spec fn MpxyShmemSize(h: Hart) -> UInt64;

pub open spec fn MsgDataMaxLen(c: MpxyChannel) -> UInt64;

} // verus!
