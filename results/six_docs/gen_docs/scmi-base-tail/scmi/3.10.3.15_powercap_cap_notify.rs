pub open spec fn 3.10.3.15_powercap_cap_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_notify_enable & 0xFFFFFFFE) != 0)
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_domain_id as int) < 0)
    && (result == SCMI_SUCCESS ==> (old_s.cmd_input_notify_enable & 1) == 1)
    && (result == SCMI_SUCCESS ==> (old_s.cmd_input_domain_id as int) >= 0)
    && (result == SCMI_SUCCESS ==> (old_s.cmd_input_notify_enable & 0xFFFFFFFE) == 0)
}