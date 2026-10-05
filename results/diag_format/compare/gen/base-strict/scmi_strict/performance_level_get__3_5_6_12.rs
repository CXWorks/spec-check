pub open spec fn performance_level_get__3_5_6_12_spec(result: Int32, performance_level: UInt32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (ResultEqual(result, SUCCESS) ==> performance_level == CurrentPerformanceLevelOrIndex(domain_id))
    && (old_s == new_s)
}