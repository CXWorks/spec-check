use vstd::prelude::*;
verus! {

pub type PsciReturnCode = i32;
pub type PsciFunctionId = u32;

pub const SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const MEM_PROTECT_CHECK_RANGE: PsciFunctionId = 0x84000014;

#[allow(non_upper_case_globals)]
pub spec const base: int = 0x8000_0000;
#[allow(non_upper_case_globals)]
pub spec const length: int = 0x1000;

pub struct S {
    pub mem_protect_enabled: bool,
    pub protected_ranges: Seq<(int, int)>,
}

pub open spec fn IsFunctionImplemented(fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn IsRangeProtectedByMemProtect(s: S, start: int, end: int) -> bool;

} // verus!
