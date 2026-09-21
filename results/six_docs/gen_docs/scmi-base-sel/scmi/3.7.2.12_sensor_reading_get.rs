pub open spec fn sensor_reading_get_spec(result: int, sensor_id: UInt32, flags: UInt32, old_s: S, new_s: S) -> bool {
    (flags & 1 == 1 ==> result == SCMI_SUCCESS)
    && (flags & 1 == 0 ==> (result == SCMI_SUCCESS || result == SCMI_NOT_FOUND || result == SCMI_INVALID_PARAMETERS || result == SCMI_PROTOCOL_ERROR))
    && (result == SCMI_NOT_FOUND ==> sensor_id < 0)
    && (result == SCMI_INVALID_PARAMETERS ==> (flags & 1 != 0))
    && (result == SCMI_PROTOCOL_ERROR ==> (flags & 1 == 0))
    && (result == SCMI_SUCCESS ==> (new_s.sensor_readings != old_s.sensor_readings || new_s.sensor_readings == old_s.sensor_readings))
}