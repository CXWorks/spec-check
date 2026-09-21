pub open spec fn 3.3.2.8_power_state_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s as int) >= 0)
    && (result == SCMI_NOT_FOUND ==> (old_s as int) >= 0)
    && (result == SCMI_SUCCESS ==> (old_s as int) >= 0)
    && true
}