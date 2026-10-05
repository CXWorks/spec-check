pub open spec fn performance_limits_get__3_5_6_10_spec(result: Int32, range_max: UInt32, range_min: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(domain_id(old_s)) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> (range_max == CurrentPerformanceLimitMax(domain_id(old_s)) && range_min == CurrentPerformanceLimitMin(domain_id(old_s))))
    && (ResultEqual(result, SUCCESS) ==> new_s == old_s)
}