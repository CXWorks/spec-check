use vstd::prelude::*;
verus! {

pub type Int32 = i32;
pub type AgentId = u64;

pub struct S {
    pub telemetry_enabled: bool,
}

pub const SUCCESS: Int32 = 0;
pub const INVALID_PARAMETERS: Int32 = -2;
pub const DENIED: Int32 = -3;

pub const calling_agent: AgentId = 1;

pub open spec fn ResultEqual(a: Int32, b: Int32) -> bool;

pub open spec fn AgentPermittedToResetTelemetry(s: S, agent: AgentId) -> bool;

pub open spec fn TelemetryConfigurationIsReset() -> bool;

pub open spec fn TelemetryDeDataIsCleared() -> bool;

} // verus!
