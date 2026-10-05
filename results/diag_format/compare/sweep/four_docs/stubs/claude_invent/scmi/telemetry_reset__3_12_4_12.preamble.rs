use vstd::prelude::*;
verus! {

pub const SUCCESS: i32 = 0;
pub const INVALID_PARAMETERS: i32 = 1;
pub const DENIED: i32 = 2;

pub struct S {
    pub telemetry_reset_supported: bool,
    pub agent_allowed_telemetry_reset: bool,
    pub telemetry_configuration_reset: bool,
    pub telemetry_de_data_cleared: bool,
}

pub open spec fn AgentAllowedTelemetryReset(s: S) -> bool;

pub open spec fn TelemetryResetSupported(s: S) -> bool;

pub open spec fn TelemetryConfigurationIsReset(s: S) -> bool;

pub open spec fn TelemetryDeDataIsCleared(s: S) -> bool;

} // verus!
