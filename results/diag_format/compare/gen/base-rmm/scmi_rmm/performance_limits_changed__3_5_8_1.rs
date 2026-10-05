pub open spec fn performance_limits_changed__3_5_8_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsRegisteredForLimitChangeNotification(old_s, agent, domain_id) ==> result.is_Err())
    && (!PerformanceLimitsChanged(old_s, domain_id) ==> result.is_Err())
    && (result.is_Ok() ==> (NotificationSentToAgent(old_s, new_s, agent, PERFORMANCE_LIMITS_CHANGED)
        && agent_id == IdOfAgentCausingLimitChange(old_s, domain_id)
        && range_max == PerformanceLimitMax(old_s, domain_id)
        && range_min == PerformanceLimitMin(old_s, domain_id)))
}