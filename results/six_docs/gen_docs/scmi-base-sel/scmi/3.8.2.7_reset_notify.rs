pub open spec fn 3.8.2.7_reset_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND ==> (old_s.domain_id as int) < 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.notify_enable as int) < 0)
}