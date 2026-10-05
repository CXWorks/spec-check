pub open spec fn performance_limits_get__3_5_6_10_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!old_s.performance_domains.contains(&old_s.performance_domains[old_s.domain_id]) ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (new_s.performance_limits_range_min == old_s.performance_limits_range_min) && (new_s.performance_limits_range_max == old_s.performance_limits_range_max))
}