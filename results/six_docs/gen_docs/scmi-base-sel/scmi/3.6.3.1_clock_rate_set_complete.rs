pub open spec fn clock_rate_set_complete_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS)
    && (result == SCMI_DENIED)
    && (result == SCMI_COMMS_ERROR)
    && (result == SCMI_GENERIC_ERROR)
    && (result == SCMI_HARDWARE_ERROR)
    && (result == SCMI_INVALID_PARAMETERS)
    && (result == SCMI_IN_USE)
    && (result == SCMI_NOT_FOUND)
    && (result == SCMI_NOT_SUPPORTED)
    && (result == SCMI_OUT_OF_RANGE)
    && (result == SCMI_PARTIAL_ERROR)
    && (result == SCMI_PROTOCOL_ERROR)
}