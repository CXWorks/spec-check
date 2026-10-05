use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type Int64 = i64;

pub struct S {
    pub sdei_supported: bool,
}

pub const SDEI_SUCCESS: Int64 = 0;
pub const NOT_SUPPORTED: Int64 = -1;
pub const INVALID_PARAMETERS: Int64 = -2;

pub open spec fn SdeiIsSupported(s: S) -> bool;

pub open spec fn IsKnownEventNumber(s: S, event: Int32) -> bool;

pub open spec fn ResultEqual(result: Int64, expected: Int64) -> bool;

pub open spec fn Bits(value: Int64, hi: int, lo: int) -> int;

pub open spec fn EventHandlerIsRunning(s: S, event: Int32) -> bool;

pub open spec fn EventHandlerIsEnabled(s: S, event: Int32) -> bool;

pub open spec fn EventHandlerIsRegistered(s: S, event: Int32) -> bool;

pub open spec fn EventStatusMapsToHandlerState(s: S, result: Int64, event: Int32) -> bool;

} // verus!
