pub open spec fn sensor_reading_complete_spec(result: RsiCommandReturnCode, sensor_id: UInt32, readings: [UInt64], old_s: S, new_s: S) -> bool {
    (result == RSI_ERROR_INPUT ==> sensor_id == 0)
    && (result == RSI_ERROR_STATE ==> sensor_id == 0)
    && (result == RSI_INCOMPLETE ==> sensor_id == 0)
    && (result == RSI_ERROR_UNKNOWN ==> sensor_id == 0)
    && (result == RSI_SUCCESS ==> sensor_id != 0)
    && (result == RSI_SUCCESS ==> readings.len() > 0)
    && (result == RSI_SUCCESS ==> old_s.sensor_readings == new_s.sensor_readings)
    && (result == RSI_SUCCESS ==> old_s.sensor_id == new_s.sensor_id)
}