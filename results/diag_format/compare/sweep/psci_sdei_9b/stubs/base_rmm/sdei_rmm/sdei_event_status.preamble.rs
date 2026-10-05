use vstd::prelude::*;

verus! {

pub type int = u64;

pub type Int64 = i64;

pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const NOT_SUPPORTED: Int64 = -1;

pub const INVALID_PARAMETERS: Int64 = -2;

pub open spec fn IsSdeiSupported() -> bool;

pub open spec fn IsValidEventNumber(event: Int32) -> bool;

pub open spec fn ResultEqual(a: Int64, b: Int64) -> bool;

pub open spec fn EventHandlerIsRunning(event: Int32) -> u8;

pub open spec fn EventHandlerIsEnabled(event: Int32) -> u8;

pub open spec fn EventHandlerIsRegistered(event: Int32) -> u8;

} // verus!
