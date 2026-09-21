pub open spec fn reduce_sustained_performance_level_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_domain_id as int) < 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_sustained_level as int) < 0)
    && (result == SCMI_NOT_FOUND ==> !old_s.domain_exists(old_s.cmd_input_domain_id))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> (old_s.cmd_input_sustained_level as int) > old_s.max_sustained_level(old_s.cmd_input_domain_id))
    && (result == SCMI_SUCCESS ==> true)
    && (result == SCMI_SUCCESS ==> old_s == new_s)
}