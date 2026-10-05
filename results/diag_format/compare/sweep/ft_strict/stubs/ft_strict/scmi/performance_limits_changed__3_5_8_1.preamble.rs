use vstd::prelude::*;

verus! {

// NOTE: This function cannot type-check as written, and no preamble can fix it.
// Its last clause contains `domain_id == false`, which compares a
// UInt32 value with a bool. Only a change to the function can fix that.
// The likely intent is `NotificationSentToAgent(new_s, agent, domain_id) == false`.
// One preamble change would compile: aliasing UInt32 to bool. That would
// make every UInt32 field a bool and change what the spec means, so it is
// not done here.
// The third argument of NotificationSentToAgent is generic, so the call
// accepts either form.

pub type UInt32 = u32;

pub struct S {
    pub dummy: u32,
}

// The function uses `agent` without declaring it, so it is declared here.
pub spec const agent: UInt32 = 0;

pub open spec fn IsRegisteredForLimitChangeNotification(s: S, agent_id: UInt32, domain_id: UInt32) -> bool;

pub open spec fn PerformanceLimitsChanged(s: S, domain_id: UInt32) -> bool;

pub open spec fn NotificationSentToAgent<T>(s: S, agent_id: UInt32, domain_id: T) -> bool;

pub open spec fn LimitChangeCausingAgent(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMax(s: S, domain_id: UInt32) -> UInt32;

pub open spec fn PerformanceLimitMin(s: S, domain_id: UInt32) -> UInt32;

} // verus!
