pub open spec fn performance_limits_get__3_5_6_10_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, domain_id: u32, range_max: u32, range_min: u32) -> bool {
    (!IsValidPerformanceDomain(domain_id) ==> ResultEqual(result, RMI_ERROR_REALM))
    && (result.is_Ok() ==> (range_max == PerformanceDomain(domain_id).limits.max && range_min == PerformanceDomain(domain_id).limits.min))
}