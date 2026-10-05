use vstd::prelude::*;
verus! {

pub type UInt32 = u32;
pub type Int32 = i32;
pub type AgentId = u32;

pub enum PowercapCaiSetReturnCode {
    NotFound,
    NotSupported,
    InvalidParameters,
    Denied,
}

pub struct S {
    pub dummy: int,
}

pub spec const SUCCESS: Result<Int32, PowercapCaiSetReturnCode> = Ok(0i32);
pub spec const NOT_FOUND: Result<Int32, PowercapCaiSetReturnCode> = Err(PowercapCaiSetReturnCode::NotFound);
pub spec const NOT_SUPPORTED: Result<Int32, PowercapCaiSetReturnCode> = Err(PowercapCaiSetReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<Int32, PowercapCaiSetReturnCode> = Err(PowercapCaiSetReturnCode::InvalidParameters);
pub spec const DENIED: Result<Int32, PowercapCaiSetReturnCode> = Err(PowercapCaiSetReturnCode::Denied);

pub open spec fn IsValidPowercapDomainId(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn IsCaiSetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsSupportedCai(s: S, domain_id: UInt32, cai: UInt32) -> bool;
pub open spec fn AgentMaySetCai(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub open spec fn CallingAgent() -> AgentId;
pub open spec fn PowercapCai(s: S, domain_id: UInt32, cpli: UInt32) -> UInt32;
pub open spec fn ResultEqual(a: Result<Int32, PowercapCaiSetReturnCode>, b: Result<Int32, PowercapCaiSetReturnCode>) -> bool;

} // verus!
