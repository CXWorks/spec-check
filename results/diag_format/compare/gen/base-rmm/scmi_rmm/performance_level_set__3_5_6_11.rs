pub open spec fn performance_level_set__3_5_6_11_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsInAllowedPerformanceRange(old_s, domain_id, performance_level) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!AgentMaySetPerformanceLevel(old_s, caller, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> PlatformHasAcceptedAndScheduled(old_s, domain_id, performance_level))
    && (ResultEqual(result, SUCCESS) ==> new_s.PerformanceDomain(domain_id).performance_level == old_s.PerformanceDomain(domain_id).performance_level)
}