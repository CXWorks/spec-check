pub open spec fn performance_limits_set__3_5_6_9_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PerfDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && ((range_max == 0 && range_min == 0) ==> ResultEqual(result, OUT_OF_RANGE))
    && (range_max != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_max) ==> ResultEqual(result, OUT_OF_RANGE))
    && (range_min != 0 && !IsWithinDescribedLevels(old_s, domain_id, range_min) ==> ResultEqual(result, OUT_OF_RANGE))
    && (!CallerMayChangePerfLimits(old_s, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> (
        (range_max != 0 ==> PerfLimitMax(new_s, domain_id) == LimitFieldToLevel(old_s, domain_id, range_max))
        && (range_max == 0 ==> PerfLimitMax(new_s, domain_id) == PrevPerfLimitMax(old_s, domain_id))
        && (range_min != 0 ==> PerfLimitMin(new_s, domain_id) == LimitFieldToLevel(old_s, domain_id, range_min))
        && (range_min == 0 ==> PerfLimitMin(new_s, domain_id) == PrevPerfLimitMin(old_s, domain_id))
        && PerfLevelEventuallyWithinLimits(new_s, domain_id)
    ))
}