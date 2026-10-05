pub open spec fn sensor_reading_get__3_7_2_12_spec(result: Int32, readings: Array<SENSOR_READING>, old_s: S, new_s: S) -> bool {
    (!SensorExists(old_s, sensor_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidReadingFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!SensorIsEnabled(old_s, sensor_id) ==> ResultEqual(result, PROTOCOL_ERROR))
    && (Bits(old_s, flags, 0, 0) == 0 && SensorHardwareFault(old_s, sensor_id) ==> ResultEqual(result, HARDWARE_ERROR))
    && ResultEqual(result, SUCCESS)
    && (Bits(old_s, flags, 0, 0) == 0 ==> ReadingsMatchCurrentSensorValue(readings, sensor_id))
    && (Bits(old_s, flags, 0, 0) == 1 ==> SensorReadingCompleteEnqueued(old_s, sensor_id))
    && (SensorIsScalar(old_s, sensor_id) ==> ReadingCount(readings) == 1)
    && (!SensorIsScalar(old_s, sensor_id) ==> ReadingCount(readings) == SensorAxisCount(old_s, sensor_id))
    && (!SensorIsScalar(old_s, sensor_id) ==> ReadingsInAxisOrder(readings, sensor_id))
    && (forall|i: UInt32| (i < ReadingCount(readings) && !TimestampCollected(old_s, sensor_id)) ==> (readings[i].timestamp_low == 0 && readings[i].timestamp_high == 0))
    && (SensorAt(old_s, sensor_id).pending_reading_complete == (Bits(old_s, flags, 0, 0) == 1))
}