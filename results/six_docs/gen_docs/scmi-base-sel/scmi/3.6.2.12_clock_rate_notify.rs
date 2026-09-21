pub open spec fn clock_rate_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_notify_enable & 0x1 != 0 || old_s.cmd_input_notify_enable != 0))
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_clock_id is invalid clock id))
    && (result == SCMI_SUCCESS ==> (old_s.cmd_input_notify_enable & 0x1 == 1))
    && (result == SCMI_SUCCESS ==> (new_s.cmd_input_notify_enable == old_s.cmd_input_notify_enable))
    && (result == SCMI_SUCCESS ==> (new_s.cmd_input_clock_id == old_s.cmd_input_clock_id))
}