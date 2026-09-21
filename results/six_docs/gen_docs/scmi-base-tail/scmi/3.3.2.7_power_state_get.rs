pub open spec fn 3.3.2.7_power_state_get_spec(result: int32, power_state: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> power_state == 0)
    && (result == SCMI_NOT_FOUND ==> power_state == 0)
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND ==> power_state == 0)
}