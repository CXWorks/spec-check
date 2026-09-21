pub open spec fn pinctrl_request_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.cmd_input_flags & 0x3 != 0))
    && (result == SCMI_NOT_FOUND ==> (old_s.cmd_input_identifier is invalid pin or group))
    && (result == SCMI_DENIED ==> (old_s.cmd_input_identifier is not allowed for current agent))
    && (result == SCMI_IN_USE ==> (old_s.cmd_input_identifier is currently under exclusive control))
    && (result == SCMI_SUCCESS ==> (new_s has exclusive control of old_s.cmd_input_identifier))
}