use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub enum PsciStatusCode {
    Success,
    NotSupported,
    InvalidParameters,
    Denied,
    AlreadyOn,
    OnPending,
    InternalFailure,
    NotPresent,
    Disabled,
    InvalidAddress,
}

pub struct S {
    pub dummy: u64,
}

#[allow(non_upper_case_globals)]
pub spec const recipient: UInt32 = 0;

pub open spec fn AgentRegisteredForSystemPowerStateNotify(s: S, agent: UInt32) -> bool;

pub open spec fn Bits(s: S, value: UInt32, hi: u32, lo: u32) -> UInt32;

pub open spec fn PlatformImposesShutdownTimeout(s: S) -> bool;

pub open spec fn ShutdownTimeoutExpired(s: S, timeout: UInt32) -> bool;

pub open spec fn SystemShutdownRequestReceived(s: S) -> bool;

pub open spec fn PlatformMayForceSystemShutdown(s: S) -> bool;

} // verus!
