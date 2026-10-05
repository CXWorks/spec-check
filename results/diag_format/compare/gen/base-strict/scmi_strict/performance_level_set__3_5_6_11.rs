pub open spec fn performance_level_set__3_5_6_11_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsWithinAllowedPerformanceRange(old_s, domain_id, performance_level) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!AgentPermittedToSetPerformanceLevel(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (result.is_Ok() ==> PerformanceLevelRequestScheduled(old_s, domain_id, performance_level))
    && ((!LevelIndexingModeInUse(old_s, domain_id) && performance_level == 0) ==> PlatformPolicyDeterminesPerformanceLevel(old_s, domain_id))
    && (PerformanceDomain(old_s, domain_id).requested_level == PerformanceDomain(new_s, domain_id).requested_level)
}