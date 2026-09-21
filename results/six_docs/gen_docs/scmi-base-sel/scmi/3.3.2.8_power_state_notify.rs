pub open spec fn 3.3.2.8_power_state_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    ((result == SCMI_INVALID_PARAMETERS) ==> (old_s.cmd_input_notify_enable & 0x1 != 0 || (old_s.cmd_input_notify_enable & 0xFFFFFFFE) != 0))
    && ((result == SCMI_NOT_FOUND) ==> (old_s.cmd_input_domain_id is invalid domain))
    && ((result == SCMI_SUCCESS) ==> (old_s.cmd_input_notify_enable & 0x1 == 0 || old_s.cmd_input_notify_enable & 0x1 == 1))
    && true
}