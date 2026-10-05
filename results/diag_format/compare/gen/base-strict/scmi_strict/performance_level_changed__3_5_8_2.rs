pub open spec fn performance_level_changed__3_5_8_2_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, recipient_agent: u32, domain_id: u32, performance_level: u32, agent_id: u32) -> bool {
    (result.is_Ok() ==> IsSubscribedToPerfLevelNotifications(old_s, recipient_agent, domain_id))
    && (result.is_Ok() ==> PerformanceLevelChanged(old_s, domain_id))
    && (result.is_Ok() ==> performance_level == PerformanceLevel(old_s, domain_id))
    && (result.is_Ok() ==> agent_id == PerfLevelChangeInitiator(old_s, domain_id))
    && (result.is_Ok() ==> old_s == new_s)
}