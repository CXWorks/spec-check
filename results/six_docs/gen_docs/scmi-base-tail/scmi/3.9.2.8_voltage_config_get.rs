pub open spec fn voltage_config_get_spec(result: int32, config: uint32, old_s: S, new_s: S) -> bool {
    (result == SCMI_SUCCESS ==> config == 0)
    && (result == SCMI_SUCCESS ==> (config & 0xF) == (config & 0xF))
    && (result == SCMI_NOT_FOUND ==> true)
    && (result == SCMI_NOT_SUPPORTED ==> true)
    && (result == SCMI_DENIED ==> true)
    && (result == SCMI_INVALID_PARAMETERS ==> true)
    && (result == SCMI_OUT_OF_RANGE ==> true)
    && (result == SCMI_HARDWARE_ERROR ==> true)
    && (result == SCMI_PROTOCOL_ERROR ==> true)
    && (result == SCMI_COMMS_ERROR ==> true)
    && (result == SCMI_BUSY ==> true)
    && (result == SCMI_PARTIAL_ERROR ==> true)
}