pub open spec fn performance_level_get__3_5_6_12_spec(status: int, performance_level: uint32, domain_id: uint32, old_s: S, new_s: S) -> bool {
    (!IsValidPerformanceDomain(domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (ResultEqual(status, SUCCESS) ==> performance_level == CurrentPerformanceLevel(domain_id))
}