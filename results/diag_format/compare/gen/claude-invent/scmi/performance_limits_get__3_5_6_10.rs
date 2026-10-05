pub open spec fn performance_limits_get__3_5_6_10_spec(status: i32, range_max: u32, range_min: u32, old_s: S, new_s: S, domain_id: u32) -> bool {
    (!IsValidPerformanceDomain(old_s, domain_id) ==> status == NOT_FOUND)
    && (status == SUCCESS ==> (
        IsValidPerformanceDomain(old_s, domain_id)
        && range_max == PerformanceLimitMax(old_s, domain_id)
        && range_min == PerformanceLimitMin(old_s, domain_id)
    ))
    && (IsValidPerformanceDomain(old_s, domain_id) ==> status != NOT_FOUND)
    && (new_s == old_s)
}
