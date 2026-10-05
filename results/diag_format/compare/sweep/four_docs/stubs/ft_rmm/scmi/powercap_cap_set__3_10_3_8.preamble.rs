use vstd::prelude::*;

verus! {

pub struct uint32 {
    pub v: u32,
}

impl uint32 {
    pub open spec fn spec_index(self, i: int) -> int;
}

pub type int32 = i32;

pub type AgentId = u32;

pub type CommandId = u32;

pub type MessageId = u32;

pub struct S {
    pub dummy: int,
}

pub enum PowercapCapSetReturnCode {
    NotFound,
    NotSupported,
    InvalidParameters,
    Denied,
}

pub spec const NOT_FOUND: PowercapCapSetReturnCode = PowercapCapSetReturnCode::NotFound;
pub spec const NOT_SUPPORTED: PowercapCapSetReturnCode = PowercapCapSetReturnCode::NotSupported;
pub spec const INVALID_PARAMETERS: PowercapCapSetReturnCode = PowercapCapSetReturnCode::InvalidParameters;
pub spec const DENIED: PowercapCapSetReturnCode = PowercapCapSetReturnCode::Denied;

pub spec const SUCCESS: Result<int32, PowercapCapSetReturnCode> = Ok(0i32);

pub spec const agent: AgentId = 0u32;

pub spec const POWERCAP_CAP_SET: CommandId = 4u32;

pub spec const POWERCAP_CAP_SET_COMPLETE: MessageId = 5u32;

pub open spec fn ResultEqual(result: Result<int32, PowercapCapSetReturnCode>, code: PowercapCapSetReturnCode) -> bool;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: uint32) -> bool;

pub open spec fn IsValidCpli(s: S, domain_id: uint32, cpli: uint32) -> bool;

pub open spec fn IsRequestSupported(s: S, domain_id: uint32, cpli: uint32, flags: uint32, power_cap: uint32) -> bool;

pub open spec fn IsSupportedPowerCap(s: S, domain_id: uint32, cpli: uint32, power_cap: uint32) -> bool;

pub open spec fn IsValidCapSetFlags(s: S, flags: uint32) -> bool;

pub open spec fn AgentMaySetPowerCap(s: S, agent_id: AgentId, domain_id: uint32) -> bool;

pub open spec fn RequestedPowerCap(s: S, agent_id: AgentId, domain_id: uint32, cpli: uint32) -> uint32;

pub open spec fn CommandQueued(s: S, cmd: CommandId, domain_id: uint32, cpli: uint32, power_cap: uint32) -> bool;

pub open spec fn DelayedResponseSent(s: S, msg: MessageId) -> bool;

} // verus!
