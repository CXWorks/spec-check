pub open spec fn pinctrl_request_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SCMI_INVALID_PARAMETERS ==> (old_s.flags_bits_1_0 == 0 || old_s.flags_bits_1_0 > 1))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_IN_USE ==> true)
    && (result == SCMI_SUCCESS ==> true)
    && (result == SCMI_BUSY ==> true)
    && (result == SCMI_COMMS_ERROR ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
}