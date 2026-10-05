pub open spec fn performance_level_changed__3_5_8_2_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, message_id: UInt, protocol_id: UInt, agent_id: UInt32, domain_id: UInt32, performance_level: UInt32) -> bool {
    (!AgentIsSubscribedToLevelChange(old_s, recipient, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PerformanceLevelChanged(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (AgentIsSubscribedToLevelChange(old_s, recipient, domain_id) && PerformanceLevelChanged(old_s, domain_id) ==> result.is_Ok() && PerformanceLevelOf(old_s, domain_id) == performance_level)
}