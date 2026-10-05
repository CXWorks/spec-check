use vstd::prelude::*;

verus! {

pub type Int32 = i32;
pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct S {
    pub reset_type: UInt64,
}

pub const SYSTEM_RESET2: UInt32 = 0x84000012;

pub const SYSTEM_WARM_RESET: UInt64 = 0;

pub const NOT_SUPPORTED: Int32 = -1;
pub const INVALID_PARAMETERS: Int32 = -2;

pub open spec fn IsImplemented(function_id: UInt32) -> bool;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn Bits64(value: UInt64, hi: int, lo: int) -> UInt64;

} // verus!
