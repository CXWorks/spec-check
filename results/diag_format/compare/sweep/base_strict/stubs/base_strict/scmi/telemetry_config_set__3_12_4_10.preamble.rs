use vstd::prelude::*;
verus! {

pub type Int32 = i32;

pub struct S {
    pub dummy: u64,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const OUT_OF_RANGE: Int32 = -3;

pub open spec fn control(s: S) -> u64;
pub open spec fn ProtocolAttributes1(s: S) -> u64;
pub open spec fn group_identifier(s: S) -> u64;
pub open spec fn sampling_rate(s: S) -> u64;

pub open spec fn Bits64(x: u64, hi: int, lo: int) -> int;
pub open spec fn SignedBits(x: u64, hi: int, lo: int) -> int;
pub open spec fn Pow10(e: int) -> int;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn IsValidEventGroup(g: u64) -> bool;
pub open spec fn AnyDeEnabled(sel: int, g: u64) -> bool;
pub open spec fn EnabledDeOrGroupLimitReached(sel: int, g: u64, mode: int) -> bool;
pub open spec fn TelemetryEnabled(sel: int, g: u64) -> bool;
pub open spec fn AllInterfacesSupportOnDemand(s: S) -> bool;
pub open spec fn TelemetryMode(sel: int, g: u64) -> int;
pub open spec fn TelemetryDisabledAfterReadingComplete(sel: int, g: u64) -> bool;
pub open spec fn SamplingRate(sel: int, g: u64) -> int;

} // verus!
