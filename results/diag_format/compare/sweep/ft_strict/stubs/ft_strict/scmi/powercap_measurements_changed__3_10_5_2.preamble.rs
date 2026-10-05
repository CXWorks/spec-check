use vstd::prelude::*;
verus! {

pub type UInt32 = u32;

pub type AgentId = u32;

pub enum RsiCommandReturnCode {
    RSI_SUCCESS,
    RSI_NOT_FOUND,
    RSI_INVALID_PARAMETERS,
    RSI_DENIED,
    RSI_NOT_SUPPORTED,
}

pub use RsiCommandReturnCode::*;

pub struct S {
    pub dummy: int,
}

pub open spec fn recipient_agent(s: S) -> AgentId;

pub open spec fn IsRegisteredForMeasurementsChangedNotification(s: S, agent: AgentId, domain_id: UInt32) -> bool;

pub open spec fn MaiChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn AveragePowerBelowLowerThreshold(s: S, domain_id: UInt32) -> bool;

pub open spec fn AveragePowerAboveHigherThreshold(s: S, domain_id: UInt32) -> bool;

pub open spec fn AveragePowerAtThresholdBreach(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn AveragePowerAtMaiChange(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PowerCapMai(s: S, domain_id: UInt32) -> UInt32;

} // verus!
