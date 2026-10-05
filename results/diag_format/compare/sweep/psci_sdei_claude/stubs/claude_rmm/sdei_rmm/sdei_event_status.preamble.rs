use vstd::prelude::*;

verus! {

pub type Int64 = i64;
pub type Int32 = i32;

pub struct S {
    pub sdei_supported: bool,
    pub event_state: Map<Int32, u64>,
}

pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub open spec fn IsSdeiSupported(s: S) -> bool;

pub open spec fn IsValidEventNumber(s: S, event: Int32) -> bool;

pub open spec fn ResultEqual(result: Int64, code: Int64) -> bool;

pub open spec fn EventHandlerIsRunning(s: S, event: Int32) -> bool;

pub open spec fn EventHandlerIsEnabled(s: S, event: Int32) -> bool;

pub open spec fn EventHandlerIsRegistered(s: S, event: Int32) -> bool;

} // verus!
