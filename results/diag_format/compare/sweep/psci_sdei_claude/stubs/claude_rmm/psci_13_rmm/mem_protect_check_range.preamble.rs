use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type UInt32 = u32;

pub type PsciReturnCode = i32;

pub type PsciFunctionId = u32;

pub struct S {
    pub mem_protect_enabled: bool,
}

pub const SUCCESS: PsciReturnCode = 0;

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub const DENIED: PsciReturnCode = -3;

pub const MEM_PROTECT_CHECK_RANGE: PsciFunctionId = 0x84000014;

pub open spec fn IsFunctionImplemented(fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn IsRangeProtectedByMemProtect(start: UInt64, end: int) -> bool;

} // verus!
