pub open spec fn 3.5.6.12_performance_level_get_spec(result: int32, performance_level: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> performance_level == old_s.performance_level_get(old_s, result))
    && (result == SCMI_NOT_FOUND ==> old_s.performance_level_get(old_s, result) == 0)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> old_s.performance_level_get(old_s, result) == 0)
    && (result == SCMI_SUCCESS ==> new_s.performance_level_get(new_s, result) == old_s.performance_level_get(old_s, result))
    && (result == SCMI_NOT_FOUND ==> new_s.performance_level_get(new_s, result) == 0)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> new_s.performance_level_get(new_s, result) == 0)
}