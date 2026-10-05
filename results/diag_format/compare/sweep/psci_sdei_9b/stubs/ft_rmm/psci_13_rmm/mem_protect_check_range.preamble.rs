use vstd::prelude::*;
verus! {

pub type Address = u64;
pub type UInt64 = u64;
pub type PsciReturnCode = i32;
pub type FunctionId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const MEM_PROTECT_CHECK_RANGE: FunctionId = 0x84000014;

pub open spec fn IsFunctionImplemented(s: S, fid: FunctionId) -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

pub open spec fn IsRangeProtectedByMemProtect(s: S, start: Address, end: int) -> bool;

} // verus!
