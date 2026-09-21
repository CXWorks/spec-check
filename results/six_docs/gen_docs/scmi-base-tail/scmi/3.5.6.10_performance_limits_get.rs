pub open spec fn 3.5.6.10_performance_limits_get_spec(result: int32, range_max: uint32, range_min: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> (range_max >= range_min))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result != SCMI_SUCCESS ==> true)
}