pub open spec fn sensor_reading_complete_spec(result: int32, sensor_id: uint32, readings: array<SENSOR_READING>, old_s: S, new_s: S) -> bool {
    (result == SCMI_HARDWARE_ERROR ==> <true>)
    && (result == SCMI_SUCCESS ==> <true>)
    && (result == SCMI_INVALID_PARAMETERS ==> <true>)
    && (result == SCMI_NOT_FOUND ==> <true>)
    && (result == SCMI_NOT_SUPPORTED ==> <true>)
    && (result == SCMI_OUT_OF_RANGE ==> <true>)
    && (result == SCMI_PROTOCOL_ERROR ==> <true>)
    && (result == SCMI_COMMS_ERROR ==> <true>)
    && (result == SCMI_DENIED ==> <true>)
    && (result == SCMI_GENERIC_ERROR ==> <true>)
    && (result == SCMI_IN_USE ==> <true>)
    && (result == SCMI_PARTIAL_ERROR ==> <true>)
}