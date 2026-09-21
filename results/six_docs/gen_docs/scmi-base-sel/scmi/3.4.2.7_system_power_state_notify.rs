pub open spec fn 3.4.2.7_system_power_state_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
}