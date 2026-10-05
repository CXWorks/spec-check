use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const SUCCESS: PsciReturnCode = 0;

pub const DENIED: PsciReturnCode = -3;

pub struct S {
    pub dummy: nat,
}

pub open spec fn MemProtectRangeIsProtected(s: S, base: UInt64, length: UInt64) -> bool;

} // verus!
