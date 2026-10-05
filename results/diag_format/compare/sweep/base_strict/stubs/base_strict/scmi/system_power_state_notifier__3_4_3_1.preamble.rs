use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub type AgentId = u64;

pub enum RmiStatusCode {
    Success,
    ErrorInput,
    ErrorRealm,
    ErrorRec,
    ErrorRtt,
    ErrorDevice,
    ErrorNotSupported,
}

pub struct S {
    pub system_state: UInt64,
    pub timeout: UInt64,
}

pub spec const recipient: AgentId = 1;

pub open spec fn AgentRegisteredForSystemPowerStateNotify(agent: AgentId) -> bool;

pub open spec fn Bits64(s: S, hi: int, lo: int) -> UInt64;

pub open spec fn PlatformImposesShutdownTimeout() -> bool;

pub open spec fn ShutdownTimeoutExpired(timeout: UInt64) -> bool;

pub open spec fn SystemShutdownRequestReceived() -> bool;

pub open spec fn PlatformMayForceSystemShutdown() -> bool;

} // verus!
