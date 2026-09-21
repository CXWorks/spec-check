pub open spec fn 3.4.2.7_system_power_state_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_notify_enable != 0 && old_s.cmd_input_notify_enable != 1))
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_SUCCESS ==> (old_s.cmd_input_notify_enable == 0 || old_s.cmd_input_notify_enable == 1))
    && (result != SCMI_INVALID_PARAMETERS && result != SCMI_NOT_SUPPORTED && result != SCMI_SUCCESS ==> true)
}