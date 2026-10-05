use vstd::prelude::*;
verus! {

pub type RmiStatusCode = u64;
pub type AgentId = u64;
pub type UInt64 = u64;

pub struct S {
    pub dummy: u64,
}

pub spec const DENIED: RmiStatusCode = 1;
pub spec const INVALID_PARAMETERS: RmiStatusCode = 2;

pub spec const calling_agent: AgentId = 0;
pub spec const flags: UInt64 = 0;

pub open spec fn IsAgentPermittedTelemetryReset(agent: AgentId) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn TelemetryConfigurationIsReset() -> bool;

pub open spec fn TelemetryAccumulatedDeDataIsReset() -> bool;

} // verus!
