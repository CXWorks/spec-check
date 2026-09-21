pub open spec fn 3.3.2.9_power_state_change_requested_notify_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_reserved != 0))
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_domain_id points to invalid domain))
    && (result == SCMI_SUCCESS ==> true)
}