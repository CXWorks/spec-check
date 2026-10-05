use vstd::prelude::*;
verus! {

pub struct S {
    pub telemetry_state: Seq<u32>,
}

pub const SUCCESS: i32 = 0i32;
pub const INVALID_PARAMETERS: i32 = 1i32;
pub const OUT_OF_RANGE: i32 = 2i32;
pub const IN_USE: i32 = 3i32;

pub open spec fn TelemetryDeIdValid(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryEventGroupIdValid(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryEnableLimitReached(s: S, identifier: u32, is_group: bool) -> bool;
pub open spec fn TelemetryConfigInUse(s: S, identifier: u32, flags: u32) -> bool;
pub open spec fn TelemetryAllDisabled(s: S) -> bool;
pub open spec fn TelemetryConfigUnchangedExcept(old_s: S, new_s: S, identifier: u32, is_group: bool) -> bool;
pub open spec fn TelemetryDeEnabled(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryDeTimestampEnabled(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryGroupAllDisabled(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryGroupAllEnabled(s: S, identifier: u32) -> bool;
pub open spec fn TelemetryGroupTimestampEnabled(s: S, identifier: u32) -> bool;

} // verus!
