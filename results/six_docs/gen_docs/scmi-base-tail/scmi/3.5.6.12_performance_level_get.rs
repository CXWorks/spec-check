pub open spec fn 3.5.6.12_performance_level_get_spec(result: int32, performance_level: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_SUCCESS ==> performance_level == performance_level)
}