use vstd::prelude::*;

verus! {

pub type Address = u64;

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type PsciFunctionId = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: PsciReturnCode = 0;

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const DENIED: PsciReturnCode = -3;

pub const MEM_PROTECT: PsciFunctionId = 0x84000013;

pub const MEM_PROTECT_CHECK_RANGE: PsciFunctionId = 0x84000014;

pub open spec fn IsImplemented(s: S, fid: PsciFunctionId) -> bool;

pub open spec fn ResultEqual(a: PsciReturnCode, b: PsciReturnCode) -> bool;

pub open spec fn RangeIsProtectedByMemProtect(s: S, start: Address, end: int) -> bool;

} // verus!
