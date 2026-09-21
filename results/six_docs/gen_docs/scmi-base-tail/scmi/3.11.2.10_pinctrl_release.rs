pub open spec fn pinctrl_release_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags != 0 || (old_s.selector != 0 && old_s.selector != 1)))
    && (result == SCMI_NOT_FOUND ==> (old_s.identifier is invalid pin or group))
    && (result == SCMI_SUCCESS ==> (old_s.pin_state[old_s.identifier] == PIN_CONTROLLED && new_s.pin_state[old_s.identifier] == PIN_FREE))
    && (result == SCMI_SUCCESS ==> (old_s.group_state[old_s.identifier] == PIN_CONTROLLED && new_s.group_state[old_s.identifier] == PIN_FREE))
    && (result != SCMI_SUCCESS && result != SCMI_NOT_FOUND && result != SCMI_INVALID_PARAMETERS ==> (result == SCMI_BUSY || result == SCMI_COMMS_ERROR || result == SCMI_DENIED || result == SCMI_GENERIC_ERROR || result == SCMI_HARDWARE_ERROR || result == SCMI_IN_USE || result == SCMI_NOT_SUPPORTED || result == SCMI_OUT_OF_RANGE || result == SCMI_PARTIAL_ERROR || result == SCMI_PROTOCOL_ERROR))
}