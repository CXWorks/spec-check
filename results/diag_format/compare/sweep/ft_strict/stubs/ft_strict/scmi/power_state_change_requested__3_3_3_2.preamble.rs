use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

pub spec const recipient_agent: UInt32 = 1;

pub open spec fn IsRegisteredForPowerStateChangeRequested(s: S, agent_id: UInt32) -> bool;

pub open spec fn PlatformReceivedPowerStateChangeRequest(s: S, agent_id: UInt32, domain_id: UInt32, power_state: UInt32) -> bool;

} // verus!
