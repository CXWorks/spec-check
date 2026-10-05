use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type UInt64 = u64;

pub const FALSE: bool = false;

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub struct S {
    pub resume_addr: UInt64,
}

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn IsIdentifiablyInvalidResumeAddress(addr: UInt64) -> bool;

pub open spec fn HandlerRunning(s: S) -> bool;

} // verus!
