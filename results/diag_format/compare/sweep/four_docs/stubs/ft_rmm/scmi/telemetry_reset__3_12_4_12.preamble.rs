use vstd::prelude::*;
verus! {

pub type int32 = i32;

pub struct CallingAgent {
    pub id: int,
}

pub struct S {
    pub telemetry_config_reset: bool,
    pub telemetry_de_data_reset: bool,
}

pub const SUCCESS: int32 = 0;
pub const INVALID_PARAMETERS: int32 = 1;
pub const DENIED: int32 = 2;

pub open spec fn ResultEqual(a: int32, b: int32) -> bool;

pub open spec fn ResultNotEqual(a: int32, b: int32) -> bool;

pub open spec fn IsAgentPermittedTelemetryReset(s: S, agent: CallingAgent) -> bool;

pub open spec fn TelemetryConfigurationIsReset(s: S) -> bool;

pub open spec fn TelemetryAccumulatedDeDataIsReset(s: S) -> bool;

} // verus!
