use vstd::prelude::*;

verus! {

pub type PsciReturnCode = i32;

pub type FunctionId = u32;

pub struct S {
    pub protected_ranges: Seq<(int, int)>,
    pub mem_protect_enabled: bool,
}

pub const SUCCESS: PsciReturnCode = 0;
pub const NOT_SUPPORTED: PsciReturnCode = -1;
pub const DENIED: PsciReturnCode = -3;

pub const MEM_PROTECT: FunctionId = 0x8400_0013;
pub const MEM_PROTECT_CHECK_RANGE: FunctionId = 0x8400_0014;

#[allow(non_upper_case_globals)]
pub spec const base: int = 0x1000;
#[allow(non_upper_case_globals)]
pub spec const length: int = 0x2000;

pub open spec fn IsImplemented(fid: FunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn RangeIsProtectedByMemProtect(s: S, lo: int, hi: int) -> bool;

} // verus!
