pub open spec fn performance_level_set__3_5_6_11_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !DomainExists(old_s, domain_id))
    && (result == OUT_OF_RANGE ==> !PerformanceLevelInRange(old_s, domain_id, performance_level))
    && (result == DENIED ==> !AgentPermittedToChangePerformanceLevel(old_s, domain_id, performance_level))
    && (result == SUCCESS ==> DomainExists(old_s, domain_id) && PerformanceLevelInRange(old_s, domain_id, performance_level) && AgentPermittedToChangePerformanceLevel(old_s, domain_id, performance_level))
}