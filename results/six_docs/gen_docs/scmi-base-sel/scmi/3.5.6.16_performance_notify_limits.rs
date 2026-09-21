pub open spec fn 3.5.6.16_performance_notify_limits_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND ==> (old_s.domain_id as int) < 0)
    && (result == SCMI_NOT_SUPPORTED ==> (old_s.domain_id as int) >= 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.notify_enable as int) != 0)
}