use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
}

pub struct S {
    pub dummy: int,
}

#[allow(non_upper_case_globals)]
pub const agent: UInt32 = 1;

#[allow(non_upper_case_globals)]
pub const domain_id: UInt32 = 2;

#[allow(non_upper_case_globals)]
pub const agent_id: UInt32 = 3;

#[allow(non_upper_case_globals)]
pub const range_max: UInt32 = 4;

#[allow(non_upper_case_globals)]
pub const range_min: UInt32 = 5;

pub const PERFORMANCE_LIMITS_CHANGED: UInt32 = 6;

pub open spec fn IsRegisteredForLimitChangeNotification(s: S, a: UInt32, d: UInt32) -> bool;

pub open spec fn PerformanceLimitsChanged(s: S, d: UInt32) -> bool;

pub open spec fn NotificationSentToAgent(old_s: S, new_s: S, a: UInt32, notification: UInt32) -> bool;

pub open spec fn IdOfAgentCausingLimitChange(s: S, d: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMax(s: S, d: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMin(s: S, d: UInt32) -> UInt32;

} // verus!
