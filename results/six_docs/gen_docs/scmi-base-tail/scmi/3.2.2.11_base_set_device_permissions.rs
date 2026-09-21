pub open spec fn base_set_device_permissions_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_flags & 1 != 0 || old_s.cmd_input_flags != 0))
    && (result == SCMI_NOT_FOUND ==> (true))
    && (result == SCMI_NOT_SUPPORTED ==> (true))
    && (result == SCMI_DENIED ==> (true))
    && (result == SCMI_SUCCESS ==> (true))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_NOT_SUPPORTED && result != SCMI_DENIED && result != SCMI_INVALID_PARAMETERS ==> (true))
}