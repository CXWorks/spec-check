pub open spec fn sensor_reading_get__3_7_2_12_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidReadingFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!SensorIsEnabled(old_s, sensor_id) ==> ResultEqual(result, PROTOCOL_ERROR))
    && (flags.async == 0 && SensorHasHardwareFault(old_s, sensor_id) ==> ResultEqual(result, HARDWARE_ERROR))
    && (ResultEqual(result, SUCCESS) ==> (
        (flags.async == 0 ==> (
            readings == CurrentSensorReadings(old_s, sensor_id)
            && N == (SensorIsAxial(old_s, sensor_id) ? SensorNumAxes(old_s, sensor_id) : 1)
            && (SensorCollectsTimestamp(old_s, sensor_id) ==> (
                readings[i].timestamp_low != 0 || readings[i].timestamp_high != 0
            ))
            && (SensorCollectsTimestamp(old_s, sensor_id) == false ==> (
                readings[i].timestamp_low == 0 && readings[i].timestamp_high == 0
            ))
        ))
        || (flags.async == 1 ==> AsyncReadingRequestEnqueued(old_s, sensor_id))
    ))
}