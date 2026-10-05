use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub type PsciReturnCode = i32;

pub type FunctionId = u64;

pub struct S {
    pub dummy: u64,
}

pub const SYSTEM_OFF2: FunctionId = 0x8400_0015;

pub const NOT_SUPPORTED: PsciReturnCode = -1;

pub const INVALID_PARAMETERS: PsciReturnCode = -2;

pub open spec fn IsFunctionImplemented(fid: FunctionId) -> bool;

pub open spec fn AreInputParametersValid() -> bool;

pub open spec fn ResultEqual(result: PsciReturnCode, expected: PsciReturnCode) -> bool;

} // verus!
