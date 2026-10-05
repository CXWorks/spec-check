use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub enum RmiStatusCode {
    Success,
    Denied,
    InvalidParameters,
}

pub struct Agent {
    pub id: u64,
}

pub struct S {
    pub agent: Agent,
    pub flags: u64,
}

pub const DENIED: RmiStatusCode = RmiStatusCode::Denied;

pub const INVALID_PARAMETERS: RmiStatusCode = RmiStatusCode::InvalidParameters;

pub open spec fn calling_agent(s: S) -> Agent;

pub open spec fn flags(s: S) -> u64;

pub open spec fn AgentPermittedToResetTelemetry(a: Agent) -> bool;

pub open spec fn ResultEqual(result: Result<(), RmiStatusCode>, code: RmiStatusCode) -> bool;

pub open spec fn TelemetryConfigurationIsReset() -> bool;

pub open spec fn TelemetryDeDataIsCleared() -> bool;

} // verus!
