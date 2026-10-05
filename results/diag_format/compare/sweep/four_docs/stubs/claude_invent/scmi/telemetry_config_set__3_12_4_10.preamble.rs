use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: i32 = 0;
pub const INVALID_PARAMETERS: i32 = 2;
pub const OUT_OF_RANGE: i32 = 7;

pub open spec fn TelemetryProtocolAttributes1(s: S) -> u32;

pub open spec fn IsValidEventGroup(s: S, group_identifier: UInt32) -> bool;

pub open spec fn TelemetryModeSupported(s: S, mode: u32) -> bool;

pub open spec fn AnyDeEnabled(s: S, group_type: u32, group_identifier: UInt32) -> bool;

pub open spec fn TelemetryEnableLimitReached(s: S, group_type: u32, group_identifier: UInt32, mode: u32) -> bool;

pub open spec fn TelemetryCollectionEnabled(s: S, group_type: u32, group_identifier: UInt32) -> bool;

pub open spec fn TelemetryModeApplied(old_s: S, new_s: S, mode: u32) -> bool;

pub open spec fn TelemetrySamplingRateApplied(s: S, group_type: u32, group_identifier: UInt32, rate: u32) -> bool;

} // verus!
