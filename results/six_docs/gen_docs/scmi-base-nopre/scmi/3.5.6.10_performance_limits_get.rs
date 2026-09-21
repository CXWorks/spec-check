pub open spec fn 3.5.6.10_performance_limits_get_spec(result: RsiCommandReturnCode, range_max: UInt32, range_min: UInt32, old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> (domain_id(old_s) < 0))
    && (result == RSI_ERROR_STATE ==> (domain_id(old_s) >= old_s.num_domains))
    && (result == RSI_SUCCESS ==> (range_min <= range_max))
    && (result == RSI_SUCCESS ==> (range_min >= 0))
    && (result == RSI_SUCCESS ==> (range_max < old_s.num_performance_levels))
    && (result == RSI_SUCCESS ==> (new_s.performance_limits[domain_id(old_s)].min == range_min))
    && (result == RSI_SUCCESS ==> (new_s.performance_limits[domain_id(old_s)].max == range_max))
    && (result == RSI_SUCCESS ==> (new_s.performance_limits[domain_id(old_s)].min == old_s.performance_limits[domain_id(old_s)].min))
    && (result == RSI_SUCCESS ==> (new_s.performance_limits[domain_id(old_s)].max == old_s.performance_limits[domain_id(old_s)].max))
}