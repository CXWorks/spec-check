use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub const SUCCESS: PsciReturnCode = 0;

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub const DENIED: PsciReturnCode = -3;

pub struct S {
    pub system_reset2_implemented: bool,
    pub reset_pending: bool,
}

pub open spec fn IsSystemReset2Implemented(s: S) -> bool;

} // verus!
