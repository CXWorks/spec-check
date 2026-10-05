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

pub struct PowercapDomainInfo {
    pub cai: UInt32,
}

pub struct S {
    pub dummy: u32,
}

pub spec const SUCCESS: Result<Int32, PowercapCaiSetReturnCode> = Result::Ok(0i32);
pub spec const NOT_FOUND: Result<Int32, PowercapCaiSetReturnCode> = Result::Err(PowercapCaiSetReturnCode::NotFound);
pub spec const NOT_SUPPORTED: Result<Int32, PowercapCaiSetReturnCode> = Result::Err(PowercapCaiSetReturnCode::NotSupported);
pub spec const INVALID_PARAMETERS: Result<Int32, PowercapCaiSetReturnCode> = Result::Err(PowercapCaiSetReturnCode::InvalidParameters);
pub spec const DENIED: Result<Int32, PowercapCaiSetReturnCode> = Result::Err(PowercapCaiSetReturnCode::Denied);

pub spec const calling_agent: AgentId = 0u32;

pub open spec fn IsValidPowercapDomain(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsValidCpli(s: S, domain_id: UInt32, cpli: UInt32) -> bool;
pub open spec fn IsCaiSetSupported(s: S, domain_id: UInt32) -> bool;
pub open spec fn IsSupportedCai(s: S, domain_id: UInt32, cai: UInt32) -> bool;
pub open spec fn AreValidCaiSetFlags(s: S, flags: UInt32) -> bool;
pub open spec fn AgentMaySetCai(s: S, agent: AgentId, domain_id: UInt32) -> bool;
pub open spec fn ResultEqual(r1: Result<Int32, PowercapCaiSetReturnCode>, r2: Result<Int32, PowercapCaiSetReturnCode>) -> bool;
pub open spec fn PowercapDomain(s: S, domain_id: UInt32) -> PowercapDomainInfo;

} // verus!
