pub open spec fn 3.3.2.6_power_state_set_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    || (result == SCMI_NOT_FOUND)
    || (result == SCMI_INVALID_PARAMETERS)
    || (result == SCMI_NOT_SUPPORTED)
    || (result == SCMI_DENIED)
    || (result == SCMI_IN_USE)
    || (result == SCMI_PROTOCOL_ERROR)
    || (result == SCMI_HARDWARE_ERROR)
    || (result == SCMI_COMMS_ERROR)
    || (result == SCMI_GENERIC_ERROR)
    || (result == SCMI_OUT_OF_RANGE)
    || (result == SCMI_PARTIAL_ERROR)
}