pub open spec fn 3.2.2.10_base_notify_errors_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_INVALID_PARAMETERS ==> (old_s.notify_enable == 0 || old_s.notify_enable == 1))
}