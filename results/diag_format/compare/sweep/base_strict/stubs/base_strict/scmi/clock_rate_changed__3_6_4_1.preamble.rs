use vstd::prelude::*;

verus! {

pub type AgentId = u32;
pub type ClockId = u32;
pub type RateValue = u64;

pub enum RmiStatusCode {
    Success,
    NotFound,
    InvalidParameters,
    Denied,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub struct S {
    pub dummy: int,
}

pub spec const agent_id: AgentId = 1;
pub spec const clock_id: ClockId = 2;
pub spec const rate: RateValue = 3;

pub open spec fn RecipientAgent() -> AgentId;

pub open spec fn IsRegisteredForClockRateNotification(s: S, agent: AgentId) -> bool;

pub open spec fn ClockRateChangedByOtherAgentOrPlatform(s: S, agent: AgentId, recipient: AgentId) -> bool;

pub open spec fn ClockRateTransitionCompleted(s: S, clock: ClockId) -> bool;

pub open spec fn ClockRate(s: S, clock: ClockId) -> int;

pub open spec fn RateLow(r: RateValue) -> int;

pub open spec fn RateHigh(r: RateValue) -> int;

} // verus!
