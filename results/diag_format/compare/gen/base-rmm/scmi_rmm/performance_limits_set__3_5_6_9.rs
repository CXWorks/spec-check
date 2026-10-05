pub open spec fn performance_limits_set__3_5_6_9_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PerformanceDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!LimitsWithinDescribedLevels(old_s, domain_id, range_max, range_min) ==> ResultEqual(result, OUT_OF_RANGE))
    && (range_max == 0 && range_min == 0 ==> ResultEqual(result, OUT_OF_RANGE))
    && (!AgentMayChangePerformanceLimits(old_s, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (range_max != 0 ==> PerformanceDomain(new_s, domain_id).range_max == range_max)
        && (range_max == 0 ==> PerformanceDomain(new_s, domain_id).range_max == old_s.PerformanceDomain(domain_id).range_max)
        && (range_min != 0 ==> PerformanceDomain(new_s, domain_id).range_min == range_min)
        && (range_min == 0 ==> PerformanceDomain(new_s, domain_id).range_min == old_s.PerformanceDomain(domain_id).range_min)
    ))
}