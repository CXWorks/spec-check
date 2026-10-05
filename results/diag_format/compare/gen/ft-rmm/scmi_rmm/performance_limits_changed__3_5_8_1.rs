pub open spec fn performance_limits_changed__3_5_8_1_spec(agent_id: UInt32, domain_id: UInt32, range_max: UInt32, range_min: UInt32, old_s: S, new_s: S) -> bool {
  (IsRegisteredForLimitChangeNotification(old_s, agent, domain_id) && PerformanceLimitsChanged(old_s, domain_id) ==> NotificationSentToAgent(new_s, agent, PERFORMANCE_LIMITS_CHANGED))
  && (agent_id == IdOfAgentCausingLimitChange(new_s, domain_id))
  && (range_max == PerformanceLimitMax(new_s, domain_id))
  && (range_min == PerformanceLimitMin(new_s, domain_id))
  && ((!(IsRegisteredForLimitChangeNotification(old_s, agent, domain_id)) || !PerformanceLimitsChanged(old_s, domain_id)) ==> !(NotificationSentToAgent(new_s, agent, PERFORMANCE_LIMITS_CHANGED)))
}