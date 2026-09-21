pub open spec fn 3.3.2.9_power_state_change_requested_notify_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_NOT_FOUND ==> (old_s.domain_id != new_s.domain_id))
}