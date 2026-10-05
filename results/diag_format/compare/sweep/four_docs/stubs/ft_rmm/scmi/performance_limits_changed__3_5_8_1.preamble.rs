use vstd::prelude::*;

verus! {

pub type UInt32 = u32;

pub type AgentId = u32;

pub type NotificationType = u32;

pub struct S {
    pub dummy: int,
}

pub spec const agent: AgentId = 0;

pub spec const PERFORMANCE_LIMITS_CHANGED: NotificationType = 1;

pub open spec fn IsRegisteredForLimitChangeNotification(s: S, a: AgentId, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLimitsChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentToAgent(s: S, a: AgentId, n: NotificationType) -> bool;

pub open spec fn IdOfAgentCausingLimitChange(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMin(s: S, domain_id: UInt32) -> UInt32;

} // verus!
