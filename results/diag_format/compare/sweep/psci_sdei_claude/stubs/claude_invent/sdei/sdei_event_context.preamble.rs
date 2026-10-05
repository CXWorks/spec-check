use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
    pub handler_running: bool,
    pub regs: Seq<i64>,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;
pub const DENIED: Int64 = -3;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn SdeiHandlerRunningOnCallingPe(s: S) -> bool;

pub open spec fn SdeiEventContextRegister(s: S, idx: int) -> Int64;

} // verus!
